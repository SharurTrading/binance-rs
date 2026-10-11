// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Clients given one account key share that account's counters within a pool.
//!
//! Binance counts orders per account, not per client. Spot's unfilled order count
//! is tracked by (sub)account and shared across all IP addresses, API keys and APIs
//! (<https://github.com/binance/binance-spot-api-docs/blob/master/faqs/order_count_decrement.md>);
//! USDⓈ-M and COIN-M share one order budget of 1,200 a minute and 300 per ten seconds
//! (<https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice>,
//! checked 2026-10-11). Each test builds its own registry.

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
    AccountKey, Clock, Credentials, Decimal, Error, Outcome, Symbol, WeightPools, coinm, margin,
    usdm,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use support::{HttpFixture, deadline};

const MINUTE_START: u64 = 1_700_000_040_000;
const USDM_ORDER: &str =
    r#"{"orderId":7,"clientOrderId":"account-key","status":"NEW","executedQty":"0"}"#;
const COINM_ORDER: &str =
    r#"{"orderId":8,"clientOrderId":"account-key","status":"NEW","executedQty":"0"}"#;

struct ManualClock(AtomicU64);

impl ManualClock {
    fn at(millis: u64) -> Arc<Self> {
        Arc::new(Self(AtomicU64::new(millis)))
    }
}

impl Clock for ManualClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}

fn assert_refused(error: &Error, retry_after: Duration) {
    assert_eq!(error.outcome(), Some(Outcome::NotSent), "{error:?}");
    assert!(
        matches!(error, Error::Admission { retry_after: actual } if *actual == retry_after),
        "{error:?}"
    );
}

fn usdm_client(
    config: usdm::Config,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> usdm::RestClient {
    usdm::RestClient::new(
        config
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap()
}

fn coinm_client(
    config: coinm::Config,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> coinm::RestClient {
    coinm::RestClient::new(
        config
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap()
}

fn usdm_order() -> usdm::rest_requests::NewOrder {
    usdm::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(binance_client::ClientOrderId::new("account-key").unwrap())
}

fn coinm_order() -> coinm::rest_requests::NewOrder {
    coinm::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSD_PERP").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(binance_client::ClientOrderId::new("account-key").unwrap())
}

async fn usdm_place(client: &usdm::RestClient) -> Result<(), Error> {
    client.new_order(&usdm_order(), deadline()).await.map(drop)
}

async fn coinm_place(client: &coinm::RestClient) -> Result<(), Error> {
    client.new_order(&coinm_order(), deadline()).await.map(drop)
}

async fn usdm_time(client: &usdm::RestClient) -> Result<(), Error> {
    client
        .check_server_time(&usdm::rest_requests::CheckServerTime::new(), deadline())
        .await
        .map(drop)
}

async fn coinm_time(client: &coinm::RestClient) -> Result<(), Error> {
    client
        .check_server_time(&coinm::rest_requests::CheckServerTime::new(), deadline())
        .await
        .map(drop)
}

#[tokio::test]
async fn usdm_and_coinm_clients_of_one_key_share_order_counters() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let key = AccountKey::new("account-one");
    let usdm_venue = HttpFixture::keep_alive(200, "", USDM_ORDER, None).await;
    let coinm_venue = HttpFixture::keep_alive(200, "", COINM_ORDER, None).await;
    let linear = usdm_client(
        usdm::Config::with_pools_for_account(usdm::Environment::Production, &pools, &key).unwrap(),
        &usdm_venue,
        &clock,
    );
    let inverse = coinm_client(
        coinm::Config::with_pools_for_account(coinm::Environment::Production, &pools, &key)
            .unwrap(),
        &coinm_venue,
        &clock,
    );

    // The shared futures order budget is 300 per ten seconds; each product spends half.
    for _ in 0..150 {
        usdm_place(&linear).await.unwrap();
        coinm_place(&inverse).await.unwrap();
    }
    assert_refused(
        &usdm_place(&linear).await.unwrap_err(),
        Duration::from_secs(10),
    );
    assert_refused(
        &coinm_place(&inverse).await.unwrap_err(),
        Duration::from_secs(10),
    );
    usdm_venue.finish().await;
    coinm_venue.finish().await;
}

#[tokio::test]
async fn clients_of_different_keys_on_one_pool_keep_separate_order_counters() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let exhausted = HttpFixture::new(
        200,
        "X-MBX-ORDER-COUNT-1M: 1200\r\n",
        USDM_ORDER,
        None,
        false,
    )
    .await;
    let venue = HttpFixture::new(200, "", COINM_ORDER, None, false).await;
    let first = usdm_client(
        usdm::Config::with_pools_for_account(
            usdm::Environment::Production,
            &pools,
            &AccountKey::new("account-one"),
        )
        .unwrap(),
        &exhausted,
        &clock,
    );
    let second = coinm_client(
        coinm::Config::with_pools_for_account(
            coinm::Environment::Production,
            &pools,
            &AccountKey::new("account-two"),
        )
        .unwrap(),
        &venue,
        &clock,
    );
    usdm_place(&first).await.unwrap();
    coinm_place(&second).await.unwrap();
    assert_eq!(venue.connections_accepted(), 1);
    exhausted.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn an_order_count_header_seen_by_one_keyed_client_holds_every_client_of_the_key() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START + 15_000);
    let key = AccountKey::new(b"account-one".as_slice());
    let exhausted = HttpFixture::new(
        200,
        "X-MBX-ORDER-COUNT-1M: 1200\r\n",
        USDM_ORDER,
        None,
        false,
    )
    .await;
    let venue = HttpFixture::keep_alive(200, "", COINM_ORDER, None).await;
    let reporter = usdm_client(
        usdm::Config::with_pools_for_account(usdm::Environment::Production, &pools, &key).unwrap(),
        &exhausted,
        &clock,
    );
    let same_key = [
        coinm_client(
            coinm::Config::with_pools_for_account(
                coinm::Environment::Production,
                &pools,
                &AccountKey::new("account-one"),
            )
            .unwrap(),
            &venue,
            &clock,
        ),
        coinm_client(
            coinm::Config::with_pools_for_account(coinm::Environment::Production, &pools, &key)
                .unwrap(),
            &venue,
            &clock,
        ),
    ];
    let unkeyed = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
        &venue,
        &clock,
    );
    usdm_place(&reporter).await.unwrap();
    for client in &same_key {
        assert_refused(
            &coinm_place(client).await.unwrap_err(),
            Duration::from_secs(45),
        );
    }
    coinm_place(&unkeyed).await.unwrap();
    exhausted.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_keyed_client_shares_ip_weight_with_unkeyed_clients_of_its_pool() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let spent = HttpFixture::new(
        200,
        "X-MBX-USED-WEIGHT-1M: 2400\r\n",
        "{\"serverTime\":1}",
        None,
        false,
    )
    .await;
    let venue = HttpFixture::new(200, "", "{\"serverTime\":1}", None, false).await;
    let keyed = usdm_client(
        usdm::Config::with_pools_for_account(
            usdm::Environment::Production,
            &pools,
            &AccountKey::new("account-one"),
        )
        .unwrap(),
        &spent,
        &clock,
    );
    let unkeyed = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
        &venue,
        &clock,
    );
    let other_key = usdm_client(
        usdm::Config::with_pools_for_account(
            usdm::Environment::Production,
            &pools,
            &AccountKey::new("account-two"),
        )
        .unwrap(),
        &venue,
        &clock,
    );
    usdm_time(&keyed).await.unwrap();
    assert_refused(
        &coinm_time(&unkeyed).await.unwrap_err(),
        Duration::from_mins(1),
    );
    assert_refused(
        &usdm_time(&other_key).await.unwrap_err(),
        Duration::from_mins(1),
    );
    assert_eq!(venue.connections_accepted(), 0);
    spent.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn unkeyed_clients_of_one_pool_keep_their_own_order_counters() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let exhausted = HttpFixture::new(
        200,
        "X-MBX-ORDER-COUNT-1M: 1200\r\n",
        USDM_ORDER,
        None,
        false,
    )
    .await;
    let venue = HttpFixture::new(200, "", COINM_ORDER, None, false).await;
    let first = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Production, &pools).unwrap(),
        &exhausted,
        &clock,
    );
    let second = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
        &venue,
        &clock,
    );
    usdm_place(&first).await.unwrap();
    coinm_place(&second).await.unwrap();
    assert_eq!(venue.connections_accepted(), 1);
    exhausted.finish().await;
    venue.finish().await;
}

fn margin_quota(limit: u64) -> margin::rest_models::QueryCurrentMarginOrderCountUsageResponse {
    serde_json::from_value(serde_json::json!([{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":limit,"count":0}])).unwrap()
}

fn margin_client(
    config: margin::Config,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> margin::RestClient {
    margin::RestClient::new(
        config
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap()
}

async fn margin_place(client: &margin::RestClient) -> Result<(), Error> {
    client
        .margin_account_new_order(
            &margin::rest_requests::MarginAccountNewOrder::new()
                .symbol(Symbol::new("BTCUSDT").unwrap())
                .side("BUY")
                .type_value("MARKET")
                .quantity(Decimal::new(1, 3))
                .new_client_order_id(margin::ClientOrderId::new("account-key").unwrap())
                .build()
                .unwrap(),
            deadline(),
        )
        .await
        .map(drop)
}

#[tokio::test]
async fn margin_clients_of_one_key_share_its_native_order_windows() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let key = AccountKey::new("account-one");
    let venue = HttpFixture::new(
        200,
        "",
        r#"{"symbol":"BTCUSDT","orderId":1,"clientOrderId":"account-key","isIsolated":false,"transactTime":1000}"#,
        None,
        false,
    )
    .await;
    // The quota is installed on the keyed owner, so a later client of the key holds it.
    let first = margin_client(
        margin::Config::with_pools_for_account(&pools, &key)
            .unwrap()
            .clock(clock.clone())
            .order_limits(&margin_quota(1))
            .unwrap(),
        &venue,
        &clock,
    );
    let same_key = margin_client(
        margin::Config::with_pools_for_account(&pools, &key).unwrap(),
        &venue,
        &clock,
    );
    let other_key = margin_client(
        margin::Config::with_pools_for_account(&pools, &AccountKey::new("account-two"))
            .unwrap()
            .clock(clock.clone())
            .order_limits(&margin_quota(1))
            .unwrap(),
        &venue,
        &clock,
    );
    margin_place(&first).await.unwrap();
    assert_refused(
        &margin_place(&same_key).await.unwrap_err(),
        Duration::from_secs(10),
    );
    margin_place(&other_key).await.unwrap();
    // An unkeyed client has its own owner, with no quota installed on it.
    let unkeyed = margin_client(margin::Config::with_pools(&pools).unwrap(), &venue, &clock);
    assert_eq!(
        margin_place(&unkeyed).await.unwrap_err().outcome(),
        Some(Outcome::NotSent)
    );
    assert_eq!(venue.connections_accepted(), 2);
    venue.finish().await;
}

#[test]
fn an_account_key_debug_never_prints_the_key() {
    let key = AccountKey::new("account-secret-identifier");
    assert_eq!(format!("{key:?}"), "AccountKey([REDACTED])");
    assert_eq!(format!("{key:#?}"), "AccountKey([REDACTED])");
}
