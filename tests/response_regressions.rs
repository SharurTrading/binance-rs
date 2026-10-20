// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Typed evidence, beyond whole-body deserialization of official examples.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic wire assertions"
)]
use binance_client::{Decimal, spot, usdm};
use serde_json::{Value, json};

fn example(product: &str, operation: &str) -> Value {
    let corpus: Value =
        serde_json::from_str(include_str!("fixtures/response-shapes.json")).unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["product"] == product && case["operation"] == operation)
        .unwrap()["response"]
        .clone()
}

#[test]
fn spot_cancel_all_preserves_both_receipts_errors_and_unknown_members() {
    use spot::wire::BatchResult;
    let mut wire = example("spot", "deleteOpenOrders")
        .as_array()
        .unwrap()
        .clone();
    wire.push(json!({"code":-2011,"msg":"synthetic rejection"}));
    wire.push(json!({"futureReceipt":{"id":123}}));
    wire.push(json!({"symbol":"BTCUSDT","orderId":99,"price":"invalid"}));
    let receipts: spot::rest_models::DeleteOpenOrdersResponse =
        serde_json::from_value(json!(wire)).unwrap();
    assert_eq!(receipts.len(), 6);
    assert!(matches!(&receipts[0], BatchResult::Success(
        spot::rest_models::DeleteOpenOrdersResponseItem::Order(order)) if order.order_id == 11));
    assert!(receipts.iter().any(|receipt| matches!(receipt,
        BatchResult::Success(spot::rest_models::DeleteOpenOrdersResponseItem::OrderList(list))
        if list.order_list_id == 1929 && list.orders.len() == 2 && list.order_reports.len() == 2)));
    let end = receipts.len();
    assert!(matches!(&receipts[end-3], BatchResult::Failure(error) if error.code == -2011));
    assert!(matches!(&receipts[end - 2], BatchResult::Unknown(_)));
    assert!(matches!(&receipts[end - 1], BatchResult::Unknown(_)));
    if let BatchResult::Unknown(evidence) = &receipts[end - 2] {
        assert_eq!(evidence.as_value()["futureReceipt"]["id"], 123);
    }
    let ws: spot::ws_models::OpenOrdersCancelAllResponse =
        serde_json::from_value(example("spot", "openOrdersCancelAll")).unwrap();
    assert_eq!(ws.len(), 2);
    assert!(matches!(&ws[0], BatchResult::Success(
        spot::ws_models::OpenOrdersCancelAllResponseItem::Order(order))
        if order.order_id == 12_569_099_453));
    assert!(matches!(&ws[1], BatchResult::Success(
        spot::ws_models::OpenOrdersCancelAllResponseItem::OrderList(list))
        if list.order_list_id == 19_431 && list.orders.len() == 2 && list.order_reports.len() == 2));
}

#[test]
fn portfolio_margin_cancel_all_keeps_list_identity_and_unknown_siblings() {
    use binance_client::advanced_trading::portfolio_margin::rest_models::{
        CancelMarginAccountAllOpenOrdersOnASymbolResponse as Receipts,
        CancelMarginAccountAllOpenOrdersOnASymbolResponseItem as Receipt,
    };
    let mut wire = example(
        "portfolio_margin",
        "cancelMarginAccountAllOpenOrdersOnASymbol",
    )
    .as_array()
    .unwrap()
    .clone();
    wire.push(json!({"futureReceipt":123}));
    let receipts: Receipts = serde_json::from_value(json!(wire)).unwrap();
    assert!(matches!(&receipts[0], Receipt::Order(order) if order.order_list_id == Some(-1)));
    assert!(
        receipts
            .iter()
            .any(|r| matches!(r, Receipt::OrderList(list) if !list.orders.is_empty()))
    );
    assert!(matches!(receipts.last(), Some(Receipt::Unknown(evidence))
        if evidence.as_value()["futureReceipt"] == 123));
}

#[test]
fn hybrid_cancel_validates_root_order_and_nested_list_reports() {
    let corpus: Value =
        serde_json::from_str(include_str!("fixtures/response-shapes.json")).unwrap();
    let wire = corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| {
            case["operation"] == "orderCancel"
                && case["response"].get("orders").is_some()
                && case["response"].get("orderId").is_some()
        })
        .unwrap()["response"]
        .clone();
    let receipt: spot::ws_models::OrderCancelResponse =
        serde_json::from_value(wire.clone()).unwrap();
    match receipt {
        spot::ws_models::OrderCancelResponse::OrderList(list) => {
            assert_eq!(list.order_id, wire["orderId"].as_i64());
            assert_eq!(list.orders.len(), wire["orders"].as_array().unwrap().len());
            assert_eq!(
                list.order_reports.len(),
                wire["orderReports"].as_array().unwrap().len()
            );
            assert_eq!(
                list.price,
                Some(Decimal::from_str_exact(wire["price"].as_str().unwrap()).unwrap())
            );
        }
        _ => panic!("hybrid cancel must expose typed list and order evidence"),
    }
    for pointer in ["/price", "/orderReports/0/price"] {
        let mut malformed = wire.clone();
        *malformed.pointer_mut(pointer).unwrap() = json!("invalid-price");
        assert!(serde_json::from_value::<spot::ws_models::OrderCancelResponse>(malformed).is_err());
    }
}

#[test]
fn sor_receipts_accept_documented_arrays_and_native_scalars_with_allocation_fields() {
    let wire = example("spot", "sorOrderPlace");
    let receipts: spot::ws_models::SorOrderPlaceResponse =
        serde_json::from_value(wire.clone()).unwrap();
    match receipts {
        spot::ws_models::SorOrderPlaceResponse::Multiple(rows) => {
            let fills = rows[0].fills.as_ref().unwrap();
            assert_eq!(fills[0].alloc_id, Some(0));
            assert_eq!(
                fills[0].match_type.as_deref(),
                Some("ONE_PARTY_TRADE_REPORT")
            );
        }
        _ => panic!("documented SOR response is an array"),
    }
    let scalar: spot::ws_models::SorOrderPlaceResponse =
        serde_json::from_value(wire[0].clone()).unwrap();
    assert!(matches!(
        scalar,
        spot::ws_models::SorOrderPlaceResponse::Single(_)
    ));
}

#[test]
fn futures_book_levels_are_typed_and_invalid_prices_fail() {
    let mut wire = json!({"lastUpdateId":42,"bids":[["1.00000001","2.5"]],"asks":[]});
    let book: usdm::ws_models::OrderBookResponse = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(book.last_update_id, 42);
    assert_eq!(
        book.bids[0].price(),
        Decimal::from_str_exact("1.00000001").unwrap()
    );
    assert_eq!(book.bids[0].quantity(), Decimal::new(25, 1));
    assert!(!book.extra.contains_key("bids"));
    wire["bids"][0][0] = json!("1e-29");
    assert!(serde_json::from_value::<usdm::ws_models::OrderBookResponse>(wire).is_err());
}

#[test]
fn arbitrary_indicator_keys_retain_exact_financial_values() {
    let wire = json!({"indicators":{"SOLUSDT":[{"indicator":"UFR","value":"0.125","triggerValue":"0.99"}],
        "ACCOUNT":[{"indicator":"TMV","value":1,"triggerValue":2}]}});
    let result: usdm::rest_models::FuturesTradingQuantitativeRulesIndicatorsResponse =
        serde_json::from_value(wire.clone()).unwrap();
    let indicators = result.indicators.unwrap();
    assert_eq!(indicators["SOLUSDT"][0].value, Some(Decimal::new(125, 3)));
    assert_eq!(
        indicators["SOLUSDT"][0].trigger_value,
        Some(Decimal::new(99, 2))
    );
    assert_eq!(indicators["ACCOUNT"][0].value, Some(Decimal::ONE));
    let result: binance_client::advanced_trading::portfolio_margin::rest_models::
        PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponse = serde_json::from_value(wire).unwrap();
    assert_eq!(
        result.indicators.unwrap()["SOLUSDT"][0].value,
        Some(Decimal::new(125, 3))
    );
}

#[test]
fn non_trade_income_and_quoted_timestamps_have_exact_meaning() {
    let income: usdm::rest_models::GetIncomeHistoryResponse = serde_json::from_value(json!([
        {"symbol":"","incomeType":"TRANSFER","income":"1.25","asset":"USDT"},
        {"symbol":"BTCUSDT","incomeType":"REALIZED_PNL","income":"0.01","asset":"USDT"}
    ]))
    .unwrap();
    assert!(income[0].symbol.is_none());
    assert_eq!(income[1].symbol.as_ref().unwrap().as_str(), "BTCUSDT");
    for timestamp in [json!(1_583_139_600_000_i64), json!("1583139600000")] {
        let rows: usdm::rest_models::LongShortRatioResponse =
            serde_json::from_value(json!([{"symbol":"BTCUSDT","timestamp":timestamp}])).unwrap();
        assert_eq!(rows[0].timestamp, Some(1_583_139_600_000));
    }
    for invalid in [json!(1.5), json!("1.5"), json!("9223372036854775808")] {
        assert!(
            serde_json::from_value::<usdm::rest_models::LongShortRatioResponse>(
                json!([{"timestamp":invalid}])
            )
            .is_err()
        );
    }
}
