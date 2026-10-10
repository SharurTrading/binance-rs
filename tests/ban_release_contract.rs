// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! A `418` ban without usable timing holds a pool until the caller releases it.
//!
//! Binance documents a `418` ban as lasting "from 2 minutes to 3 days" and offers no
//! other way to learn that one without timing has ended
//! (<https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#http-418>,
//! checked 2026-10-11). The release is the caller's decision; a known `Retry-After`
//! cooldown is the venue's and still holds.

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
    Budgets, Clock, Credentials, Error, Outcome, WeightPools, coinm, convert, margin, options,
    spot, usdm, wallet,
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

fn spot_client(
    pools: &WeightPools,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> spot::RestClient {
    spot::RestClient::new(
        spot_config(pools)
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone()),
    )
    .unwrap()
}

fn spot_config(pools: &WeightPools) -> spot::Config {
    spot::Config::with_pools(spot::Environment::Production, pools).unwrap()
}

async fn spot_time(client: &spot::RestClient) -> Result<(), Error> {
    client
        .time(&spot::rest_requests::Time::new(), deadline())
        .await
        .map(drop)
}

fn usdm_client(
    pools: &WeightPools,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> usdm::RestClient {
    usdm::RestClient::new(
        usdm::Config::with_pools(usdm::Environment::Production, pools)
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone()),
    )
    .unwrap()
}

async fn usdm_time(client: &usdm::RestClient) -> Result<(), Error> {
    client
        .check_server_time(&usdm::rest_requests::CheckServerTime::new(), deadline())
        .await
        .map(drop)
}

fn coinm_client(
    pools: &WeightPools,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> coinm::RestClient {
    coinm::RestClient::new(
        coinm::Config::with_pools(coinm::Environment::Production, pools)
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone()),
    )
    .unwrap()
}

async fn coinm_time(client: &coinm::RestClient) -> Result<(), Error> {
    client
        .check_server_time(&coinm::rest_requests::CheckServerTime::new(), deadline())
        .await
        .map(drop)
}

async fn server_time() -> HttpFixture {
    HttpFixture::new(200, "", "{\"serverTime\":1}", None, false).await
}

async fn ban(headers: &str) -> HttpFixture {
    HttpFixture::new(
        418,
        headers,
        "{\"code\":-1003,\"msg\":\"fixture ban\"}",
        None,
        false,
    )
    .await
}

fn assert_timing_unknown(error: &Error) {
    assert_eq!(error.outcome(), Some(Outcome::NotSent), "{error:?}");
    assert!(matches!(error, Error::CooldownTimingUnknown), "{error:?}");
}

#[tokio::test]
async fn a_ban_without_timing_refuses_every_client_of_the_pool() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let banned = ban("").await;
    let venue = server_time().await;
    let banned_client = spot_client(&pools, &banned, &clock);
    let first = spot_client(&pools, &venue, &clock);
    let second = spot_client(&pools, &venue, &clock);

    assert!(matches!(
        spot_time(&banned_client).await,
        Err(Error::Venue(_))
    ));
    assert_timing_unknown(&spot_time(&first).await.unwrap_err());
    assert_timing_unknown(&spot_time(&second).await.unwrap_err());
    assert_timing_unknown(&spot_time(&banned_client).await.unwrap_err());
    // However long the caller waits, the ban has no expiry the client could read.
    clock.set(MINUTE_START + 3 * 86_400_000);
    assert_timing_unknown(&spot_time(&first).await.unwrap_err());
    assert_eq!(venue.connections_accepted(), 0);
    banned.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn releasing_a_ban_without_timing_admits_sends_again() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let banned = ban("").await;
    let venue = server_time().await;
    let banned_client = spot_client(&pools, &banned, &clock);
    let other = spot_client(&pools, &venue, &clock);
    assert!(spot_time(&banned_client).await.is_err());
    assert_timing_unknown(&spot_time(&other).await.unwrap_err());

    other.release_unknown_ban().unwrap();

    spot_time(&other).await.unwrap();
    assert_eq!(venue.connections_accepted(), 1);
    banned.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_release_does_not_lift_a_known_retry_after_cooldown() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let banned = ban("Retry-After: 30\r\n").await;
    let venue = server_time().await;
    let banned_client = spot_client(&pools, &banned, &clock);
    let other = spot_client(&pools, &venue, &clock);
    assert!(spot_time(&banned_client).await.is_err());

    other.release_unknown_ban().unwrap();
    let held = spot_time(&other).await.unwrap_err();
    assert!(
        matches!(held, Error::Admission { retry_after } if retry_after == Duration::from_secs(30)),
        "{held:?}"
    );
    assert_eq!(venue.connections_accepted(), 0);

    clock.set(MINUTE_START + 30_000);
    spot_time(&other).await.unwrap();
    banned.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn one_release_frees_every_client_of_the_pool() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let banned = ban("").await;
    let venue = server_time().await;
    let banned_client = spot_client(&pools, &banned, &clock);
    let first = spot_client(&pools, &venue, &clock);
    let second = spot_client(&pools, &venue, &clock);
    assert!(spot_time(&banned_client).await.is_err());
    assert_timing_unknown(&spot_time(&first).await.unwrap_err());

    // A configuration reaches the same pool as its clients, without a client.
    spot_config(&pools).release_unknown_ban().unwrap();

    spot_time(&first).await.unwrap();
    spot_time(&second).await.unwrap();
    banned.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_futures_release_frees_usdm_and_coinm_clients_but_not_other_pools() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let banned = ban("").await;
    let venue = server_time().await;
    let banned_client = usdm_client(&pools, &banned, &clock);
    let inverse = coinm_client(&pools, &venue, &clock);
    let spot = spot_client(&pools, &venue, &clock);
    assert!(usdm_time(&banned_client).await.is_err());
    assert_timing_unknown(&coinm_time(&inverse).await.unwrap_err());
    // The ban is the futures pool's: Spot never saw it.
    spot_time(&spot).await.unwrap();

    inverse.release_unknown_ban().unwrap();

    coinm_time(&inverse).await.unwrap();
    usdm_time(&banned_client).await.unwrap_err();
    banned.finish().await;
    venue.finish().await;
}

#[test]
fn a_release_without_a_ban_is_a_no_op_on_every_configuration_and_client() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let reading = || {
        spot_config(&pools)
            .clock(clock.clone())
            .pool_usage()
            .unwrap()
    };
    let usage = reading();

    spot_config(&pools).release_unknown_ban().unwrap();
    usdm::Config::with_pools(usdm::Environment::Production, &pools)
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    coinm::Config::with_pools(coinm::Environment::Production, &pools)
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    options::Config::with_pools(options::Environment::Production, &pools)
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    wallet::Config::with_pools(&pools)
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    convert::Config::with_pools(&pools)
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    margin::Config::with_pools(&pools)
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    Budgets::sapi().unwrap().release_unknown_ban().unwrap();

    spot::RestClient::new(spot_config(&pools))
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    usdm::RestClient::new(usdm::Config::with_pools(usdm::Environment::Production, &pools).unwrap())
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    coinm::RestClient::new(
        coinm::Config::with_pools(coinm::Environment::Production, &pools).unwrap(),
    )
    .unwrap()
    .release_unknown_ban()
    .unwrap();
    options::RestClient::new(
        options::Config::with_pools(options::Environment::Production, &pools).unwrap(),
    )
    .unwrap()
    .release_unknown_ban()
    .unwrap();
    wallet::RestClient::new(wallet::Config::with_pools(&pools).unwrap())
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    convert::RestClient::new(convert::Config::with_pools(&pools).unwrap())
        .unwrap()
        .release_unknown_ban()
        .unwrap();
    margin::RestClient::new(margin::Config::with_pools(&pools).unwrap())
        .unwrap()
        .release_unknown_ban()
        .unwrap();

    assert_eq!(reading(), usage);
}

#[tokio::test]
async fn a_sapi_release_frees_a_banned_wallet_pool() {
    let banned = ban("").await;
    let read = wallet::rest_requests::WithdrawHistory::new();
    let banned_client = wallet::RestClient::new(
        wallet::Config::with_pools(&WeightPools::new())
            .unwrap()
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap())
            .rest_url(&banned.url)
            .unwrap(),
    )
    .unwrap();
    assert!(
        banned_client
            .withdraw_history(&read, deadline())
            .await
            .is_err()
    );
    assert_timing_unknown(
        &banned_client
            .withdraw_history(&read, deadline())
            .await
            .unwrap_err(),
    );

    banned_client.release_unknown_ban().unwrap();

    // Released, the next attempt reaches the venue, which here answers 418 again.
    let again = banned_client
        .withdraw_history(&read, deadline())
        .await
        .unwrap_err();
    assert!(matches!(again, Error::Venue(_)), "{again:?}");
    banned.finish().await;
}
