// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Reserved account-stream lifecycle controls retain native mutation evidence.

#[cfg(test)]
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;

use binance_client::{
    Credentials, Error, ExitReserve, Outcome, RequestId, WeightPools, coinm, spot, usdm,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};
use tokio::{
    net::{TcpListener, TcpStream},
    time::Instant,
};
use tokio_websockets::{Limits, Message, ServerBuilder, WebSocketStream};

fn refused(error: &Error) {
    assert!(matches!(error, Error::Admission { .. }), "{error:?}");
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
}

#[cfg(test)]
async fn accept(listener: TcpListener) -> WebSocketStream<TcpStream> {
    let (stream, _) = listener.accept().await.unwrap();
    ServerBuilder::new()
        .limits(Limits::unlimited())
        .accept(stream)
        .await
        .unwrap()
        .1
}

#[cfg(test)]
async fn finish(mut peer: WebSocketStream<TcpStream>) {
    while let Some(message) = peer.next().await {
        let message = message.unwrap();
        if message.is_close() {
            peer.flush().await.unwrap();
            break;
        }
        assert!(!message.is_text(), "a refused request reached the wire");
    }
}

#[cfg(test)]
fn spot_config(url: &str) -> spot::Config {
    spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .websocket_url(url)
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap())
}

macro_rules! listen_key_lifecycle {
    ($name:ident, $product:ident, $path:literal) => {
        #[tokio::test]
        async fn $name() {
            let mut fixture = HttpFixture::new(
                200,
                "",
                "{\"listenKey\":\"synthetic-listen-key\"}",
                None,
                false,
            )
            .await;
            let config =
                $product::Config::with_pools($product::Environment::Demo, &WeightPools::new())
                    .unwrap()
                    .rest_url(&fixture.url)
                    .unwrap()
                    .credentials(
                        Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
                    )
                    .clock(Arc::new(FixedClock(1_700_000_001_000)));
            config
                .set_exit_reserve(ExitReserve::default().request_weight(10_000))
                .unwrap();
            let observer = config.clone();
            let client = $product::RestClient::new(config).unwrap();
            let start = $product::rest_requests::StartUserDataStream::new();
            let renew = $product::rest_requests::KeepaliveUserDataStream::new();
            let close = $product::rest_requests::CloseUserDataStream::new();
            refused(
                &client
                    .start_user_data_stream(&start, deadline())
                    .await
                    .unwrap_err(),
            );
            refused(
                &client
                    .keepalive_user_data_stream(&renew, deadline())
                    .await
                    .unwrap_err(),
            );
            refused(
                &client
                    .close_user_data_stream(&close, deadline())
                    .await
                    .unwrap_err(),
            );
            let started = client
                .start_user_data_stream_for_recovery(&start, deadline())
                .await
                .unwrap();
            assert_eq!(
                started.data.listen_key.as_ref().unwrap().as_str(),
                "synthetic-listen-key"
            );
            assert_eq!(observer.pool_usage().unwrap().request_weight.used, 1);
            client
                .keepalive_user_data_stream_for_recovery(&renew, deadline())
                .await
                .unwrap();
            assert_eq!(observer.pool_usage().unwrap().request_weight.used, 2);
            client
                .close_user_data_stream_for_recovery(&close, deadline())
                .await
                .unwrap();
            assert_eq!(observer.pool_usage().unwrap().request_weight.used, 3);
            assert_eq!(observer.pool_usage().unwrap().request_weight.limit, 2400);
            refused(
                &client
                    .start_user_data_stream(&start, deadline())
                    .await
                    .unwrap_err(),
            );
            assert_eq!(fixture.connections_accepted(), 3);
            for method in ["POST", "PUT", "DELETE"] {
                let wire = fixture.requests.recv().await.unwrap();
                assert!(wire.starts_with(&format!("{method} {}", $path)), "{wire}");
            }
            fixture.finish().await;
        }
    };
}

listen_key_lifecycle!(
    usdm_start_early_renewal_and_teardown_use_native_reserved_weight,
    usdm,
    "/fapi/v1/listenKey"
);
listen_key_lifecycle!(
    coinm_start_early_renewal_and_teardown_use_native_reserved_weight,
    coinm,
    "/dapi/v1/listenKey"
);

macro_rules! refused_controls {
    ($name:ident, $product:ident) => {
        #[tokio::test]
        async fn $name() {
            for (status, headers) in [(429, "Retry-After: 2\r\n"), (418, ""), (500, "")] {
                let fixture = HttpFixture::new(
                    status,
                    headers,
                    "{\"code\":-1003,\"msg\":\"synthetic control refusal\"}",
                    None,
                    false,
                )
                .await;
                let budgets = binance_client::Budgets::new(
                    binance_client::BudgetLimits::$product().weight_per_minute(1),
                )
                .unwrap();
                let config =
                    $product::Config::with_pools($product::Environment::Demo, &WeightPools::new())
                        .unwrap()
                        .rest_url(&fixture.url)
                        .unwrap()
                        .credentials(
                            Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
                        )
                        .clock(Arc::new(FixedClock(1_700_000_001_000)))
                        .budgets(budgets);
                config
                    .set_exit_reserve(ExitReserve::default().request_weight(1))
                    .unwrap();
                let client = $product::RestClient::new(config).unwrap();
                let request = $product::rest_requests::StartUserDataStream::new();
                assert!(matches!(
                    client
                        .start_user_data_stream_for_recovery(&request, Instant::now())
                        .await,
                    Err(Error::Expired(_))
                ));
                assert_eq!(fixture.connections_accepted(), 0);
                let first = client
                    .start_user_data_stream_for_recovery(&request, deadline())
                    .await
                    .unwrap_err();
                assert_eq!(first.outcome(), Some(Outcome::Unknown), "{first:?}");
                let second = client
                    .start_user_data_stream_for_recovery(&request, deadline())
                    .await
                    .unwrap_err();
                if status == 418 {
                    assert!(matches!(second, Error::CooldownTimingUnknown), "{second:?}");
                    assert_eq!(second.outcome(), Some(Outcome::NotSent));
                } else {
                    refused(&second);
                }
                assert_eq!(fixture.connections_accepted(), 1);
                fixture.finish().await;
            }
        }
    };
}

refused_controls!(
    usdm_stream_controls_preserve_deadlines_native_exhaustion_cooldowns_and_uncertainty,
    usdm
);
refused_controls!(
    coinm_stream_controls_preserve_deadlines_native_exhaustion_cooldowns_and_uncertainty,
    coinm
);

#[tokio::test]
async fn spot_handshake_and_signed_stream_controls_use_only_their_native_costs() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let mut peer = accept(listener).await;
        for (method, result) in [
            (
                "userDataStream.subscribe.signature",
                json!({"subscriptionId":7}),
            ),
            ("userDataStream.unsubscribe", json!({})),
        ] {
            let request: Value =
                serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
            assert_eq!(request["method"], method);
            if method == "userDataStream.subscribe.signature" {
                assert_eq!(request["params"]["apiKey"], "synthetic-api-key");
                assert!(request["params"]["signature"].is_string());
            } else {
                assert_eq!(request["params"]["subscriptionId"], 7);
            }
            peer.send(Message::text(
                json!({"id":request["id"], "status":200, "result":result}).to_string(),
            ))
            .await
            .unwrap();
        }
        finish(peer).await;
    });
    let config = spot_config(&url);
    config
        .set_exit_reserve(ExitReserve::default().request_weight(10_000))
        .unwrap();
    let observer = config.clone();
    let ordinary = spot::WsClient::connect(config.clone()).await;
    assert!(matches!(ordinary, Err(Error::Admission { .. })));
    let (client, mut events, driver) = spot::WsClient::connect_for_recovery(config).await.unwrap();
    let driver = tokio::spawn(driver.run());
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 2);
    let subscribe = spot::ws_requests::UserDataStreamSubscribeSignature::new();
    refused(
        &client
            .user_data_stream_subscribe_signature(
                &subscribe,
                RequestId::new("ordinary").unwrap(),
                deadline(),
            )
            .await
            .unwrap_err(),
    );
    assert_eq!(
        client
            .user_data_stream_subscribe_signature_for_recovery(
                &subscribe,
                RequestId::new("subscribe").unwrap(),
                deadline()
            )
            .await
            .unwrap()
            .data
            .subscription_id,
        7
    );
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 4);
    let unsubscribe = spot::ws_requests::UserDataStreamUnsubscribe::new().subscription_id(7);
    refused(
        &client
            .user_data_stream_unsubscribe(
                &unsubscribe,
                RequestId::new("ordinary-end").unwrap(),
                deadline(),
            )
            .await
            .unwrap_err(),
    );
    client
        .user_data_stream_unsubscribe_for_recovery(
            &unsubscribe,
            RequestId::new("unsubscribe").unwrap(),
            deadline(),
        )
        .await
        .unwrap();
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 6);
    refused(
        &client
            .time(
                &spot::ws_requests::Time::new(),
                RequestId::new("ordinary-time").unwrap(),
                deadline(),
            )
            .await
            .unwrap_err(),
    );
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn spot_recovery_subscription_retains_deadline_exhaustion_cooldown_and_mutation_evidence() {
    for status in [429, 418, 500] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("ws://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let mut peer = accept(listener).await;
            let request: Value =
                serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
            assert_eq!(request["method"], "userDataStream.subscribe.signature");
            let mut response = json!({"id":request["id"], "status":status, "error":{"code":-1003,"msg":"synthetic control failure"}});
            if status == 429 {
                response["retryAfter"] = json!(1_700_000_003_000_u64);
            }
            peer.send(Message::text(response.to_string()))
                .await
                .unwrap();
            finish(peer).await;
        });
        let budgets =
            binance_client::Budgets::new(binance_client::BudgetLimits::spot().weight_per_minute(4))
                .unwrap();
        let config = spot_config(&url).budgets(budgets);
        config
            .set_exit_reserve(ExitReserve::default().request_weight(4))
            .unwrap();
        let observer = config.clone();
        let (client, mut events, driver) =
            spot::WsClient::connect_for_recovery(config).await.unwrap();
        let driver = tokio::spawn(driver.run());
        let request = spot::ws_requests::UserDataStreamSubscribeSignature::new();
        let expired = client
            .user_data_stream_subscribe_signature_for_recovery(
                &request,
                RequestId::new("expired").unwrap(),
                Instant::now(),
            )
            .await
            .unwrap_err();
        assert!(matches!(expired, Error::Expired(_)), "{expired:?}");
        assert_eq!(observer.pool_usage().unwrap().request_weight.used, 2);
        let failure = client
            .user_data_stream_subscribe_signature_for_recovery(
                &request,
                RequestId::new("failed").unwrap(),
                deadline(),
            )
            .await
            .unwrap_err();
        assert_eq!(failure.outcome(), Some(Outcome::Unknown), "{failure:?}");
        assert_eq!(observer.pool_usage().unwrap().request_weight.used, 4);
        let blocked = client
            .user_data_stream_subscribe_signature_for_recovery(
                &request,
                RequestId::new("blocked").unwrap(),
                deadline(),
            )
            .await
            .unwrap_err();
        if status == 418 {
            assert!(
                matches!(blocked, Error::CooldownTimingUnknown),
                "{blocked:?}"
            );
        } else {
            refused(&blocked);
        }
        client.close().await.unwrap();
        while events.recv().await.is_some() {}
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
}
