// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Advanced Trading uses native routes, caller identities and exact amounts.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic contract assertions"
)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::advanced_trading::{algo, copy_trading, portfolio_margin};
use binance_client::{Credentials, Decimal, Error, Outcome, Symbol, WeightPools};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};

fn algo_config() -> algo::Config {
    algo::Config::with_pools(&WeightPools::new())
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
}

fn twap() -> algo::rest_requests::TimeWeightedAveragePriceSpotAlgo {
    algo::rest_requests::TimeWeightedAveragePriceSpotAlgo::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .quantity(Decimal::from_str_exact("0.123456789123456789").unwrap())
        .duration(300)
}

#[test]
fn algo_placement_requires_caller_id_and_provider_duration() {
    assert!(twap().build().is_err());
    assert!(algo::ClientAlgoId::new("short").is_err());
    let id = algo::ClientAlgoId::new("0123456789abcdef0123456789abcdef").unwrap();
    assert!(twap().client_algo_id(id.clone()).build().is_ok());
    assert!(
        twap()
            .client_algo_id(id.clone())
            .duration(299)
            .build()
            .is_err()
    );
    assert!(twap().client_algo_id(id).duration(86_401).build().is_err());
    assert!(
        algo::rest_requests::CancelAlgoOrderSpotAlgo::new()
            .build()
            .is_err()
    );
}

#[tokio::test]
async fn uncertain_algo_is_not_replayed_and_retains_caller_identity() {
    let mut fixture = HttpFixture::new(
        200,
        "X-SAPI-USED-UID-WEIGHT-1M: 3000\r\n",
        "{",
        Some(100),
        false,
    )
    .await;
    let client = algo::RestClient::new(algo_config().rest_url(&fixture.url).unwrap()).unwrap();
    let request =
        twap().client_algo_id(algo::ClientAlgoId::new("0123456789abcdef0123456789abcdef").unwrap());
    let error = client
        .time_weighted_average_price_spot_algo(&request, deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        client_order_ids,
        meta: Some(meta),
        ..
    } = error
    else {
        panic!("transport evidence")
    };
    assert_eq!(
        client_order_ids["clientAlgoId"],
        "0123456789abcdef0123456789abcdef"
    );
    assert_eq!(meta.rates.counters["x-sapi-used-uid-weight-1m"], 3000);
    let wire = fixture.requests.recv().await.unwrap();
    assert!(wire.starts_with("POST /sapi/v1/algo/spot/newOrderTwap HTTP/1.1\r\n"));
    assert!(wire.contains("quantity=0.123456789123456789"));
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn copy_trading_read_uses_its_native_signed_sapi_route() {
    let mut fixture = HttpFixture::new(
        200,
        "",
        r#"{"code":"000000","message":"success","data":{"isLeadTrader":false},"success":true}"#,
        None,
        false,
    )
    .await;
    let client = copy_trading::RestClient::new(
        copy_trading::Config::with_pools(&WeightPools::new())
            .unwrap()
            .clock(Arc::new(FixedClock(1_700_000_001_000)))
            .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    client
        .get_futures_lead_trader_status(
            &copy_trading::rest_requests::GetFuturesLeadTraderStatus::new(),
            deadline(),
        )
        .await
        .unwrap();
    let wire = fixture.requests.recv().await.unwrap();
    assert!(wire.starts_with("GET /sapi/v1/copyTrading/futures/userStatus?"));
    assert!(wire.contains("signature="));
    fixture.finish().await;
}

#[test]
fn portfolio_balances_keep_native_asset_and_product_components() {
    let data: portfolio_margin::rest_models::AccountBalanceResponse = serde_json::from_str(r#"[{"asset":"BTC","totalWalletBalance":"0.123456789123456789","crossMarginAsset":"0.1","crossMarginBorrowed":"0","crossMarginFree":"0.1","crossMarginInterest":"0","crossMarginLocked":"0","umWalletBalance":"0","umUnrealizedPNL":"0","cmWalletBalance":"0.023456789123456789","cmUnrealizedPNL":"","negativeBalance":"0"}]"#).unwrap();
    let portfolio_margin::rest_models::AccountBalanceResponse::Multiple(data) = data else {
        panic!("native balance array")
    };
    assert_eq!(data[0].asset.as_str(), "BTC");
    assert_eq!(
        data[0].total_wallet_balance,
        Decimal::from_str_exact("0.123456789123456789").unwrap()
    );
    assert_eq!(data[0].cm_unrealized_pnl, None);
    assert!(
        serde_json::from_str::<portfolio_margin::rest_models::AccountBalanceResponse>(
            r#"[{"asset":"BTC","totalWalletBalance":"invalid"}]"#
        )
        .is_err()
    );
}

fn native_config<P: binance_client::advanced_trading::config::Product>()
-> binance_client::advanced_trading::config::Config<P> {
    binance_client::advanced_trading::config::Config::with_pools(&WeightPools::new())
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
}

#[tokio::test]
async fn stocks_quote_absence_is_explicit_but_malformed_data_fails() {
    use binance_client::advanced_trading::stocks;
    for (body, absent) in [
        ("", true),
        (
            r#"{"symbol":"AAPL","bidPrice":"182.123456789123456789","askPrice":"183","bidSize":100,"askSize":200}"#,
            false,
        ),
    ] {
        let fixture = HttpFixture::new(200, "", body, None, false).await;
        let client =
            stocks::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
        let response = client
            .latest_quote(
                &stocks::rest_requests::LatestQuote::new().symbol(Symbol::new("AAPL").unwrap()),
                deadline(),
            )
            .await
            .unwrap();
        assert_eq!(response.data.is_none(), absent);
        if let Some(quote) = response.data {
            assert_eq!(
                quote.bid_price,
                Decimal::from_str_exact("182.123456789123456789").unwrap()
            );
        }
        fixture.finish().await;
    }
    let fixture = HttpFixture::new(
        200,
        "",
        r#"{"symbol":"AAPL","bidPrice":"invalid"}"#,
        None,
        false,
    )
    .await;
    let client = stocks::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
    assert_eq!(
        client
            .latest_quote(
                &stocks::rest_requests::LatestQuote::new().symbol(Symbol::new("AAPL").unwrap()),
                deadline()
            )
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::ReadFailed)
    );
    fixture.finish().await;
}

#[test]
fn stocks_order_matrix_and_precision_follow_the_native_contract() {
    use binance_client::advanced_trading::stocks::{
        ClientOrderId, rest_requests::PlaceEquityOrder,
    };
    let base = PlaceEquityOrder::new()
        .symbol(Symbol::new("AAPL").unwrap())
        .client_order_id(ClientOrderId::new("0123456789abcdef0123456789abcdef").unwrap());
    let limit = base
        .clone()
        .side("BUY")
        .order_type("LIMIT")
        .quantity(Decimal::ONE)
        .price(Decimal::new(18250, 2));
    assert!(limit.clone().build().is_err());
    assert!(limit.clone().trading_session("RTH").build().is_ok());
    assert!(
        limit
            .clone()
            .trading_session("RTH")
            .price(Decimal::new(182_501, 3))
            .build()
            .is_err()
    );
    assert!(
        limit
            .clone()
            .trading_session("RTH")
            .notional(Decimal::ONE)
            .build()
            .is_err()
    );
    let fractional = limit
        .trading_session("RTH")
        .time_in_force("GTC")
        .quantity(Decimal::new(5, 1));
    assert!(fractional.clone().build().is_err());
    assert!(fractional.trading_session("EXTENDED").build().is_ok());
    let market = base
        .side("BUY")
        .order_type("MARKET")
        .notional(Decimal::from(20));
    assert!(market.clone().build().is_ok());
    assert!(market.clone().quantity(Decimal::ONE).build().is_err());
    assert!(market.side("SELL").build().is_err());
}

#[test]
fn portfolio_orders_and_list_legs_require_native_ids_and_conditional_fields() {
    use binance_client::ClientOrderId;
    use portfolio_margin::rest_requests::{MarginAccountNewOco, NewCmConditionalOrder, NewUmOrder};
    let limit = NewUmOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("LIMIT")
        .quantity(Decimal::ONE)
        .time_in_force("GTC");
    assert!(limit.clone().build().is_err());
    let limit = limit.new_client_order_id(ClientOrderId::new("caller-um").unwrap());
    assert!(limit.clone().build().is_err());
    assert!(limit.clone().price(Decimal::ONE).build().is_ok());
    assert!(
        limit
            .price(Decimal::ONE)
            .price_match("QUEUE")
            .build()
            .is_err()
    );
    let conditional = NewCmConditionalOrder::new()
        .symbol(Symbol::new("BTCUSD_PERP").unwrap())
        .side("SELL")
        .strategy_type("STOP_MARKET")
        .new_client_strategy_id(ClientOrderId::new("caller-cm-strategy").unwrap());
    assert!(conditional.clone().build().is_err());
    assert!(
        conditional
            .stop_price(Decimal::ONE)
            .quantity(Decimal::ONE)
            .build()
            .is_ok()
    );
    let oco = MarginAccountNewOco::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("SELL")
        .quantity(Decimal::ONE)
        .price(Decimal::from(20))
        .stop_price(Decimal::from(10));
    assert!(oco.clone().build().is_err());
    assert!(
        oco.list_client_order_id(ClientOrderId::new("list").unwrap())
            .limit_client_order_id(ClientOrderId::new("limit-leg").unwrap())
            .stop_client_order_id(ClientOrderId::new("stop-leg").unwrap())
            .build()
            .is_ok()
    );
}

#[tokio::test]
async fn portfolio_pro_bnb_cap_is_rolling_and_shared_by_clones() {
    use binance_client::advanced_trading::portfolio_margin_pro as pro;
    let fixture = HttpFixture::new(200, "", r#"{"tranId":123}"#, None, false).await;
    let client = pro::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = pro::rest_requests::BnbTransfer::new()
        .amount(Decimal::ONE)
        .transfer_side("TO_UM");
    client.bnb_transfer(&request, deadline()).await.unwrap();
    client
        .clone()
        .bnb_transfer(&request, deadline())
        .await
        .unwrap();
    let error = client.bnb_transfer(&request, deadline()).await.unwrap_err();
    assert!(matches!(error, Error::Admission { .. }));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(fixture.connections_accepted(), 2);
    fixture.finish().await;
}

#[tokio::test]
async fn portfolio_all_symbol_authority_is_not_sent_and_expires_before_dispatch() {
    let fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let client =
        portfolio_margin::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = portfolio_margin::rest_requests::QueryCurrentMarginOpenOrder::new();
    assert_eq!(
        client
            .query_current_margin_open_order(&request, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::NotSent)
    );
    let request = request.trading_symbol_count(
        portfolio_margin::TradingSymbolCount::new(3, 1_700_000_001_000).unwrap(),
    );
    assert_eq!(
        client
            .query_current_margin_open_order(&request, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::NotSent)
    );
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}

#[tokio::test]
async fn alpha_rest_keeps_full_depth_ids_levels_and_public_unsigned_route() {
    use binance_client::advanced_trading::alpha;
    let mut fixture=HttpFixture::new(200,"",r#"{"code":"000000","success":true,"data":{"lastUpdateId":123,"symbol":"ALPHA_173USDT","bids":[["0.123456789123456789","0"]],"asks":[["0.2","3"]]}}"#,None,false).await;
    let client = alpha::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
    let response = client
        .full_depth(
            &alpha::rest_requests::FullDepth::new().symbol(Symbol::new("ALPHA_173USDT").unwrap()),
            deadline(),
        )
        .await
        .unwrap();
    let data = response.data.data;
    assert_eq!(data.last_update_id, 123);
    assert_eq!(
        data.bids[0].price,
        Decimal::from_str_exact("0.123456789123456789").unwrap()
    );
    assert_eq!(data.bids[0].quantity, Decimal::ZERO);
    let wire = fixture.requests.recv().await.unwrap();
    assert!(wire.starts_with("GET /bapi/defi/v1/public/alpha-trade/fullDepth?"));
    assert!(!wire.contains("signature="));
    assert!(!wire.to_ascii_lowercase().contains("x-mbx-apikey"));
    fixture.finish().await;
}

#[test]
fn advanced_pool_scopes_are_explicit_and_separate() {
    use binance_client::{
        AccountKey, VenuePool,
        advanced_trading::{alpha, portfolio_margin_pro, stocks},
    };
    let pools = WeightPools::new();
    let key = AccountKey::new("synthetic-account");
    let pm = portfolio_margin::Config::with_pools_for_account(&pools, &key).unwrap();
    let pro = portfolio_margin_pro::Config::with_pools_for_account(&pools, &key).unwrap();
    let stock = stocks::Config::with_pools_for_account(&pools, &key).unwrap();
    let alpha = alpha::Config::with_pools(&pools).unwrap();
    assert_eq!(pm.pool_key().unwrap().pool, VenuePool::PortfolioMargin);
    assert_eq!(pro.pool_key(), stock.pool_key());
    assert_eq!(alpha.pool_key().unwrap().pool, VenuePool::Alpha);
    assert_ne!(pm.pool_key(), pro.pool_key());
}

#[tokio::test]
async fn service_failure_over_http_200_retains_exact_codes_and_never_proves_success() {
    use binance_client::advanced_trading::{alpha, stocks};
    let fixture = HttpFixture::new(
        200,
        "",
        r#"{"code":"000123","success":false,"message":"synthetic refusal"}"#,
        None,
        false,
    )
    .await;
    let client = alpha::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
    let Error::Venue(error) = client
        .token_list(&alpha::rest_requests::TokenList::new(), deadline())
        .await
        .unwrap_err()
    else {
        panic!("native service failure")
    };
    assert_eq!(error.text_code.as_ref().unwrap().as_str(), "000123");
    assert_eq!(error.outcome, Outcome::ReadFailed);
    fixture.finish().await;
    for (status, outcome) in [
        ("F", Outcome::Rejected),
        ("FUTURE_STATUS", Outcome::Unknown),
    ] {
        let fixture =
            HttpFixture::new(200, "", &format!(r#"{{"status":"{status}"}}"#), None, false).await;
        let client =
            stocks::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
        let request = stocks::rest_requests::PlaceEquityOrder::new()
            .symbol(Symbol::new("AAPL").unwrap())
            .client_order_id(
                binance_client::advanced_trading::stocks::ClientOrderId::new(
                    "0123456789abcdef0123456789abcdef",
                )
                .unwrap(),
            )
            .side("BUY")
            .order_type("MARKET")
            .notional(Decimal::ONE);
        let Error::Venue(error) = client
            .place_equity_order(&request, deadline())
            .await
            .unwrap_err()
        else {
            panic!("native submission status")
        };
        assert_eq!(error.service_status.as_ref().unwrap().as_str(), status);
        assert_eq!(error.outcome, outcome);
        assert_eq!(
            error.client_order_ids["clientOrderId"],
            "0123456789abcdef0123456789abcdef"
        );
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn cancellation_failures_pending_and_partial_results_retain_native_evidence() {
    use binance_client::advanced_trading::stocks;
    for (code, outcome) in [
        (486_446, Outcome::Unknown),
        (486_447, Outcome::Partial),
        (999_999, Outcome::Unknown),
    ] {
        let fixture = HttpFixture::new(
            200,
            "",
            &format!(r#"{{"code":{code},"success":false}}"#),
            None,
            false,
        )
        .await;
        let client =
            stocks::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
        let Error::Venue(failure) = client
            .cancel_all_equity_orders(
                &stocks::rest_requests::CancelAllEquityOrders::new(),
                deadline(),
            )
            .await
            .unwrap_err()
        else {
            panic!("native cancel evidence")
        };
        assert_eq!(failure.outcome, outcome);
        assert_eq!(failure.code, Some(code));
        if outcome == Outcome::Partial {
            assert!(failure.partial.unwrap().legs.is_empty());
        }
        fixture.finish().await;
    }
    let fixture = HttpFixture::new(
        200,
        "",
        r#"{"orderId":"synthetic","status":"F"}"#,
        None,
        false,
    )
    .await;
    let client = stocks::RestClient::new(native_config().rest_url(&fixture.url).unwrap()).unwrap();
    assert_eq!(
        client
            .cancel_equity_order(
                &stocks::rest_requests::CancelEquityOrder::new().order_id("synthetic"),
                deadline()
            )
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::Rejected)
    );
    fixture.finish().await;
    for body in [
        r#"{"code":"-2011","msg":"synthetic"}"#,
        r#"{"code":"FUTURE","msg":"synthetic"}"#,
    ] {
        let fixture = HttpFixture::new(200, "", body, None, false).await;
        let client =
            portfolio_margin::RestClient::new(native_config().rest_url(&fixture.url).unwrap())
                .unwrap();
        let Error::Venue(failure) = client
            .cancel_all_cm_open_conditional_orders(
                &portfolio_margin::rest_requests::CancelAllCmOpenConditionalOrders::new()
                    .symbol(Symbol::new("BTCUSD_PERP").unwrap()),
                deadline(),
            )
            .await
            .unwrap_err()
        else {
            panic!("native PM code")
        };
        assert_eq!(failure.outcome, Outcome::Unknown);
        assert!(failure.text_code.is_some());
        fixture.finish().await;
    }
}

#[test]
fn reconciliation_reads_require_ids_and_account_money_while_ignored_pro_field_stays_opaque() {
    use binance_client::advanced_trading::{alpha, portfolio_margin_pro as pro};
    assert!(
        serde_json::from_str::<portfolio_margin::rest_models::QueryUmOrderResponse>(
            r#"{"symbol":"BTCUSDT"}"#
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<pro::rest_models::GetPortfolioMarginProAccountInfoResponse>("{}")
            .is_err()
    );
    let data:pro::rest_models::GetPortfolioMarginProAccountInfoResponse=serde_json::from_str(r#"{"uniMMR":"2","accountEquity":"1","actualEquity":"1","accountMaintMargin":"0.1","accountInitialMargin":"0.2","accountStatus":"NORMAL","accountType":"PM_1","totalAvailableBalance":"122,559.90199491"}"#).unwrap();
    assert_eq!(
        data.extra.as_value()["totalAvailableBalance"],
        "122,559.90199491"
    );
    assert!(serde_json::from_str::<alpha::rest_models::AggregatedTradesResponse>(r#"{"code":"000000","success":true,"data":[{"a":1,"p":"bad","q":"1","f":1,"l":1,"T":1,"m":false}]}"#).is_err());
}

#[test]
fn native_conditional_bounds_and_price_match_amendments_are_usable() {
    use binance_client::advanced_trading::portfolio_margin_pro::rest_requests::SetMarginCallLevel;

    use binance_client::ClientOrderId;
    use portfolio_margin::rest_requests::{
        ModifyUmOrder, NewCmConditionalOrder, NewUmAlgoOrder, NewUmOrder,
    };
    let amendment = ModifyUmOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .quantity(Decimal::ONE)
        .orig_client_order_id(ClientOrderId::new("original").unwrap());
    assert!(amendment.clone().build().is_err());
    assert!(amendment.clone().price_match("QUEUE").build().is_ok());
    assert!(
        amendment
            .price_match("QUEUE")
            .price(Decimal::ONE)
            .build()
            .is_err()
    );
    let long_id = ClientOrderId::new("0123456789abcdef0123456789abcdef123").unwrap();
    assert!(
        NewUmOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(long_id)
            .build()
            .is_err()
    );
    let cm = NewCmConditionalOrder::new()
        .symbol(Symbol::new("BTCUSD_PERP").unwrap())
        .side("SELL")
        .strategy_type("TRAILING_STOP_MARKET")
        .quantity(Decimal::ONE)
        .new_client_strategy_id(ClientOrderId::new("cm-trailing").unwrap());
    assert!(
        cm.clone()
            .callback_rate(Decimal::new(1, 2))
            .build()
            .is_err()
    );
    assert!(cm.clone().callback_rate(Decimal::from(5)).build().is_ok());
    assert!(cm.callback_rate(Decimal::new(51, 1)).build().is_err());
    let um = NewUmAlgoOrder::new()
        .algo_type("CONDITIONAL")
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("SELL")
        .type_value("TRAILING_STOP_MARKET")
        .quantity(Decimal::ONE)
        .client_algo_id(ClientOrderId::new("um-trailing").unwrap());
    assert!(um.clone().callback_rate(Decimal::from(10)).build().is_ok());
    assert!(um.callback_rate(Decimal::new(101, 1)).build().is_err());
    assert!(
        SetMarginCallLevel::new()
            .margin_call_level(Decimal::new(11, 1))
            .build()
            .is_ok()
    );
    assert!(
        SetMarginCallLevel::new()
            .margin_call_level(Decimal::new(21, 1))
            .build()
            .is_err()
    );
}

#[tokio::test]
async fn portfolio_documented_503_refusals_stay_distinct_without_replay() {
    for (body, outcome) in [
        (
            r#"{"code":-1000,"msg":"Service Unavailable."}"#,
            Outcome::Rejected,
        ),
        (
            r#"{"code":-1000,"msg":"Internal error; unable to process your request. Please try again."}"#,
            Outcome::Rejected,
        ),
        (
            r#"{"code":-1008,"msg":"synthetic overload"}"#,
            Outcome::Rejected,
        ),
        (
            r#"{"code":-1000,"msg":"Unknown error, please check your request or try again later."}"#,
            Outcome::Unknown,
        ),
        (
            r#"{"code":-1000,"msg":"future server failure"}"#,
            Outcome::Unknown,
        ),
    ] {
        let fixture = HttpFixture::new(503, "", body, None, false).await;
        let client =
            portfolio_margin::RestClient::new(native_config().rest_url(&fixture.url).unwrap())
                .unwrap();
        let error = client
            .cancel_all_cm_open_conditional_orders(
                &portfolio_margin::rest_requests::CancelAllCmOpenConditionalOrders::new()
                    .symbol(Symbol::new("BTCUSD_PERP").unwrap()),
                deadline(),
            )
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(outcome));
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn portfolio_reported_account_order_limit_binds_across_clients_without_count() {
    use binance_client::AccountKey;
    let pools = WeightPools::new();
    let clock: Arc<dyn binance_client::Clock> = Arc::new(FixedClock(1_700_000_001_000));
    let account = AccountKey::new("synthetic-papi-account");
    let config = || {
        portfolio_margin::Config::with_pools_for_account(&pools, &account)
            .unwrap()
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
    };
    let query = HttpFixture::new(
        200,
        "",
        r#"[{"rateLimitType":"ORDERS","interval":"MINUTE","intervalNum":1,"limit":1}]"#,
        None,
        false,
    )
    .await;
    let client = portfolio_margin::RestClient::new(config().rest_url(&query.url).unwrap()).unwrap();
    client
        .query_user_rate_limit(
            &portfolio_margin::rest_requests::QueryUserRateLimit::new(),
            deadline(),
        )
        .await
        .unwrap();
    query.finish().await;
    let order = HttpFixture::new(
        503,
        "",
        r#"{"code":-1000,"msg":"future server failure"}"#,
        None,
        false,
    )
    .await;
    let first = portfolio_margin::RestClient::new(config().rest_url(&order.url).unwrap()).unwrap();
    let second = portfolio_margin::RestClient::new(config().rest_url(&order.url).unwrap()).unwrap();
    let request = portfolio_margin::rest_requests::NewUmOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(binance_client::ClientOrderId::new("synthetic-papi-order").unwrap());
    assert_eq!(
        first
            .new_um_order(&request, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::Unknown)
    );
    assert!(matches!(
        second.new_um_order(&request, deadline()).await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(order.connections_accepted(), 1);
    order.finish().await;
}
