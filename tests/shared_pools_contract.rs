// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Every client of one venue pool and environment counts against one IP weight pool.
//!
//! USDⓈ-M and COIN-M share one IP weight limit and one `X-MBX-USED-WEIGHT-1M`
//! counter (<https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice>,
//! section A.3, checked 2026-10-10); Spot's limits are its own
//! (<https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#ip-limits>).
//! Exchange information states each pool's limits in `rateLimits`
//! (<https://github.com/binance/binance-spot-api-docs/blob/master/enums.md#rate-limiters-ratelimittype>).
//! Only one test draws on the process's registry; every other test builds its own.

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
    BudgetLimits, Budgets, Clock, Credentials, Error, Outcome, Symbol, WeightPools, coinm, spot,
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
const FIVE_MINUTE_START: u64 = 1_700_000_100_000;

struct ManualClock(AtomicU64);

impl ManualClock {
    fn at(millis: u64) -> Arc<Self> {
        Arc::new(Self(AtomicU64::new(millis)))
    }

    fn set(&self, millis: u64) {
        self.0.store(millis, Ordering::SeqCst);
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

fn spot_client(
    config: spot::Config,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> spot::RestClient {
    spot::RestClient::new(config.rest_url(&fixture.url).unwrap().clock(clock.clone())).unwrap()
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

async fn spot_time(client: &spot::RestClient) -> Result<(), Error> {
    client
        .time(&spot::rest_requests::Time::new(), deadline())
        .await
        .map(drop)
}

/// A reply reporting the futures pool's whole documented minute weight as used.
async fn futures_pool_spent() -> HttpFixture {
    HttpFixture::new(
        200,
        "X-MBX-USED-WEIGHT-1M: 2400\r\n",
        "{\"serverTime\":1}",
        None,
        false,
    )
    .await
}

async fn server_time() -> HttpFixture {
    HttpFixture::new(200, "", "{\"serverTime\":1}", None, false).await
}

#[tokio::test]
async fn clients_of_one_pool_and_environment_share_its_weight() {
    let usdm_venue = HttpFixture::new(200, "", "[]", None, false).await;
    let coinm_venue = server_time().await;
    let clock = ManualClock::at(MINUTE_START);
    // Both draw on the process's futures production pool through `Config::new`.
    let first = usdm_client(
        usdm::Config::new(usdm::Environment::Production).unwrap(),
        &usdm_venue,
        &clock,
    );
    let second = usdm_client(
        usdm::Config::new(usdm::Environment::Production).unwrap(),
        &usdm_venue,
        &clock,
    );
    let inverse = coinm_client(
        coinm::Config::new(coinm::Environment::Production).unwrap(),
        &coinm_venue,
        &clock,
    );
    let prints =
        usdm::rest_requests::OldTradesLookup::new().symbol(Symbol::new("BTCUSDT").unwrap());

    // Twelve print pages at weight 200 spend the documented 2,400 between two clients.
    for page in 0..12 {
        let client = if page % 2 == 0 { &first } else { &second };
        client.old_trades_lookup(&prints, deadline()).await.unwrap();
    }
    assert_refused(
        &second
            .old_trades_lookup(&prints, deadline())
            .await
            .unwrap_err(),
        Duration::from_mins(1),
    );
    assert_refused(
        &coinm_time(&inverse).await.unwrap_err(),
        Duration::from_mins(1),
    );
    assert_eq!(usdm_venue.connections_accepted(), 12);
    assert_eq!(coinm_venue.connections_accepted(), 0);
    usdm_venue.finish().await;
    coinm_venue.finish().await;
}

#[tokio::test]
async fn spot_and_futures_pools_never_share() {
    let clock = ManualClock::at(MINUTE_START);
    let spot_spent = HttpFixture::new(
        200,
        "X-MBX-USED-WEIGHT-1M: 6000\r\n",
        "{\"serverTime\":1}",
        None,
        false,
    )
    .await;
    let futures_spent = futures_pool_spent().await;
    let spot_venue = server_time().await;
    let futures_venue = server_time().await;

    let after_spot = WeightPools::new();
    let spot_spender = spot_client(
        spot::Config::with_pools(spot::Environment::Production, &after_spot).unwrap(),
        &spot_spent,
        &clock,
    );
    spot_time(&spot_spender).await.unwrap();
    assert_refused(
        &spot_time(&spot_spender).await.unwrap_err(),
        Duration::from_mins(1),
    );
    let usdm_after_spot = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Production, &after_spot).unwrap(),
        &futures_venue,
        &clock,
    );
    usdm_time(&usdm_after_spot).await.unwrap();

    let after_futures = WeightPools::new();
    let futures_spender = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Production, &after_futures).unwrap(),
        &futures_spent,
        &clock,
    );
    coinm_time(&futures_spender).await.unwrap();
    assert_refused(
        &coinm_time(&futures_spender).await.unwrap_err(),
        Duration::from_mins(1),
    );
    let spot_after_futures = spot_client(
        spot::Config::with_pools(spot::Environment::Production, &after_futures).unwrap(),
        &spot_venue,
        &clock,
    );
    spot_time(&spot_after_futures).await.unwrap();

    assert_eq!(spot_venue.connections_accepted(), 1);
    assert_eq!(futures_venue.connections_accepted(), 1);
    for fixture in [spot_spent, futures_spent, spot_venue, futures_venue] {
        fixture.finish().await;
    }
}

#[tokio::test]
async fn demo_and_production_pools_never_share() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let spent = futures_pool_spent().await;
    let venue = server_time().await;
    let demo = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Demo, &pools).unwrap(),
        &spent,
        &clock,
    );
    usdm_time(&demo).await.unwrap();

    let production = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
        &venue,
        &clock,
    );
    coinm_time(&production).await.unwrap();
    let other_demo = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Demo, &pools).unwrap(),
        &venue,
        &clock,
    );
    assert_refused(
        &coinm_time(&other_demo).await.unwrap_err(),
        Duration::from_mins(1),
    );

    let spot_spent = HttpFixture::new(
        200,
        "X-MBX-USED-WEIGHT-1M: 6000\r\n",
        "{\"serverTime\":1}",
        None,
        false,
    )
    .await;
    let spot_production = spot_client(
        spot::Config::with_pools(spot::Environment::Production, &pools).unwrap(),
        &spot_spent,
        &clock,
    );
    spot_time(&spot_production).await.unwrap();
    let spot_demo = spot_client(
        spot::Config::with_pools(spot::Environment::Demo, &pools).unwrap(),
        &venue,
        &clock,
    );
    spot_time(&spot_demo).await.unwrap();
    assert_eq!(venue.connections_accepted(), 2);
    for fixture in [spent, spot_spent, venue] {
        fixture.finish().await;
    }
}

#[tokio::test]
async fn a_stated_request_weight_limit_replaces_the_baseline() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let exchange = HttpFixture::new(
        200,
        "",
        r#"{"rateLimits":[
            {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":3},
            {"rateLimitType":"ORDERS","interval":"MINUTE","intervalNum":1,"limit":1200}
        ],"symbols":[]}"#,
        None,
        false,
    )
    .await;
    let venue = server_time().await;
    let reader = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Production, &pools).unwrap(),
        &exchange,
        &clock,
    );
    let other = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
        &venue,
        &clock,
    );

    // Exchange information costs weight 1; the venue states a minute limit of 3.
    reader
        .exchange_information(&usdm::rest_requests::ExchangeInformation::new(), deadline())
        .await
        .unwrap();
    coinm_time(&other).await.unwrap();
    usdm_time(&reader).await.unwrap();
    assert_refused(
        &coinm_time(&other).await.unwrap_err(),
        Duration::from_mins(1),
    );
    assert_eq!(exchange.connections_accepted(), 2);
    assert_eq!(venue.connections_accepted(), 1);
    exchange.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_stated_raw_request_limit_replaces_the_spot_baseline() {
    let pools = WeightPools::new();
    // Both the minute and the five-minute windows begin here.
    let clock = ManualClock::at(FIVE_MINUTE_START);
    let exchange = HttpFixture::new(
        200,
        "",
        r#"{"rateLimits":[
            {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000},
            {"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":2}
        ],"symbols":[]}"#,
        None,
        false,
    )
    .await;
    let venue = server_time().await;
    let reader = spot_client(
        spot::Config::with_pools(spot::Environment::Demo, &pools).unwrap(),
        &exchange,
        &clock,
    );
    let other = spot_client(
        spot::Config::with_pools(spot::Environment::Demo, &pools).unwrap(),
        &venue,
        &clock,
    );
    reader
        .exchange_info(&spot::rest_requests::ExchangeInfo::new(), deadline())
        .await
        .unwrap();
    spot_time(&other).await.unwrap();
    assert_refused(
        &spot_time(&other).await.unwrap_err(),
        Duration::from_mins(5),
    );
    assert_eq!(venue.connections_accepted(), 1);
    exchange.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_used_weight_header_seen_by_one_client_holds_every_client_of_the_pool() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START + 15_000);
    let spent = futures_pool_spent().await;
    let venue = server_time().await;
    let reporter = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Production, &pools).unwrap(),
        &spent,
        &clock,
    );
    let others = [
        coinm_client(
            coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
            &venue,
            &clock,
        ),
        coinm_client(
            coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
            &venue,
            &clock,
        ),
    ];
    usdm_time(&reporter).await.unwrap();
    for other in &others {
        assert_refused(
            &coinm_time(other).await.unwrap_err(),
            Duration::from_secs(45),
        );
    }
    clock.set(MINUTE_START + 60_000);
    coinm_time(&others[0]).await.unwrap();
    assert_eq!(venue.connections_accepted(), 1);
    spent.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_retry_after_holds_every_client_of_the_pool() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let limited = HttpFixture::new(
        429,
        "Retry-After: 2\r\n",
        "{\"code\":-1003,\"msg\":\"fixture rate limit\"}",
        None,
        false,
    )
    .await;
    let venue = server_time().await;
    let limited_client = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Demo, &pools).unwrap(),
        &limited,
        &clock,
    );
    let other = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Demo, &pools).unwrap(),
        &venue,
        &clock,
    );
    assert!(matches!(
        coinm_time(&limited_client).await,
        Err(Error::Venue(_))
    ));
    assert_refused(
        &usdm_time(&other).await.unwrap_err(),
        Duration::from_secs(2),
    );
    clock.set(MINUTE_START + 2000);
    usdm_time(&other).await.unwrap();
    assert_eq!(venue.connections_accepted(), 1);
    limited.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_ban_without_retry_timing_holds_every_client_of_the_pool() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let banned = HttpFixture::new(
        418,
        "",
        "{\"code\":-1003,\"msg\":\"fixture ban\"}",
        None,
        false,
    )
    .await;
    let venue = server_time().await;
    let banned_client = spot_client(
        spot::Config::with_pools(spot::Environment::Production, &pools).unwrap(),
        &banned,
        &clock,
    );
    let other = spot_client(
        spot::Config::with_pools(spot::Environment::Production, &pools).unwrap(),
        &venue,
        &clock,
    );
    assert!(matches!(
        spot_time(&banned_client).await,
        Err(Error::Venue(_))
    ));
    let refusal = spot_time(&other).await.unwrap_err();
    assert!(
        matches!(refusal, Error::CooldownTimingUnknown),
        "{refusal:?}"
    );
    assert_eq!(venue.connections_accepted(), 0);
    banned.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn an_explicit_budgets_owner_keeps_a_client_isolated() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let spent = futures_pool_spent().await;
    let venue = server_time().await;
    let reporter = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Production, &pools).unwrap(),
        &spent,
        &clock,
    );
    usdm_time(&reporter).await.unwrap();

    let isolated = coinm_client(
        coinm::Config::with_pools(coinm::Environment::Production, &pools)
            .unwrap()
            .budgets(Budgets::new(BudgetLimits::coinm().weight_per_minute(1)).unwrap()),
        &venue,
        &clock,
    );
    coinm_time(&isolated).await.unwrap();
    assert_refused(
        &coinm_time(&isolated).await.unwrap_err(),
        Duration::from_mins(1),
    );
    assert_eq!(venue.connections_accepted(), 1);
    spent.finish().await;
    venue.finish().await;
}
