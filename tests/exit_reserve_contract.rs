// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Public reserve configuration preserves shared owners and native wire eligibility.

#[cfg(test)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::{
    AccountKey, BudgetLimits, Budgets, ClientOrderId, Decimal, Error, ExitReserve, Outcome, Symbol,
    WeightPools, coinm, usdm,
};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, config, deadline};

#[cfg(test)]
fn order(id: &str) -> usdm::rest_requests::NewOrder {
    usdm::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("SELL")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(ClientOrderId::new(id).unwrap())
}

#[cfg(test)]
fn coinm_order(id: &str) -> coinm::rest_requests::NewOrder {
    coinm::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSD_PERP").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(ClientOrderId::new(id).unwrap())
}

fn refused(error: &Error) {
    assert!(matches!(error, Error::Admission { .. }), "{error:?}");
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
}

#[tokio::test]
async fn reserve_survives_new_default_sibling_and_cancel_is_sent_once() {
    let mut fixture =
        HttpFixture::new(200, "", "{\"serverTime\":1,\"orderId\":7}", None, false).await;
    let account = AccountKey::new("exit-reserve-contract");
    let first = usdm::Config::new_for_account(usdm::Environment::Demo, &account).unwrap();
    first
        .set_exit_reserve(ExitReserve::default().request_weight(2398))
        .unwrap();
    let first = usdm::RestClient::new(
        first
            .rest_url(&fixture.url)
            .unwrap()
            .clock(Arc::new(FixedClock(1000)))
            .credentials(
                binance_client::Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
            ),
    )
    .unwrap();
    // Drawing another default configuration must not overwrite the pool's reserve.
    let sibling = usdm::RestClient::new(
        usdm::Config::new_for_account(usdm::Environment::Demo, &account)
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(Arc::new(FixedClock(1000)))
            .credentials(
                binance_client::Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
            ),
    )
    .unwrap();
    let time = usdm::rest_requests::CheckServerTime::new();
    first.check_server_time(&time, deadline()).await.unwrap();
    sibling.check_server_time(&time, deadline()).await.unwrap();
    refused(
        &sibling
            .check_server_time(&time, deadline())
            .await
            .unwrap_err(),
    );
    sibling
        .cancel_order(
            &usdm::rest_requests::CancelOrder::new()
                .symbol(Symbol::new("BTCUSDT").unwrap())
                .order_id(7),
            deadline(),
        )
        .await
        .unwrap();
    assert_eq!(fixture.connections_accepted(), 3);
    for _ in 0..2 {
        assert!(
            fixture
                .requests
                .recv()
                .await
                .unwrap()
                .starts_with("GET /fapi/v1/time")
        );
    }
    assert!(
        fixture
            .requests
            .recv()
            .await
            .unwrap()
            .starts_with("DELETE /fapi/v1/order")
    );
    fixture.finish().await;
}

#[tokio::test]
async fn reducing_flag_accesses_order_reserve_and_failed_acknowledgment_is_not_retried() {
    let fixture = HttpFixture::new(500, "", "{}", None, false).await;
    let budgets = Budgets::new(BudgetLimits::usdm().orders(2, 2)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().orders(1))
        .unwrap();
    let client =
        usdm::RestClient::new(config().rest_url(&fixture.url).unwrap().budgets(budgets)).unwrap();
    let first = client
        .new_order(&order("ordinary"), deadline())
        .await
        .unwrap_err();
    assert_eq!(first.outcome(), Some(Outcome::Unknown));
    refused(
        &client
            .new_order(&order("ordinary-refused"), deadline())
            .await
            .unwrap_err(),
    );
    let reducing = client
        .new_order(&order("reducing").reduce_only("true"), deadline())
        .await
        .unwrap_err();
    assert_eq!(reducing.outcome(), Some(Outcome::Unknown));
    refused(
        &client
            .new_order(&order("total-refused").reduce_only("true"), deadline())
            .await
            .unwrap_err(),
    );
    assert_eq!(fixture.connections_accepted(), 2);
    fixture.finish().await;
}

#[tokio::test]
async fn runtime_reserve_retained_configuration_replaces_policy_for_connected_siblings() {
    let fixture = HttpFixture::new(200, "", "{\"serverTime\":1}", None, false).await;
    let owner = config()
        .rest_url(&fixture.url)
        .unwrap()
        .budgets(Budgets::new(BudgetLimits::usdm().weight_per_minute(4)).unwrap());
    let first = usdm::RestClient::new(owner.clone()).unwrap();
    let sibling = usdm::RestClient::new(owner.clone()).unwrap();
    let time = usdm::rest_requests::CheckServerTime::new();
    first.check_server_time(&time, deadline()).await.unwrap();
    owner
        .set_exit_reserve(ExitReserve::default().request_weight(3))
        .unwrap();
    refused(
        &sibling
            .check_server_time(&time, deadline())
            .await
            .unwrap_err(),
    );
    owner
        .set_exit_reserve(ExitReserve::default().request_weight(2))
        .unwrap();
    sibling.check_server_time(&time, deadline()).await.unwrap();
    refused(
        &first
            .check_server_time(&time, deadline())
            .await
            .unwrap_err(),
    );
    owner.set_exit_reserve(ExitReserve::default()).unwrap();
    first.check_server_time(&time, deadline()).await.unwrap();
    sibling.check_server_time(&time, deadline()).await.unwrap();
    refused(
        &first
            .check_server_time(&time, deadline())
            .await
            .unwrap_err(),
    );
    assert_eq!(fixture.connections_accepted(), 4);
    fixture.finish().await;
}

#[tokio::test]
async fn usdm_hedged_long_sell_uses_last_native_order_slot_without_reduce_only() {
    let mut fixture = HttpFixture::new(
        500,
        "X-MBX-ORDER-COUNT-10S: 299\r\nX-MBX-ORDER-COUNT-1M: 299\r\n",
        "{}",
        None,
        false,
    )
    .await;
    let pools = WeightPools::new();
    let account = AccountKey::new("hedge-owner");
    let owner = usdm::Config::with_pools_for_account(usdm::Environment::Demo, &pools, &account)
        .unwrap()
        .rest_url(&fixture.url)
        .unwrap()
        .clock(Arc::new(FixedClock(1000)))
        .credentials(
            binance_client::Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
        );
    owner
        .set_exit_reserve(ExitReserve::default().orders(1))
        .unwrap();
    let client = usdm::RestClient::new(owner).unwrap();
    let ordinary = client
        .new_order(&order("ordinary"), deadline())
        .await
        .unwrap_err();
    assert_eq!(ordinary.outcome(), Some(Outcome::Unknown));
    for request in [
        order("plain-sell"),
        order("both-sell").position_side("BOTH"),
        order("long-buy").side("BUY").position_side("LONG"),
        order("short-sell").position_side("SHORT"),
    ] {
        refused(&client.new_order(&request, deadline()).await.unwrap_err());
    }
    let hedged = order("hedged-close").position_side("LONG");
    let sent = client.new_order(&hedged, deadline()).await.unwrap_err();
    assert_eq!(sent.outcome(), Some(Outcome::Unknown));
    refused(
        &client
            .new_order(&order("another-close").position_side("LONG"), deadline())
            .await
            .unwrap_err(),
    );
    assert_eq!(fixture.connections_accepted(), 2);
    let first = fixture.requests.recv().await.unwrap();
    let second = fixture.requests.recv().await.unwrap();
    assert!(first.starts_with("POST /fapi/v1/order"));
    assert!(second.starts_with("POST /fapi/v1/order"));
    assert!(second.contains("side=SELL"));
    assert!(second.contains("positionSide=LONG"));
    assert!(!second.contains("reduceOnly"));
    fixture.finish().await;
}

#[tokio::test]
async fn coinm_hedged_short_buy_spends_same_uid_futures_capacity_but_not_another_uid() {
    let mut fixture = HttpFixture::new(
        500,
        "X-MBX-ORDER-COUNT-10S: 299\r\nX-MBX-ORDER-COUNT-1M: 299\r\n",
        "{}",
        None,
        false,
    )
    .await;
    let pools = WeightPools::new();
    let account = AccountKey::new("shared-futures-uid");
    let credentials =
        || binance_client::Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap();
    let usdm_owner =
        usdm::Config::with_pools_for_account(usdm::Environment::Demo, &pools, &account)
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(Arc::new(FixedClock(1000)))
            .credentials(credentials());
    usdm_owner
        .set_exit_reserve(ExitReserve::default().orders(1))
        .unwrap();
    let usdm = usdm::RestClient::new(usdm_owner).unwrap();
    let coinm = coinm::RestClient::new(
        coinm::Config::with_pools_for_account(coinm::Environment::Demo, &pools, &account)
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(Arc::new(FixedClock(1000)))
            .credentials(credentials()),
    )
    .unwrap();
    assert_eq!(
        usdm.new_order(&order("um-ordinary"), deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::Unknown)
    );
    refused(
        &coinm
            .new_order(
                &coinm_order("cm-opener").side("SELL").position_side("SHORT"),
                deadline(),
            )
            .await
            .unwrap_err(),
    );
    let close = coinm_order("cm-short-close").position_side("SHORT");
    assert_eq!(
        coinm
            .new_order(&close, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::Unknown)
    );
    refused(
        &usdm
            .new_order(
                &order("um-close-after-full").position_side("LONG"),
                deadline(),
            )
            .await
            .unwrap_err(),
    );
    let other = AccountKey::new("other-futures-uid");
    let other_client = usdm::RestClient::new(
        usdm::Config::with_pools_for_account(usdm::Environment::Demo, &pools, &other)
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(Arc::new(FixedClock(1000)))
            .credentials(credentials()),
    )
    .unwrap();
    assert_eq!(
        other_client
            .new_order(&order("other-uid"), deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::Unknown)
    );
    assert_eq!(fixture.connections_accepted(), 3);
    let first = fixture.requests.recv().await.unwrap();
    let second = fixture.requests.recv().await.unwrap();
    let third = fixture.requests.recv().await.unwrap();
    assert!(first.starts_with("POST /fapi/v1/order"));
    assert!(second.starts_with("POST /dapi/v1/order"));
    assert!(second.contains("side=BUY"));
    assert!(second.contains("positionSide=SHORT"));
    assert!(!second.contains("reduceOnly"));
    assert!(third.starts_with("POST /fapi/v1/order"));
    fixture.finish().await;
}

/// Each path in order, as the fixture received it.
#[cfg(test)]
async fn received(fixture: &mut HttpFixture, paths: &[&str]) {
    for path in paths {
        let request = fixture.requests.recv().await.unwrap();
        assert!(request.starts_with(path), "{path}: {request}");
    }
}

#[tokio::test]
async fn usdm_exit_reads_draw_on_the_reserve_that_refuses_the_same_ordinary_reads() {
    let mut fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(8)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(7))
        .unwrap();
    let client =
        usdm::RestClient::new(config().rest_url(&fixture.url).unwrap().budgets(budgets)).unwrap();
    let symbol = || Symbol::new("BTCUSDT").unwrap();
    let orders = usdm::rest_requests::CurrentAllOpenOrders::new().symbol(symbol());
    let algo = usdm::rest_requests::CurrentAllAlgoOpenOrders::new().symbol(symbol());
    let positions = usdm::rest_requests::PositionInformationV3::new().symbol(symbol());
    // The ordinary share is the one weight unit above the reserve.
    client
        .current_all_open_orders(&orders, deadline())
        .await
        .unwrap();
    refused(
        &client
            .current_all_open_orders(&orders, deadline())
            .await
            .unwrap_err(),
    );
    refused(
        &client
            .current_all_algo_open_orders(&algo, deadline())
            .await
            .unwrap_err(),
    );
    refused(
        &client
            .position_information_v3(&positions, deadline())
            .await
            .unwrap_err(),
    );
    client
        .current_all_open_orders_for_exit(&orders, deadline())
        .await
        .unwrap();
    client
        .current_all_algo_open_orders_for_exit(&algo, deadline())
        .await
        .unwrap();
    client
        .position_information_v3_for_exit(&positions, deadline())
        .await
        .unwrap();
    // Weights 1 + 1 + 1 + 5 spent the venue's whole 8; the reserve lends nothing beyond it.
    refused(
        &client
            .current_all_open_orders_for_exit(&orders, deadline())
            .await
            .unwrap_err(),
    );
    assert_eq!(fixture.connections_accepted(), 4);
    received(
        &mut fixture,
        &[
            "GET /fapi/v1/openOrders?",
            "GET /fapi/v1/openOrders?",
            "GET /fapi/v1/openAlgoOrders?",
            "GET /fapi/v3/positionRisk?",
        ],
    )
    .await;
    fixture.finish().await;
}

#[tokio::test]
async fn coinm_exit_reads_draw_on_the_reserve_that_refuses_the_same_ordinary_reads() {
    let mut fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let budgets = Budgets::new(BudgetLimits::coinm().weight_per_minute(4)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(3))
        .unwrap();
    let client = coinm::RestClient::new(
        coinm::Config::with_pools(coinm::Environment::Demo, &WeightPools::new())
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(Arc::new(FixedClock(1_700_000_001_000)))
            .credentials(
                binance_client::Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
            )
            .budgets(budgets),
    )
    .unwrap();
    let symbol = || Symbol::new("BTCUSD_PERP").unwrap();
    let orders = coinm::rest_requests::CurrentAllOpenOrders::new().symbol(symbol());
    let algo = coinm::rest_requests::OpenAlgoOrders::new().symbol(symbol());
    let positions = coinm::rest_requests::PositionInformation::new();
    client
        .current_all_open_orders(&orders, deadline())
        .await
        .unwrap();
    refused(
        &client
            .current_all_open_orders(&orders, deadline())
            .await
            .unwrap_err(),
    );
    refused(
        &client
            .open_algo_orders(&algo, deadline())
            .await
            .unwrap_err(),
    );
    refused(
        &client
            .position_information(&positions, deadline())
            .await
            .unwrap_err(),
    );
    client
        .current_all_open_orders_for_exit(&orders, deadline())
        .await
        .unwrap();
    client
        .open_algo_orders_for_exit(&algo, deadline())
        .await
        .unwrap();
    client
        .position_information_for_exit(&positions, deadline())
        .await
        .unwrap();
    // Weights 1 + 1 + 1 + 1 spent the venue's whole 4.
    refused(
        &client
            .open_algo_orders_for_exit(&algo, deadline())
            .await
            .unwrap_err(),
    );
    assert_eq!(fixture.connections_accepted(), 4);
    received(
        &mut fixture,
        &[
            "GET /dapi/v1/openOrders?",
            "GET /dapi/v1/openOrders?",
            "GET /dapi/v1/openAlgoOrders?",
            "GET /dapi/v1/positionRisk?",
        ],
    )
    .await;
    fixture.finish().await;
}

#[tokio::test]
async fn spot_exit_open_orders_read_draws_on_the_reserve_that_refuses_the_ordinary_read() {
    let mut fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let budgets = Budgets::new(BudgetLimits::spot().weight_per_minute(12)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(6))
        .unwrap();
    let client = binance_client::spot::RestClient::new(
        binance_client::spot::Config::with_pools(
            binance_client::spot::Environment::Demo,
            &WeightPools::new(),
        )
        .unwrap()
        .rest_url(&fixture.url)
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(
            binance_client::Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap(),
        )
        .budgets(budgets),
    )
    .unwrap();
    let orders = binance_client::spot::rest_requests::GetOpenOrders::new()
        .symbol(Symbol::new("BTCUSDT").unwrap());
    // A symbol-scoped open-orders read weighs 6, the whole ordinary share.
    client.get_open_orders(&orders, deadline()).await.unwrap();
    refused(
        &client
            .get_open_orders(&orders, deadline())
            .await
            .unwrap_err(),
    );
    client
        .get_open_orders_for_exit(&orders, deadline())
        .await
        .unwrap();
    refused(
        &client
            .get_open_orders_for_exit(&orders, deadline())
            .await
            .unwrap_err(),
    );
    assert_eq!(fixture.connections_accepted(), 2);
    received(
        &mut fixture,
        &["GET /api/v3/openOrders?", "GET /api/v3/openOrders?"],
    )
    .await;
    fixture.finish().await;
}
