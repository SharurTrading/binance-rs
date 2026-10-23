// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! A lost receipt clock holds shared admission without dating received quota evidence.

#[cfg(test)]
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;

#[cfg(test)]
mod tests {
    use crate::support::deadline;
    use binance_client::{
        ClientOrderId, Clock, Credentials, Decimal, Error, ExitReserve, Outcome, Symbol,
        TransportFailure, WeightPools, spot, usdm,
    };
    use futures_util::{SinkExt, StreamExt};
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    };
    use std::time::Duration;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
        task::JoinHandle,
    };
    use tokio_util::sync::CancellationToken;
    use tokio_websockets::{Limits, ServerBuilder};

    const SENT: u64 = 1_700_000_039_000;
    const RECEIVED: u64 = 1_700_000_041_000;

    struct ReceiptClock {
        now: AtomicU64,
        fail_once: AtomicBool,
    }

    impl ReceiptClock {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                now: AtomicU64::new(SENT),
                fail_once: AtomicBool::new(false),
            })
        }

        fn fail_receipt(&self) {
            self.now.store(RECEIVED, Ordering::SeqCst);
            self.fail_once.store(true, Ordering::SeqCst);
        }
    }

    impl Clock for ReceiptClock {
        fn now_millis(&self) -> Result<u64, Error> {
            if self.fail_once.swap(false, Ordering::SeqCst) {
                Err(Error::Configuration("synthetic receipt clock unavailable"))
            } else {
                Ok(self.now.load(Ordering::SeqCst))
            }
        }
    }

    #[derive(Clone, Copy)]
    enum Reply {
        Http {
            status: u16,
            body: &'static str,
            retry: &'static str,
        },
        Upgrade,
    }

    struct Venue {
        url: String,
        count: Arc<AtomicUsize>,
        stop: CancellationToken,
        task: JoinHandle<()>,
    }

    impl Venue {
        async fn new(clock: Arc<ReceiptClock>, reply: Reply) -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url = format!("http://{}", listener.local_addr().unwrap());
            let count = Arc::new(AtomicUsize::new(0));
            let accepted = Arc::clone(&count);
            let stop = CancellationToken::new();
            let cancelled = stop.clone();
            let task = tokio::spawn(async move {
                loop {
                    let (mut stream, _) = tokio::select! {
                        () = cancelled.cancelled() => break,
                        connection = listener.accept() => connection.unwrap(),
                    };
                    let first = accepted.fetch_add(1, Ordering::SeqCst) == 0;
                    match reply {
                        Reply::Http {
                            status,
                            body,
                            retry,
                        } => {
                            read_request(&mut stream).await;
                            if first {
                                clock.fail_receipt();
                            }
                            let response = format!(
                                "HTTP/1.1 {status} synthetic\r\nConnection: close\r\nContent-Length: {}\r\nX-MBX-USED-WEIGHT-1M: 17\r\nSet-Cookie: synthetic-private-header\r\n{retry}\r\n{body}",
                                body.len()
                            );
                            stream.write_all(response.as_bytes()).await.unwrap();
                            stream.shutdown().await.unwrap();
                        }
                        Reply::Upgrade => {
                            if first {
                                clock.fail_receipt();
                            }
                            let (_, mut peer) = ServerBuilder::new()
                                .limits(Limits::unlimited())
                                .add_header(
                                    "x-mbx-used-weight-1m".parse().unwrap(),
                                    "17".parse().unwrap(),
                                )
                                .unwrap()
                                .accept(stream)
                                .await
                                .unwrap();
                            loop {
                                let message = tokio::select! {
                                    () = cancelled.cancelled() => break,
                                    message = peer.next() => message,
                                };
                                match message {
                                    Some(Ok(message)) if message.is_close() => {
                                        peer.flush().await.unwrap();
                                        break;
                                    }
                                    None | Some(Err(_)) => break,
                                    Some(Ok(_)) => (),
                                }
                            }
                        }
                    }
                }
            });
            Self {
                url,
                count,
                stop,
                task,
            }
        }

        fn connections(&self) -> usize {
            self.count.load(Ordering::SeqCst)
        }

        async fn finish(self) {
            self.stop.cancel();
            self.task.await.unwrap();
        }
    }

    async fn read_request(stream: &mut TcpStream) {
        let mut bytes = Vec::new();
        let mut chunk = [0; 1024];
        loop {
            let read = stream.read(&mut chunk).await.unwrap();
            assert_ne!(read, 0);
            bytes.extend_from_slice(&chunk[..read]);
            if let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
                let header = std::str::from_utf8(&bytes[..end]).unwrap();
                let length = header
                    .lines()
                    .find_map(|line| {
                        line.split_once(':')
                            .filter(|(name, _)| name.eq_ignore_ascii_case("content-length"))
                            .map(|(_, value)| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                if bytes.len() >= end + 4 + length {
                    break;
                }
            }
        }
    }

    fn evidence(error: &Error, status: u16, outcome: Outcome, operation: &str) {
        assert_eq!(error.outcome(), Some(outcome));
        let Error::Transport {
            failure: Some(TransportFailure::RateEvidenceFailed),
            meta: Some(meta),
            ..
        } = error
        else {
            panic!("missing received rate evidence: {error:?}")
        };
        assert_eq!(meta.status, status);
        assert_eq!(meta.operation, operation);
        assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 17);
        assert_eq!(
            meta.rates.retry_after,
            if status == 429 {
                Some(Duration::from_secs(2))
            } else {
                None
            }
        );
        assert!(!meta.rates.retry_after_unusable);
        assert!(!format!("{error:?}").contains("synthetic-private"));
    }

    macro_rules! http_read {
        ($name:ident, $status:literal, $retry:literal) => {
            #[tokio::test]
            async fn $name() {
                let clock = ReceiptClock::new();
                let venue = Venue::new(
                    Arc::clone(&clock),
                    Reply::Http {
                        status: $status,
                        body: "{\"serverTime\":1700000041000}",
                        retry: $retry,
                    },
                )
                .await;
                let owner = usdm::Config::with_pools(usdm::Environment::Demo, &WeightPools::new())
                    .unwrap()
                    .rest_url(&venue.url)
                    .unwrap()
                    .clock(clock.clone());
                let observer = owner.clone();
                let client = usdm::RestClient::new(owner.clone()).unwrap();
                let sibling = usdm::RestClient::new(owner).unwrap();
                let request = usdm::rest_requests::CheckServerTime::new();
                let first = client.check_server_time(&request, deadline()).await;
                let second = sibling.check_server_time(&request, deadline()).await;
                assert_eq!(venue.connections(), 1);
                let first = first.unwrap_err();
                evidence(&first, $status, Outcome::ReadFailed, "checkServerTime");
                assert!(
                    matches!(second, Err(Error::CooldownTimingUnknown)),
                    "{second:?}"
                );
                assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
                clock.now.store(1_700_000_161_000, Ordering::SeqCst);
                assert!(matches!(
                    sibling.check_server_time(&request, deadline()).await,
                    Err(Error::CooldownTimingUnknown)
                ));
                assert_eq!(venue.connections(), 1);
                venue.finish().await;
            }
        };
    }

    http_read!(
        a_successful_http_read_with_no_receipt_clock_holds_quota_across_a_window,
        200,
        ""
    );
    http_read!(
        an_http_retry_after_with_no_receipt_clock_never_expires_from_send_time,
        429,
        "Retry-After: 2\r\n"
    );

    #[tokio::test]
    async fn a_mutation_receipt_clock_failure_preserves_unknown_outcome_and_caller_identity() {
        let clock = ReceiptClock::new();
        let venue = Venue::new(Arc::clone(&clock), Reply::Http { status: 200, body: "{\"orderId\":7,\"clientOrderId\":\"receipt-order\",\"symbol\":\"BTCUSDT\",\"status\":\"NEW\"}", retry: "" }).await;
        let owner = usdm::Config::with_pools(usdm::Environment::Demo, &WeightPools::new())
            .unwrap()
            .rest_url(&venue.url)
            .unwrap()
            .clock(clock)
            .credentials(
                Credentials::hmac("synthetic-api-key", "synthetic-private-secret").unwrap(),
            );
        let client = usdm::RestClient::new(owner).unwrap();
        let request = usdm::rest_requests::NewOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(ClientOrderId::new("receipt-order").unwrap());
        let first = client.new_order(&request, deadline()).await;
        let second = client
            .check_server_time(&usdm::rest_requests::CheckServerTime::new(), deadline())
            .await;
        assert_eq!(venue.connections(), 1);
        let first = first.unwrap_err();
        evidence(&first, 200, Outcome::Unknown, "newOrder");
        let Error::Transport {
            client_order_ids,
            meta: Some(meta),
            ..
        } = first
        else {
            panic!("missing mutation identity")
        };
        assert_eq!(client_order_ids["newClientOrderId"], "receipt-order");
        assert_eq!(meta.client_order_ids["newClientOrderId"], "receipt-order");
        assert!(matches!(second, Err(Error::CooldownTimingUnknown)));
        venue.finish().await;
    }

    async fn websocket_error(owner: spot::Config, recovery: bool) -> Option<Error> {
        let result = if recovery {
            spot::WsClient::connect_for_recovery(owner).await
        } else {
            spot::WsClient::connect(owner).await
        };
        match result {
            Err(error) => Some(error),
            Ok((client, mut events, driver)) => {
                let driver = tokio::spawn(driver.run());
                client.close().await.unwrap();
                while events.recv().await.is_some() {}
                driver.await.unwrap().unwrap();
                None
            }
        }
    }

    macro_rules! websocket_receipt {
        ($name:ident, $reply:expr, $status:literal, $recovery:literal) => {
            #[tokio::test]
            async fn $name() {
                let clock = ReceiptClock::new();
                let venue = Venue::new(Arc::clone(&clock), $reply).await;
                let url = venue.url.replacen("http://", "ws://", 1);
                let owner = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
                    .unwrap()
                    .websocket_url(&url)
                    .unwrap()
                    .clock(clock.clone());
                if $recovery {
                    owner
                        .set_exit_reserve(ExitReserve::default().request_weight(10_000))
                        .unwrap();
                }
                let observer = owner.clone();
                let first = websocket_error(owner.clone(), $recovery).await;
                let second = websocket_error(owner.clone(), $recovery).await;
                assert_eq!(venue.connections(), 1);
                let first = first.unwrap();
                evidence(&first, $status, Outcome::ReadFailed, "connect");
                assert!(
                    matches!(second, Some(Error::CooldownTimingUnknown)),
                    "{second:?}"
                );
                assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
                clock.now.store(1_700_000_161_000, Ordering::SeqCst);
                assert!(matches!(
                    websocket_error(owner, $recovery).await,
                    Some(Error::CooldownTimingUnknown)
                ));
                assert_eq!(venue.connections(), 1);
                venue.finish().await;
            }
        };
    }

    websocket_receipt!(
        ordinary_accepted_upgrade_without_receipt_time_holds_shared_admission,
        Reply::Upgrade,
        101,
        false
    );
    websocket_receipt!(
        recovery_accepted_upgrade_without_receipt_time_holds_shared_admission,
        Reply::Upgrade,
        101,
        true
    );
    websocket_receipt!(
        ordinary_rejected_upgrade_without_receipt_time_never_ages_retry_after_from_send_time,
        Reply::Http {
            status: 429,
            body: "synthetic-private-refusal",
            retry: "Retry-After: 2\r\n"
        },
        429,
        false
    );
    websocket_receipt!(
        recovery_rejected_upgrade_without_receipt_time_never_ages_retry_after_from_send_time,
        Reply::Http {
            status: 429,
            body: "synthetic-private-refusal",
            retry: "Retry-After: 2\r\n"
        },
        429,
        true
    );
}
