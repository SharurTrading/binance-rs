// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Conditional isolated-wallet evidence follows the official stream schemas.
use binance_client::{coinm, usdm};
use serde_json::json;

fn position() -> serde_json::Value {
    json!({"s":"BTCUSDT","pa":"1","ep":"100","cr":"0","up":"-1","mt":"cross","ps":"BOTH"})
}

// Loads a recorded capture inside the calling test: the recorded evidence is
// compile-time checked in, like the synthetic fixtures beside it.
macro_rules! captured_cross_margin_fixture {
    ($market:literal) => {
        serde_json::from_str::<serde_json::Value>(include_str!(concat!(
            "fixtures/",
            $market,
            "-cross-margin-account-update-2026-10-11.json"
        )))
        .unwrap()
    };
}

/// The authorized 2026-10-11 demo capture for issue #69: both markets'
/// one-way cross-margin `ACCOUNT_UPDATE` pushes carried `iw` as a reported
/// zero rather than omitting it, and every money field stays exact.
#[test]
fn captured_usdm_cross_margin_update_decodes_exactly() {
    let fixture = captured_cross_margin_fixture!("usdm");
    assert_eq!(fixture["market"], "usdm");
    assert_eq!(fixture["credentialed"], json!(true));
    let events = fixture["events"].as_array().unwrap();
    let expected = [("open", "0.001"), ("close", "0")];
    assert_eq!(events.len(), expected.len());
    for (event, (phase, pa)) in events.iter().zip(expected) {
        assert_eq!(event["phase"], phase);
        let raw: serde_json::Value = serde_json::from_str(event["raw"].as_str().unwrap()).unwrap();
        assert_eq!(raw["e"], "ACCOUNT_UPDATE");
        let decoded: usdm::stream_models::AccountUpdateEvent =
            serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(decoded.a.m, usdm::enums::AccountUpdateReason::Order);
        let raw_balance = &raw["a"]["B"][0];
        let balances = decoded.a.upper_b.as_ref().unwrap();
        assert_eq!(balances[0].a, "USDT");
        for (field, decoded_balance) in [
            ("wb", balances[0].wb),
            ("cw", balances[0].cw),
            ("bc", balances[0].bc),
        ] {
            assert_eq!(
                decoded_balance,
                binance_client::Decimal::from_str_exact(raw_balance[field].as_str().unwrap())
                    .unwrap()
            );
        }
        let raw_position = &raw["a"]["P"][0];
        let position = &decoded.a.upper_p.as_ref().unwrap()[0];
        assert_eq!(position.s, "BTCUSDT");
        assert_eq!(position.mt, "cross");
        assert_eq!(position.ps, usdm::enums::PositionSide::Both);
        assert_eq!(position.iw, Some(binance_client::Decimal::ZERO));
        assert_eq!(
            position.pa,
            binance_client::Decimal::from_str_exact(pa).unwrap()
        );
        // The exact break-even price survives the capture on this market too.
        assert_eq!(
            position.bep,
            Some(
                binance_client::Decimal::from_str_exact(raw_position["bep"].as_str().unwrap())
                    .unwrap()
            )
        );
        // The margin asset is retained evidence even while untyped.
        assert!(
            position
                .extra
                .get("ma")
                .is_some_and(serde_json::Value::is_string)
        );
        for field in ["ep", "cr", "up"] {
            let exact =
                binance_client::Decimal::from_str_exact(raw_position[field].as_str().unwrap())
                    .unwrap();
            assert_eq!(
                match field {
                    "ep" => position.ep,
                    "cr" => position.cr,
                    _ => position.up,
                },
                exact
            );
        }
    }
}

#[test]
fn captured_coinm_cross_margin_update_decodes_exactly() {
    let fixture = captured_cross_margin_fixture!("coinm");
    assert_eq!(fixture["market"], "coinm");
    assert_eq!(fixture["credentialed"], json!(true));
    let events = fixture["events"].as_array().unwrap();
    let expected = [("open", "1"), ("close", "0")];
    assert_eq!(events.len(), expected.len());
    for (event, (phase, pa)) in events.iter().zip(expected) {
        assert_eq!(event["phase"], phase);
        let raw: serde_json::Value = serde_json::from_str(event["raw"].as_str().unwrap()).unwrap();
        assert_eq!(raw["e"], "ACCOUNT_UPDATE");
        let decoded: coinm::stream_models::AccountUpdateEvent =
            serde_json::from_value(raw.clone()).unwrap();
        // The venue's account-alias field is redacted before recording; its
        // absence is part of the fixture contract.
        assert_eq!(decoded.i, None);
        assert_eq!(decoded.a.m, Some(coinm::enums::AccountUpdateReason::Order));
        let raw_balance = &raw["a"]["B"][0];
        let balances = decoded.a.upper_b.as_ref().unwrap();
        assert_eq!(balances[0].a.as_str(), "BTC");
        for (field, decoded_balance) in [
            ("wb", balances[0].wb),
            ("cw", balances[0].cw),
            ("bc", balances[0].bc),
        ] {
            assert_eq!(
                decoded_balance,
                binance_client::Decimal::from_str_exact(raw_balance[field].as_str().unwrap())
                    .unwrap()
            );
        }
        let raw_position = &raw["a"]["P"][0];
        let position = &decoded.a.upper_p.as_ref().unwrap()[0];
        assert_eq!(position.s.as_str(), "BTCUSD_PERP");
        assert_eq!(position.mt, "cross");
        assert_eq!(position.ps, coinm::enums::PositionSide::Both);
        assert_eq!(position.iw, Some(binance_client::Decimal::ZERO));
        assert_eq!(
            position.pa,
            binance_client::Decimal::from_str_exact(pa).unwrap()
        );
        // The inverse market's exact break-even price survives the capture.
        assert_eq!(
            position.bep,
            Some(
                binance_client::Decimal::from_str_exact(raw_position["bep"].as_str().unwrap())
                    .unwrap()
            )
        );
        // The margin asset is retained evidence even while untyped.
        assert!(
            position
                .extra
                .as_value()
                .get("ma")
                .is_some_and(serde_json::Value::is_string)
        );
        for field in ["ep", "cr", "up"] {
            let exact =
                binance_client::Decimal::from_str_exact(raw_position[field].as_str().unwrap())
                    .unwrap();
            assert_eq!(
                match field {
                    "ep" => position.ep,
                    "cr" => position.cr,
                    _ => position.up,
                },
                exact
            );
        }
    }
}

// The recorded demo evidence replays through the real user-data socket path:
// every captured frame is delivered in source order, none becomes a gap.
macro_rules! captured_wallet_socket {
    ($test:ident, $market:ident, $fixture:literal, $expected_pas:expr) => {
        #[tokio::test]
        async fn $test() {
            use binance_client::$market::{Config, Environment, StreamEvent, Streams, event_payloads::UserPayload, streams::StreamPayload};
            use binance_client::{SensitiveString, WeightPools};
            use futures_util::{SinkExt, StreamExt};
            use tokio_websockets::{Message, ServerBuilder};
            let fixture: serde_json::Value = serde_json::from_str(include_str!($fixture)).unwrap();
            let raws: Vec<String> = fixture["events"]
                .as_array()
                .unwrap()
                .iter()
                .map(|event| event["raw"].as_str().unwrap().to_owned())
                .collect();
            let server_raws = raws.clone();
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url = format!("ws://{}", listener.local_addr().unwrap());
            let server = tokio::spawn(async move {
                let (socket, _) = listener.accept().await.unwrap();
                let (_, mut ws) = ServerBuilder::new().accept(socket).await.unwrap();
                for raw in server_raws {
                    ws.send(Message::text(raw)).await.unwrap();
                }
                while let Some(message) = ws.next().await {
                    if message.unwrap().is_close() {
                        ws.flush().await.unwrap();
                        break;
                    }
                }
            });
            let config = Config::with_pools(Environment::Demo, &WeightPools::new()).unwrap().streams_url(&url).unwrap();
            let (mut stream, driver) = Streams::user_data(config, &SensitiveString::new("synthetic-listen-key")).await.unwrap();
            let driver = tokio::spawn(driver.run());
            let generation = stream.generation();
            assert!(matches!(stream.recv().await, Some(StreamEvent::Established(g)) if g == generation));
            let mut delivered_pas = Vec::new();
            for _ in &raws {
                let Some(StreamEvent::Data { generation: g, payload: StreamPayload::User(UserPayload::AccountUpdate(update)) }) = stream.recv().await else {
                    panic!("captured cross-margin evidence became a socket gap");
                };
                assert_eq!(g, generation);
                for position in update.a.upper_p.as_ref().unwrap() {
                    assert_eq!(position.mt, "cross");
                    assert_eq!(position.iw, Some(binance_client::Decimal::ZERO));
                    delivered_pas.push(position.pa.to_string());
                }
            }
            // Source order is part of ingress, not just frame count.
            assert_eq!(delivered_pas, $expected_pas);
            stream.close().await.unwrap();
            assert!(matches!(stream.recv().await, Some(StreamEvent::Retired(g)) if g == generation));
            driver.await.unwrap().unwrap();
            server.await.unwrap();
        }
    };
}
captured_wallet_socket!(
    usdm_captured_cross_margin_evidence_replays_through_the_user_data_socket,
    usdm,
    "fixtures/usdm-cross-margin-account-update-2026-10-11.json",
    vec!["0.001", "0"]
);
captured_wallet_socket!(
    coinm_captured_cross_margin_evidence_replays_through_the_user_data_socket,
    coinm,
    "fixtures/coinm-cross-margin-account-update-2026-10-11.json",
    vec!["1", "0"]
);

#[test]
fn account_updates_preserve_absent_isolated_wallet_without_fabricating_zero() {
    let value = position();
    let linear: usdm::stream_models::AccountUpdateEventAUpperPItem =
        serde_json::from_value(value.clone()).unwrap();
    assert_eq!(linear.iw, None);
    let mut inverse = value;
    inverse["s"] = json!("BTCUSD_PERP");
    let inverse: coinm::stream_models::AccountUpdateEventAUpperPItem =
        serde_json::from_value(inverse).unwrap();
    assert_eq!(inverse.iw, None);
}

#[test]
fn supplied_zero_isolated_wallet_is_distinct_from_absence() {
    let mut value = position();
    value["iw"] = json!("0");
    let linear: usdm::stream_models::AccountUpdateEventAUpperPItem =
        serde_json::from_value(value.clone()).unwrap();
    assert_eq!(linear.iw, Some(binance_client::Decimal::ZERO));
    value["s"] = json!("BTCUSD_PERP");
    let inverse: coinm::stream_models::AccountUpdateEventAUpperPItem =
        serde_json::from_value(value).unwrap();
    assert_eq!(inverse.iw, Some(binance_client::Decimal::ZERO));
    let call: usdm::stream_models::MarginCallEventPItem = serde_json::from_value(json!({
        "s":"BTCUSDT","ps":"LONG","pa":"1","mt":"CROSSED","mp":"99",
        "up":"-1","mm":"0.5","iw":"0"
    }))
    .unwrap();
    assert_eq!(call.iw, Some(binance_client::Decimal::ZERO));
}

#[test]
fn margin_call_preserves_absence_and_refuses_malformed_wallet_money() {
    let mut value =
        json!({"s":"BTCUSDT","ps":"LONG","pa":"1","mt":"CROSSED","mp":"99","up":"-1","mm":"0.5"});
    assert!(
        serde_json::from_value::<usdm::stream_models::MarginCallEventPItem>(value.clone()).is_ok()
    );
    value["iw"] = json!("not-money");
    assert!(serde_json::from_value::<usdm::stream_models::MarginCallEventPItem>(value).is_err());
}

// These are synthetic schema contracts, not captured account events. Optional
// iw follows each market's authoritative schema; no cross-account live claim.
macro_rules! conditional_wallet_socket {
    ($test:ident, $market:ident, $symbol:literal) => {
        #[tokio::test]
        async fn $test() {
            use binance_client::$market::{Config, Environment, StreamEvent, Streams, event_payloads::UserPayload, streams::StreamPayload};
            use binance_client::{SensitiveString, WeightPools};
            use futures_util::{SinkExt, StreamExt};
            use tokio_websockets::{Message, ServerBuilder};
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url = format!("ws://{}", listener.local_addr().unwrap());
            let server = tokio::spawn(async move {
                let (socket, _) = listener.accept().await.unwrap();
                let (_, mut ws) = ServerBuilder::new().accept(socket).await.unwrap();
                let mut p = position();
                p["s"] = json!($symbol);
                ws.send(Message::text(json!({"e":"ACCOUNT_UPDATE","E":1,"T":1,"a":{"m":"ORDER","P":[p]}}).to_string())).await.unwrap();
                let mut malformed = position();
                malformed["s"] = json!($symbol);
                malformed["iw"] = json!("not-money");
                ws.send(Message::text(json!({"e":"ACCOUNT_UPDATE","E":2,"T":2,"a":{"m":"ORDER","P":[malformed]}}).to_string())).await.unwrap();
                while let Some(message) = ws.next().await {
                    if message.unwrap().is_close() {
                        ws.flush().await.unwrap();
                        break;
                    }
                }
            });
            let config = Config::with_pools(Environment::Demo, &WeightPools::new()).unwrap().streams_url(&url).unwrap();
            let (mut stream, driver) = Streams::user_data(config, &SensitiveString::new("synthetic-listen-key")).await.unwrap();
            let driver = tokio::spawn(driver.run());
            let generation = stream.generation();
            assert!(matches!(stream.recv().await, Some(StreamEvent::Established(g)) if g == generation));
            let Some(StreamEvent::Data { generation: g, payload: StreamPayload::User(UserPayload::AccountUpdate(event)) }) = stream.recv().await else {
                panic!("optional wallet evidence became a socket gap");
            };
            assert_eq!(g, generation);
            let positions = event.a.upper_p.unwrap();
            assert_eq!(positions[0].iw, None);
            assert_eq!(positions[0].mt, "cross");
            assert_eq!(positions[0].pa, binance_client::Decimal::ONE);
            assert!(matches!(stream.recv().await, Some(StreamEvent::Gap { generation: g, error: binance_client::Error::Gap(_) }) if g == generation));
            stream.close().await.unwrap();
            assert!(matches!(stream.recv().await, Some(StreamEvent::Retired(g)) if g == generation));
            driver.await.unwrap().unwrap();
            server.await.unwrap();
        }
    };
}
conditional_wallet_socket!(
    usdm_missing_wallet_is_delivered_before_joined_retirement,
    usdm,
    "BTCUSDT"
);
conditional_wallet_socket!(
    coinm_missing_wallet_is_delivered_before_joined_retirement,
    coinm,
    "BTCUSD_PERP"
);

#[test]
fn inverse_break_even_price_refuses_malformed_or_unrepresentable_money() {
    for price in ["not-money", "0.12345678901234567890123456789"] {
        let mut value = position();
        value["s"] = json!("BTCUSD_PERP");
        value["bep"] = json!(price);
        assert!(
            serde_json::from_value::<coinm::stream_models::AccountUpdateEventAUpperPItem>(value)
                .is_err()
        );
    }
}
