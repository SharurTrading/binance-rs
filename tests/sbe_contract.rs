// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native binary stream decoding without floats, truncation defaults or guessed schemas.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic protocol assertions"
)]
use binance_client::{
    Decimal,
    spot::sbe::{MarketEvent, decode_market},
};

// Official schema 3:4: MyFiltersResponse has no fixed entry fields; each
// symbol filter is a length-prefixed nested message, not a zero-byte entry.
#[test]
fn api_filters_decode_zero_fixed_blocks_with_variable_entries() {
    let mut b = [0_u16, 105, 3, 4]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    b.extend_from_slice(&0_u16.to_le_bytes());
    b.extend_from_slice(&0_u32.to_le_bytes()); // exchange filters
    b.extend_from_slice(&0_u16.to_le_bytes());
    b.extend_from_slice(&1_u32.to_le_bytes()); // symbol filters
    b.push(17); // MaxPositionFilter: header plus exponent and mantissa
    for value in [9_u16, 12, 3, 4] {
        b.extend_from_slice(&value.to_le_bytes());
    }
    b.push((-3_i8).cast_unsigned());
    b.extend_from_slice(&1234_i64.to_le_bytes());
    b.extend_from_slice(&0_u16.to_le_bytes());
    b.extend_from_slice(&0_u32.to_le_bytes()); // asset filters
    let filters: binance_client::spot::rest_models::MyFiltersResponse =
        binance_client::spot::sbe::decode_api(&b).unwrap();
    assert!(filters.exchange_filters.unwrap().is_empty());
    assert!(matches!(&filters.symbol_filters.unwrap()[0],
        binance_client::spot::rest_models::MyFiltersResponseSymbolFiltersItem::MaxPosition(f)
        if f.max_position == "1.234".parse::<Decimal>().unwrap()));
    for n in 0..b.len() {
        assert!(binance_client::spot::sbe::decode_api::<serde_json::Value>(&b[..n]).is_err());
    }
    // Even an empty group must carry the schema's fixed width.
    b[8..10].copy_from_slice(&1_u16.to_le_bytes());
    assert!(binance_client::spot::sbe::decode_api::<serde_json::Value>(&b).is_err());
}

#[test]
fn api_list_status_preserves_the_documented_text_default() {
    // Official ListStatusEvent 606: three int64 fields, three enums,
    // optional subscription ID, and an empty orders group with fixed width 8.
    let mut b = [29_u16, 606, 3, 4]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    for value in [100_i64, 99, 42] {
        b.extend_from_slice(&value.to_le_bytes());
    }
    b.extend_from_slice(&[1, 1, 1]);
    b.extend_from_slice(&7_u16.to_le_bytes());
    b.extend_from_slice(&8_u16.to_le_bytes());
    b.extend_from_slice(&0_u16.to_le_bytes());
    b.push(7);
    b.extend_from_slice(b"BTCUSDT");
    b.push(6);
    b.extend_from_slice(b"list-1");
    b.push(0); // absent optional rejectReason projects to text "NONE"
    let value: serde_json::Value = binance_client::spot::sbe::decode_api(&b).unwrap();
    assert_eq!(value["subscriptionId"], 7);
    assert_eq!(value["event"]["r"], "NONE");
    assert_eq!(value["event"]["C"], "list-1");
    assert_eq!(value["event"]["O"], serde_json::json!([]));
}

#[test]
fn api_depth_uses_its_own_schema_and_exact_nested_price_levels() {
    let mut b = [10_u16, 200, 3, 4]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    b.extend_from_slice(&42_i64.to_le_bytes());
    b.extend_from_slice(&[(-8_i8).cast_unsigned(), (-3_i8).cast_unsigned()]);
    b.extend_from_slice(&16_u16.to_le_bytes());
    b.extend_from_slice(&1_u32.to_le_bytes());
    b.extend_from_slice(&100_000_001_i64.to_le_bytes());
    b.extend_from_slice(&2500_i64.to_le_bytes());
    b.extend_from_slice(&16_u16.to_le_bytes());
    b.extend_from_slice(&0_u32.to_le_bytes());
    let depth: binance_client::spot::rest_models::DepthResponse =
        binance_client::spot::sbe::decode_api(&b).unwrap();
    assert_eq!(depth.last_update_id, Some(42));
    assert_eq!(
        depth.bids.unwrap()[0].price,
        "1.00000001".parse::<Decimal>().unwrap()
    );
    assert!(decode_market(&b).is_err());
    for n in 0..b.len() {
        assert!(binance_client::spot::sbe::decode_api::<binance_client::spot::rest_models::DepthResponse>(&b[..n]).is_err());
    }
}

fn header(block: u16, template: u16) -> Vec<u8> {
    [block, template, 1, 0]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect()
}
fn quote() -> Vec<u8> {
    let mut b = header(50, 10001);
    b.extend_from_slice(&1_234_567_890_123_i64.to_le_bytes());
    b.extend_from_slice(&42_i64.to_le_bytes());
    b.extend_from_slice(&[(-8_i8).cast_unsigned(), (-3_i8).cast_unsigned()]);
    for n in [100_000_001_i64, 2500, 100_000_002, 3250] {
        b.extend_from_slice(&n.to_le_bytes());
    }
    b.push(7);
    b.extend_from_slice(b"BTCUSDT");
    b
}
#[test]
fn native_quote_preserves_exact_prices_asset_symbol_and_microseconds() {
    let MarketEvent::BestBidAsk(q) = decode_market(&quote()).unwrap() else {
        panic!("quote")
    };
    assert_eq!(q.event_time, 1_234_567_890_123);
    assert_eq!(q.book_update_id, 42);
    assert_eq!(q.symbol.as_str(), "BTCUSDT");
    assert_eq!(q.bid_price, "1.00000001".parse::<Decimal>().unwrap());
    assert_eq!(q.ask_qty, "3.250".parse::<Decimal>().unwrap());
}
#[test]
fn every_truncation_and_unknown_schema_is_refused() {
    let b = quote();
    for n in 0..b.len() {
        assert!(decode_market(&b[..n]).is_err(), "prefix {n}");
    }
    let mut unknown = b.clone();
    unknown[4] = 2;
    assert!(decode_market(&unknown).is_err());
    let mut unknown = b.clone();
    unknown[2..4].copy_from_slice(&10004_u16.to_le_bytes());
    assert!(decode_market(&unknown).is_err());
    // A template this build does not model is never read as the shape of a
    // neighbouring one, even when its block length would fit.
    for template in [10005_u16, 10001 + 100, u16::MAX] {
        let mut wrong = quote();
        wrong[2..4].copy_from_slice(&template.to_le_bytes());
        assert!(decode_market(&wrong).is_err(), "template {template}");
        assert!(binance_client::spot::sbe::decode_api::<
            binance_client::spot::rest_models::DepthResponse,
        >(&wrong)
        .is_err());
    }
    let mut trailing = b;
    trailing.push(1);
    assert!(decode_market(&trailing).is_err());
}
#[test]
fn decimal_scale_never_rounds_financial_wire_values() {
    let mut b = quote();
    b[24] = (-29_i8).cast_unsigned();
    assert!(decode_market(&b).is_err());
    let mut b = quote();
    b[24] = 127;
    assert!(decode_market(&b).is_err());
}

#[test]
fn trades_keep_native_order_boolean_and_constant_semantics() {
    let mut b = header(18, 10000);
    b.extend_from_slice(&100_i64.to_le_bytes());
    b.extend_from_slice(&99_i64.to_le_bytes());
    b.extend_from_slice(&[(-2_i8).cast_unsigned(), (-3_i8).cast_unsigned()]);
    b.extend_from_slice(&25_u16.to_le_bytes());
    b.extend_from_slice(&2_u32.to_le_bytes());
    for id in [30_i64, 31] {
        b.extend_from_slice(&id.to_le_bytes());
        b.extend_from_slice(&101_i64.to_le_bytes());
        b.extend_from_slice(&2005_i64.to_le_bytes());
        b.push(1);
    }
    b.push(7);
    b.extend_from_slice(b"BTCUSDT");
    let MarketEvent::Trades(t) = decode_market(&b).unwrap() else {
        panic!("trade")
    };
    assert_eq!(t.trades.iter().map(|v| v.id).collect::<Vec<_>>(), [30, 31]);
    assert!(t.trades.iter().all(|v| v.is_buyer_maker && v.is_best_match));
    assert_eq!(t.trades[0].qty, "2.005".parse::<Decimal>().unwrap());
    b[56] = 2;
    assert!(decode_market(&b).is_err());
}

#[test]
fn snapshot_and_diff_keep_their_distinct_update_ids_and_zero_deletions() {
    for diff in [false, true] {
        let mut b = header(if diff { 26 } else { 18 }, if diff { 10003 } else { 10002 });
        b.extend_from_slice(&100_i64.to_le_bytes());
        if diff {
            b.extend_from_slice(&41_i64.to_le_bytes());
        }
        b.extend_from_slice(&42_i64.to_le_bytes());
        b.extend_from_slice(&[(-2_i8).cast_unsigned(), (-3_i8).cast_unsigned()]);
        b.extend_from_slice(&16_u16.to_le_bytes());
        b.extend_from_slice(&1_u16.to_le_bytes());
        b.extend_from_slice(&101_i64.to_le_bytes());
        b.extend_from_slice(&0_i64.to_le_bytes());
        b.extend_from_slice(&16_u16.to_le_bytes());
        b.extend_from_slice(&0_u16.to_le_bytes());
        b.push(7);
        b.extend_from_slice(b"BTCUSDT");
        match decode_market(&b).unwrap() {
            MarketEvent::DepthDiff(d) => {
                assert_eq!(d.first_book_update_id, 41);
                assert_eq!(d.last_book_update_id, 42);
                assert_eq!(d.bids[0].qty, Decimal::ZERO);
            }
            MarketEvent::DepthSnapshot(d) => {
                assert_eq!(d.book_update_id, 42);
                assert_eq!(d.bids[0].qty, Decimal::ZERO);
            }
            _ => panic!("depth"),
        }
        for n in 0..b.len() {
            assert!(decode_market(&b[..n]).is_err());
        }
    }
}

#[tokio::test]
async fn binary_ingress_drains_in_order_with_ping_and_explicit_retirement() {
    use binance_client::{
        Credentials, Error, Signer, Symbol,
        spot::sbe::{MarketConfig, MarketStream, MarketStreams, StreamEvent},
    };
    use futures_util::{SinkExt, StreamExt};
    use std::{sync::Arc, time::Duration};
    use tokio::net::TcpListener;
    use tokio_websockets::{Limits, Message, ServerBuilder};
    struct NoSigning;
    impl Signer for NoSigning {
        fn sign(&self, _: &[u8]) -> Result<String, Error> {
            Err(Error::Signing)
        }
    }
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}", listener.local_addr().unwrap());
    let (sent_tx, sent_rx) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let (request, mut ws) = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(stream)
            .await
            .unwrap();
        assert_eq!(
            request.headers().get("x-mbx-apikey").unwrap(),
            "synthetic-sbe-key"
        );
        assert!(
            request
                .uri()
                .to_string()
                .contains("streams=btcusdt%40bestBidAsk")
        );
        for _ in 0..3 {
            ws.send(Message::binary(quote())).await.unwrap();
        }
        ws.send(Message::ping(b"echo".as_slice())).await.unwrap();
        let pong = ws.next().await.unwrap().unwrap();
        assert!(pong.is_pong());
        assert_eq!(pong.as_payload().as_ref(), b"echo");
        sent_tx.send(()).unwrap();
        ws.send(Message::binary(vec![1, 2, 3])).await.unwrap();
        while let Some(frame) = ws.next().await {
            if frame.unwrap().is_close() {
                break;
            }
        }
    });
    let credentials =
        Credentials::external_ed25519("synthetic-sbe-key", Arc::new(NoSigning)).unwrap();
    let config = MarketConfig::production(credentials)
        .unwrap()
        .endpoint(&endpoint)
        .unwrap();
    let (mut events, driver) = MarketStreams::connect(
        config,
        &[MarketStream::best_bid_ask(&Symbol::new("BTCUSDT").unwrap()).unwrap()],
    )
    .await
    .unwrap();
    let owner = tokio::spawn(driver.run());
    tokio::time::timeout(Duration::from_secs(3), sent_rx)
        .await
        .unwrap()
        .unwrap();
    assert!(events.queue_stats().unwrap().depth >= 4);
    let Some(StreamEvent::Established(g)) = events.recv().await else {
        panic!("established")
    };
    for _ in 0..3 {
        let Some(StreamEvent::Data {
            generation,
            payload: MarketEvent::BestBidAsk(q),
        }) = events.recv().await
        else {
            panic!("data")
        };
        assert_eq!(generation, g);
        assert_eq!(q.book_update_id, 42);
    }
    assert!(
        matches!(events.recv().await, Some(StreamEvent::Gap { generation, error: Error::BinaryDecode { .. } }) if generation == g)
    );
    assert!(
        matches!(events.recv().await, Some(StreamEvent::Retired(generation)) if generation == g)
    );
    assert_eq!(events.queue_stats().unwrap().depth, 0);
    owner.await.unwrap().unwrap();
    server.await.unwrap();
}

#[test]
fn api_balance_event_retains_subscription_asset_and_partial_delta() {
    let mut b = [27_u16, 601, 3, 4]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    b.extend_from_slice(&100_i64.to_le_bytes());
    b.extend_from_slice(&99_i64.to_le_bytes());
    b.push((-3_i8).cast_unsigned());
    b.extend_from_slice(&(-125_i64).to_le_bytes());
    b.extend_from_slice(&42_i16.to_le_bytes());
    b.push(3);
    b.extend_from_slice(b"BNB");
    let value: serde_json::Value = binance_client::spot::sbe::decode_api(&b).unwrap();
    assert_eq!(value["subscriptionId"], 42);
    assert_eq!(value["event"]["e"], "balanceUpdate");
    assert_eq!(value["event"]["a"], "BNB");
    assert_eq!(value["event"]["d"], "-0.125");
}

#[test]
fn recursive_response_envelopes_are_explicitly_refused() {
    fn envelope(result: &[u8]) -> Vec<u8> {
        let mut b = [3_u16, 50, 3, 4]
            .into_iter()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>();
        b.push(0);
        b.extend_from_slice(&200_u16.to_le_bytes());
        b.extend_from_slice(&19_u16.to_le_bytes());
        b.extend_from_slice(&0_u16.to_le_bytes());
        b.push(0);
        b.extend_from_slice(&u32::try_from(result.len()).unwrap().to_le_bytes());
        b.extend_from_slice(result);
        b
    }
    let inner = envelope(&[]);
    let outer = envelope(&inner);
    assert!(matches!(
        binance_client::spot::sbe::decode_api::<serde_json::Value>(&outer),
        Err(binance_client::Error::BinaryDecode {
            reason: "recursive binary envelope",
            ..
        })
    ));
}

#[test]
fn sbe_klines_have_explicitly_absent_reserved_json_column() {
    let mut b = [2_u16, 203, 3, 4]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    b.extend_from_slice(&[(-8_i8).cast_unsigned(), (-3_i8).cast_unsigned()]);
    b.extend_from_slice(&120_u16.to_le_bytes());
    b.extend_from_slice(&1_u32.to_le_bytes());
    b.extend_from_slice(&100_i64.to_le_bytes());
    for n in [100_000_001_i64, 100_000_002, 100_000_000, 100_000_001] {
        b.extend_from_slice(&n.to_le_bytes());
    }
    b.extend_from_slice(&125_i128.to_le_bytes());
    b.extend_from_slice(&199_i64.to_le_bytes());
    b.extend_from_slice(&125_000_001_i128.to_le_bytes());
    b.extend_from_slice(&1_i64.to_le_bytes());
    b.extend_from_slice(&125_i128.to_le_bytes());
    b.extend_from_slice(&125_000_001_i128.to_le_bytes());
    let rows: Vec<binance_client::spot::wire::Kline> =
        binance_client::spot::sbe::decode_api(&b).unwrap();
    assert!(rows[0].reserved.is_null());
    assert_eq!(rows[0].open, "1.00000001".parse::<Decimal>().unwrap());
}

#[tokio::test]
async fn rest_sbe_negotiation_preserves_units_and_refuses_successful_json_fallback() {
    use binance_client::{TimeUnit, spot};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };
    for binary in [true, false] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = vec![];
            while !request.ends_with(b"\r\n\r\n") {
                request.push(stream.read_u8().await.unwrap());
            }
            let request = String::from_utf8(request).unwrap().to_lowercase();
            assert!(request.contains("accept: application/sbe\r\n"));
            assert!(request.contains("x-mbx-sbe: 3:4\r\n"));
            assert!(request.contains("x-mbx-time-unit: microsecond\r\n"));
            let mut body = [8_u16, 102, 3, 4]
                .into_iter()
                .flat_map(u16::to_le_bytes)
                .collect::<Vec<_>>();
            body.extend_from_slice(&1_790_471_000_123_456_i64.to_le_bytes());
            if !binary {
                body = br#"{"serverTime":1790471000123456}"#.to_vec();
            }
            let kind = if binary {
                "application/sbe"
            } else {
                "application/json"
            };
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nx-mbx-used-weight-1m: 42\r\nConnection: close\r\n\r\n",body.len()).as_bytes()).await.unwrap();
            stream.write_all(&body).await.unwrap();
        });
        let config = spot::Config::with_pools(spot::Environment::Demo, &binance_client::WeightPools::new())
            .unwrap()
            .rest_url(&endpoint)
            .unwrap();
        let client = spot::RestClient::new_sbe(config).unwrap();
        let response = client
            .time(
                &spot::rest_requests::Time::new(),
                tokio::time::Instant::now() + std::time::Duration::from_secs(3),
            )
            .await;
        if binary {
            let response = response.unwrap();
            assert_eq!(response.meta.time_unit, TimeUnit::Microseconds);
            assert_eq!(response.data.server_time, Some(1_790_471_000_123_456));
            assert_eq!(response.meta.rates.counters["x-mbx-used-weight-1m"], 42);
        } else {
            assert!(response.is_err());
        }
        server.await.unwrap();
    }
}

#[tokio::test]
async fn websocket_sbe_uses_binary_correlated_replies_and_explicit_format_override() {
    use binance_client::{RequestId, TimeUnit, spot};
    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpListener;
    use tokio_websockets::{Limits, Message, ServerBuilder};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let (request, mut peer) = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(stream)
            .await
            .unwrap();
        let query = url::Url::parse(&format!("ws://fixture{}", request.uri())).unwrap();
        let pairs = query.query_pairs().collect::<Vec<_>>();
        assert_eq!(
            pairs
                .iter()
                .filter(|(key, _)| key == "responseFormat")
                .count(),
            1
        );
        assert!(
            pairs
                .iter()
                .any(|(k, v)| k == "responseFormat" && v == "sbe")
        );
        assert!(pairs.iter().any(|(k, v)| k == "sbeSchemaId" && v == "3"));
        let request: serde_json::Value =
            serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
        assert_eq!(request["method"], "time");
        let id = request["id"].as_str().unwrap();
        let mut result = [8_u16, 102, 3, 4]
            .into_iter()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>();
        result.extend_from_slice(&1_790_471_000_123_456_i64.to_le_bytes());
        let mut b = [3_u16, 50, 3, 4]
            .into_iter()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>();
        b.push(0);
        b.extend_from_slice(&200_u16.to_le_bytes());
        b.extend_from_slice(&19_u16.to_le_bytes());
        b.extend_from_slice(&0_u16.to_le_bytes());
        b.push(u8::try_from(id.len()).unwrap());
        b.extend_from_slice(id.as_bytes());
        b.extend_from_slice(&u32::try_from(result.len()).unwrap().to_le_bytes());
        b.extend_from_slice(&result);
        peer.send(Message::binary(b)).await.unwrap();
        while let Some(frame) = peer.next().await {
            if frame.unwrap().is_close() {
                peer.flush().await.unwrap();
                break;
            }
        }
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &binance_client::WeightPools::new())
        .unwrap()
        .websocket_url(&endpoint)
        .unwrap();
    let (client, mut events, driver) = spot::WsClient::connect_sbe(config).await.unwrap();
    let owner = tokio::spawn(driver.run());
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::Established(_))
    ));
    let response = client
        .time(
            &spot::ws_requests::Time::new(),
            RequestId::new("binary-time").unwrap(),
            tokio::time::Instant::now() + std::time::Duration::from_secs(3),
        )
        .await
        .unwrap();
    assert_eq!(response.meta.time_unit, TimeUnit::Microseconds);
    assert_eq!(response.data.server_time, Some(1_790_471_000_123_456));
    client.close().await.unwrap();
    while let Some(event) = events.recv().await {
        if matches!(event, spot::ApiEvent::Retired(_)) {
            break;
        }
    }
    owner.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn binary_api_refuses_json_success_and_a_ban_without_retry_timing() {
    use binance_client::{Error, RequestId, spot};
    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpListener;
    use tokio_websockets::{Limits, Message, ServerBuilder};
    for status in [200, 418] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("ws://{}", listener.local_addr().unwrap());
        let peer = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut peer = ServerBuilder::new()
                .limits(Limits::unlimited())
                .accept(stream)
                .await
                .unwrap()
                .1;
            let request: serde_json::Value =
                serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
            let reply = if status == 200 {
                serde_json::json!({"id":request["id"],"status":200,"result":{"serverTime":100}})
            } else {
                serde_json::json!({"id":request["id"],"status":418,"error":{"code":-1003,"msg":"synthetic ban"}})
            };
            peer.send(Message::text(reply.to_string())).await.unwrap();
            let frame = peer.next().await.unwrap().unwrap();
            assert!(
                frame.is_close(),
                "a second application send must be refused"
            );
            peer.flush().await.unwrap();
        });
        let config = spot::Config::with_pools(spot::Environment::Demo, &binance_client::WeightPools::new())
            .unwrap()
            .websocket_url(&endpoint)
            .unwrap();
        let (client, mut events, driver) = spot::WsClient::connect_sbe(config).await.unwrap();
        let owner = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        assert!(
            client
                .time(
                    &spot::ws_requests::Time::new(),
                    RequestId::new("first").unwrap(),
                    tokio::time::Instant::now() + std::time::Duration::from_secs(3)
                )
                .await
                .is_err()
        );
        if status == 418 {
            assert!(matches!(
                client
                    .time(
                        &spot::ws_requests::Time::new(),
                        RequestId::new("banned").unwrap(),
                        tokio::time::Instant::now() + std::time::Duration::from_secs(3)
                    )
                    .await,
                Err(Error::CooldownTimingUnknown)
            ));
            client.close().await.unwrap();
        }
        while let Some(event) = events.recv().await {
            if matches!(event, spot::ApiEvent::Retired(_)) {
                break;
            }
        }
        owner.await.unwrap().unwrap();
        peer.await.unwrap();
    }
}

#[allow(dead_code, reason = "shared credential-free transport fixture helpers")]
mod support;
#[tokio::test]
async fn malformed_binary_mutation_retains_ids_status_rates_and_never_retries() {
    use binance_client::{Credentials, Error, Outcome, Symbol, spot};
    let body = String::from_utf8(
        [24_u16, 300, 3, 4]
            .into_iter()
            .flat_map(u16::to_le_bytes)
            .collect(),
    )
    .unwrap();
    let fixture = support::HttpFixture::new(
        200,
        "Content-Type: application/sbe\r\nx-mbx-used-weight-1m: 42\r\n",
        &body,
        None,
        false,
    )
    .await;
    let config = spot::Config::with_pools(spot::Environment::Demo, &binance_client::WeightPools::new())
        .unwrap()
        .rest_url(&fixture.url)
        .unwrap()
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap());
    let client = spot::RestClient::new_sbe(config).unwrap();
    let request = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity("0.125".parse().unwrap())
        .new_client_order_id(spot::ClientOrderId::new("binary-no-retry").unwrap());
    let error = client
        .new_order(
            &request,
            tokio::time::Instant::now() + std::time::Duration::from_secs(3),
        )
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        client_order_ids,
        meta: Some(meta),
        ..
    } = error
    else {
        panic!("safe mutation evidence")
    };
    assert_eq!(client_order_ids["newClientOrderId"], "binary-no-retry");
    assert_eq!(meta.status, 200);
    assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 42);
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[test]
fn null_required_market_timestamp_and_identity_are_not_native_values() {
    for range in [8..16, 16..24] {
        let mut bytes = quote();
        bytes[range].copy_from_slice(&i64::MIN.to_le_bytes());
        assert!(decode_market(&bytes).is_err());
    }
}

#[test]
fn market_stream_constructor_preserves_unicode_and_refuses_delimiter_injection() {
    use binance_client::{Symbol, spot::sbe::MarketStream};
    let name = MarketStream::trades(&Symbol::new("测试币USDT").unwrap()).unwrap();
    assert_eq!(name.name(), "测试币usdt@trade");
    for symbol in ["A/B", "A@B", "A B", "A?B"] {
        assert!(MarketStream::trades(&Symbol::new(symbol).unwrap()).is_err());
    }
}
