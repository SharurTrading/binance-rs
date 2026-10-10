// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Conditional isolated-wallet evidence follows the official stream schemas.
use binance_client::{coinm, usdm};
use serde_json::json;

fn position() -> serde_json::Value {
    json!({"s":"BTCUSDT","pa":"1","ep":"100","cr":"0","up":"-1","mt":"cross","ps":"BOTH"})
}

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
