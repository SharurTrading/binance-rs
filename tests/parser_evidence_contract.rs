// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Required financial evidence and native unavailable-price sentinels.

#![allow(clippy::unwrap_used, reason = "synthetic contract assertions")]

use binance_client::{Decimal, coinm, spot, usdm};
use serde_json::json;

// Official COIN-M stream schema: index price is financial evidence; delivery
// symbols retain the funding-rate field as an empty string, never a zero.
// https://developers.binance.info/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/1.0.0/schema.yaml
fn coinm_mark() -> serde_json::Value {
    json!({"e":"markPriceUpdate","E":1_596_095_725_000_i64,"s":"BTCUSD_201225",
        "p":"10934.62615417","P":"10962.17178236","i":"10933.62615417","r":"","T":0,"st":2})
}

#[test]
fn coinm_mark_refuses_malformed_index_price() {
    let mut value = coinm_mark();
    value["i"] = json!("not-a-price");
    assert!(serde_json::from_value::<coinm::stream_models::MarkPriceStreamEvent>(value).is_err());
}

#[test]
fn coinm_mark_refuses_malformed_funding_rate() {
    let mut value = coinm_mark();
    value["r"] = json!("not-a-rate");
    assert!(serde_json::from_value::<coinm::stream_models::MarkPriceStreamEvent>(value).is_err());
}

#[test]
fn coinm_mark_refuses_malformed_observed_moving_average() {
    // #71 records public COIN-M wire observations of ap; it is optional because
    // the official COIN-M schema does not yet document its presence.
    let mut value = coinm_mark();
    value["ap"] = json!("not-a-price");
    assert!(serde_json::from_value::<coinm::stream_models::MarkPriceStreamEvent>(value).is_err());
}

#[test]
fn coinm_mark_funding_rate_requires_its_native_string_evidence() {
    let mut value = coinm_mark();
    value.as_object_mut().unwrap().remove("r");
    assert!(serde_json::from_value::<coinm::stream_models::MarkPriceStreamEvent>(value).is_err());
    let mut value = coinm_mark();
    value["r"] = serde_json::Value::Null;
    assert!(serde_json::from_value::<coinm::stream_models::MarkPriceStreamEvent>(value).is_err());
}

#[test]
fn coinm_marks_preserve_exact_prices_and_distinguish_empty_rate_from_zero() {
    for (rate, expected) in [
        ("", None),
        ("0.00000000", Some(Decimal::new(0, 8))),
        ("0.00008187", Some(Decimal::new(8_187, 8))),
    ] {
        let mut value = coinm_mark();
        value["r"] = json!(rate);
        value["ap"] = json!("10934.62615417");
        let event: coinm::stream_models::MarkPriceStreamEvent =
            serde_json::from_value(value.clone()).unwrap();
        assert_eq!(event.i, Decimal::new(1_093_362_615_417, 8));
        assert_eq!(event.p, Decimal::new(1_093_462_615_417, 8));
        assert_eq!(event.upper_p, Decimal::new(1_096_217_178_236, 8));
        assert_eq!(event.r, expected);
        assert_eq!(event.ap, Some(Decimal::new(1_093_462_615_417, 8)));
        assert_eq!(event.st, Some(2));
        let pair: Vec<coinm::stream_models::MarkPriceOfAllSymbolsOfAPairEventItem> =
            serde_json::from_value(json!([value])).unwrap();
        assert_eq!(pair[0].i, event.i);
        assert_eq!(pair[0].r, expected);
        assert_eq!(pair[0].ap, event.ap);
    }
    let event: coinm::stream_models::MarkPriceStreamEvent =
        serde_json::from_value(coinm_mark()).unwrap();
    assert_eq!(event.ap, None);
}

#[test]
fn coinm_marks_refuse_unrepresentable_values_and_missing_index_evidence() {
    for field in ["i", "r", "ap"] {
        let mut value = coinm_mark();
        value[field] = json!("0.12345678901234567890123456789");
        assert!(
            serde_json::from_value::<coinm::stream_models::MarkPriceStreamEvent>(value.clone())
                .is_err()
        );
        assert!(
            serde_json::from_value::<
                Vec<coinm::stream_models::MarkPriceOfAllSymbolsOfAPairEventItem>,
            >(json!([value]))
            .is_err()
        );
    }
    let mut value = coinm_mark();
    value.as_object_mut().unwrap().remove("i");
    assert!(serde_json::from_value::<coinm::stream_models::MarkPriceStreamEvent>(value).is_err());
}

#[test]
fn coinm_pair_marks_refuse_malformed_financial_evidence() {
    for field in ["i", "r", "ap"] {
        let mut value = coinm_mark();
        value[field] = json!("not-a-decimal");
        assert!(
            serde_json::from_value::<
                Vec<coinm::stream_models::MarkPriceOfAllSymbolsOfAPairEventItem>,
            >(json!([value]))
            .is_err()
        );
    }
}

// The current official USD-M Basis example deliberately sends an empty
// annualized rate for PERPETUAL; its other values remain exact financial data.
// https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#basis
#[test]
fn usdm_basis_accepts_documented_unavailable_annualized_rate() {
    let response: usdm::rest_models::BasisResponse = serde_json::from_value(json!([{
        "indexPrice":"34400.15945055","contractType":"PERPETUAL","basisRate":"0.0004",
        "futuresPrice":"34414.10","annualizedBasisRate":"","basis":"13.94054945",
        "pair":"BTCUSDT","timestamp":1_698_742_800_000_i64
    }]))
    .unwrap();
    assert_eq!(response[0].annualized_basis_rate, None);
    assert_eq!(
        response[0].index_price,
        Some(Decimal::new(3_440_015_945_055, 8))
    );
    assert_eq!(response[0].basis_rate, Some(Decimal::new(4, 4)));
    assert_eq!(response[0].futures_price, Some(Decimal::new(3_441_410, 2)));
    assert_eq!(response[0].basis, Some(Decimal::new(1_394_054_945, 8)));
    assert_eq!(response[0].timestamp, Some(1_698_742_800_000));
}

#[test]
fn usdm_basis_numeric_annualized_rate_stays_distinct_from_unavailable() {
    for rate in ["0", "0.0283"] {
        let response: usdm::rest_models::BasisResponse =
            serde_json::from_value(json!([{"annualizedBasisRate":rate}])).unwrap();
        assert_eq!(
            response[0].annualized_basis_rate,
            Some(rate.parse::<Decimal>().unwrap())
        );
    }
    for rate in ["not-a-rate", "null", " "] {
        assert!(
            serde_json::from_value::<usdm::rest_models::BasisResponse>(
                json!([{"annualizedBasisRate":rate}])
            )
            .is_err()
        );
    }
}

// Binance's reference-price push includes r, with JSON null when no price exists.
// https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#reference-price-streams
#[test]
fn spot_reference_price_requires_the_nullable_price_field() {
    assert!(
        serde_json::from_value::<spot::stream_models::ReferencePriceEvent>(json!({
            "e":"referencePrice","s":"BAZUSD","t":1_770_313_263_917_i64
        }))
        .is_err()
    );
}

#[test]
fn spot_reference_price_preserves_explicit_null_and_exact_value() {
    for (price, expected) in [
        (serde_json::Value::Null, None),
        (json!("1.00"), Some(Decimal::new(100, 2))),
        (json!("0"), Some(Decimal::ZERO)),
    ] {
        let event: spot::stream_models::ReferencePriceEvent =
            serde_json::from_value(json!({"e":"referencePrice","s":"BAZUSD",
                "r":price,"t":1_770_313_263_917_i64}))
            .unwrap();
        assert_eq!(event.r, expected);
    }
    assert!(
        serde_json::from_value::<spot::stream_models::ReferencePriceEvent>(json!({
            "e":"referencePrice","s":"BAZUSD","r":"","t":1_770_313_263_917_i64
        }))
        .is_err()
    );
}
