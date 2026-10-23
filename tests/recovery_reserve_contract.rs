// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit recovery reads preserve native costs, responses and unsent refusals.

#[cfg(test)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::{
    BudgetLimits, Budgets, Credentials, Error, ExitReserve, Outcome, Symbol, WeightPools, coinm,
    spot, usdm,
};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, config, deadline};
use tokio::time::Instant;

fn refused(error: &Error) {
    assert!(matches!(error, Error::Admission { .. }), "{error:?}");
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
}

#[cfg(test)]
fn response_fixture(product: &str, operation: &str) -> serde_json::Value {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/response-shapes.json")).unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| {
            case["product"] == product
                && case["transport"] == "rest"
                && case["operation"] == operation
        })
        .unwrap()["response"]
        .clone()
}

macro_rules! recovery_read {
    ($name:ident, $product:ident, $response:ty, $operation:literal, $ordinary:ident, $recovery:ident, $request:expr, $weight:literal, $path:literal, $time:ident, $time_request:expr) => {
        #[tokio::test]
        async fn $name() {
            let native = response_fixture(stringify!($product), $operation);
            let body = serde_json::to_string(&native).unwrap();
            let mut fixture = HttpFixture::new(200, "", &body, None, false).await;
            let budgets =
                Budgets::new(BudgetLimits::$product().weight_per_minute($weight)).unwrap();
            budgets
                .set_exit_reserve(ExitReserve::default().request_weight($weight))
                .unwrap();
            let owner =
                $product::Config::with_pools($product::Environment::Demo, &WeightPools::new())
                    .unwrap()
                    .rest_url(&fixture.url)
                    .unwrap()
                    .clock(Arc::new(FixedClock(1_700_000_001_000)))
                    .credentials(
                        Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
                    )
                    .budgets(budgets);
            let client = $product::RestClient::new(owner).unwrap();
            let request = $request;
            refused(&client.$ordinary(&request, deadline()).await.unwrap_err());
            let result = client.$recovery(&request, deadline()).await.unwrap();
            assert_eq!(
                result.data,
                serde_json::from_value::<$response>(native).unwrap()
            );
            refused(&client.$time(&$time_request, deadline()).await.unwrap_err());
            assert_eq!(fixture.connections_accepted(), 1);
            let wire = fixture.requests.recv().await.unwrap();
            assert!(wire.starts_with(concat!("GET ", $path)), "{wire}");
            let parsed = url::Url::parse(&format!(
                "http://fixture{}",
                wire.split_whitespace().nth(1).unwrap()
            ))
            .unwrap();
            let pairs: std::collections::BTreeMap<_, _> = parsed
                .query_pairs()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();
            for (key, value) in serde_json::to_value(&request).unwrap().as_object().unwrap() {
                if !value.is_null() {
                    let expected = value
                        .as_str()
                        .map_or_else(|| value.to_string(), str::to_owned);
                    assert_eq!(pairs.get(key), Some(&expected));
                }
            }
            fixture.finish().await;
        }
    };
}

recovery_read!(
    spot_get_account_uses_only_its_native_weight,
    spot,
    spot::rest_models::GetAccountResponse,
    "getAccount",
    get_account,
    get_account_for_recovery,
    spot::rest_requests::GetAccount::new(),
    20,
    "/api/v3/account",
    time_for_recovery,
    spot::rest_requests::Time::new()
);

recovery_read!(
    spot_get_open_orders_uses_only_its_native_weight,
    spot,
    spot::rest_models::GetOpenOrdersResponse,
    "getOpenOrders",
    get_open_orders,
    get_open_orders_for_recovery,
    spot::rest_requests::GetOpenOrders::new().symbol(Symbol::new("BTCUSDT").unwrap()),
    6,
    "/api/v3/openOrders",
    time_for_recovery,
    spot::rest_requests::Time::new()
);

recovery_read!(
    spot_my_trades_uses_only_its_native_weight,
    spot,
    spot::rest_models::MyTradesResponse,
    "myTrades",
    my_trades,
    my_trades_for_recovery,
    spot::rest_requests::MyTrades::new().symbol(Symbol::new("BTCUSDT").unwrap()),
    20,
    "/api/v3/myTrades",
    time_for_recovery,
    spot::rest_requests::Time::new()
);

recovery_read!(
    spot_time_uses_only_its_native_weight,
    spot,
    spot::rest_models::TimeResponse,
    "time",
    time,
    time_for_recovery,
    spot::rest_requests::Time::new(),
    1,
    "/api/v3/time",
    time_for_recovery,
    spot::rest_requests::Time::new()
);

recovery_read!(
    usdm_futures_account_balance_v3_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::FuturesAccountBalanceV3Response,
    "futuresAccountBalanceV3",
    futures_account_balance_v3,
    futures_account_balance_v3_for_recovery,
    usdm::rest_requests::FuturesAccountBalanceV3::new(),
    5,
    "/fapi/v3/balance",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_get_current_position_mode_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::GetCurrentPositionModeResponse,
    "getCurrentPositionMode",
    get_current_position_mode,
    get_current_position_mode_for_recovery,
    usdm::rest_requests::GetCurrentPositionMode::new(),
    30,
    "/fapi/v1/positionSide/dual",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_account_information_v3_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::AccountInformationV3Response,
    "accountInformationV3",
    account_information_v3,
    account_information_v3_for_recovery,
    usdm::rest_requests::AccountInformationV3::new(),
    5,
    "/fapi/v3/account",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_position_information_v3_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::PositionInformationV3Response,
    "positionInformationV3",
    position_information_v3,
    position_information_v3_for_recovery,
    usdm::rest_requests::PositionInformationV3::new().symbol(Symbol::new("BTCUSDT").unwrap()),
    5,
    "/fapi/v3/positionRisk",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_current_all_open_orders_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::CurrentAllOpenOrdersResponse,
    "currentAllOpenOrders",
    current_all_open_orders,
    current_all_open_orders_for_recovery,
    usdm::rest_requests::CurrentAllOpenOrders::new().symbol(Symbol::new("BTCUSDT").unwrap()),
    1,
    "/fapi/v1/openOrders",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_current_all_algo_open_orders_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::CurrentAllAlgoOpenOrdersResponse,
    "currentAllAlgoOpenOrders",
    current_all_algo_open_orders,
    current_all_algo_open_orders_for_recovery,
    usdm::rest_requests::CurrentAllAlgoOpenOrders::new().symbol(Symbol::new("BTCUSDT").unwrap()),
    1,
    "/fapi/v1/openAlgoOrders",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_account_trade_list_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::AccountTradeListResponse,
    "accountTradeList",
    account_trade_list,
    account_trade_list_for_recovery,
    usdm::rest_requests::AccountTradeList::new().symbol(Symbol::new("BTCUSDT").unwrap()),
    5,
    "/fapi/v1/userTrades",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_check_server_time_uses_only_its_native_weight,
    usdm,
    usdm::rest_models::CheckServerTimeResponse,
    "checkServerTime",
    check_server_time,
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new(),
    1,
    "/fapi/v1/time",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_futures_account_balance_uses_only_its_native_weight,
    coinm,
    coinm::rest_models::FuturesAccountBalanceResponse,
    "futuresAccountBalance",
    futures_account_balance,
    futures_account_balance_for_recovery,
    coinm::rest_requests::FuturesAccountBalance::new(),
    1,
    "/dapi/v1/balance",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_get_current_position_mode_uses_only_its_native_weight,
    coinm,
    coinm::rest_models::GetCurrentPositionModeResponse,
    "getCurrentPositionMode",
    get_current_position_mode,
    get_current_position_mode_for_recovery,
    coinm::rest_requests::GetCurrentPositionMode::new(),
    30,
    "/dapi/v1/positionSide/dual",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_account_information_uses_only_its_native_weight,
    coinm,
    coinm::rest_models::AccountInformationResponse,
    "accountInformation",
    account_information,
    account_information_for_recovery,
    coinm::rest_requests::AccountInformation::new(),
    5,
    "/dapi/v1/account",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_current_all_open_orders_uses_only_its_native_weight,
    coinm,
    coinm::rest_models::CurrentAllOpenOrdersResponse,
    "currentAllOpenOrders",
    current_all_open_orders,
    current_all_open_orders_for_recovery,
    coinm::rest_requests::CurrentAllOpenOrders::new().symbol(Symbol::new("BTCUSD_PERP").unwrap()),
    1,
    "/dapi/v1/openOrders",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_open_algo_orders_uses_only_its_native_weight,
    coinm,
    coinm::rest_models::OpenAlgoOrdersResponse,
    "openAlgoOrders",
    open_algo_orders,
    open_algo_orders_for_recovery,
    coinm::rest_requests::OpenAlgoOrders::new().symbol(Symbol::new("BTCUSD_PERP").unwrap()),
    1,
    "/dapi/v1/openAlgoOrders",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_account_trade_list_uses_only_its_native_weight,
    coinm,
    coinm::rest_models::AccountTradeListResponse,
    "accountTradeList",
    account_trade_list,
    account_trade_list_for_recovery,
    coinm::rest_requests::AccountTradeList::new().symbol(Symbol::new("BTCUSD_PERP").unwrap()),
    5,
    "/dapi/v1/userTrades",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_check_server_time_uses_only_its_native_weight,
    coinm,
    coinm::rest_models::CheckServerTimeResponse,
    "checkServerTime",
    check_server_time,
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new(),
    1,
    "/dapi/v1/time",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    spot_unscoped_open_orders_uses_eighty_weight,
    spot,
    spot::rest_models::GetOpenOrdersResponse,
    "getOpenOrders",
    get_open_orders,
    get_open_orders_for_recovery,
    spot::rest_requests::GetOpenOrders::new(),
    80,
    "/api/v3/openOrders",
    time_for_recovery,
    spot::rest_requests::Time::new()
);

recovery_read!(
    spot_order_scoped_trades_uses_five_weight,
    spot,
    spot::rest_models::MyTradesResponse,
    "myTrades",
    my_trades,
    my_trades_for_recovery,
    spot::rest_requests::MyTrades::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .order_id(7),
    5,
    "/api/v3/myTrades",
    time_for_recovery,
    spot::rest_requests::Time::new()
);

recovery_read!(
    usdm_unscoped_open_orders_uses_forty_weight,
    usdm,
    usdm::rest_models::CurrentAllOpenOrdersResponse,
    "currentAllOpenOrders",
    current_all_open_orders,
    current_all_open_orders_for_recovery,
    usdm::rest_requests::CurrentAllOpenOrders::new(),
    40,
    "/fapi/v1/openOrders",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    usdm_unscoped_algo_orders_uses_forty_weight,
    usdm,
    usdm::rest_models::CurrentAllAlgoOpenOrdersResponse,
    "currentAllAlgoOpenOrders",
    current_all_algo_open_orders,
    current_all_algo_open_orders_for_recovery,
    usdm::rest_requests::CurrentAllAlgoOpenOrders::new(),
    40,
    "/fapi/v1/openAlgoOrders",
    check_server_time_for_recovery,
    usdm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_unscoped_open_orders_uses_forty_weight,
    coinm,
    coinm::rest_models::CurrentAllOpenOrdersResponse,
    "currentAllOpenOrders",
    current_all_open_orders,
    current_all_open_orders_for_recovery,
    coinm::rest_requests::CurrentAllOpenOrders::new(),
    40,
    "/dapi/v1/openOrders",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

recovery_read!(
    coinm_unscoped_algo_orders_uses_forty_weight,
    coinm,
    coinm::rest_models::OpenAlgoOrdersResponse,
    "openAlgoOrders",
    open_algo_orders,
    open_algo_orders_for_recovery,
    coinm::rest_requests::OpenAlgoOrders::new(),
    40,
    "/dapi/v1/openAlgoOrders",
    check_server_time_for_recovery,
    coinm::rest_requests::CheckServerTime::new()
);

#[tokio::test]
async fn expired_recovery_read_is_unsent_and_does_not_consume_the_last_slot() {
    let fixture = HttpFixture::new(200, "", "{\"serverTime\":1700000001000}", None, false).await;
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(1)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(1))
        .unwrap();
    let client =
        usdm::RestClient::new(config().rest_url(&fixture.url).unwrap().budgets(budgets)).unwrap();
    let request = usdm::rest_requests::CheckServerTime::new();
    let error = client
        .check_server_time_for_recovery(&request, Instant::now())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Expired(_)));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(
        client
            .check_server_time_for_recovery(&request, deadline())
            .await
            .unwrap()
            .data
            .server_time,
        Some(1_700_000_001_000)
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn recovery_read_obeys_venue_cooldown_and_is_not_retried() {
    let fixture = HttpFixture::new(
        429,
        "Retry-After: 2\r\n",
        "{\"code\":-1003,\"msg\":\"fixture cooldown\"}",
        None,
        false,
    )
    .await;
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(10)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(10))
        .unwrap();
    let client =
        usdm::RestClient::new(config().rest_url(&fixture.url).unwrap().budgets(budgets)).unwrap();
    let request = usdm::rest_requests::CheckServerTime::new();
    let sent = client
        .check_server_time_for_recovery(&request, deadline())
        .await
        .unwrap_err();
    assert_eq!(sent.outcome(), Some(Outcome::ReadFailed));
    refused(
        &client
            .check_server_time_for_recovery(&request, deadline())
            .await
            .unwrap_err(),
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn untimed_venue_ban_refuses_recovery_before_a_second_send() {
    let fixture = HttpFixture::new(
        418,
        "",
        "{\"code\":-1003,\"msg\":\"fixture ban\"}",
        None,
        false,
    )
    .await;
    let client = usdm::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = usdm::rest_requests::CheckServerTime::new();
    assert_eq!(
        client
            .check_server_time_for_recovery(&request, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::ReadFailed)
    );
    let refused = client
        .check_server_time_for_recovery(&request, deadline())
        .await
        .unwrap_err();
    assert!(matches!(refused, Error::CooldownTimingUnknown));
    assert_eq!(refused.outcome(), Some(Outcome::NotSent));
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
