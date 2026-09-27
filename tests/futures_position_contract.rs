// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Synthetic regressions for native Futures position quantities.
#![allow(clippy::unwrap_used, reason = "synthetic contract assertions")]
use binance_client::{Decimal, coinm, usdm};
use serde_json::json;

#[test]
fn futures_positions_refuse_malformed_financial_amounts() {
    for value in [
        "not-a-decimal",
        "",
        "NaN",
        "0.00000000000000000000000000001",
    ] {
        let position = json!({"symbol":"BTCUSD_PERP","positionSide":"BOTH","positionAmt":value});
        let account = json!({"positions":[position.clone()]});
        assert!(
            serde_json::from_value::<coinm::rest_models::AccountInformationResponse>(
                account.clone()
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<coinm::ws_models::AccountInformationResponse>(account)
                .is_err()
        );
        assert!(
            serde_json::from_value::<coinm::rest_models::PositionInformationResponse>(json!([
                position.clone()
            ]))
            .is_err()
        );
        assert!(
            serde_json::from_value::<coinm::ws_models::PositionInformationResponse>(json!([
                position
            ]))
            .is_err()
        );
        assert!(
            serde_json::from_value::<usdm::ws_models::PositionInformationV2Response>(
                json!([{"symbol":"BTCUSDT","positionSide":"BOTH","positionAmt":value}])
            )
            .is_err()
        );
    }
}

#[test]
fn signed_position_quantities_remain_exact_and_native() {
    macro_rules! check {
        ($model:ty, $symbol:literal, $text:expr) => {
            let model: $model = serde_json::from_value(json!({
                "symbol":$symbol,"positionSide":"SHORT","positionAmt":$text
            })).unwrap();
            assert_eq!(model.position_amt, Some(Decimal::from_str_exact($text).unwrap()));
            let retained = serde_json::to_value(model).unwrap();
            assert_eq!(retained["symbol"], $symbol);
            assert_eq!(retained["positionSide"], "SHORT");
        };
    }
    for text in ["-9007199254740993", "0", "1"] {
        check!(
            coinm::rest_models::AccountInformationResponsePositionsItem,
            "BTCUSD_PERP",
            text
        );
        check!(
            coinm::ws_models::AccountInformationResponsePositionsItem,
            "BTCUSD_PERP",
            text
        );
        check!(
            coinm::rest_models::PositionInformationResponseItem,
            "BTCUSD_PERP",
            text
        );
        check!(
            coinm::ws_models::PositionInformationResponseItem,
            "BTCUSD_PERP",
            text
        );
    }
    for text in ["-0.1234567890123456789012345678", "0", "30"] {
        check!(
            usdm::ws_models::PositionInformationV2ResponseItem,
            "BTCUSDT",
            text
        );
    }
}

#[test]
fn missing_position_amounts_do_not_fabricate_zero() {
    macro_rules! check {
        ($model:ty) => {
            let model: $model = serde_json::from_value(json!({})).unwrap();
            assert_eq!(model.position_amt, None);
            assert!(
                serde_json::to_value(model)
                    .unwrap()
                    .get("positionAmt")
                    .is_none()
            );
        };
    }
    check!(coinm::rest_models::AccountInformationResponsePositionsItem);
    check!(coinm::ws_models::AccountInformationResponsePositionsItem);
    check!(coinm::rest_models::PositionInformationResponseItem);
    check!(coinm::ws_models::PositionInformationResponseItem);
    check!(usdm::ws_models::PositionInformationV2ResponseItem);
}

#[test]
fn inverse_bracket_quantities_and_contract_size_use_exact_decimals() {
    let bracket =
        json!({"bracket":1,"initialLeverage":125,"qtyCap":9_007_199_254_740_993_i64,"qtylFloor":7});
    macro_rules! check {
        ($model:ty) => {
            let model: $model = serde_json::from_value(bracket.clone()).unwrap();
            assert_eq!(
                model.qty_cap,
                Some(Decimal::from(9_007_199_254_740_993_i64))
            );
            assert_eq!(model.qtyl_floor, Some(Decimal::from(7)));
            assert_eq!(model.bracket, Some(1));
            assert_eq!(model.initial_leverage, Some(125));
        };
    }
    check!(coinm::rest_models::NotionalBracketForPairResponseItemBracketsItem);
    check!(coinm::rest_models::NotionalBracketForSymbolResponseItemBracketsItem);
    let metadata: coinm::rest_models::ExchangeInformationResponse = serde_json::from_value(json!({
        "symbols":[{"symbol":"BTCUSD_PERP","contractSize":100,"baseAsset":"BTC","quoteAsset":"USD","marginAsset":"BTC"}]
    })).unwrap();
    let symbol = &metadata.symbols.unwrap()[0];
    assert_eq!(symbol.contract_size, Some(Decimal::from(100)));
    let retained = serde_json::to_value(symbol).unwrap();
    assert_eq!(retained["baseAsset"], "BTC");
    assert_eq!(retained["quoteAsset"], "USD");
    assert_eq!(retained["marginAsset"], "BTC");
}
