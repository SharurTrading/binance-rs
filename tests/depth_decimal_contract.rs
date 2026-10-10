// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Depth levels keep exact signed prices and refuse numbers Decimal cannot retain exactly.
use binance_client::core_trading::{coinm, spot, usdm};

#[test]
fn futures_depth_refuses_nonrepresentable_price_and_quantity() {
    for token in [
        "0.12345678901234567890123456789",
        "1e-29",
        "79228162514264337593543950336",
    ] {
        for level in [format!(r#"["{token}","1"]"#), format!(r#"["1","{token}"]"#)] {
            let json = format!(
                r#"{{"e":"depthUpdate","E":1,"T":1,"s":"BTCUSDT","U":1,"u":2,"pu":0,"b":[{level}],"a":[]}}"#
            );
            assert!(
                serde_json::from_str::<usdm::stream_models::PartialBookDepthStreamsEvent>(&json)
                    .is_err()
            );
            assert!(
                serde_json::from_str::<coinm::stream_models::PartialBookDepthStreamsEvent>(&json)
                    .is_err()
            );
        }
    }
}

#[test]
fn futures_depth_keeps_exact_signed_prices_and_fractional_quantities() {
    let json = r#"{"e":"depthUpdate","E":1,"T":1,"s":"BTCUSDT","U":1,"u":2,"pu":0,"b":[["-0.125","0.0000000000000000000000000001"]],"a":[["0","79228162514264337593543950335"]]}"#;
    let usdm: usdm::stream_models::PartialBookDepthStreamsEvent =
        serde_json::from_str(json).unwrap();
    let coinm: coinm::stream_models::PartialBookDepthStreamsEvent =
        serde_json::from_str(json).unwrap();
    for (bid, ask) in [(usdm.b, usdm.a), (coinm.b, coinm.a)] {
        assert_eq!(bid[0][0], rust_decimal::Decimal::new(-125, 3));
        assert_eq!(bid[0][1], rust_decimal::Decimal::new(1, 28));
        assert_eq!(ask[0][0], rust_decimal::Decimal::ZERO);
        assert_eq!(ask[0][1], rust_decimal::Decimal::MAX);
    }
}

const ZERO_AND_NEGATIVE_LEVELS: &str = r#""b":[["0","1.5"]],"a":[["-37.625","0.25"]]"#;

fn spot_event() -> String {
    format!(
        r#"{{"e":"depthUpdate","E":1,"s":"BNBBTC","U":157,"u":160,{ZERO_AND_NEGATIVE_LEVELS}}}"#
    )
}

fn futures_event() -> String {
    format!(
        r#"{{"e":"depthUpdate","E":1,"T":1,"s":"BTCUSDT","U":157,"u":160,"pu":156,{ZERO_AND_NEGATIVE_LEVELS}}}"#
    )
}

fn snapshot() -> String {
    r#"{"lastUpdateId":160,"bids":[["0","1.5"]],"asks":[["-37.625","0.25"]]}"#.to_owned()
}

fn assert_zero_bid_and_negative_ask(
    bid: (rust_decimal::Decimal, rust_decimal::Decimal),
    ask: (rust_decimal::Decimal, rust_decimal::Decimal),
) {
    assert_eq!(
        bid,
        (
            rust_decimal::Decimal::ZERO,
            rust_decimal::Decimal::new(15, 1)
        )
    );
    assert_eq!(
        ask,
        (
            rust_decimal::Decimal::new(-37_625, 3),
            rust_decimal::Decimal::new(25, 2)
        )
    );
}

#[test]
fn spot_diff_depth_and_snapshots_keep_zero_and_negative_level_prices() {
    let event: spot::stream_models::DiffBookDepthEvent =
        serde_json::from_str(&spot_event()).unwrap();
    assert_eq!((event.upper_u, event.u), (157, 160));
    assert_zero_bid_and_negative_ask(
        (event.b[0].price, event.b[0].quantity),
        (event.a[0].price, event.a[0].quantity),
    );
    let rest: spot::rest_models::DepthResponse = serde_json::from_str(&snapshot()).unwrap();
    let ws: spot::ws_models::DepthResponse = serde_json::from_str(&snapshot()).unwrap();
    for (last_update_id, bids, asks) in [
        (rest.last_update_id, rest.bids.unwrap(), rest.asks.unwrap()),
        (ws.last_update_id, ws.bids.unwrap(), ws.asks.unwrap()),
    ] {
        assert_eq!(last_update_id, Some(160));
        assert_zero_bid_and_negative_ask(
            (bids[0].price, bids[0].quantity),
            (asks[0].price, asks[0].quantity),
        );
    }
}

#[test]
fn usdm_diff_depth_and_snapshots_keep_zero_and_negative_level_prices() {
    let event: usdm::stream_models::DiffBookDepthStreamsEvent =
        serde_json::from_str(&futures_event()).unwrap();
    let rpi: usdm::stream_models::RpiDiffBookDepthStreamsEvent =
        serde_json::from_str(&futures_event()).unwrap();
    for (ids, bids, asks) in [
        ((event.upper_u, event.u, event.pu), event.b, event.a),
        ((rpi.upper_u, rpi.u, rpi.pu), rpi.b, rpi.a),
    ] {
        assert_eq!(ids, (157, 160, 156));
        assert_zero_bid_and_negative_ask(
            (bids[0].price, bids[0].quantity),
            (asks[0].price, asks[0].quantity),
        );
    }
    let book: usdm::rest_models::OrderBookResponse = serde_json::from_str(&snapshot()).unwrap();
    let rpi_book: usdm::rest_models::RpiOrderBookResponse =
        serde_json::from_str(&snapshot()).unwrap();
    for (last_update_id, bids, asks) in [
        (book.last_update_id, book.bids.unwrap(), book.asks.unwrap()),
        (
            rpi_book.last_update_id,
            rpi_book.bids.unwrap(),
            rpi_book.asks.unwrap(),
        ),
    ] {
        assert_eq!(last_update_id, Some(160));
        assert_zero_bid_and_negative_ask(
            (bids[0].price, bids[0].quantity),
            (asks[0].price, asks[0].quantity),
        );
    }
}

#[test]
fn coinm_diff_depth_and_snapshot_keep_zero_and_negative_level_prices() {
    let event: coinm::stream_models::DiffBookDepthStreamsEvent =
        serde_json::from_str(&futures_event()).unwrap();
    assert_eq!((event.upper_u, event.u, event.pu), (157, 160, 156));
    assert_zero_bid_and_negative_ask(
        (event.b[0].price, event.b[0].quantity),
        (event.a[0].price, event.a[0].quantity),
    );
    let book: coinm::rest_models::OrderBookResponse = serde_json::from_str(&snapshot()).unwrap();
    assert_eq!(book.last_update_id, Some(160));
    assert_zero_bid_and_negative_ask(
        (
            book.bids.as_ref().unwrap()[0].price,
            book.bids.as_ref().unwrap()[0].quantity,
        ),
        (
            book.asks.as_ref().unwrap()[0].price,
            book.asks.as_ref().unwrap()[0].quantity,
        ),
    );
}

#[test]
fn malformed_level_prices_are_refused_in_every_market() {
    for level in [
        r#"["NaN","1"]"#,
        r#"["one","1"]"#,
        r#"["","1"]"#,
        r#"["--1","1"]"#,
        r#"["1.2.3","1"]"#,
        r#"[true,"1"]"#,
        r#"[null,"1"]"#,
        r#"["79228162514264337593543950336","1"]"#,
    ] {
        assert!(
            serde_json::from_str::<spot::wire::PriceLevel>(level).is_err(),
            "{level}"
        );
        assert!(
            serde_json::from_str::<usdm::wire::PriceLevel>(level).is_err(),
            "{level}"
        );
        assert!(
            serde_json::from_str::<coinm::wire::PriceLevel>(level).is_err(),
            "{level}"
        );
    }
}

#[test]
fn futures_levels_refuse_a_negative_absolute_quantity() {
    let level = r#"["100","-0.5"]"#;
    assert!(serde_json::from_str::<usdm::wire::PriceLevel>(level).is_err());
    assert!(serde_json::from_str::<coinm::wire::PriceLevel>(level).is_err());
}

#[test]
fn spot_level_keeps_a_negative_quantity_exactly() {
    let level: spot::wire::PriceLevel = serde_json::from_str(r#"["100","-0.5"]"#).unwrap();
    assert_eq!(
        (level.price, level.quantity),
        (
            rust_decimal::Decimal::new(100, 0),
            rust_decimal::Decimal::new(-5, 1)
        )
    );
}
