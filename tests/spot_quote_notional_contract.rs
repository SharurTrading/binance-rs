// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Spot quote notional limits and the self-trade prevention mode a venue applied.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;
use binance_client::WeightPools;
use binance_client::spot::enums::SelfTradePreventionMode;
use binance_client::{Credentials, Decimal, Error, Outcome, Symbol, spot};
use serde_json::json;
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};

fn config() -> spot::Config {
    spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap())
}

fn decimal(text: &str) -> Decimal {
    Decimal::from_str_exact(text).unwrap()
}

fn limit_order(side: &str, kind: &str) -> spot::rest_requests::NewOrder {
    let order = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side(side)
        .type_value(kind)
        .quantity(Decimal::from(10))
        .price(Decimal::from(30))
        .new_client_order_id(spot::ClientOrderId::new("notional").unwrap());
    if kind == "LIMIT" {
        order.time_in_force("GTC")
    } else {
        order
    }
}

/// A BUY order list the endpoint page accepts: a stop-loss above the last price and
/// a take-profit below it.
fn oco(side: &str) -> spot::rest_requests::OrderListOco {
    spot::rest_requests::OrderListOco::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side(side)
        .quantity(Decimal::ONE)
        .above_type("STOP_LOSS")
        .above_stop_price(Decimal::from(40))
        .below_type("TAKE_PROFIT")
        .below_stop_price(Decimal::from(20))
        .list_client_order_id(spot::ClientOrderId::new("list").unwrap())
        .above_client_order_id(spot::ClientOrderId::new("above").unwrap())
        .below_client_order_id(spot::ClientOrderId::new("below").unwrap())
}

fn oto(working_type: &str, pending_side: &str) -> spot::rest_requests::OrderListOto {
    let list = spot::rest_requests::OrderListOto::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .list_client_order_id(spot::ClientOrderId::new("list").unwrap())
        .working_type(working_type)
        .working_side("BUY")
        .working_client_order_id(spot::ClientOrderId::new("working").unwrap())
        .working_price(Decimal::from(30))
        .working_quantity(Decimal::ONE)
        .pending_type("MARKET")
        .pending_side(pending_side)
        .pending_client_order_id(spot::ClientOrderId::new("pending").unwrap())
        .pending_quantity(Decimal::ONE);
    if working_type == "LIMIT" {
        list.working_time_in_force("GTC")
    } else {
        list
    }
}

#[test]
fn only_a_buy_order_that_may_take_accepts_a_quote_notional_limit() {
    let limit = Decimal::from(250);
    assert!(
        limit_order("BUY", "LIMIT")
            .quote_notional_limit(limit)
            .build()
            .is_ok()
    );
    assert!(matches!(
        limit_order("SELL", "LIMIT")
            .quote_notional_limit(limit)
            .build(),
        Err(Error::Validation("quote notional limit requires BUY"))
    ));
    assert!(matches!(
        limit_order("BUY", "LIMIT_MAKER")
            .quote_notional_limit(limit)
            .build(),
        Err(Error::Validation(
            "quote notional limit conflicts with LIMIT_MAKER"
        ))
    ));
    for not_positive in [Decimal::ZERO, Decimal::from(-1)] {
        assert!(matches!(
            limit_order("BUY", "LIMIT")
                .quote_notional_limit(not_positive)
                .build(),
            Err(Error::Validation("positive Spot magnitude"))
        ));
    }
    let quote_spend = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quote_order_qty(Decimal::from(100))
        .quote_notional_limit(limit)
        .new_client_order_id(spot::ClientOrderId::new("notional").unwrap());
    assert!(matches!(
        quote_spend.build(),
        Err(Error::Validation(
            "quote notional limit conflicts with quote spend"
        ))
    ));
}

#[test]
fn each_order_list_leg_is_checked_against_its_own_quote_notional_limit() {
    let limit = Decimal::from(30);
    assert!(oco("BUY").build().is_ok());
    assert!(oco("BUY").above_quote_notional_limit(limit).build().is_ok());
    assert!(oco("BUY").below_quote_notional_limit(limit).build().is_ok());
    assert!(matches!(
        oco("SELL").below_quote_notional_limit(limit).build(),
        Err(Error::Validation("quote notional limit requires BUY"))
    ));
    assert!(matches!(
        oco("BUY").below_quote_notional_limit(Decimal::ZERO).build(),
        Err(Error::Validation("positive Spot magnitude"))
    ));
}

#[test]
fn a_post_only_or_selling_list_leg_refuses_a_quote_notional_limit() {
    let limit = Decimal::from(25);
    assert!(oto("LIMIT", "SELL").build().is_ok());
    assert!(
        oto("LIMIT", "SELL")
            .working_quote_notional_limit(limit)
            .build()
            .is_ok()
    );
    assert!(
        oto("LIMIT", "BUY")
            .pending_quote_notional_limit(limit)
            .build()
            .is_ok()
    );
    assert!(matches!(
        oto("LIMIT_MAKER", "SELL")
            .working_quote_notional_limit(limit)
            .build(),
        Err(Error::Validation(
            "quote notional limit conflicts with LIMIT_MAKER"
        ))
    ));
    assert!(matches!(
        oto("LIMIT", "SELL")
            .pending_quote_notional_limit(limit)
            .build(),
        Err(Error::Validation("quote notional limit requires BUY"))
    ));
}

#[tokio::test]
async fn buy_order_sends_its_exact_limit_and_reads_back_the_expiry_it_caused() {
    let body = json!({
        "symbol": "BTCUSDT",
        "orderId": 1,
        "orderListId": -1,
        "clientOrderId": "notional",
        "transactTime": 1_792_059_013_310_530_i64,
        "price": "0.0000",
        "origQty": "1.0000",
        "executedQty": "0.6000",
        "origQuoteOrderQty": "0.0000",
        "quoteNotionalLimit": "60000.0000",
        "cummulativeQuoteQty": "60000.0000",
        "status": "EXPIRED",
        "expiryReason": "INSUFFICIENT_QUOTE_NOTIONAL_LIMIT",
        "timeInForce": "GTC",
        "type": "MARKET",
        "side": "BUY",
        "workingTime": 1_792_059_013_310_530_i64,
        "fills": [{"price": "100000.0000", "qty": "0.6000", "commission": "0.0000", "commissionAsset": "BTC", "tradeId": 0}],
        "selfTradePreventionMode": "NONE"
    });
    let mut f = HttpFixture::new(200, "", &body.to_string(), None, false).await;
    let client = spot::RestClient::new(config().rest_url(&f.url).unwrap()).unwrap();
    let order = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .quote_notional_limit(decimal("60000.5"))
        .new_client_order_id(spot::ClientOrderId::new("notional").unwrap());
    let response = client.new_order(&order, deadline()).await.unwrap().data;
    assert_eq!(response.quote_notional_limit, Some(decimal("60000.0000")));
    assert_eq!(response.executed_qty, Some(decimal("0.6000")));
    assert_eq!(
        response.expiry_reason.as_deref(),
        Some("INSUFFICIENT_QUOTE_NOTIONAL_LIMIT")
    );
    let wire = f.requests.recv().await.unwrap();
    let payload = wire.split("\r\n\r\n").nth(1).unwrap();
    assert!(payload.contains("quoteNotionalLimit=60000.5&"));
    assert!(!payload.contains("quoteOrderQty="));
    assert_eq!(f.connections_accepted(), 1);
    f.finish().await;
}

#[test]
fn full_response_keeps_the_mode_applied_to_each_prevented_match() {
    let order = |matches: serde_json::Value| {
        serde_json::from_value::<spot::rest_models::NewOrderResponse>(json!({
            "symbol": "BTCUSDT",
            "orderId": 5,
            "clientOrderId": "taker",
            "selfTradePreventionMode": "TRANSFER",
            "fills": [],
            "preventedMatches": matches
        }))
    };
    let response = order(json!([
        {"preventedMatchId": 0, "makerOrderId": 2, "price": "1.200000", "makerPreventedQuantity": "1.200000"},
        {"preventedMatchId": 1, "makerSymbol": "BTCUSDT", "makerOrderId": 3, "price": "1.100000",
         "takerPreventedQuantity": "0.20000000", "makerPreventedQuantity": "1.300000",
         "effectiveSelfTradePreventionMode": "DECREMENT"}
    ]))
    .unwrap();
    let matches = response.prevented_matches.unwrap();
    assert_eq!(matches[0].prevented_match_id, 0);
    assert_eq!(matches[0].effective_self_trade_prevention_mode, None);
    assert_eq!(matches[0].taker_prevented_quantity, None);
    assert_eq!(
        matches[1].maker_symbol.as_ref().unwrap().as_str(),
        "BTCUSDT"
    );
    assert_eq!(matches[1].maker_order_id, Some(3));
    assert_eq!(matches[1].price, Some(decimal("1.100000")));
    assert_eq!(
        matches[1].taker_prevented_quantity,
        Some(decimal("0.20000000"))
    );
    assert_eq!(
        matches[1].effective_self_trade_prevention_mode,
        Some(SelfTradePreventionMode::Decrement)
    );
    assert!(order(json!([{"makerOrderId": 2}])).is_err());
    assert!(order(json!([{"preventedMatchId": 0, "price": "not-a-price"}])).is_err());
}

#[test]
fn execution_report_keeps_the_limit_and_the_applied_prevention_mode() {
    let report = |extra: serde_json::Value| {
        let mut event = json!({
            "e": "executionReport", "E": 1, "s": "BTCUSDT", "c": "owned", "S": "BUY",
            "o": "MARKET", "x": "TRADE_PREVENTION", "X": "EXPIRED", "i": 7, "q": "10.00000000",
            "p": "0.00000000", "z": "3.00000000", "l": "0.00000000", "L": "0.00000000",
            "n": "0", "T": 2, "V": "TRANSFER"
        });
        event
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        serde_json::from_value::<spot::stream_models::ExecutionReportEvent>(event)
    };
    let limited = report(json!({"Qq": "90.00000000", "eV": "DECREMENT"})).unwrap();
    assert_eq!(limited.qq, Some(decimal("90.00000000")));
    assert_eq!(limited.e_v, Some(SelfTradePreventionMode::Decrement));
    assert_eq!(limited.upper_v, Some(SelfTradePreventionMode::Transfer));
    let plain = report(json!({})).unwrap();
    assert_eq!(plain.qq, None);
    assert_eq!(plain.e_v, None);
    assert!(report(json!({"Qq": "ninety"})).is_err());
}

#[test]
fn exchange_information_states_which_orders_may_carry_a_limit() {
    let symbol = |extra: serde_json::Value| {
        let mut symbol = json!({"symbol": "BTCUSDT", "status": "TRADING", "baseAsset": "BTC", "quoteAsset": "USDT"});
        symbol
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        serde_json::from_value::<spot::rest_models::ExchangeInfoResponse>(
            json!({"symbols": [symbol]}),
        )
        .unwrap()
        .symbols
        .unwrap()
        .remove(0)
    };
    let stated = symbol(
        json!({"marketQuoteNotionalLimitAllowed": true, "limitQuoteNotionalLimitAllowed": false}),
    );
    assert_eq!(stated.market_quote_notional_limit_allowed, Some(true));
    assert_eq!(stated.limit_quote_notional_limit_allowed, Some(false));
    let unstated = symbol(json!({}));
    assert_eq!(unstated.market_quote_notional_limit_allowed, None);
    assert_eq!(unstated.limit_quote_notional_limit_allowed, None);
}

#[test]
fn order_list_reports_keep_the_mode_applied_to_each_prevented_match() {
    let list = json!({
        "orderListId": 1,
        "contingencyType": "OTO",
        "listClientOrderId": "list",
        "symbol": "BTCUSDT",
        "orders": [{"symbol": "BTCUSDT", "orderId": 4, "clientOrderId": "working"}],
        "orderReports": [{
            "symbol": "BTCUSDT",
            "orderId": 4,
            "orderListId": 1,
            "clientOrderId": "working",
            "selfTradePreventionMode": "EXPIRE_MAKER",
            "preventedMatches": [{
                "preventedMatchId": 9,
                "makerOrderId": 2,
                "price": "1.100000",
                "makerPreventedQuantity": "1.300000",
                "effectiveSelfTradePreventionMode": "EXPIRE_BOTH"
            }]
        }]
    });
    let rest = serde_json::from_value::<spot::rest_models::OrderListOtoResponse>(list.clone())
        .unwrap()
        .order_reports
        .unwrap();
    let matched = &rest[0].prevented_matches.as_ref().unwrap()[0];
    assert_eq!(matched.prevented_match_id, 9);
    assert_eq!(matched.maker_prevented_quantity, Some(decimal("1.300000")));
    assert_eq!(
        matched.effective_self_trade_prevention_mode,
        Some(SelfTradePreventionMode::ExpireBoth)
    );
    let socket = serde_json::from_value::<spot::ws_models::OrderListPlaceOcoResponse>(list)
        .unwrap()
        .order_reports
        .unwrap();
    let matched = &socket[0].prevented_matches.as_ref().unwrap()[0];
    assert_eq!(
        matched.effective_self_trade_prevention_mode,
        Some(SelfTradePreventionMode::ExpireBoth)
    );
}

#[test]
fn every_order_response_that_can_expire_types_its_expiry_reason() {
    let legacy = serde_json::from_value::<spot::rest_models::OrderOcoResponse>(json!({
        "orderListId": 1,
        "orders": [{"symbol": "BTCUSDT", "orderId": 4, "clientOrderId": "limit"}],
        "orderReports": [{"symbol": "BTCUSDT", "orderId": 4, "clientOrderId": "limit",
            "status": "EXPIRED", "expiryReason": "OCO_TRIGGER"}]
    }))
    .unwrap();
    assert_eq!(
        legacy.order_reports.unwrap()[0].expiry_reason.as_deref(),
        Some("OCO_TRIGGER")
    );
    let routed = serde_json::from_value::<spot::rest_models::SorOrderResponse>(json!({
        "symbol": "BTCUSDT", "orderId": 2, "clientOrderId": "routed", "status": "EXPIRED",
        "expiryReason": "INSUFFICIENT_LIQUIDITY"
    }))
    .unwrap();
    assert_eq!(
        routed.expiry_reason.as_deref(),
        Some("INSUFFICIENT_LIQUIDITY")
    );
}

#[test]
fn socket_cancel_replace_types_the_fills_of_its_new_order() {
    let replaced = serde_json::from_value::<spot::ws_models::OrderCancelReplaceResponse>(json!({
        "cancelResult": "SUCCESS",
        "newOrderResult": "SUCCESS",
        "cancelResponse": {"symbol": "BTCUSDT", "orderId": 1, "clientOrderId": "cancel",
            "origClientOrderId": "old"},
        "newOrderResponse": {"symbol": "BTCUSDT", "orderId": 2, "clientOrderId": "new",
            "fills": [{"price": "20.00000000", "qty": "5.00000000", "commission": "0.00000000",
                "commissionAsset": "BTC", "tradeId": 7}]}
    }))
    .unwrap();
    let Some(spot::wire::BatchResult::Success(order)) = replaced.new_order_response else {
        panic!("accepted new order")
    };
    let fills = order.fills.unwrap();
    assert_eq!(fills[0].price, Some(decimal("20.00000000")));
    assert_eq!(fills[0].qty, Some(decimal("5.00000000")));
    assert_eq!(fills[0].trade_id, Some(7));
}

/// The error page calls both codes request refusals but does not state, as it does
/// for `-1013`, that the request never reached the matching engine.
#[tokio::test]
async fn overflowed_and_bad_asset_refusals_leave_the_outcome_unknown() {
    for body in [
        r#"{"code":-1018,"msg":"Parameter 'quoteNotionalLimit' overflowed."}"#,
        r#"{"code":-1108,"msg":"Invalid asset."}"#,
    ] {
        let f = HttpFixture::new(400, "", body, None, false).await;
        let client = spot::RestClient::new(config().rest_url(&f.url).unwrap()).unwrap();
        let order = limit_order("BUY", "LIMIT").quote_notional_limit(Decimal::from(250));
        let error = client.new_order(&order, deadline()).await.unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown));
        assert_eq!(f.connections_accepted(), 1);
        f.finish().await;
    }
}
