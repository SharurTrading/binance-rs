// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Synthetic regressions for categorical order modes and exact financial evidence.
#![allow(clippy::unwrap_used, reason = "synthetic contract assertions")]
use binance_client::{coinm, spot, usdm};
use serde_json::json;

#[test]
fn coinm_rest_and_socket_orders_preserve_price_match_modes() {
    let payload = json!({"orderId":7,"clientOrderId":"synthetic-order","priceMatch":"NONE","status":"CANCELED","executedQty":"0"});
    macro_rules! check {
        ($model:ty) => {
            let model: $model = serde_json::from_value(payload.clone()).unwrap();
            let wire = serde_json::to_value(model).unwrap();
            assert_eq!(wire["priceMatch"], "NONE");
        };
    }
    check!(coinm::rest_models::NewOrderResponse);
    check!(coinm::rest_models::QueryOrderResponse);
    check!(coinm::rest_models::CancelOrderResponse);
    check!(coinm::ws_models::NewOrderResponse);
    check!(coinm::ws_models::QueryOrderResponse);
    check!(coinm::ws_models::CancelOrderResponse);
}

#[test]
fn spot_rest_and_socket_preserve_peg_price_type() {
    let payload = json!({"orderId":7,"clientOrderId":"synthetic-order","orderListId":-1,"pegPriceType":"PRIMARY_PEG"});
    macro_rules! check {
        ($model:ty) => {
            let model: $model = serde_json::from_value(payload.clone()).unwrap();
            let wire = serde_json::to_value(model).unwrap();
            assert_eq!(wire["pegPriceType"], "PRIMARY_PEG");
        };
    }
    check!(spot::rest_models::GetOrderResponse);
    check!(spot::rest_models::DeleteOrderResponse);
    check!(spot::ws_models::OrderStatusResponse);
    check!(spot::ws_models::OrderCancelResponse);
}

#[test]
fn futures_cumulative_quote_refuses_malformed_financial_data() {
    let payload = json!({"orderId":7,"clientOrderId":"synthetic-order","cumQuote":"not-a-decimal"});
    assert!(
        serde_json::from_value::<usdm::rest_models::QueryOrderResponse>(payload.clone()).is_err()
    );
    assert!(
        serde_json::from_value::<usdm::rest_models::TestOrderResponse>(payload.clone()).is_err()
    );
    assert!(serde_json::from_value::<usdm::ws_models::QueryOrderResponse>(payload).is_err());
}

#[test]
fn inverse_liquidation_fee_refuses_malformed_financial_data() {
    let payload = json!({"symbols":[{"symbol":"BTCUSD_PERP","liquidationFee":"not-a-decimal"}]});
    assert!(
        serde_json::from_value::<coinm::rest_models::ExchangeInformationResponse>(payload).is_err()
    );
}

#[test]
fn spot_socket_cancel_preserves_the_complete_order_list_alternative() {
    let payload = json!({"orderListId":9,"listClientOrderId":"synthetic-list",
        "orders":[{"orderId":7,"clientOrderId":"synthetic-first"},{"orderId":8,"clientOrderId":"synthetic-second"}],
        "orderReports":[{"orderId":7,"clientOrderId":"synthetic-first","status":"CANCELED"},{"orderId":8,"clientOrderId":"synthetic-second","status":"CANCELED"}]});
    let model: spot::ws_models::OrderCancelResponse =
        serde_json::from_value(payload.clone()).unwrap();
    let retained = serde_json::to_value(model).unwrap();
    assert_eq!(retained["orders"], payload["orders"]);
    assert_eq!(
        retained["orderReports"][1]["clientOrderId"],
        "synthetic-second"
    );
    let mut missing_reports = payload;
    missing_reports
        .as_object_mut()
        .unwrap()
        .remove("orderReports");
    assert!(
        serde_json::from_value::<spot::ws_models::OrderCancelResponse>(missing_reports).is_err()
    );
}

#[test]
fn financial_quote_and_fee_fields_retain_sub_float_precision() {
    let text = "0.0000000000000000000000000001";
    let expected = binance_client::Decimal::from_str_exact(text).unwrap();
    let order: usdm::rest_models::QueryOrderResponse =
        serde_json::from_value(json!({"orderId":7,"cumQuote":text})).unwrap();
    assert_eq!(order.cum_quote, Some(expected));
    let socket_order: usdm::ws_models::QueryOrderResponse =
        serde_json::from_value(json!({"orderId":7,"cumQuote":text})).unwrap();
    assert_eq!(socket_order.cum_quote, Some(expected));
    let metadata: coinm::rest_models::ExchangeInformationResponse =
        serde_json::from_value(json!({"symbols":[{"liquidationFee":text}]})).unwrap();
    assert_eq!(metadata.symbols.unwrap()[0].liquidation_fee, Some(expected));
}

#[test]
fn remaining_documented_futures_amounts_refuse_malformed_strings() {
    macro_rules! refuses {
        ($model:ty, $payload:expr) => {
            assert!(serde_json::from_value::<$model>($payload).is_err());
        };
    }
    refuses!(
        usdm::rest_models::BasisResponseItem,
        json!({"annualizedBasisRate":"malformed"})
    );
    refuses!(
        usdm::rest_models::AllOrdersResponseItem,
        json!({"orderId":7,"cumBase":"malformed"})
    );
    refuses!(
        usdm::rest_models::NewAlgoOrderResponse,
        json!({"algoId":7,"callbackRate":"malformed"})
    );
    refuses!(
        usdm::rest_models::QueryAlgoOrderResponse,
        json!({"algoId":7,"icebergQuantity":"malformed"})
    );
    refuses!(
        coinm::rest_models::GetFundingRateHistoryOfPerpetualFuturesResponseItem,
        json!({"fundingRate":"malformed"})
    );
    refuses!(
        coinm::rest_models::IndexPriceAndMarkPriceResponseItem,
        json!({"lastFundingRate":"malformed"})
    );
    refuses!(
        coinm::rest_models::GetPositionMarginChangeHistoryResponseItem,
        json!({"amount":"malformed"})
    );
    refuses!(
        coinm::rest_models::AccountInformationResponseAssetsItem,
        json!({"asset":"BTC","walletBalance":"1","maxWithdrawAmount":"malformed"})
    );
}

#[test]
fn documented_unavailable_amounts_do_not_become_zero() {
    let algo: usdm::rest_models::NewAlgoOrderResponse = serde_json::from_value(
        json!({"algoId":7,"activatePrice":"","callbackRate":"","icebergQuantity":"null"}),
    )
    .unwrap();
    assert_eq!(algo.activate_price, None);
    assert_eq!(algo.callback_rate, None);
    assert_eq!(algo.iceberg_quantity, None);
    let delivery: coinm::rest_models::IndexPriceAndMarkPriceResponseItem =
        serde_json::from_value(json!({"lastFundingRate":"","interestRate":""})).unwrap();
    assert_eq!(delivery.last_funding_rate, None);
    assert_eq!(delivery.interest_rate, None);
    let zero: coinm::rest_models::IndexPriceAndMarkPriceResponseItem =
        serde_json::from_value(json!({"lastFundingRate":"0","interestRate":"0"})).unwrap();
    assert_eq!(zero.last_funding_rate, Some(binance_client::Decimal::ZERO));
    assert_eq!(zero.interest_rate, Some(binance_client::Decimal::ZERO));
    assert!(
        serde_json::from_value::<usdm::rest_models::QueryOrderResponse>(
            json!({"orderId":7,"executedQty":""})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<usdm::rest_models::QueryAlgoOrderResponse>(
            json!({"algoId":7,"icebergQuantity":""})
        )
        .is_err()
    );
}
