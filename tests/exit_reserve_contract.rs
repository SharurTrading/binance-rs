// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Public reserve configuration preserves shared owners and native wire eligibility.

#[cfg(test)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::{
    AccountKey, BudgetLimits, Budgets, ClientOrderId, Decimal, Error, ExitReserve, Outcome, Symbol,
    usdm,
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
