// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Product subscription routes, private events, and late-answer attribution.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "local fixture assertions"
)]

use binance_client::WeightPools;
use binance_client::{Credentials, Decimal, Error, Outcome, RequestId, Symbol, coinm, spot, usdm};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::time::Duration;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::Instant,
};
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;
use tokio_websockets::{Limits, Message, ServerBuilder, WebSocketStream};

async fn listener() -> (TcpListener, String) {
    let l = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", l.local_addr().unwrap());
    (l, url)
}
async fn accept(l: TcpListener) -> WebSocketStream<TcpStream> {
    let (stream, _) = l.accept().await.unwrap();
    ServerBuilder::new()
        .limits(Limits::unlimited())
        .accept(stream)
        .await
        .unwrap()
        .1
}
async fn finish(mut peer: WebSocketStream<TcpStream>) {
    while let Some(m) = peer.next().await {
        if m.unwrap().is_close() {
            peer.flush().await.unwrap();
            break;
        }
    }
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(3)
}

#[tokio::test]
async fn spot_api_signature_subscription_delivers_partial_balances_and_shutdown_in_order() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(l).await;
        let request: Value =
            serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
        assert_eq!(request["method"], "userDataStream.subscribe.signature");
        assert_eq!(request["params"]["apiKey"], "synthetic-api-key");
        assert!(request["params"]["signature"].as_str().is_some());
        peer.send(Message::text(
            json!({"id":request["id"],"status":200,"result":{"subscriptionId":7}}).to_string(),
        ))
        .await
        .unwrap();
        for event in [
            json!({"subscriptionId":7,"event":{"e":"outboundAccountPosition","E":1,"u":1,"B":[{"a":"BTC","f":"1.0000000000000000000000000001","l":"2"}]}}),
            json!({"subscriptionId":7,"event":{"e":"eventStreamTerminated","E":2}}),
            json!({"event":{"e":"serverShutdown","E":3}}),
        ] {
            peer.send(Message::text(event.to_string())).await.unwrap();
        }
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .websocket_url(&url)
        .unwrap()
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let (client, mut events, driver) = spot::WsClient::connect(config).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = client.generation();
    assert!(matches!(events.recv().await,Some(spot::ApiEvent::Established(g)) if g==generation));
    let response = client
        .user_data_stream_subscribe_signature(
            &spot::ws_requests::UserDataStreamSubscribeSignature::new(),
            RequestId::new("subscribe").unwrap(),
            deadline(),
        )
        .await
        .unwrap();
    assert_eq!(response.data.subscription_id, 7);
    match events.recv().await.unwrap() {
        spot::ApiEvent::UserData {
            generation: g,
            subscription_id: 7,
            payload: spot::event_payloads::UserPayload::OutboundAccountPosition(p),
        } => {
            assert_eq!(g, generation);
            assert_eq!(p.upper_b.len(), 1);
            assert_eq!(p.upper_b[0].a.as_str(), "BTC");
            assert_eq!(
                p.upper_b[0].f,
                Decimal::from_str_exact("1.0000000000000000000000000001").unwrap()
            );
        }
        other => panic!("unexpected event {other:?}"),
    }
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::UserData {
            payload: spot::event_payloads::UserPayload::EventStreamTerminated(_),
            ..
        })
    ));
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::ServerShutdown { .. })
    ));
    client.close().await.unwrap();
    assert!(matches!(events.recv().await,Some(spot::ApiEvent::Retired(g)) if g==generation));
    assert!(events.recv().await.is_none());
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

/// A user-data event this Spot build does not model is retained as unknown evidence
/// on the live socket. Another product's event name is not borrowed into a Spot
/// continuity failure that would retire the generation.
#[tokio::test]
async fn spot_user_data_retains_another_products_event_name_as_unknown() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(l).await;
        let request: Value =
            serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
        assert_eq!(request["method"], "userDataStream.subscribe.signature");
        peer.send(Message::text(
            json!({"id":request["id"],"status":200,"result":{"subscriptionId":9}}).to_string(),
        ))
        .await
        .unwrap();
        // `ACCOUNT_UPDATE` is a Futures user-data event; Spot does not document it,
        // and Spot cannot decode its balance/position evidence into a Spot model.
        peer.send(Message::text(
            json!({"subscriptionId":9,"event":{"e":"ACCOUNT_UPDATE","E":1}}).to_string(),
        ))
        .await
        .unwrap();
        // The socket stays alive: the next documented Spot event still arrives.
        peer.send(Message::text(
            json!({"subscriptionId":9,"event":{"e":"outboundAccountPosition","E":2,"u":1,"B":[{"a":"BTC","f":"1.0","l":"0"}]}}).to_string(),
        ))
        .await
        .unwrap();
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .websocket_url(&url)
        .unwrap()
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let (client, mut events, driver) = spot::WsClient::connect(config).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = client.generation();
    assert!(matches!(events.recv().await,Some(spot::ApiEvent::Established(g)) if g==generation));
    client
        .user_data_stream_subscribe_signature(
            &spot::ws_requests::UserDataStreamSubscribeSignature::new(),
            RequestId::new("subscribe").unwrap(),
            deadline(),
        )
        .await
        .unwrap();
    match events.recv().await {
        Some(spot::ApiEvent::UserData {
            generation: g,
            subscription_id: 9,
            payload: spot::event_payloads::UserPayload::Unknown(_),
        }) if g == generation => {}
        other => panic!("unexpected event {other:?}"),
    }
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::UserData {
            payload: spot::event_payloads::UserPayload::OutboundAccountPosition(_),
            ..
        })
    ));
    client.close().await.unwrap();
    assert!(matches!(events.recv().await,Some(spot::ApiEvent::Retired(g)) if g==generation));
    assert!(events.recv().await.is_none());
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn spot_late_order_reply_keeps_original_caller_and_generation_after_timeout() {
    let (l, url) = listener().await;
    let (sent_tx, sent_rx) = oneshot::channel();
    let (release_tx, release_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut peer = accept(l).await;
        let request: Value =
            serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
        assert_eq!(request["method"], "order.place");
        assert_eq!(request["params"]["newClientOrderId"], "caller &/订单");
        sent_tx.send(()).unwrap();
        release_rx.await.unwrap();
        peer.send(Message::text(json!({"id":request["id"],"status":200,"result":{"orderId":9,"clientOrderId":"caller &/订单"}}).to_string())).await.unwrap();
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .websocket_url(&url)
        .unwrap()
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let (client, mut events, driver) = spot::WsClient::connect(config).await.unwrap();
    let driver = tokio::spawn(driver.run());
    events.recv().await.unwrap();
    let c = client.clone();
    let request = tokio::spawn(async move {
        c.order_place(
            &spot::ws_requests::OrderPlace::new()
                .symbol(Symbol::new("BTCUSDT").unwrap())
                .side("BUY")
                .type_value("MARKET")
                .quote_order_qty(Decimal::ONE)
                .new_client_order_id(spot::ClientOrderId::new("caller &/订单").unwrap()),
            RequestId::new("original-request").unwrap(),
            Instant::now() + Duration::from_secs(1),
        )
        .await
    });
    sent_rx.await.unwrap();
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(1)).await;
    let error = request.await.unwrap().unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        client_order_ids, ..
    } = error
    else {
        panic!("missing caller ID")
    };
    assert_eq!(client_order_ids["newClientOrderId"], "caller &/订单");
    tokio::time::resume();
    release_tx.send(()).unwrap();
    match events.recv().await.unwrap() {
        spot::ApiEvent::LateResponse {
            generation,
            id,
            result: Ok(response),
        } => {
            assert_eq!(generation, client.generation());
            assert_eq!(id.as_str(), "original-request");
            assert_eq!(
                response.meta.client_order_ids["newClientOrderId"],
                "caller &/订单"
            );
        }
        other => panic!("unexpected event {other:?}"),
    }
    client.close().await.unwrap();
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::Retired(_))
    ));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn coinm_market_stream_uses_plain_combined_path_and_joins_retirement() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let (stream, _) = l.accept().await.unwrap();
        let (request, mut peer) = ServerBuilder::new().accept(stream).await.unwrap();
        assert_eq!(request.uri().path(), "/stream");
        let query = url::form_urlencoded::parse(request.uri().query().unwrap().as_bytes())
            .collect::<Vec<_>>();
        assert_eq!(query[0].1, "btcusd_perp@depth@100ms");
        peer.send(Message::text(json!({"stream":"btcusd_perp@depth@100ms","data":{"e":"depthUpdate","E":1,"T":1,"s":"BTCUSD_PERP","U":1,"u":2,"pu":0,"b":[],"a":[]}}).to_string())).await.unwrap();
        finish(peer).await;
    });
    let config = coinm::Config::with_pools(coinm::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(&url)
        .unwrap();
    let stream =
        coinm::Stream::diff_book_depth_streams(&Symbol::new("BTCUSD_PERP").unwrap(), "100ms")
            .unwrap();
    let (mut events, driver) = coinm::Streams::connect(config, &[stream]).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = events.generation();
    assert!(
        matches!(events.recv().await,Some(coinm::StreamEvent::Established(g)) if g==generation)
    );
    assert!(
        matches!(events.recv().await,Some(coinm::StreamEvent::Data{generation:g,..}) if g==generation)
    );
    events.close().await.unwrap();
    assert!(matches!(events.recv().await,Some(coinm::StreamEvent::Retired(g)) if g==generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn coinm_contract_info_without_brackets_is_delivered_without_a_gap() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let (stream, _) = l.accept().await.unwrap();
        let (_, mut peer) = ServerBuilder::new().accept(stream).await.unwrap();
        peer.send(Message::text(json!({"stream":"!contractInfo","data":{"e":"contractInfo","E":1,"s":"BTCUSD_261225","ps":"BTCUSD","ct":"CURRENT_QUARTER","dt":1_798_185_600_000_i64,"ot":1_782_979_200_000_i64,"cs":"SETTLING","st":2}}).to_string())).await.unwrap();
        peer.send(Message::text(json!({"stream":"!contractInfo","data":{"e":"contractInfo","E":2,"s":"BTCUSD_PERP","ps":"BTCUSD","ct":"PERPETUAL","dt":4_133_404_800_000_i64,"ot":1_597_042_800_000_i64,"cs":"TRADING","bks":[{"bs":1,"bnf":0,"bnc":5,"mmr":0.004,"cf":0,"mi":101,"ma":125}],"st":2}}).to_string())).await.unwrap();
        finish(peer).await;
    });
    let (mut events, driver) =
        coinm_market(&url, &[coinm::Stream::contract_info_stream().unwrap()]).await;
    let driver = tokio::spawn(driver.run());
    let generation = events.generation();
    assert!(
        matches!(events.recv().await,Some(coinm::StreamEvent::Established(g)) if g==generation)
    );
    let mut delivered = Vec::new();
    for _ in 0..2 {
        match events.recv().await.unwrap() {
            coinm::StreamEvent::Data {
                generation: g,
                payload:
                    coinm::streams::StreamPayload::Market {
                        payload: coinm::event_payloads::MarketPayload::ContractInfoStream(row),
                        ..
                    },
            } if g == generation => delivered.push((row.cs.clone(), row.bks.map(|b| b.len()))),
            other => panic!("unexpected event {other:?}"),
        }
    }
    assert_eq!(
        delivered,
        [
            // COIN-M does not document `SETTLING`; it is kept, not refused.
            (
                coinm::enums::ContractStatus::Unknown("SETTLING".to_owned()),
                None
            ),
            (coinm::enums::ContractStatus::Trading, Some(1))
        ]
    );
    events.close().await.unwrap();
    assert!(matches!(events.recv().await,Some(coinm::StreamEvent::Retired(g)) if g==generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn usdm_unknown_stream_name_is_retained_and_keeps_the_generation_alive() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let (stream, _) = l.accept().await.unwrap();
        let (request, mut peer) = ServerBuilder::new().accept(stream).await.unwrap();
        assert_eq!(request.uri().path(), "/public/stream");
        // A known subscribed stream, a future venue stream name, then the known
        // stream again: every frame is delivered in source order on one socket.
        peer.send(Message::text(json!({"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":1,"T":1,"s":"BTCUSDT","U":1,"u":2,"pu":0,"b":[],"a":[]}}).to_string())).await.unwrap();
        peer.send(Message::text(
            json!({"stream":"btcusdt@futureStream@1s","data":{"e":"futureEvent","x":42}})
                .to_string(),
        ))
        .await
        .unwrap();
        peer.send(Message::text(json!({"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":2,"T":2,"s":"BTCUSDT","U":3,"u":3,"pu":2,"b":[],"a":[]}}).to_string())).await.unwrap();
        finish(peer).await;
    });
    let config = usdm::Config::with_pools(usdm::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(&url)
        .unwrap();
    let stream =
        usdm::Stream::diff_book_depth_streams(&Symbol::new("BTCUSDT").unwrap(), "100ms").unwrap();
    let (mut events, driver) = usdm::Streams::connect(config, usdm::Route::Public, &[stream])
        .await
        .unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = events.generation();
    assert!(matches!(events.recv().await,Some(usdm::StreamEvent::Established(g)) if g==generation));
    match events.recv().await.unwrap() {
        usdm::StreamEvent::Data {
            generation: g,
            payload:
                usdm::streams::StreamPayload::Market {
                    stream,
                    payload: usdm::event_payloads::MarketPayload::DiffBookDepthStreams(_),
                },
        } => {
            assert_eq!(g, generation);
            assert_eq!(stream, "btcusdt@depth@100ms");
        }
        other => panic!("unexpected event {other:?}"),
    }
    // An unrecognized stream name stays observable evidence on a live socket:
    // retained as an unknown payload, not a continuity gap or teardown.
    match events.recv().await.unwrap() {
        usdm::StreamEvent::Data {
            generation: g,
            payload:
                usdm::streams::StreamPayload::Market {
                    stream,
                    payload: usdm::event_payloads::MarketPayload::Unknown(value),
                },
        } => {
            assert_eq!(g, generation);
            assert_eq!(stream, "btcusdt@futureStream@1s");
            assert_eq!(value.as_value().get("x").and_then(Value::as_i64), Some(42));
        }
        other => panic!("unexpected event {other:?}"),
    }
    assert!(
        matches!(events.recv().await,Some(usdm::StreamEvent::Data{generation:g,..}) if g==generation)
    );
    events.close().await.unwrap();
    assert!(matches!(events.recv().await,Some(usdm::StreamEvent::Retired(g)) if g==generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn coinm_unknown_stream_name_is_retained_and_keeps_the_generation_alive() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let (stream, _) = l.accept().await.unwrap();
        let (request, mut peer) = ServerBuilder::new().accept(stream).await.unwrap();
        assert_eq!(request.uri().path(), "/stream");
        peer.send(Message::text(
            json!({"stream":"btcusd_perp@futureStream@1s","data":{"e":"futureEvent","x":7}})
                .to_string(),
        ))
        .await
        .unwrap();
        peer.send(Message::text(json!({"stream":"btcusd_perp@depth@100ms","data":{"e":"depthUpdate","E":1,"T":1,"s":"BTCUSD_PERP","U":1,"u":2,"pu":0,"b":[],"a":[]}}).to_string())).await.unwrap();
        finish(peer).await;
    });
    let config = coinm::Config::with_pools(coinm::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(&url)
        .unwrap();
    let stream =
        coinm::Stream::diff_book_depth_streams(&Symbol::new("BTCUSD_PERP").unwrap(), "100ms")
            .unwrap();
    let (mut events, driver) = coinm::Streams::connect(config, &[stream]).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = events.generation();
    assert!(
        matches!(events.recv().await,Some(coinm::StreamEvent::Established(g)) if g==generation)
    );
    match events.recv().await.unwrap() {
        coinm::StreamEvent::Data {
            generation: g,
            payload:
                coinm::streams::StreamPayload::Market {
                    stream,
                    payload: coinm::event_payloads::MarketPayload::Unknown(value),
                },
        } => {
            assert_eq!(g, generation);
            assert_eq!(stream, "btcusd_perp@futureStream@1s");
            assert_eq!(value.as_value().get("x").and_then(Value::as_i64), Some(7));
        }
        other => panic!("unexpected event {other:?}"),
    }
    // The same socket keeps delivering known payloads in source order.
    assert!(
        matches!(events.recv().await,Some(coinm::StreamEvent::Data{generation:g,payload:coinm::streams::StreamPayload::Market{payload:coinm::event_payloads::MarketPayload::DiffBookDepthStreams(_),..}}) if g==generation)
    );
    events.close().await.unwrap();
    assert!(matches!(events.recv().await,Some(coinm::StreamEvent::Retired(g)) if g==generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn spot_unknown_stream_name_is_retained_and_keeps_the_generation_alive() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let (stream, _) = l.accept().await.unwrap();
        let (request, mut peer) = ServerBuilder::new().accept(stream).await.unwrap();
        assert_eq!(request.uri().path(), "/stream");
        peer.send(Message::text(
            json!({"stream":"btcusdt@futureStream@100ms","data":{"x":"retained"}}).to_string(),
        ))
        .await
        .unwrap();
        peer.send(Message::text(json!({"stream":"btcusdt@depth@100ms","data":{"e":"depthUpdate","E":1,"s":"BTCUSDT","U":1,"u":2,"b":[["1","2"]],"a":[]}}).to_string())).await.unwrap();
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(&url)
        .unwrap();
    let stream = spot::Stream::diff_book_depth(&Symbol::new("BTCUSDT").unwrap(), "100ms").unwrap();
    let (mut events, driver) = spot::Streams::connect(config, &[stream]).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = events.generation();
    assert!(matches!(events.recv().await,Some(spot::StreamEvent::Established(g)) if g==generation));
    match events.recv().await.unwrap() {
        spot::StreamEvent::Data {
            generation: g,
            payload:
                spot::streams::StreamPayload::Market {
                    stream,
                    payload: spot::event_payloads::MarketPayload::Unknown(value),
                },
        } => {
            assert_eq!(g, generation);
            assert_eq!(stream, "btcusdt@futureStream@100ms");
            assert_eq!(
                value.as_value().get("x").and_then(Value::as_str),
                Some("retained")
            );
        }
        other => panic!("unexpected event {other:?}"),
    }
    assert!(
        matches!(events.recv().await,Some(spot::StreamEvent::Data{generation:g,..}) if g==generation)
    );
    events.close().await.unwrap();
    assert!(matches!(events.recv().await,Some(spot::StreamEvent::Retired(g)) if g==generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn spot_late_cancel_replace_preserves_both_legs_and_microsecond_generation() {
    let (l, url) = listener().await;
    let (sent_tx, sent_rx) = oneshot::channel();
    let (release_tx, release_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut peer = accept(l).await;
        let request: Value =
            serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
        assert_eq!(request["method"], "order.cancelReplace");
        assert_eq!(request["params"]["timestamp"], 1_700_000_001_000_000_u64);
        assert_eq!(request["params"]["newClientOrderId"], "caller &/订单");
        sent_tx.send(()).unwrap();
        release_rx.await.unwrap();
        peer.send(Message::text(json!({"id":request["id"],"status":409,"error":{"code":-2021,"data":{"cancelResult":"SUCCESS","newOrderResult":"FAILURE","cancelResponse":{"orderId":9,"clientOrderId":"cancel"},"newOrderResponse":{"code":-2010}}}}).to_string())).await.unwrap();
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .time_unit(binance_client::TimeUnit::Microseconds)
        .clock(std::sync::Arc::new(support::FixedClock(1_700_000_001_000)))
        .websocket_url(&url)
        .unwrap()
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let (client, mut events, driver) = spot::WsClient::connect(config).await.unwrap();
    assert_eq!(events.time_unit(), binance_client::TimeUnit::Microseconds);
    let driver = tokio::spawn(driver.run());
    events.recv().await.unwrap();
    let c = client.clone();
    let request = tokio::spawn(async move {
        c.order_cancel_replace(
            &spot::ws_requests::OrderCancelReplace::new()
                .symbol(Symbol::new("BTCUSDT").unwrap())
                .side("BUY")
                .type_value("LIMIT")
                .time_in_force("GTC")
                .quantity(Decimal::ONE)
                .price(Decimal::from(10))
                .cancel_replace_mode("ALLOW_FAILURE")
                .cancel_order_id(9)
                .cancel_new_client_order_id(spot::ClientOrderId::new("cancel").unwrap())
                .new_client_order_id(spot::ClientOrderId::new("caller &/订单").unwrap()),
            RequestId::new("original-request").unwrap(),
            Instant::now() + Duration::from_secs(1),
        )
        .await
    });
    sent_rx.await.unwrap();
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(1)).await;
    let error = request.await.unwrap().unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        client_order_ids, ..
    } = error
    else {
        panic!("missing caller ID")
    };
    assert_eq!(client_order_ids["newClientOrderId"], "caller &/订单");
    tokio::time::resume();
    release_tx.send(()).unwrap();
    match events.recv().await.unwrap() {
        spot::ApiEvent::LateResponse {
            generation,
            id,
            result: Err(Error::Venue(response)),
        } => {
            assert_eq!(response.outcome, Outcome::Partial);
            let partial = response.partial.unwrap();
            assert_eq!(partial.legs["cancel"].order_id, Some(9));
            assert_eq!(partial.legs["newOrder"].code, Some(-2010));
            assert_eq!(generation, client.generation());
            assert_eq!(id.as_str(), "original-request");
            assert_eq!(
                response.client_order_ids["newClientOrderId"],
                "caller &/订单"
            );
        }
        other => panic!("unexpected event {other:?}"),
    }
    client.close().await.unwrap();
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::Retired(_))
    ));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

/// One aggregate-trade frame every market's model decodes.
fn agg_trade_frame(stream: &str, symbol: &str) -> Message {
    Message::text(
        json!({"stream":stream,"data":{"e":"aggTrade","E":1,"a":5,"s":symbol,"p":"10.5","q":"2","f":100,"l":101,"T":1,"m":true,"M":true}})
            .to_string(),
    )
}

async fn next_control(peer: &mut WebSocketStream<TcpStream>) -> Value {
    let message = peer.next().await.unwrap().unwrap();
    assert!(message.is_text(), "expected a control message");
    serde_json::from_slice(message.as_payload()).unwrap()
}

/// The client sends nothing more but its close: an unsent refusal never reaches the venue.
async fn expect_only_close(mut peer: WebSocketStream<TcpStream>) {
    while let Some(message) = peer.next().await {
        let message = message.unwrap();
        assert!(!message.is_text(), "an unsent control reached the venue");
        if message.is_close() {
            peer.flush().await.unwrap();
            break;
        }
    }
}

async fn spot_market(
    url: &str,
    streams: &[spot::Stream],
) -> (spot::Streams, spot::ConnectionDriver) {
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(url)
        .unwrap();
    spot::Streams::connect(config, streams).await.unwrap()
}

async fn usdm_market(
    url: &str,
    streams: &[usdm::Stream],
) -> (usdm::Streams, usdm::ConnectionDriver) {
    let config = usdm::Config::with_pools(usdm::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(url)
        .unwrap();
    usdm::Streams::connect(config, usdm::Route::Market, streams)
        .await
        .unwrap()
}

async fn coinm_market(
    url: &str,
    streams: &[coinm::Stream],
) -> (coinm::Streams, coinm::ConnectionDriver) {
    let config = coinm::Config::with_pools(coinm::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(url)
        .unwrap();
    coinm::Streams::connect(config, streams).await.unwrap()
}

/// Live stream membership, identical in every market that offers it.
macro_rules! membership_contract {
    (
        $suite:ident,
        $product:ident,
        $connect:ident,
        $agg:ident,
        $agg_payload:ident,
        $path:literal,
        $incoming_limit:literal,
        $symbol_a:literal,
        $symbol_b:literal
    ) => {
        mod $suite {
            use super::*;
            use $product::{StreamEvent, Streams, event_payloads::MarketPayload, streams::StreamPayload};

            fn stream(symbol: &str) -> $product::Stream {
                $product::Stream::$agg(&Symbol::new(symbol).unwrap()).unwrap()
            }

            async fn established(events: &mut Streams) -> u64 {
                let generation = events.generation();
                assert!(matches!(events.recv().await, Some(StreamEvent::Established(g)) if g == generation));
                generation
            }

            async fn expect_trade(events: &mut Streams, generation: u64, name: &str) {
                match events.recv().await.unwrap() {
                    StreamEvent::Data {
                        generation: g,
                        payload: StreamPayload::Market { stream, payload: MarketPayload::$agg_payload(trade) },
                    } => {
                        assert_eq!(g, generation);
                        assert_eq!(stream, name);
                        assert_eq!(Option::<i64>::from(trade.a), Some(5));
                    }
                    other => panic!("unexpected event {other:?}"),
                }
            }

            async fn retire(
                events: &mut Streams,
                driver: tokio::task::JoinHandle<Result<(), Error>>,
                generation: u64,
            ) {
                events.close().await.unwrap();
                assert!(matches!(events.recv().await, Some(StreamEvent::Retired(g)) if g == generation));
                driver.await.unwrap().unwrap();
            }

            #[tokio::test]
            async fn subscribe_adds_a_stream_within_the_same_generation() {
                let (l, url) = listener().await;
                let (a, b) = (stream($symbol_a), stream($symbol_b));
                let added = b.name().to_owned();
                let wire = added.clone();
                let server = tokio::spawn(async move {
                    let mut peer = accept(l).await;
                    let request = next_control(&mut peer).await;
                    assert_eq!(request["method"], "SUBSCRIBE");
                    assert_eq!(request["params"], json!([wire]));
                    assert!(request["id"].is_u64());
                    assert_eq!(request.as_object().unwrap().len(), 3);
                    peer.send(Message::text(json!({"result":null,"id":request["id"]}).to_string())).await.unwrap();
                    peer.send(agg_trade_frame(&wire, $symbol_b)).await.unwrap();
                    finish(peer).await;
                });
                let (mut events, driver) = $connect(&url, &[a]).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                events.subscribe(&[b], deadline()).await.unwrap();
                expect_trade(&mut events, generation, &added).await;
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }

            #[tokio::test]
            async fn unsubscribe_stops_a_stream_without_retiring_the_generation() {
                let (l, url) = listener().await;
                let (a, b) = (stream($symbol_a), stream($symbol_b));
                let (kept, removed) = (a.name().to_owned(), b.name().to_owned());
                let wire = (kept.clone(), removed.clone());
                let server = tokio::spawn(async move {
                    let mut peer = accept(l).await;
                    let request = next_control(&mut peer).await;
                    assert_eq!(request["method"], "UNSUBSCRIBE");
                    assert_eq!(request["params"], json!([wire.1]));
                    peer.send(Message::text(json!({"result":null,"id":request["id"]}).to_string())).await.unwrap();
                    peer.send(agg_trade_frame(&wire.0, $symbol_a)).await.unwrap();
                    expect_only_close(peer).await;
                });
                let (mut events, driver) = $connect(&url, &[a, b.clone()]).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                events.unsubscribe(std::slice::from_ref(&b), deadline()).await.unwrap();
                expect_trade(&mut events, generation, &kept).await;
                // The released stream is no longer a member, so releasing it again is refused unsent.
                assert!(matches!(
                    events.unsubscribe(&[b], deadline()).await,
                    Err(Error::Validation(_))
                ));
                assert_ne!(kept, removed);
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }

            #[tokio::test]
            async fn a_venue_error_reply_is_a_typed_refusal_naming_its_code() {
                let (l, url) = listener().await;
                let (a, b) = (stream($symbol_a), stream($symbol_b));
                let kept = a.name().to_owned();
                let wire = kept.clone();
                let server = tokio::spawn(async move {
                    let mut peer = accept(l).await;
                    let request = next_control(&mut peer).await;
                    assert_eq!(request["method"], "SUBSCRIBE");
                    peer.send(Message::text(
                        json!({"code":2,"msg":"Invalid request: too many parameters","id":request["id"]}).to_string(),
                    ))
                    .await
                    .unwrap();
                    peer.send(agg_trade_frame(&wire, $symbol_a)).await.unwrap();
                    expect_only_close(peer).await;
                });
                let (mut events, driver) = $connect(&url, &[a]).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                let error = events.subscribe(std::slice::from_ref(&b), deadline()).await.unwrap_err();
                assert_eq!(error.outcome(), Some(Outcome::Rejected));
                match error {
                    Error::ControlRefused { operation, code, message } => {
                        assert_eq!(operation, "SUBSCRIBE");
                        assert_eq!(code, 2);
                        assert_eq!(message.as_deref(), Some("Invalid request: too many parameters"));
                    }
                    other => panic!("unexpected error {other:?}"),
                }
                // The generation lives on, and the refused stream never became a member.
                expect_trade(&mut events, generation, &kept).await;
                assert!(matches!(
                    events.unsubscribe(&[b], deadline()).await,
                    Err(Error::Validation(_))
                ));
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }

            #[tokio::test]
            async fn control_messages_and_pongs_share_the_incoming_message_ceiling() {
                let (l, url) = listener().await;
                let (ponged, pongs) = oneshot::channel();
                let server = tokio::spawn(async move {
                    let mut peer = accept(l).await;
                    for _ in 0..2 {
                        peer.send(Message::ping(b"ping".as_slice())).await.unwrap();
                        assert!(peer.next().await.unwrap().unwrap().is_pong());
                    }
                    ponged.send(()).unwrap();
                    for _ in 0..($incoming_limit - 2) {
                        let request = next_control(&mut peer).await;
                        assert_eq!(request["method"], "LIST_SUBSCRIPTIONS");
                        peer.send(Message::text(json!({"result":[],"id":request["id"]}).to_string())).await.unwrap();
                    }
                    expect_only_close(peer).await;
                });
                let (mut events, driver) = $connect(&url, &[stream($symbol_a)]).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                pongs.await.unwrap();
                for _ in 0..($incoming_limit - 2) {
                    assert!(events.list_subscriptions(deadline()).await.unwrap().is_empty());
                }
                match events.list_subscriptions(deadline()).await {
                    Err(Error::Admission { retry_after }) => {
                        assert!(retry_after > Duration::ZERO && retry_after <= Duration::from_secs(1));
                    }
                    other => panic!("expected an unsent refusal, got {other:?}"),
                }
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }

            #[tokio::test]
            async fn an_empty_routed_socket_connects_and_accepts_a_later_subscribe() {
                let (l, url) = listener().await;
                let a = stream($symbol_a);
                let added = a.name().to_owned();
                let wire = added.clone();
                let server = tokio::spawn(async move {
                    let (tcp, _) = l.accept().await.unwrap();
                    let (request, mut peer) = ServerBuilder::new().accept(tcp).await.unwrap();
                    assert_eq!(request.uri().path(), $path);
                    assert_eq!(request.uri().query(), None);
                    let request = next_control(&mut peer).await;
                    assert_eq!(request["method"], "SUBSCRIBE");
                    assert_eq!(request["params"], json!([wire]));
                    peer.send(Message::text(json!({"result":null,"id":request["id"]}).to_string())).await.unwrap();
                    peer.send(agg_trade_frame(&wire, $symbol_a)).await.unwrap();
                    finish(peer).await;
                });
                let (mut events, driver) = $connect(&url, &[]).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                events.subscribe(&[a], deadline()).await.unwrap();
                expect_trade(&mut events, generation, &added).await;
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }

            #[tokio::test]
            async fn list_subscriptions_returns_the_venues_answer() {
                let (l, url) = listener().await;
                let a = stream($symbol_a);
                let subscribed = a.name().to_owned();
                let wire = subscribed.clone();
                let server = tokio::spawn(async move {
                    let mut peer = accept(l).await;
                    let request = next_control(&mut peer).await;
                    assert_eq!(request, json!({"method":"LIST_SUBSCRIPTIONS","id":request["id"]}));
                    assert!(request["id"].is_u64());
                    peer.send(Message::text(
                        json!({"result":[wire,"other@kline_1m"],"id":request["id"]}).to_string(),
                    ))
                    .await
                    .unwrap();
                    finish(peer).await;
                });
                let (mut events, driver) = $connect(&url, &[a]).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                // The venue's answer is returned as given, even a stream this client never named.
                assert_eq!(
                    events.list_subscriptions(deadline()).await.unwrap(),
                    vec![subscribed, "other@kline_1m".to_owned()]
                );
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }

            #[tokio::test]
            async fn an_answer_after_the_deadline_is_attributed_as_late() {
                let (l, url) = listener().await;
                let b = stream($symbol_b);
                let added = b.name().to_owned();
                let (seen, sent) = oneshot::channel();
                let (release, gate) = oneshot::channel::<()>();
                let server = tokio::spawn(async move {
                    let mut peer = accept(l).await;
                    let request = next_control(&mut peer).await;
                    seen.send(()).unwrap();
                    gate.await.unwrap();
                    peer.send(Message::text(json!({"result":null,"id":request["id"]}).to_string())).await.unwrap();
                    finish(peer).await;
                });
                let (mut events, driver) = $connect(&url, &[stream($symbol_a)]).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                let late = [b];
                let (result, ()) = tokio::join!(events.subscribe(&late, deadline()), async {
                    sent.await.unwrap();
                    tokio::time::pause();
                    tokio::time::advance(Duration::from_secs(4)).await;
                });
                tokio::time::resume();
                assert_eq!(result.unwrap_err().outcome(), Some(Outcome::Unknown));
                release.send(()).unwrap();
                match events.recv().await.unwrap() {
                    StreamEvent::LateControl { generation: g, control, streams, result } => {
                        assert_eq!(g, generation);
                        assert_eq!(control, binance_client::StreamControl::Subscribe);
                        assert_eq!(streams, vec![added]);
                        assert!(matches!(result, Ok(None)));
                    }
                    other => panic!("unexpected event {other:?}"),
                }
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }

            #[tokio::test]
            async fn subscribing_past_the_documented_1024_streams_is_refused_unsent() {
                let (l, url) = listener().await;
                let server = tokio::spawn(async move { expect_only_close(accept(l).await).await });
                let full: Vec<_> = (0..1024).map(|i| stream(&format!("S{i}X"))).collect();
                let (mut events, driver) = $connect(&url, &full).await;
                let driver = tokio::spawn(driver.run());
                let generation = established(&mut events).await;
                assert!(matches!(
                    events.subscribe(&[stream("ONEMOREX")], deadline()).await,
                    Err(Error::Validation(_))
                ));
                retire(&mut events, driver, generation).await;
                server.await.unwrap();
            }
        }
    };
}

membership_contract!(
    spot_membership,
    spot,
    spot_market,
    agg_trade,
    AggTrade,
    "/stream",
    5,
    "BTCUSDT",
    "ETHUSDT"
);
membership_contract!(
    usdm_membership,
    usdm,
    usdm_market,
    aggregate_trade_streams,
    AggregateTradeStreams,
    "/market/stream",
    10,
    "BTCUSDT",
    "ETHUSDT"
);
membership_contract!(
    coinm_membership,
    coinm,
    coinm_market,
    aggregate_trade_streams,
    AggregateTradeStreams,
    "/stream",
    10,
    "BTCUSD_PERP",
    "ETHUSD_PERP"
);

#[tokio::test]
async fn a_market_stream_is_refused_on_the_public_route() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move { expect_only_close(accept(l).await).await });
    let config = usdm::Config::with_pools(usdm::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(&url)
        .unwrap();
    let symbol = Symbol::new("BTCUSDT").unwrap();
    let depth = usdm::Stream::diff_book_depth_streams(&symbol, "100ms").unwrap();
    let (mut events, driver) = usdm::Streams::connect(config, usdm::Route::Public, &[depth])
        .await
        .unwrap();
    let driver = tokio::spawn(driver.run());
    events.recv().await.unwrap();
    assert!(matches!(
        events
            .subscribe(
                &[usdm::Stream::aggregate_trade_streams(&symbol).unwrap()],
                deadline()
            )
            .await,
        Err(Error::Validation(_))
    ));
    events.close().await.unwrap();
    assert!(matches!(
        events.recv().await,
        Some(usdm::StreamEvent::Retired(_))
    ));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn a_user_data_socket_refuses_membership_changes() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move { expect_only_close(accept(l).await).await });
    let config = coinm::Config::with_pools(coinm::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(&url)
        .unwrap();
    let (mut events, driver) =
        coinm::Streams::user_data(config, &binance_client::SensitiveString::new("fixture"))
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    events.recv().await.unwrap();
    let symbol = Symbol::new("BTCUSD_PERP").unwrap();
    let stream = coinm::Stream::aggregate_trade_streams(&symbol).unwrap();
    assert!(matches!(
        events.subscribe(&[stream], deadline()).await,
        Err(Error::Validation(_))
    ));
    assert!(matches!(
        events.list_subscriptions(deadline()).await,
        Err(Error::Validation(_))
    ));
    events.close().await.unwrap();
    assert!(matches!(
        events.recv().await,
        Some(coinm::StreamEvent::Retired(_))
    ));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}
