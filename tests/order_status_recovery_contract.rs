// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Bounded owned-order status recovery retains native admission and failure evidence.

#[cfg(test)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::{
    BudgetLimits, Budgets, ClientOrderId, Error, ExitReserve, Outcome, Symbol, TransportFailure,
    usdm,
};
use rust_decimal::Decimal;
use std::{collections::BTreeMap, time::Duration};
use support::{HttpFixture, config, deadline};
use tokio::time::Instant;

const STATUS: &str = r#"{"orderId":7,"clientOrderId":"owned-status","symbol":"BTCUSDT","status":"FILLED","executedQty":"0.125","avgPrice":"64000.25"}"#;

#[cfg(test)]
fn request() -> usdm::rest_requests::QueryOrder {
    usdm::rest_requests::QueryOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .orig_client_order_id(ClientOrderId::new("owned-status").unwrap())
        .recv_window(5_000)
        .build()
        .unwrap()
}

#[cfg(test)]
fn owner(fixture: &HttpFixture, limit: u64) -> usdm::Config {
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(limit)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(limit))
        .unwrap();
    config().rest_url(&fixture.url).unwrap().budgets(budgets)
}

fn admission(error: &Error) {
    assert!(matches!(error, Error::Admission { .. }), "{error:?}");
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
}

#[tokio::test]
async fn reserved_status_read_preserves_native_response_parameters_and_one_weight() {
    let mut fixture = HttpFixture::new(200, "", STATUS, None, false).await;
    let owner = owner(&fixture, 1);
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let request = request();
    admission(&client.query_order(&request, deadline()).await.unwrap_err());
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
    let reply = client
        .query_order_for_recovery(&request, deadline())
        .await
        .unwrap();
    assert_eq!(reply.data.order_id, 7);
    assert_eq!(
        reply
            .data
            .client_order_id
            .as_ref()
            .map(ClientOrderId::as_str),
        Some("owned-status")
    );
    assert_eq!(
        reply.data.symbol.as_ref().map(Symbol::as_str),
        Some("BTCUSDT")
    );
    assert_eq!(
        reply
            .data
            .status
            .as_ref()
            .map(usdm::enums::OrderStatus::as_str),
        Some("FILLED")
    );
    assert_eq!(reply.data.executed_qty, Some(Decimal::new(125, 3)));
    assert_eq!(reply.data.avg_price, Some(Decimal::new(6_400_025, 2)));
    assert_eq!(reply.meta.operation, "queryOrder");
    assert_eq!(reply.meta.status, 200);
    assert_eq!(
        reply
            .meta
            .client_order_ids
            .get("origClientOrderId")
            .map(String::as_str),
        Some("owned-status")
    );
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 1);
    admission(
        &client
            .query_order_for_recovery(&request, deadline())
            .await
            .unwrap_err(),
    );
    admission(&client.query_order(&request, deadline()).await.unwrap_err());
    assert_eq!(fixture.connections_accepted(), 1);
    let wire = fixture.requests.recv().await.unwrap();
    assert!(wire.starts_with("GET /fapi/v1/order?"));
    let url = url::Url::parse(&format!(
        "http://fixture{}",
        wire.split_whitespace().nth(1).unwrap()
    ))
    .unwrap();
    let pairs: BTreeMap<_, _> = url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    assert_eq!(pairs.get("symbol").map(String::as_str), Some("BTCUSDT"));
    assert_eq!(
        pairs.get("origClientOrderId").map(String::as_str),
        Some("owned-status")
    );
    assert_eq!(pairs.get("recvWindow").map(String::as_str), Some("5000"));
    assert_eq!(
        pairs.get("timestamp").map(String::as_str),
        Some("1700000001000")
    );
    assert!(pairs.contains_key("signature"));
    assert_eq!(pairs.len(), 5);
    fixture.finish().await;
}

#[tokio::test]
async fn expired_status_read_is_unsent_and_preserves_remaining_quota() {
    let fixture = HttpFixture::new(200, "", STATUS, None, false).await;
    let owner = owner(&fixture, 1);
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let error = client
        .query_order_for_recovery(&request(), Instant::now())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Expired("queryOrder")));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
    assert_eq!(fixture.connections_accepted(), 0);
    client
        .query_order_for_recovery(&request(), deadline())
        .await
        .unwrap();
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 1);
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn original_deadline_still_expires_after_prior_recovery_work() {
    let fixture = HttpFixture::new(200, "", r#"{"serverTime":1700000001000}"#, None, false).await;
    let owner = owner(&fixture, 2);
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let original_deadline = deadline();
    client
        .check_server_time_for_recovery(
            &usdm::rest_requests::CheckServerTime::new(),
            original_deadline,
        )
        .await
        .unwrap();
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(4)).await;
    let error = client
        .query_order_for_recovery(&request(), original_deadline)
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Expired("queryOrder")));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 1);
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn venue_cooldowns_refuse_another_protected_status_send() {
    for (status, headers) in [(429, "Retry-After: 2\r\n"), (418, "")] {
        let fixture = HttpFixture::new(
            status,
            headers,
            r#"{"code":-1003,"msg":"fixture cooldown"}"#,
            None,
            false,
        )
        .await;
        let owner = owner(&fixture, 10);
        let observer = owner.clone();
        let client = usdm::RestClient::new(owner).unwrap();
        let error = client
            .query_order_for_recovery(&request(), deadline())
            .await
            .unwrap_err();
        let Error::Venue(failure) = error else {
            panic!("expected native rate failure: {error:?}")
        };
        assert_eq!(failure.status, status);
        assert_eq!(failure.code, Some(-1003));
        assert_eq!(failure.operation, "queryOrder");
        assert_eq!(failure.outcome, Outcome::ReadFailed);
        assert_eq!(
            failure.rates.retry_after,
            if status == 429 {
                Some(Duration::from_secs(2))
            } else {
                None
            }
        );
        let refused = client
            .query_order_for_recovery(&request(), deadline())
            .await
            .unwrap_err();
        if status == 429 {
            admission(&refused);
        } else {
            assert!(matches!(refused, Error::CooldownTimingUnknown));
            assert_eq!(refused.outcome(), Some(Outcome::NotSent));
        }
        assert_eq!(observer.pool_usage().unwrap().request_weight.used, 1);
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn missing_native_order_preserves_the_venue_code_and_is_not_retried() {
    let fixture = HttpFixture::new(
        400,
        "X-MBX-USED-WEIGHT-1M: 3\r\n",
        r#"{"code":-2013,"msg":"Order does not exist."}"#,
        None,
        false,
    )
    .await;
    let owner = owner(&fixture, 10);
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let error = client
        .query_order_for_recovery(&request(), deadline())
        .await
        .unwrap_err();
    let Error::Venue(failure) = error else {
        panic!("expected native order failure: {error:?}")
    };
    assert_eq!(failure.status, 400);
    assert_eq!(failure.code, Some(-2013));
    assert_eq!(failure.operation, "queryOrder");
    assert_eq!(failure.outcome, Outcome::ReadFailed);
    assert_eq!(
        failure
            .client_order_ids
            .get("origClientOrderId")
            .map(String::as_str),
        Some("owned-status")
    );
    assert_eq!(failure.rates.counters.get("x-mbx-used-weight-1m"), Some(&3));
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 3);
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn malformed_and_truncated_status_responses_keep_safe_evidence_without_retry() {
    for (body, length, expected) in [
        ("not-json", None, TransportFailure::JsonDecodeFailed),
        (
            r#"{"status":"FILLED"}"#,
            None,
            TransportFailure::ResponseDecodeFailed,
        ),
        (
            r#"{"orderId":7,"executedQty":"bad"}"#,
            None,
            TransportFailure::ResponseDecodeFailed,
        ),
        ("{", Some(50), TransportFailure::ResponseBodyFailed),
    ] {
        let fixture =
            HttpFixture::new(200, "X-MBX-USED-WEIGHT-1M: 3\r\n", body, length, false).await;
        let owner = owner(&fixture, 10);
        let observer = owner.clone();
        let client = usdm::RestClient::new(owner).unwrap();
        let error = client
            .query_order_for_recovery(&request(), deadline())
            .await
            .unwrap_err();
        let Error::Transport {
            outcome,
            failure,
            meta: Some(meta),
            client_order_ids,
            ..
        } = error
        else {
            panic!("expected preserved transport evidence: {error:?}")
        };
        assert_eq!(outcome, Outcome::ReadFailed);
        assert_eq!(failure, Some(expected));
        assert_eq!(meta.status, 200);
        assert_eq!(meta.operation, "queryOrder");
        assert_eq!(meta.rates.counters.get("x-mbx-used-weight-1m"), Some(&3));
        assert_eq!(
            client_order_ids
                .get("origClientOrderId")
                .map(String::as_str),
            Some("owned-status")
        );
        assert_eq!(observer.pool_usage().unwrap().request_weight.used, 3);
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn status_without_order_identity_is_rejected_before_charge_or_send() {
    let fixture = HttpFixture::new(200, "", STATUS, None, false).await;
    let owner = owner(&fixture, 1);
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let invalid = usdm::rest_requests::QueryOrder::new().symbol(Symbol::new("BTCUSDT").unwrap());
    let error = client
        .query_order_for_recovery(&invalid, deadline())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        Error::Validation("order identity required")
    ));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}
