// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Caller-owned socket tasks, correlation, exact frames, lossless ingress, and retirement.

#[allow(
    dead_code,
    reason = "shared HTTP fixture helpers are used by the HTTP integration suite"
)]
#[cfg(test)]
mod support;
use binance_client::usdm::{
    ApiEvent, Stream, StreamEvent, Streams, WsClient, event_payloads::ApiPayload,
    streams::StreamPayload, ws_requests::NewOrder,
};
use binance_client::{ClientOrderId, Decimal, Error, Outcome, RequestId, SensitiveString, Symbol};
use futures_util::{FutureExt, SinkExt, StreamExt};
use serde_json::{Value, json};
#[cfg(test)]
mod tests {
    use super::*;
    use support::{config, deadline};
    use tokio::{
        net::{TcpListener, TcpStream},
        sync::oneshot,
        time::Duration,
    };
    use tokio_websockets::{Limits, Message, ServerBuilder, WebSocketStream};

    fn order() -> NewOrder {
        NewOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::new(1, 3))
            .new_client_order_id(ClientOrderId::new("fixture/order:1").unwrap())
    }
    fn rest_order(id: &str) -> binance_client::usdm::rest_requests::NewOrder {
        binance_client::usdm::rest_requests::NewOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(ClientOrderId::new(id).unwrap())
    }
    async fn listener() -> (TcpListener, String) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("ws://{}", listener.local_addr().unwrap());
        (listener, url)
    }
    async fn accept(listener: TcpListener) -> WebSocketStream<TcpStream> {
        let (stream, _) = listener.accept().await.unwrap();
        ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(stream)
            .await
            .unwrap()
            .1
    }
    async fn finish_server(mut ws: WebSocketStream<TcpStream>) {
        while let Some(message) = ws.next().await {
            if message.unwrap().is_close() {
                ws.flush().await.unwrap();
                break;
            }
        }
    }
    async fn next_json(ws: &mut WebSocketStream<TcpStream>) -> Value {
        let message = ws.next().await.unwrap().unwrap();
        assert!(message.is_text());
        serde_json::from_slice(message.as_payload()).unwrap()
    }
    async fn retire(
        client: &WsClient,
        events: &mut binance_client::usdm::ApiEvents,
        driver: tokio::task::JoinHandle<Result<(), Error>>,
    ) {
        client.close().await.unwrap();
        assert!(matches!(events.recv().await,Some(ApiEvent::Retired(g)) if g==client.generation()));
        assert!(events.recv().await.is_none());
        driver.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn websocket_weight_includes_handshake_and_uses_every_remaining_request_slot() {
        // Official handshake cost and independent REST/WS IP scopes, checked 2026-09-27:
        // https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/websocket-api-general-info
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            for i in 0..2 {
                let frame = next_json(&mut ws).await;
                assert_eq!(frame["method"], "depth");
                assert_eq!(frame["id"], format!("depth-{i}"));
                ws.send(Message::text(
                    json!({"id": frame["id"], "status": 200,
                    "result": {"lastUpdateId": 1, "bids": [], "asks": []}})
                    .to_string(),
                ))
                .await
                .unwrap();
            }
            // After exhaustion the next outbound frame must be caller shutdown.
            assert!(ws.next().await.unwrap().unwrap().is_close());
            ws.flush().await.unwrap();
        });
        let budgets = binance_client::Budgets::new(
            binance_client::BudgetLimits::usdm()
                .weight_per_minute(2)
                .ws_weight_per_minute(9),
        )
        .unwrap();
        let base = config()
            .budgets(budgets)
            .clock(std::sync::Arc::new(support::FixedClock(1000)));
        let (client, mut events, driver) =
            WsClient::connect(base.clone().websocket_url(&url).unwrap())
                .await
                .unwrap();
        let driver = tokio::spawn(driver.run());
        assert!(matches!(
            events.recv().await,
            Some(ApiEvent::Established(_))
        ));
        let depth = binance_client::usdm::ws_requests::OrderBook::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .limit(5);
        // Five handshake units plus two depth reads at two units each fills nine.
        for i in 0..2 {
            client
                .order_book(
                    &depth,
                    RequestId::new(format!("depth-{i}")).unwrap(),
                    deadline(),
                )
                .await
                .unwrap();
        }
        let error = client
            .clone()
            .order_book(&depth, RequestId::new("exhausted").unwrap(), deadline())
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::NotSent));
        assert!(
            matches!(error, Error::Admission { retry_after } if retry_after == Duration::from_secs(59))
        );

        // A refused handshake must never reach another local listener.
        let (unused_listener, unused_url) = self::listener().await;
        assert!(matches!(
            WsClient::connect(base.clone().websocket_url(&unused_url).unwrap()).await,
            Err(Error::Admission { .. })
        ));
        assert!(unused_listener.accept().now_or_never().is_none());

        // Exhausted WebSocket IP weight must leave ordinary REST weight available.
        let fixture =
            support::HttpFixture::new(200, "", "{\"serverTime\":1000}", None, false).await;
        let rest =
            binance_client::usdm::RestClient::new(base.rest_url(&fixture.url).unwrap()).unwrap();
        rest.check_server_time(
            &binance_client::usdm::rest_requests::CheckServerTime::new(),
            deadline(),
        )
        .await
        .unwrap();
        // REST cancellation also charges the WebSocket IP scope. Its refusal must
        // consume neither the remaining REST unit nor a network attempt.
        let cancel = binance_client::usdm::rest_requests::CancelOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .orig_client_order_id(ClientOrderId::new("cross-transport-refused").unwrap());
        assert!(matches!(
            rest.cancel_order(&cancel, deadline()).await,
            Err(Error::Admission { .. })
        ));
        rest.check_server_time(
            &binance_client::usdm::rest_requests::CheckServerTime::new(),
            deadline(),
        )
        .await
        .unwrap();
        assert!(matches!(
            rest.check_server_time(
                &binance_client::usdm::rest_requests::CheckServerTime::new(),
                deadline()
            )
            .await,
            Err(Error::Admission { .. })
        ));
        assert_eq!(fixture.attempts(), 2);
        retire(&client, &mut events, driver).await;
        server.await.unwrap();
        fixture.finish().await;
    }

    #[tokio::test]
    async fn rest_and_websocket_orders_use_the_full_shared_account_allowance() {
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            for _ in 0..2 {
                let frame = next_json(&mut ws).await;
                assert_eq!(frame["method"], "order.place");
                ws.send(Message::text(
                    json!({"id": frame["id"], "status": 200,
                    "result": {"orderId": 7}})
                    .to_string(),
                ))
                .await
                .unwrap();
            }
            assert!(ws.next().await.unwrap().unwrap().is_close());
            ws.flush().await.unwrap();
        });
        let fixture = support::HttpFixture::new(200, "", "{\"orderId\":7}", None, false).await;
        let budgets =
            binance_client::Budgets::new(binance_client::BudgetLimits::usdm().orders(4, 4))
                .unwrap();
        let base = config().budgets(budgets.clone());
        let rest =
            binance_client::usdm::RestClient::new(base.clone().rest_url(&fixture.url).unwrap())
                .unwrap();
        let (client, mut events, driver) =
            WsClient::connect(base.clone().websocket_url(&url).unwrap())
                .await
                .unwrap();
        let driver = tokio::spawn(driver.run());
        assert!(matches!(
            events.recv().await,
            Some(ApiEvent::Established(_))
        ));
        for i in 0..2 {
            rest.new_order(&rest_order(&format!("rest-{i}")), deadline())
                .await
                .unwrap();
            client
                .new_order(
                    &order().new_client_order_id(ClientOrderId::new(format!("ws-{i}")).unwrap()),
                    RequestId::new(format!("ws-{i}")).unwrap(),
                    deadline(),
                )
                .await
                .unwrap();
        }
        assert!(matches!(
            rest.new_order(&rest_order("rest-refused"), deadline())
                .await,
            Err(Error::Admission { .. })
        ));
        let error = client
            .new_order(&order(), RequestId::new("ws-refused").unwrap(), deadline())
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::NotSent));
        assert!(matches!(error, Error::Admission { .. }));
        assert_eq!(fixture.attempts(), 2);

        // A distinct account has its own full order allowance under the same IP owner.
        let other = binance_client::usdm::RestClient::new(
            base.budgets(budgets.for_account())
                .rest_url(&fixture.url)
                .unwrap(),
        )
        .unwrap();
        for i in 0..4 {
            other
                .new_order(&rest_order(&format!("other-{i}")), deadline())
                .await
                .unwrap();
        }
        assert!(matches!(
            other
                .new_order(&rest_order("other-refused"), deadline())
                .await,
            Err(Error::Admission { .. })
        ));
        assert_eq!(fixture.attempts(), 6);
        retire(&client, &mut events, driver).await;
        server.await.unwrap();
        fixture.finish().await;
    }

    #[tokio::test]
    async fn signed_frame_keeps_decimal_strings_and_integer_timestamps() {
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            let frame = next_json(&mut ws).await;
            assert_eq!(frame["method"], "order.place");
            assert_eq!(frame["id"], "request-1");
            let params = frame["params"].as_object().unwrap();
            assert_eq!(params["quantity"], "0.001");
            assert_eq!(params["timestamp"], 1_700_000_001_000_u64);
            let payload = params
                .iter()
                .filter(|(k, _)| k.as_str() != "signature")
                .map(|(k, v)| {
                    format!(
                        "{k}={}",
                        v.as_str().map_or_else(|| v.to_string(), str::to_owned)
                    )
                })
                .collect::<Vec<_>>()
                .join("&");
            assert!(payload.contains("newClientOrderId=fixture/order:1"));
            let key = aws_lc_rs::hmac::Key::new(aws_lc_rs::hmac::HMAC_SHA256, b"synthetic-secret");
            let signature = aws_lc_rs::hmac::sign(&key, payload.as_bytes())
                .as_ref()
                .iter()
                .fold(String::new(), |mut text, b| {
                    use std::fmt::Write as _;
                    write!(&mut text, "{b:02x}").unwrap();
                    text
                });
            assert_eq!(params["signature"], signature);
            ws.send(Message::text(json!({"id":"request-1","status":200,"result":{"orderId":7},"rateLimits":[{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"count":1}]}).to_string())).await.unwrap();
            finish_server(ws).await;
        });
        let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        assert!(matches!(
            events.recv().await,
            Some(ApiEvent::Established(_))
        ));
        let response = client
            .new_order(&order(), RequestId::new("request-1").unwrap(), deadline())
            .await
            .unwrap();
        assert_eq!(response.data.order_id, 7);
        assert_eq!(response.meta.rates.counters["x-mbx-order-count-10s"], 1);
        retire(&client, &mut events, driver).await;
        server.await.unwrap();
    }

    #[tokio::test]
    async fn timed_out_mutation_retains_late_response_identity_without_retry() {
        let (listener, url) = listener().await;
        let (seen, got) = oneshot::channel();
        let (release, gate) = oneshot::channel();
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            let frame = next_json(&mut ws).await;
            seen.send(()).unwrap();
            gate.await.unwrap();
            ws.send(Message::text(
                json!({"id":frame["id"],"status":200,"result":{"orderId":42}}).to_string(),
            ))
            .await
            .unwrap();
            finish_server(ws).await;
        });
        let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        let caller = client.clone();
        let call = tokio::spawn(async move {
            caller
                .new_order(&order(), RequestId::new("late-1").unwrap(), deadline())
                .await
        });
        got.await.unwrap();
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(4)).await;
        assert_eq!(
            call.await.unwrap().unwrap_err().outcome(),
            Some(Outcome::Unknown)
        );
        tokio::time::resume();
        release.send(()).unwrap();
        match events.recv().await.unwrap() {
            ApiEvent::LateResponse {
                id,
                generation,
                result,
            } => {
                assert_eq!(id.as_str(), "late-1");
                assert_eq!(generation, client.generation());
                assert!(matches!(result.unwrap().data,ApiPayload::NewOrder(v) if v.order_id==42));
            }
            other => panic!("late correlation missing: {other:?}"),
        }
        assert!(matches!(
            client
                .new_order(&order(), RequestId::new("late-1").unwrap(), deadline())
                .await,
            Err(Error::DuplicateRequestId)
        ));
        retire(&client, &mut events, driver).await;
        server.await.unwrap();
    }

    #[tokio::test]
    async fn caller_cancellation_preserves_sent_request_evidence() {
        let (listener, url) = listener().await;
        let (seen, got) = oneshot::channel();
        let (release, gate) = oneshot::channel();
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            let frame = next_json(&mut ws).await;
            seen.send(()).unwrap();
            gate.await.unwrap();
            ws.send(Message::text(
                json!({"id":frame["id"],"status":200,"result":{"orderId":43}}).to_string(),
            ))
            .await
            .unwrap();
            finish_server(ws).await;
        });
        let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        let caller = client.clone();
        let call = tokio::spawn(async move {
            caller
                .new_order(&order(), RequestId::new("cancelled-1").unwrap(), deadline())
                .await
        });
        got.await.unwrap();
        call.abort();
        assert!(call.await.unwrap_err().is_cancelled());
        release.send(()).unwrap();
        assert!(
            matches!(events.recv().await,Some(ApiEvent::LateResponse{id,result:Ok(_),..}) if id.as_str()=="cancelled-1")
        );
        retire(&client, &mut events, driver).await;
        server.await.unwrap();
    }

    #[tokio::test]
    async fn socket_loss_settles_sent_mutation_as_unknown_and_retires() {
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            next_json(&mut ws).await;
            drop(ws);
        });
        let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        assert_eq!(
            client
                .new_order(&order(), RequestId::new("loss-1").unwrap(), deadline())
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::Unknown)
        );
        assert!(matches!(events.recv().await, Some(ApiEvent::Gap { .. })));
        assert!(matches!(events.recv().await, Some(ApiEvent::Retired(_))));
        assert!(events.recv().await.is_none());
        assert_eq!(
            client
                .new_order(
                    &order(),
                    RequestId::new("after-retirement").unwrap(),
                    deadline()
                )
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::NotSent)
        );
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn expired_unsent_command_and_hmac_session_never_send_a_frame() {
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            let message = ws.next().await.unwrap().unwrap();
            assert!(message.is_close());
            ws.flush().await.unwrap();
        });
        let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        assert_eq!(
            client
                .new_order(
                    &order(),
                    RequestId::new("expired-1").unwrap(),
                    tokio::time::Instant::now()
                )
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::NotSent)
        );
        assert_eq!(
            client
                .session_logon(RequestId::new("session-1").unwrap(), deadline())
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::NotSent)
        );
        retire(&client, &mut events, driver).await;
        server.await.unwrap();
    }

    #[tokio::test]
    async fn exact_ping_payload_is_returned_and_authentication_revocation_is_a_gap() {
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            ws.send(Message::ping(b"exact-ping-payload".as_slice()))
                .await
                .unwrap();
            let pong = ws.next().await.unwrap().unwrap();
            assert!(pong.is_pong());
            assert_eq!(&pong.as_payload()[..], b"exact-ping-payload");
            ws.send(Message::text(
                "{\"id\":null,\"status\":401,\"error\":{\"code\":-2015}}".to_owned(),
            ))
            .await
            .unwrap();
            finish_server(ws).await;
        });
        let (_, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        assert!(matches!(
            events.recv().await,
            Some(ApiEvent::Gap {
                error: Error::Gap("WebSocket authentication revoked"),
                ..
            })
        ));
        assert!(matches!(events.recv().await, Some(ApiEvent::Retired(_))));
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn private_ingress_is_lossless_and_generation_prefix_precedes_retirement() {
        let (listener, url) = listener().await;
        let (sent, done) = oneshot::channel();
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let (request, mut ws) = ServerBuilder::new()
                .limits(Limits::unlimited())
                .accept(stream)
                .await
                .unwrap();
            assert_eq!(request.uri().path(), "/private/ws/synthetic-listen-key");
            for sequence in 0..5000 {
                ws.send(Message::text(
                    json!({"e":"futureUserEvent","sequence":sequence,"secret":"redact-me"})
                        .to_string(),
                ))
                .await
                .unwrap();
            }
            sent.send(()).unwrap();
            finish_server(ws).await;
        });
        let (mut streams, driver) = Streams::user_data(
            config().streams_url(&url).unwrap(),
            &SensitiveString::new("synthetic-listen-key"),
        )
        .await
        .unwrap();
        let driver = tokio::spawn(driver.run());
        assert!(matches!(
            streams.recv().await,
            Some(StreamEvent::Established(_))
        ));
        done.await.unwrap();
        tokio::time::timeout(Duration::from_secs(3), async {
            while streams.queue_stats().unwrap().depth < 5000 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(1)).await;
        assert!(streams.queue_stats().unwrap().oldest_age >= Duration::from_secs(1));
        tokio::time::resume();
        streams.close().await.unwrap();
        for sequence in 0..5000 {
            match streams.recv().await.unwrap() {
                StreamEvent::Data {
                    generation,
                    payload:
                        StreamPayload::User(binance_client::usdm::event_payloads::UserPayload::Unknown(
                            v,
                        )),
                } => {
                    assert_eq!(generation, streams.generation());
                    assert_eq!(v.as_value()["sequence"], sequence);
                    assert!(!format!("{v:?}").contains("redact-me"));
                }
                event => panic!("ingress prefix lost: {event:?}"),
            }
        }
        assert!(matches!(
            streams.recv().await,
            Some(StreamEvent::Retired(_))
        ));
        assert!(streams.recv().await.is_none());
        let stats = streams.queue_stats().unwrap();
        assert_eq!(stats.depth, 0);
        assert_eq!(stats.accepted, stats.drained);
        assert_eq!(stats.oldest_age, Duration::ZERO);
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn mixed_market_routes_are_refused_and_depth_uses_public_route() {
        let symbol = Symbol::new("BTCUSDT").unwrap();
        let depth = Stream::diff_book_depth_streams(&symbol, "100ms").unwrap();
        let trade = Stream::aggregate_trade_streams(&symbol).unwrap();
        assert!(matches!(
            Streams::connect(config(), &[depth.clone(), trade]).await,
            Err(Error::Validation(_))
        ));
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let (request, mut ws) = ServerBuilder::new().accept(stream).await.unwrap();
            assert_eq!(request.uri().path(), "/public/stream");
            assert!(
                request
                    .uri()
                    .query()
                    .unwrap()
                    .contains("btcusdt%40depth%40100ms")
            );
            ws.send(Message::text("not-json".to_owned())).await.unwrap();
            finish_server(ws).await;
        });
        let (mut streams, driver) = Streams::connect(config().streams_url(&url).unwrap(), &[depth])
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        streams.recv().await.unwrap();
        assert!(matches!(
            streams.recv().await,
            Some(StreamEvent::Gap { .. })
        ));
        assert!(matches!(
            streams.recv().await,
            Some(StreamEvent::Retired(_))
        ));
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn private_stream_control_budget_allows_six_pongs_within_one_second() {
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            for _ in 0..6 {
                ws.send(Message::ping(b"ping".as_slice())).await.unwrap();
                assert!(ws.next().await.unwrap().unwrap().is_pong());
            }
            ws.send(Message::text("{\"e\":\"futureUserEvent\"}".to_owned()))
                .await
                .unwrap();
            finish_server(ws).await;
        });
        let (mut streams, driver) = Streams::user_data(
            config().streams_url(&url).unwrap(),
            &SensitiveString::new("fixture"),
        )
        .await
        .unwrap();
        let driver = tokio::spawn(driver.run());
        streams.recv().await.unwrap();
        assert!(matches!(
            streams.recv().await,
            Some(StreamEvent::Data { .. })
        ));
        streams.close().await.unwrap();
        while streams.recv().await.is_some() {}
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn malformed_late_mutation_retains_unknown_outcome_and_response_metadata() {
        let (listener, url) = listener().await;
        let (seen, got) = oneshot::channel();
        let (release, gate) = oneshot::channel();
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            let frame = next_json(&mut ws).await;
            seen.send(()).unwrap();
            gate.await.unwrap();
            ws.send(Message::text(json!({"id":frame["id"],"status":200,"result":{},"rateLimits":[{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"count":1}]}).to_string())).await.unwrap();
            finish_server(ws).await;
        });
        let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        let caller = client.clone();
        let call = tokio::spawn(async move {
            caller
                .new_order(
                    &order(),
                    RequestId::new("malformed-late").unwrap(),
                    deadline(),
                )
                .await
        });
        got.await.unwrap();
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(4)).await;
        call.await.unwrap().unwrap_err();
        tokio::time::resume();
        release.send(()).unwrap();
        match events.recv().await.unwrap() {
            ApiEvent::LateResponse {
                result: Err(error), ..
            } => {
                assert_eq!(error.outcome(), Some(Outcome::Unknown));
                assert!(
                    matches!(error,Error::Transport{meta:Some(meta),..} if meta.status==200 && meta.rates.counters["x-mbx-order-count-10s"]==1)
                );
            }
            other => panic!("missing malformed late reply: {other:?}"),
        }
        retire(&client, &mut events, driver).await;
        server.await.unwrap();
    }
}
