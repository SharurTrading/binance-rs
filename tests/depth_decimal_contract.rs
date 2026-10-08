// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Futures depth refuses numbers that Decimal cannot retain exactly.
use binance_client::core_trading::{coinm, usdm};

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
