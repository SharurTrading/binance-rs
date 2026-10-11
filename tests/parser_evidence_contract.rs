// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Required financial evidence and native unavailable-price sentinels.

#![allow(clippy::unwrap_used, reason = "synthetic contract assertions")]

use binance_client::{Decimal, coinm, spot, usdm};
use serde::de::DeserializeOwned;
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

// The venue documents every kline price, the close included, in the same
// string-encoded exact decimal form; USD-M already types the close as Decimal.
// https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#klinecandlestick-streams
#[test]
fn spot_kline_close_prices_are_exact_decimal_evidence() {
    // Recorded public Spot payload (BTCUSDT), issue #105.
    let wire = json!({"e":"kline","s":"BTCUSDT","E":1_791_417_540_001_i64,"k":{
        "t":1_791_417_480_000_i64,"T":1_791_417_539_999_i64,"s":"BTCUSDT","i":"1m",
        "o":"100.1","h":"102.3","l":"99.9","c":"101.23456789","v":"12","x":true}});
    let event: spot::stream_models::KlineEvent = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(event.k.c, Some(Decimal::new(10_123_456_789, 8)));
    let event: spot::stream_models::KlineOffsetEvent = serde_json::from_value(wire).unwrap();
    assert_eq!(event.k.c, Some(Decimal::new(10_123_456_789, 8)));
}

// https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#klinecandlestick-streams
#[test]
fn coinm_kline_close_prices_are_exact_decimal_evidence() {
    // Official COIN-M example kline body.
    let k = json!({"t":1_591_261_540_000_i64,"T":1_591_261_599_999_i64,"i":"1m",
        "f":606_400,"L":606_430,"o":"9638.9","c":"9639.8","h":"9639.8","l":"9638.6",
        "v":"156","n":31,"x":false,"q":"1.61836886","V":"73","Q":"0.75731156","B":"0"});
    let event: coinm::stream_models::KlineCandlestickStreamsEvent = serde_json::from_value(
        json!({"e":"kline","E":1_591_261_542_539_i64,"s":"BTCUSD_200626","k":k.clone()}),
    )
    .unwrap();
    assert_eq!(event.k.c, Some(Decimal::new(96_398, 1)));
    let event: coinm::stream_models::ContinuousContractKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({"e":"continuous_kline","E":1_591_261_542_539_i64,
            "ps":"BTCUSD","ct":"NEXT_QUARTER","k":k.clone()}))
        .unwrap();
    assert_eq!(event.k.c, Some(Decimal::new(96_398, 1)));
    let event: coinm::stream_models::IndexKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({"e":"indexPrice_kline","E":1_591_261_542_539_i64,
            "ps":"BTCUSD","k":k.clone()}))
        .unwrap();
    assert_eq!(event.k.c, Some(Decimal::new(96_398, 1)));
    let event: coinm::stream_models::MarkPriceKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({"e":"markPrice_kline","E":1_591_261_542_539_i64,
            "ps":"BTCUSD_PERP","k":k}))
        .unwrap();
    assert_eq!(event.k.c, Some(Decimal::new(96_398, 1)));
}

fn refuses_non_decimal_kline_close<E: DeserializeOwned>(wire: &serde_json::Value) {
    for close in ["not-a-price", "0.12345678901234567890123456789", ""] {
        let mut value = wire.clone();
        value["k"]["c"] = json!(close);
        assert!(
            serde_json::from_value::<E>(value).is_err(),
            "kline close {close:?} must not decode"
        );
    }
}

#[test]
fn kline_close_prices_refuse_non_decimal_evidence() {
    let spot_wire = json!({"e":"kline","E":1,"s":"BTCUSDT","k":{"i":"1m","c":"101.23456789"}});
    refuses_non_decimal_kline_close::<spot::stream_models::KlineEvent>(&spot_wire);
    refuses_non_decimal_kline_close::<spot::stream_models::KlineOffsetEvent>(&spot_wire);
    let coinm_k = json!({"i":"1m","c":"9639.8"});
    refuses_non_decimal_kline_close::<coinm::stream_models::KlineCandlestickStreamsEvent>(
        &json!({"e":"kline","E":1,"s":"BTCUSD_200626","k":coinm_k.clone()}),
    );
    refuses_non_decimal_kline_close::<
        coinm::stream_models::ContinuousContractKlineCandlestickStreamsEvent,
    >(
        &json!({"e":"continuous_kline","E":1,"ps":"BTCUSD","ct":"NEXT_QUARTER","k":coinm_k.clone()})
    );
    refuses_non_decimal_kline_close::<coinm::stream_models::IndexKlineCandlestickStreamsEvent>(
        &json!({"e":"indexPrice_kline","E":1,"ps":"BTCUSD","k":coinm_k.clone()}),
    );
    refuses_non_decimal_kline_close::<coinm::stream_models::MarkPriceKlineCandlestickStreamsEvent>(
        &json!({"e":"markPrice_kline","E":1,"ps":"BTCUSD_PERP","k":coinm_k}),
    );
}

#[test]
fn kline_close_prices_keep_sibling_optionality() {
    // `c` keeps the optionality of its sibling prices: absent or JSON-null decodes
    // to `None` instead of failing, exactly as `o`/`h`/`l` do.
    let event: spot::stream_models::KlineEvent = serde_json::from_value(json!({
        "e":"kline","E":1,"s":"BTCUSDT","k":{"i":"1m","o":"1"}}))
    .unwrap();
    assert_eq!(event.k.c, None);
    let event: spot::stream_models::KlineOffsetEvent = serde_json::from_value(json!({
        "e":"kline","E":1,"s":"BTCUSDT","k":{"i":"1m","o":"1","c":null}}))
    .unwrap();
    assert_eq!(event.k.c, None);
    let event: coinm::stream_models::KlineCandlestickStreamsEvent = serde_json::from_value(
        json!({"e":"kline","E":1,"s":"BTCUSD_200626","k":{"i":"1m","o":"1"}}),
    )
    .unwrap();
    assert_eq!(event.k.c, None);
    let event: coinm::stream_models::ContinuousContractKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({"e":"continuous_kline","E":1,"ps":"BTCUSD",
            "ct":"NEXT_QUARTER","k":{"i":"1m","o":"1","c":null}}))
        .unwrap();
    assert_eq!(event.k.c, None);
    let event: coinm::stream_models::IndexKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({"e":"indexPrice_kline","E":1,"ps":"BTCUSD",
            "k":{"i":"1m","o":"1"}}))
        .unwrap();
    assert_eq!(event.k.c, None);
    let event: coinm::stream_models::MarkPriceKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({"e":"markPrice_kline","E":1,"ps":"BTCUSD_PERP",
            "k":{"i":"1m","o":"1","c":null}}))
        .unwrap();
    assert_eq!(event.k.c, None);
}
