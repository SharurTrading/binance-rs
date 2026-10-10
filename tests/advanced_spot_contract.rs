// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Advanced Spot mutations retain both-leg evidence and caller identities.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;
use binance_client::WeightPools;
use binance_client::{Credentials, Decimal, Error, Outcome, Symbol, spot};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};

fn config() -> spot::Config {
    spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap())
}
fn replace() -> spot::rest_requests::OrderCancelReplace {
    spot::rest_requests::OrderCancelReplace::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("LIMIT")
        .time_in_force("GTC")
        .quantity(Decimal::ONE)
        .price(Decimal::from(10))
        .cancel_replace_mode("ALLOW_FAILURE")
        .cancel_orig_client_order_id(spot::ClientOrderId::new("old").unwrap())
        .cancel_new_client_order_id(spot::ClientOrderId::new("cancel").unwrap())
        .new_client_order_id(spot::ClientOrderId::new("new").unwrap())
}

#[tokio::test]
async fn cancel_replace_409_retains_success_and_rejection_without_retry() {
    let f = HttpFixture::new(409, "X-MBX-USED-WEIGHT-1M: 9\r\n", r#"{"code":-2021,"data":{"cancelResult":"SUCCESS","newOrderResult":"FAILURE","cancelResponse":{"orderId":11,"clientOrderId":"cancel","origClientOrderId":"old"},"newOrderResponse":{"code":-2010,"msg":"synthetic refusal"}}}"#, None, false).await;
    let c = spot::RestClient::new(config().rest_url(&f.url).unwrap()).unwrap();
    let Error::Venue(e) = c
        .order_cancel_replace(&replace(), deadline())
        .await
        .unwrap_err()
    else {
        panic!("venue evidence")
    };
    assert_eq!(e.outcome, Outcome::Partial);
    let partial = e.partial.unwrap();
    assert_eq!(partial.legs["cancel"].order_id, Some(11));
    assert_eq!(
        partial.legs["cancel"].client_order_id.as_deref(),
        Some("cancel")
    );
    assert_eq!(partial.legs["newOrder"].code, Some(-2010));
    assert_eq!(e.client_order_ids["cancelOrigClientOrderId"], "old");
    assert_eq!(e.rates.counters["x-mbx-used-weight-1m"], 9);
    assert_eq!(f.connections_accepted(), 1);
    f.finish().await;
}

#[tokio::test]
async fn truncated_replace_is_unknown_and_never_retried() {
    let f = HttpFixture::new(409, "Retry-After: 2\r\n", "{", Some(100), false).await;
    let c = spot::RestClient::new(config().rest_url(&f.url).unwrap()).unwrap();
    let e = c
        .order_cancel_replace(&replace(), deadline())
        .await
        .unwrap_err();
    assert_eq!(e.outcome(), Some(Outcome::Unknown));
    assert_eq!(f.connections_accepted(), 1);
    f.finish().await;
}

#[test]
fn list_ids_and_leg_ids_must_all_be_caller_supplied() {
    let b = spot::rest_requests::OrderListOco::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("SELL")
        .quantity(Decimal::ONE)
        .above_type("LIMIT_MAKER")
        .above_price(Decimal::from(20))
        .below_type("STOP_LOSS")
        .below_stop_price(Decimal::from(10));
    assert!(b.clone().build().is_err());
    assert!(
        b.list_client_order_id(spot::ClientOrderId::new("list").unwrap())
            .above_client_order_id(spot::ClientOrderId::new("above").unwrap())
            .below_client_order_id(spot::ClientOrderId::new("below").unwrap())
            .build()
            .is_ok()
    );
    assert!(replace().build().is_ok());
}

#[tokio::test]
async fn explicit_microseconds_preserve_wire_and_metadata_units() {
    let mut f = HttpFixture::new(200, "", r#"{"serverTime":1700000001000123}"#, None, false).await;
    let c = spot::RestClient::new(
        config()
            .time_unit(binance_client::TimeUnit::Microseconds)
            .rest_url(&f.url)
            .unwrap(),
    )
    .unwrap();
    let r = c
        .time(&spot::rest_requests::Time::new(), deadline())
        .await
        .unwrap();
    assert_eq!(r.meta.time_unit, binance_client::TimeUnit::Microseconds);
    assert_eq!(r.data.server_time, Some(1_700_000_001_000_123));
    assert!(
        f.requests
            .recv()
            .await
            .unwrap()
            .to_ascii_lowercase()
            .contains("x-mbx-time-unit: microsecond")
    );
    f.finish().await;
}

#[tokio::test]
async fn malformed_financial_evidence_cannot_prove_an_accepted_leg() {
    let f=HttpFixture::new(409,"",r#"{"code":-2021,"data":{"cancelResult":"SUCCESS","newOrderResult":"FAILURE","cancelResponse":{"orderId":11,"clientOrderId":"cancel","price":"not-a-price"},"newOrderResponse":{"code":-2010}}}"#,None,false).await;
    let c = spot::RestClient::new(config().rest_url(&f.url).unwrap()).unwrap();
    let e = c
        .order_cancel_replace(&replace(), deadline())
        .await
        .unwrap_err();
    assert_eq!(e.outcome(), Some(Outcome::Unknown));
    assert_eq!(f.connections_accepted(), 1);
    f.finish().await;
}

#[tokio::test]
async fn future_leg_codes_never_imply_definitive_rejection() {
    let f=HttpFixture::new(409,"",r#"{"code":-2021,"data":{"cancelResult":"SUCCESS","newOrderResult":"FAILURE","cancelResponse":{"orderId":11,"clientOrderId":"cancel"},"newOrderResponse":{"code":-999999,"msg":"do-not-log-this-body"}}}"#,None,false).await;
    let c = spot::RestClient::new(config().rest_url(&f.url).unwrap()).unwrap();
    let e = c
        .order_cancel_replace(&replace(), deadline())
        .await
        .unwrap_err();
    assert_eq!(e.outcome(), Some(Outcome::Unknown));
    assert!(!format!("{e:?}").contains("do-not-log-this-body"));
    let Error::Venue(e) = e else {
        panic!("venue evidence")
    };
    assert_eq!(e.partial.unwrap().legs["newOrder"].code, Some(-999_999));
    assert_eq!(f.connections_accepted(), 1);
    f.finish().await;
}

#[tokio::test]
async fn both_failed_and_not_attempted_legs_remain_distinct() {
    let f=HttpFixture::new(400,"",r#"{"code":-2022,"data":{"cancelResult":"FAILURE","newOrderResult":"NOT_ATTEMPTED","cancelResponse":{"code":-2011},"newOrderResponse":null}}"#,None,false).await;
    let c = spot::RestClient::new(config().rest_url(&f.url).unwrap()).unwrap();
    let Error::Venue(e) = c
        .order_cancel_replace(&replace(), deadline())
        .await
        .unwrap_err()
    else {
        panic!("venue evidence")
    };
    assert_eq!(e.outcome, Outcome::Rejected);
    let p = e.partial.unwrap();
    assert_eq!(p.legs["cancel"].outcome, Outcome::Rejected);
    assert_eq!(p.legs["newOrder"].outcome, Outcome::NotSent);
    f.finish().await;
}

#[tokio::test]
async fn oco_reserves_two_order_slots_shared_by_client_clones() {
    let f=HttpFixture::new(200,"",r#"{"orderListId":1,"orders":[{"orderId":2,"clientOrderId":"above"},{"orderId":3,"clientOrderId":"below"}]}"#,None,false).await;
    let budgets =
        binance_client::Budgets::new(binance_client::BudgetLimits::spot().orders(2, u64::MAX))
            .unwrap();
    let c = spot::RestClient::new(config().budgets(budgets).rest_url(&f.url).unwrap()).unwrap();
    let r = spot::rest_requests::OrderListOco::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("SELL")
        .quantity(Decimal::ONE)
        .above_type("LIMIT_MAKER")
        .above_price(Decimal::from(20))
        .below_type("STOP_LOSS")
        .below_stop_price(Decimal::from(10))
        .list_client_order_id(spot::ClientOrderId::new("list").unwrap())
        .above_client_order_id(spot::ClientOrderId::new("above").unwrap())
        .below_client_order_id(spot::ClientOrderId::new("below").unwrap());
    c.order_list_oco(&r, deadline()).await.unwrap();
    assert!(matches!(
        c.clone().order_list_oco(&r, deadline()).await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(f.connections_accepted(), 1);
    f.finish().await;
}
