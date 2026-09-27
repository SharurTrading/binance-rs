// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Local product signing, failure, and shared quota contracts.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]

#[allow(
    dead_code,
    reason = "fixture helpers are shared across integration suites"
)]
mod support;
use binance_client::{
    BudgetLimits, Budgets, Credentials, Decimal, Error, Outcome, Symbol, coinm, spot, usdm,
};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};

fn spot_config() -> spot::Config {
    spot::Config::new(spot::Environment::Demo)
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap())
}
fn spot_order() -> spot::rest_requests::NewOrder {
    spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("这是测试币456").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quote_order_qty(Decimal::new(123_456_789, 8))
        .new_client_order_id(spot::ClientOrderId::new("caller &/订单").unwrap())
        .recv_window(Decimal::new(6_000_346, 3))
}

#[tokio::test]
async fn spot_signs_percent_encoded_unicode_and_exact_quote_spend() {
    let mut f = HttpFixture::new(
        200,
        "",
        r#"{"orderId":9,"clientOrderId":"caller &/订单"}"#,
        None,
        false,
    )
    .await;
    let client = spot::RestClient::new(spot_config().rest_url(&f.url).unwrap()).unwrap();
    let result = client.new_order(&spot_order(), deadline()).await.unwrap();
    assert_eq!(result.data.client_order_id.as_str(), "caller &/订单");
    let wire = f.requests.recv().await.unwrap();
    assert!(wire.starts_with("POST /api/v3/order HTTP/1.1"));
    let (payload, signature) = wire
        .split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .rsplit_once("&signature=")
        .unwrap();
    assert!(payload.contains("quoteOrderQty=1.23456789"));
    assert!(payload.contains("recvWindow=6000.346"));
    assert!(payload.contains("newClientOrderId=caller+%26%2F%E8%AE%A2%E5%8D%95"));
    assert!(!payload.contains("quantity="));
    let key = aws_lc_rs::hmac::Key::new(aws_lc_rs::hmac::HMAC_SHA256, b"synthetic-secret");
    let expected = aws_lc_rs::hmac::sign(&key, payload.as_bytes())
        .as_ref()
        .iter()
        .fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            write!(s, "{b:02x}").unwrap();
            s
        });
    assert_eq!(signature, expected);
    assert_eq!(f.attempts(), 1);
    f.finish().await;
}

#[tokio::test]
async fn spot_5xx_cannot_borrow_futures_definitive_rejection() {
    let f = HttpFixture::new(503, "", r#"{"msg":"Service Unavailable."}"#, None, false).await;
    let c = spot::RestClient::new(spot_config().rest_url(&f.url).unwrap()).unwrap();
    let error = c.new_order(&spot_order(), deadline()).await.unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Venue(evidence) = error else {
        panic!("missing venue evidence")
    };
    assert_eq!(
        evidence.client_order_ids["newClientOrderId"],
        "caller &/订单"
    );
    assert_eq!(f.attempts(), 1);
    f.finish().await;
}

#[tokio::test]
async fn spot_truncated_mutation_preserves_rate_evidence_without_retry() {
    let f = HttpFixture::new(
        200,
        "X-MBX-USED-WEIGHT-1M: 99\r\nRetry-After: 2\r\n",
        "{",
        Some(100),
        false,
    )
    .await;
    let c = spot::RestClient::new(spot_config().rest_url(&f.url).unwrap()).unwrap();
    let error = c.new_order(&spot_order(), deadline()).await.unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        meta: Some(meta),
        client_order_ids,
        ..
    } = error
    else {
        panic!("missing metadata")
    };
    assert_eq!(client_order_ids["newClientOrderId"], "caller &/订单");
    assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 99);
    assert!(matches!(
        c.new_order(&spot_order(), deadline()).await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(f.attempts(), 1);
    f.finish().await;
}

#[tokio::test]
async fn futures_products_share_one_explicit_ip_owner() {
    let f = HttpFixture::new(200, "", r#"{"serverTime":1700000001000}"#, None, false).await;
    let budgets = Budgets::new(BudgetLimits::coinm().weight_per_minute(2)).unwrap();
    let um = usdm::RestClient::new(
        support::config()
            .rest_url(&f.url)
            .unwrap()
            .budgets(budgets.clone()),
    )
    .unwrap();
    let cm = coinm::RestClient::new(
        coinm::Config::new(coinm::Environment::Demo)
            .unwrap()
            .rest_url(&f.url)
            .unwrap()
            .clock(Arc::new(FixedClock(1_700_000_001_000)))
            .budgets(budgets),
    )
    .unwrap();
    um.check_server_time(&usdm::rest_requests::CheckServerTime::new(), deadline())
        .await
        .unwrap();
    cm.check_server_time(&coinm::rest_requests::CheckServerTime::new(), deadline())
        .await
        .unwrap();
    assert!(matches!(
        cm.clone()
            .check_server_time(&coinm::rest_requests::CheckServerTime::new(), deadline())
            .await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(f.attempts(), 2);
    f.finish().await;
}

#[tokio::test]
async fn spot_public_metadata_requires_no_credentials_and_is_expired_before_send() {
    let mut f = HttpFixture::new(200, "", r#"{"serverTime":1}"#, None, false).await;
    let c = spot::RestClient::new(
        spot::Config::new(spot::Environment::Demo)
            .unwrap()
            .rest_url(&f.url)
            .unwrap(),
    )
    .unwrap();
    c.time(&spot::rest_requests::Time::new(), deadline())
        .await
        .unwrap();
    assert!(
        f.requests
            .recv()
            .await
            .unwrap()
            .starts_with("GET /api/v3/time?")
    );
    assert!(matches!(
        c.time(
            &spot::rest_requests::Time::new(),
            tokio::time::Instant::now()
        )
        .await,
        Err(Error::Expired(_))
    ));
    assert_eq!(f.attempts(), 1);
    f.finish().await;
}

#[tokio::test]
async fn spot_success_refunds_weight_but_rejection_and_venue_counters_do_not() {
    for (status, body, headers, accepts_second) in [
        (
            200,
            r#"{"orderId":9,"clientOrderId":"caller &/订单"}"#,
            "",
            true,
        ),
        (400, r#"{"code":-2010,"msg":"rejected"}"#, "", false),
        (
            200,
            r#"{"orderId":9,"clientOrderId":"caller &/订单"}"#,
            "X-MBX-USED-WEIGHT-1M: 1\r\n",
            false,
        ),
    ] {
        let f = HttpFixture::new(status, headers, body, None, false).await;
        let budgets = Budgets::new(BudgetLimits::spot().weight_per_minute(1)).unwrap();
        let c = spot::RestClient::new(spot_config().rest_url(&f.url).unwrap().budgets(budgets))
            .unwrap();
        let _ = c.new_order(&spot_order(), deadline()).await;
        let second = c.new_order(&spot_order(), deadline()).await;
        assert_eq!(second.is_ok(), accepts_second);
        if !accepts_second {
            assert!(matches!(second, Err(Error::Admission { .. })));
        }
        assert_eq!(f.attempts(), if accepts_second { 2 } else { 1 });
        f.finish().await;
    }
}
