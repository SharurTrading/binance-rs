// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Upgrade failures retain safe HTTP evidence and govern the next native attempt.

#[cfg(test)]
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;

use binance_client::{Error, ExitReserve, Outcome, TransportFailure, WeightPools, spot};
use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use support::{FixedClock, HttpFixture};
use tokio::net::TcpListener;
use tokio_websockets::{Limits, Message, ServerBuilder};

#[cfg(test)]
fn config(url: &str, recovery: bool) -> spot::Config {
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .websocket_url(url)
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)));
    if recovery {
        config
            .set_exit_reserve(ExitReserve::default().request_weight(10_000))
            .unwrap();
    }
    config
}

#[cfg(test)]
async fn connect(
    config: spot::Config,
    recovery: bool,
) -> Result<(spot::WsClient, spot::ApiEvents, spot::ConnectionDriver), Error> {
    if recovery {
        spot::WsClient::connect_for_recovery(config).await
    } else {
        spot::WsClient::connect(config).await
    }
}

#[cfg(test)]
fn received(error: &Error, status: u16, cause: TransportFailure) {
    assert_eq!(error.outcome(), Some(Outcome::ReadFailed));
    let Error::Transport {
        operation: "connect",
        failure: Some(actual),
        meta: Some(meta),
        ..
    } = error
    else {
        panic!("missing typed upgrade evidence: {error:?}");
    };
    assert_eq!(*actual, cause);
    assert_eq!(meta.status, status);
    assert_eq!(meta.operation, "connect");
    assert!(meta.client_order_ids.is_empty());
    assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 17);
    assert_eq!(meta.rates.counters["x-mbx-used-weight-9m"], 9);
    let safe = format!("{error:?}");
    assert!(!safe.contains("synthetic-secret"));
    assert!(!safe.contains("listenKey="));
    assert!(!safe.contains("127.0.0.1"));
}

macro_rules! rejected_handshake {
    ($name:ident, $recovery:literal, $status:literal, $retry:literal) => {
        #[tokio::test]
        async fn $name() {
            let fixture = HttpFixture::new($status, concat!($retry, "X-MBX-USED-WEIGHT-1M: 17\r\nX-MBX-USED-WEIGHT-9M: 9\r\nSet-Cookie: synthetic-secret-header\r\n"), "synthetic-secret-body\r\nRetry-After: 9999\r\n", None, false).await;
            let url = format!("{}/private/ws/synthetic-secret-key", fixture.url.replacen("http://", "ws://", 1));
            let config = config(&url, $recovery);
            let observer = config.clone();
            let error = connect(config.clone(), $recovery).await.err().unwrap();
            let second = connect(config, $recovery).await.err().unwrap();
            assert_eq!(fixture.connections_accepted(), 1);
            if $status == 429 {
                assert!(matches!(second, Error::Admission { retry_after } if retry_after == Duration::from_secs(2)), "{second:?}");
            } else {
                assert!(matches!(second, Error::CooldownTimingUnknown), "{second:?}");
            }
            assert_eq!(second.outcome(), Some(Outcome::NotSent));
            received(&error, $status, TransportFailure::WebSocketHandshakeRejected);
            let Error::Transport { meta: Some(meta), .. } = &error else { panic!("metadata disappeared") };
            assert_eq!(meta.rates.retry_after, if $status == 429 { Some(Duration::from_secs(2)) } else { None });
            assert_eq!(observer.pool_usage().unwrap().request_weight.used, 17);
            assert_eq!(fixture.connections_accepted(), 1);
            fixture.finish().await;
        }
    };
}

rejected_handshake!(
    ordinary_429_upgrade_cooldown_refuses_a_second_attempt,
    false,
    429,
    "Retry-After: 2\r\n"
);
rejected_handshake!(
    recovery_429_upgrade_cooldown_refuses_a_second_attempt,
    true,
    429,
    "Retry-After: 2\r\n"
);
rejected_handshake!(
    ordinary_untimed_418_upgrade_refuses_a_second_attempt,
    false,
    418,
    ""
);
rejected_handshake!(
    recovery_untimed_418_upgrade_refuses_a_second_attempt,
    true,
    418,
    ""
);

#[tokio::test]
async fn ordinary_and_recovery_invalid_accept_retain_native_validation_and_received_headers() {
    for recovery in [false, true] {
        let fixture = HttpFixture::new(101, "Upgrade: websocket\r\nSec-WebSocket-Accept: invalid-synthetic-secret\r\nX-MBX-USED-WEIGHT-1M: 17\r\nX-MBX-USED-WEIGHT-9M: 9\r\n", "", None, false).await;
        let url = fixture.url.replacen("http://", "ws://", 1);
        let error = connect(config(&url, recovery), recovery)
            .await
            .err()
            .unwrap();
        received(&error, 101, TransportFailure::WebSocketHandshakeInvalid);
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn ordinary_and_recovery_successful_upgrade_preserve_frames_and_observe_only_safe_headers() {
    for recovery in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("ws://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let (_, mut peer) = ServerBuilder::new()
                .limits(Limits::unlimited())
                .add_header(
                    "x-mbx-used-weight-1m".parse().unwrap(),
                    "17".parse().unwrap(),
                )
                .unwrap()
                .add_header(
                    "set-cookie".parse().unwrap(),
                    "synthetic-secret".parse().unwrap(),
                )
                .unwrap()
                .accept(stream)
                .await
                .unwrap();
            peer.send(Message::text(r#"{"event":{"e":"serverShutdown","E":1}}"#))
                .await
                .unwrap();
            while let Some(message) = peer.next().await {
                if message.unwrap().is_close() {
                    peer.flush().await.unwrap();
                    break;
                }
            }
        });
        let owner = config(&url, recovery);
        let observer = owner.clone();
        let (client, mut events, driver) = connect(owner, recovery).await.unwrap();
        let driver = tokio::spawn(driver.run());
        assert!(matches!(
            events.recv().await,
            Some(spot::ApiEvent::Established(_))
        ));
        assert!(matches!(
            events.recv().await,
            Some(spot::ApiEvent::ServerShutdown { .. })
        ));
        assert_eq!(observer.pool_usage().unwrap().request_weight.used, 17);
        client.close().await.unwrap();
        while events.recv().await.is_some() {}
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
}

#[tokio::test]
async fn an_unanswered_upgrade_times_out_once_on_the_owned_attempt_deadline() {
    for recovery in [false, true] {
        let mut fixture = HttpFixture::hold_headers().await;
        let url = fixture.url.replacen("http://", "ws://", 1);
        let owner = config(&url, recovery)
            .timeout(Duration::from_secs(5))
            .unwrap();
        let observer = owner.clone();
        let attempt = tokio::spawn(async move { connect(owner, recovery).await });
        fixture.requests.recv().await.unwrap();
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(5)).await;
        let error = attempt.await.unwrap().err().unwrap();
        tokio::time::resume();
        assert!(
            matches!(
                error,
                Error::Transport {
                    outcome: Outcome::ReadFailed,
                    failure: Some(TransportFailure::ResponseHeadersTimeout),
                    meta: None,
                    ..
                }
            ),
            "{error:?}"
        );
        assert_eq!(observer.pool_usage().unwrap().request_weight.used, 2);
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}
