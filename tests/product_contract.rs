// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Product boundaries, exact balances, and unsent order validation.

#![allow(clippy::unwrap_used, reason = "synthetic contract assertions")]

use binance_client::{Decimal, Symbol, coinm, spot};
use serde_json::json;

#[test]
fn spot_quote_spend_is_distinct_from_base_quantity() {
    let order = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("这是测试币456").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .new_client_order_id(spot::ClientOrderId::new("operator-id").unwrap());
    assert!(order.clone().quote_order_qty(Decimal::ONE).build().is_ok());
    assert!(order.clone().quantity(Decimal::ONE).build().is_ok());
    assert!(order.clone().build().is_err());
    assert!(
        order
            .quantity(Decimal::ONE)
            .quote_order_qty(Decimal::ONE)
            .build()
            .is_err()
    );
}

#[test]
fn spot_balance_retains_asset_and_free_locked_amounts() {
    let account: spot::rest_models::GetAccountResponse = serde_json::from_value(json!({
        "balances":[{"asset":"这是测试币","free":"0.0000000000000000000000000001","locked":"2"}]
    }))
    .unwrap();
    let balance = &account.balances[0];
    assert_eq!(balance.asset.as_str(), "这是测试币");
    assert_eq!(
        balance.free,
        Decimal::from_str_exact("0.0000000000000000000000000001").unwrap()
    );
    assert_eq!(balance.locked, Decimal::TWO);
    assert!(
        serde_json::from_value::<spot::rest_models::GetAccountResponse>(json!({
            "balances":[{"asset":"BTC","free":"1e-29","locked":"0"}]
        }))
        .is_err()
    );
}

#[test]
fn inverse_balances_and_contracts_have_their_own_wire_types() {
    let balances: coinm::rest_models::FuturesAccountBalanceResponse =
        serde_json::from_value(json!([
            {"asset":"BTC","balance":"0.1","withdrawAvailable":"0.02"}
        ]))
        .unwrap();
    assert_eq!(balances[0].asset.as_str(), "BTC");
    assert_eq!(balances[0].balance, Decimal::new(1, 1));
    let kline: coinm::wire::Kline = serde_json::from_value(json!([
        1, "1", "2", "1", "2", "100", 2, "0.5", 3, "40", "0.2", "0"
    ]))
    .unwrap();
    assert_eq!(kline.contract_volume, Decimal::new(100, 0));
    assert_eq!(kline.base_volume, Decimal::new(5, 1));
}

#[test]
fn spot_fractional_receive_window_has_no_rounding() {
    assert!(
        spot::rest_requests::GetAccount::new()
            .recv_window(Decimal::new(6_000_346, 3))
            .build()
            .is_ok()
    );
    assert!(
        spot::rest_requests::GetAccount::new()
            .recv_window(Decimal::new(12345, 4))
            .build()
            .is_err()
    );
    assert!(
        spot::rest_requests::GetAccount::new()
            .recv_window(Decimal::new(60001, 0))
            .build()
            .is_err()
    );
}

#[test]
fn spot_depth_bridges_snapshot_plus_one_without_futures_pu() {
    use spot::{
        book::{BookState, DepthBook},
        rest_models::DepthResponse,
        stream_models::DiffBookDepthEvent,
    };
    let snapshot: DepthResponse =
        serde_json::from_value(json!({"lastUpdateId":100,"bids":[["1","2"]],"asks":[]})).unwrap();
    let event = |first, last, q: &str| -> DiffBookDepthEvent {
        serde_json::from_value(
            json!({"e":"depthUpdate","E":1,"s":"BTCUSDT","U":first,"u":last,"b":[["1",q]],"a":[]}),
        )
        .unwrap()
    };
    let mut b = DepthBook::new(Symbol::new("BTCUSDT").unwrap(), 3);
    b.update(3, event(99, 100, "9")).unwrap();
    b.update(3, event(101, 102, "3")).unwrap();
    b.snapshot(&snapshot).unwrap();
    assert_eq!(b.state(), BookState::Ready);
    assert_eq!(b.last_update_id(), Some(102));
    assert_eq!(b.bids().unwrap()[&Decimal::ONE], Decimal::new(3, 0));
    b.update(3, event(102, 103, "0")).unwrap();
    assert!(b.bids().unwrap().is_empty());
    assert!(b.update(3, event(105, 106, "1")).is_err());
    assert_eq!(b.state(), BookState::Gap);
    assert!(b.bids().is_err());
    assert!(b.is_partial());
}

#[test]
fn coinm_depth_keeps_its_previous_update_chain() {
    use coinm::{
        book::DepthBook, rest_models::OrderBookResponse, stream_models::DiffBookDepthStreamsEvent,
    };
    let snapshot: OrderBookResponse =
        serde_json::from_value(json!({"lastUpdateId":100,"bids":[],"asks":[]})).unwrap();
    let event = |first, last, previous| -> DiffBookDepthStreamsEvent {
        serde_json::from_value(json!({"e":"depthUpdate","E":1,"T":1,"s":"BTCUSD_PERP","U":first,"u":last,"pu":previous,"b":[],"a":[]})).unwrap()
    };
    let mut b = DepthBook::new(Symbol::new("BTCUSD_PERP").unwrap(), 7);
    b.update(7, event(99, 101, 98)).unwrap();
    b.snapshot(&snapshot).unwrap();
    assert_eq!(b.last_update_id(), Some(101));
    assert!(b.update(7, event(102, 103, 100)).is_err());
}

#[test]
fn coinm_bootstrap_gap_reports_the_count_of_discarded_pending_updates() {
    use coinm::{
        book::{BookState, DepthBook},
        rest_models::OrderBookResponse,
        stream_models::DiffBookDepthStreamsEvent,
    };
    let snapshot: OrderBookResponse =
        serde_json::from_value(json!({"lastUpdateId":100,"bids":[],"asks":[]})).unwrap();
    let event = |first, last, previous| -> DiffBookDepthStreamsEvent {
        serde_json::from_value(json!({"e":"depthUpdate","E":1,"T":1,"s":"BTCUSD_PERP","U":first,"u":last,"pu":previous,"b":[],"a":[]})).unwrap()
    };
    let mut b = DepthBook::new(Symbol::new("BTCUSD_PERP").unwrap(), 7);
    assert_eq!(b.discarded_pending_updates(), 0);
    // One buffered update bridges, one breaks the `pu` chain mid-drain, and
    // one is never applied: two accepted updates are discarded at the break.
    b.update(7, event(99, 101, 98)).unwrap();
    b.update(7, event(103, 103, 102)).unwrap();
    b.update(7, event(104, 104, 103)).unwrap();
    assert!(b.snapshot(&snapshot).is_err());
    assert_eq!(b.state(), BookState::Gap);
    assert_eq!(b.discarded_pending_updates(), 2);
    assert!(b.bids().is_err());
}

#[test]
fn known_event_and_filter_evidence_cannot_be_invented() {
    use spot::rest_models::ExchangeInfoResponseSymbolsItemFiltersItem as Filter;
    assert!(
        serde_json::from_value::<spot::stream_models::ExecutionReportEvent>(
            json!({"e":"executionReport","E":1})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<coinm::stream_models::OrderTradeUpdateEvent>(
            json!({"e":"ORDER_TRADE_UPDATE","E":1,"T":1,"o":{}})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<Filter>(json!({"filterType":"PRICE_FILTER","tickSize":"0.1"}))
            .is_err()
    );
    let lot: Filter = serde_json::from_value(
        json!({"filterType":"MARKET_LOT_SIZE","minQty":"0","maxQty":"100","stepSize":"0"}),
    )
    .unwrap();
    assert!(matches!(lot, Filter::MarketLotSize(_)));
    let future: Filter = serde_json::from_value(
        json!({"filterType":"FUTURE_FILTER","token":"sensitive-future-value"}),
    )
    .unwrap();
    assert!(matches!(future, Filter::Unknown(_)));
    assert!(!format!("{future:?}").contains("sensitive-future-value"));
}

#[test]
fn spot_cancel_all_keeps_every_member_and_its_partial_evidence() {
    use spot::{rest_models::DeleteOpenOrdersResponse, wire::BatchResult};
    let result: DeleteOpenOrdersResponse = serde_json::from_value(json!([
        {"orderId":7,"clientOrderId":"original","origClientOrderId":"original","status":"CANCELED"},
        {"orderId":8,"price":"invalid-financial-value"},
        {"code":-2011,"msg":"unknown order"},
        {"code":-999_999,"msg":"future code"}
    ]))
    .unwrap();
    assert_eq!(result.len(), 4);
    assert!(matches!(&result[0], BatchResult::Success(v) if v.order_id == 7));
    assert!(matches!(&result[1], BatchResult::Unknown(_)));
    assert!(
        matches!(&result[2], BatchResult::Failure(v) if v.outcome() == binance_client::Outcome::Rejected)
    );
    assert!(
        matches!(&result[3], BatchResult::Failure(v) if v.outcome() == binance_client::Outcome::Unknown)
    );
}

#[test]
fn websocket_ticker_financials_cannot_fall_back_to_raw_envelope_maps() {
    use spot::ws_models::TickerPriceResponse;
    assert!(
        serde_json::from_value::<TickerPriceResponse>(
            json!({"symbol":"BTCUSDT","price":"invalid-financial-value"})
        )
        .is_err()
    );
}

#[test]
fn coinm_trade_query_keeps_the_catalog_string_order_identity() {
    let request = coinm::rest_requests::AccountTradeList::new()
        .symbol(Symbol::new("BTCUSD_PERP").unwrap())
        .order_id("9223372036854775807");
    assert_eq!(
        serde_json::to_value(request).unwrap()["orderId"],
        "9223372036854775807"
    );
}

#[test]
fn coinm_price_candles_keep_ignored_columns_without_contract_volume_claims() {
    let wire = json!([
        1, "-0.0001", "0.0002", "-0.0003", "0.0001", "ignore", 2, "ignore", 7, "ignore", "ignore",
        "ignore"
    ]);
    let parsed: coinm::wire::PriceKline = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(parsed.open, Decimal::new(-1, 4));
    assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
    assert!(
        serde_json::from_value::<binance_client::usdm::ws_models::SymbolOrderBookTickerResponse>(
            json!({"symbol":"BTCUSDT","bidPrice":"invalid-financial-value"})
        )
        .is_err()
    );
}
