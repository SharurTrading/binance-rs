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
//! One test per independent venue pool draws on the process registry; others use explicit registries.

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
    BudgetLimits, Budgets, Clock, Credentials, Error, Outcome, Symbol, WeightPools, coinm, options,
    spot, usdm,
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

/// What each market's pool reports of itself, and the weight a request reports
/// before it is sent. Futures klines and depth weights follow the `limit` tables of
/// the USDⓈ-M and COIN-M market data pages; Spot depth follows the Spot REST
/// `/api/v3/depth` table (<https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#order-book>).
mod pool_report {
    use super::*;
    use binance_client::{LimitSource, WindowUsage};

    /// An exchange information reply stating only an account limit.
    const ORDERS_ONLY: &str = r#"{"rateLimits":[
        {"rateLimitType":"ORDERS","interval":"MINUTE","intervalNum":1,"limit":1200}
    ],"symbols":[]}"#;

    fn assert_window(
        window: &WindowUsage,
        interval: Duration,
        limit: u64,
        source: LimitSource,
        used: u64,
        resets_in: Duration,
    ) {
        assert_eq!(window.interval, interval, "{window:?}");
        assert_eq!(window.limit, limit, "{window:?}");
        assert_eq!(window.source, source, "{window:?}");
        assert_eq!(window.used, used, "{window:?}");
        assert_eq!(window.resets_in, resets_in, "{window:?}");
    }

    async fn used_weight(count: u64) -> HttpFixture {
        HttpFixture::new(
            200,
            &format!("X-MBX-USED-WEIGHT-1M: {count}\r\n"),
            "{\"serverTime\":1}",
            None,
            false,
        )
        .await
    }

    mod spot {
        use super::*;
        use binance_client::spot::{self, Config, Environment, rest_requests};

        const START: u64 = FIVE_MINUTE_START + 15_000;

        fn config(pools: &WeightPools, clock: &Arc<ManualClock>) -> Config {
            Config::with_pools(Environment::Production, pools)
                .unwrap()
                .clock(clock.clone())
        }

        #[tokio::test]
        async fn the_pool_reports_the_fallback_limit_until_exchange_information_states_one() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let config = config(&pools, &clock);
            let before = config.pool_usage().unwrap();
            assert_window(
                &before.request_weight,
                Duration::from_mins(1),
                6000,
                LimitSource::Documented,
                0,
                Duration::from_secs(45),
            );
            assert_window(
                &before.raw_requests.unwrap(),
                Duration::from_mins(5),
                300_000,
                LimitSource::Documented,
                0,
                Duration::from_secs(285),
            );

            let exchange = HttpFixture::new(200, "", ORDERS_ONLY, None, false).await;
            spot_client(config.clone(), &exchange, &clock)
                .exchange_info(&rest_requests::ExchangeInfo::new(), deadline())
                .await
                .unwrap();
            let after = config.pool_usage().unwrap();
            assert_window(
                &after.request_weight,
                Duration::from_mins(1),
                6000,
                LimitSource::Documented,
                20,
                Duration::from_secs(45),
            );
            assert_window(
                &after.raw_requests.unwrap(),
                Duration::from_mins(5),
                300_000,
                LimitSource::Documented,
                1,
                Duration::from_secs(285),
            );
            exchange.finish().await;
        }

        #[tokio::test]
        async fn a_stated_limit_replaces_the_fallback_in_the_report() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let exchange = HttpFixture::new(
                200,
                "",
                r#"{"rateLimits":[
                    {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1200},
                    {"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":61000}
                ],"symbols":[]}"#,
                None,
                false,
            )
            .await;
            spot_client(config(&pools, &clock), &exchange, &clock)
                .exchange_info(&rest_requests::ExchangeInfo::new(), deadline())
                .await
                .unwrap();
            // Another configuration drawn from the same pool reads the stated figures.
            let report = config(&pools, &clock).pool_usage().unwrap();
            assert_window(
                &report.request_weight,
                Duration::from_mins(1),
                1200,
                LimitSource::Stated,
                20,
                Duration::from_secs(45),
            );
            assert_window(
                &report.raw_requests.unwrap(),
                Duration::from_mins(5),
                61_000,
                LimitSource::Stated,
                1,
                Duration::from_secs(285),
            );
            exchange.finish().await;
        }

        #[tokio::test]
        async fn usage_follows_the_used_weight_header() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let venue = used_weight(1500).await;
            let client = spot_client(config(&pools, &clock), &venue, &clock);
            spot_time(&client).await.unwrap();
            assert_window(
                &client.pool_usage().unwrap().request_weight,
                Duration::from_mins(1),
                6000,
                LimitSource::Documented,
                1500,
                Duration::from_secs(45),
            );
            clock.set(START + 45_000);
            assert_window(
                &client.pool_usage().unwrap().request_weight,
                Duration::from_mins(1),
                6000,
                LimitSource::Documented,
                0,
                Duration::from_mins(1),
            );
            venue.finish().await;
        }

        #[tokio::test]
        async fn a_requests_reported_weight_is_the_weight_it_is_charged() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let venue = HttpFixture::new(
                200,
                "",
                r#"{"lastUpdateId":1,"bids":[],"asks":[]}"#,
                None,
                false,
            )
            .await;
            let client = spot_client(config(&pools, &clock), &venue, &clock);
            let shallow = rest_requests::Depth::new()
                .symbol(Symbol::new("BTCUSDT").unwrap())
                .limit(100);
            let deep = shallow.clone().limit(5000);
            assert_eq!(shallow.weight().unwrap(), 5);
            assert_eq!(deep.weight().unwrap(), 250);

            client.depth(&shallow, deadline()).await.unwrap();
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 5);
            client.depth(&deep, deadline()).await.unwrap();
            let report = client.pool_usage().unwrap();
            assert_eq!(report.request_weight.used, 255);
            assert_eq!(report.raw_requests.unwrap().used, 2);
            venue.finish().await;
        }

        fn spot_client(
            config: Config,
            fixture: &HttpFixture,
            clock: &Arc<ManualClock>,
        ) -> spot::RestClient {
            super::super::spot_client(config, fixture, clock)
        }
    }

    mod usdm {
        use super::*;
        use binance_client::usdm::{self, Config, Environment, rest_requests};

        const START: u64 = MINUTE_START + 15_000;

        fn config(pools: &WeightPools, clock: &Arc<ManualClock>) -> Config {
            Config::with_pools(Environment::Production, pools)
                .unwrap()
                .clock(clock.clone())
        }

        fn client(
            config: Config,
            fixture: &HttpFixture,
            clock: &Arc<ManualClock>,
        ) -> usdm::RestClient {
            super::super::usdm_client(config, fixture, clock)
        }

        #[tokio::test]
        async fn the_pool_reports_the_fallback_limit_until_exchange_information_states_one() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let config = config(&pools, &clock);
            let before = config.pool_usage().unwrap();
            assert_window(
                &before.request_weight,
                Duration::from_mins(1),
                2400,
                LimitSource::Documented,
                0,
                Duration::from_secs(45),
            );
            assert_eq!(before.raw_requests, None);

            let exchange = HttpFixture::new(200, "", ORDERS_ONLY, None, false).await;
            client(config.clone(), &exchange, &clock)
                .exchange_information(&rest_requests::ExchangeInformation::new(), deadline())
                .await
                .unwrap();
            assert_window(
                &config.pool_usage().unwrap().request_weight,
                Duration::from_mins(1),
                2400,
                LimitSource::Documented,
                1,
                Duration::from_secs(45),
            );
            exchange.finish().await;
        }

        #[tokio::test]
        async fn a_stated_limit_replaces_the_fallback_in_the_report() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let exchange = HttpFixture::new(
                200,
                "",
                r#"{"rateLimits":[
                    {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1200}
                ],"symbols":[]}"#,
                None,
                false,
            )
            .await;
            client(config(&pools, &clock), &exchange, &clock)
                .exchange_information(&rest_requests::ExchangeInformation::new(), deadline())
                .await
                .unwrap();
            let report = config(&pools, &clock).pool_usage().unwrap();
            assert_window(
                &report.request_weight,
                Duration::from_mins(1),
                1200,
                LimitSource::Stated,
                1,
                Duration::from_secs(45),
            );
            assert_eq!(report.raw_requests, None);
            exchange.finish().await;
        }

        #[tokio::test]
        async fn usage_follows_the_used_weight_header() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let venue = used_weight(1500).await;
            let client = client(config(&pools, &clock), &venue, &clock);
            usdm_time(&client).await.unwrap();
            assert_window(
                &client.pool_usage().unwrap().request_weight,
                Duration::from_mins(1),
                2400,
                LimitSource::Documented,
                1500,
                Duration::from_secs(45),
            );
            clock.set(START + 45_000);
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 0);
            venue.finish().await;
        }

        #[tokio::test]
        async fn a_requests_reported_weight_is_the_weight_it_is_charged() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let venue = HttpFixture::new(200, "", "[]", None, false).await;
            let client = client(config(&pools, &clock), &venue, &clock);
            let bars = rest_requests::KlineCandlestickData::new()
                .symbol(Symbol::new("BTCUSDT").unwrap())
                .interval("1m")
                .limit(1000);
            let more = bars.clone().limit(1001);
            assert_eq!(bars.weight().unwrap(), 5);
            assert_eq!(more.weight().unwrap(), 10);

            client
                .kline_candlestick_data(&bars, deadline())
                .await
                .unwrap();
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 5);
            client
                .kline_candlestick_data(&more, deadline())
                .await
                .unwrap();
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 15);

            // A depth the venue documents no weight for is refused, unsent and uncharged.
            let odd = rest_requests::OrderBook::new()
                .symbol(Symbol::new("BTCUSDT").unwrap())
                .limit(7);
            assert!(matches!(odd.weight(), Err(Error::Validation(_))));
            assert!(matches!(
                client.order_book(&odd, deadline()).await,
                Err(Error::Validation(_))
            ));
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 15);
            assert_eq!(venue.connections_accepted(), 2);
            venue.finish().await;
        }
    }

    mod coinm {
        use super::*;
        use binance_client::coinm::{self, Config, Environment, rest_requests};

        const START: u64 = MINUTE_START + 15_000;

        fn config(pools: &WeightPools, clock: &Arc<ManualClock>) -> Config {
            Config::with_pools(Environment::Production, pools)
                .unwrap()
                .clock(clock.clone())
        }

        fn client(
            config: Config,
            fixture: &HttpFixture,
            clock: &Arc<ManualClock>,
        ) -> coinm::RestClient {
            super::super::coinm_client(config, fixture, clock)
        }

        #[tokio::test]
        async fn the_pool_reports_the_fallback_limit_until_exchange_information_states_one() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let config = config(&pools, &clock);
            let before = config.pool_usage().unwrap();
            assert_window(
                &before.request_weight,
                Duration::from_mins(1),
                2400,
                LimitSource::Documented,
                0,
                Duration::from_secs(45),
            );
            assert_eq!(before.raw_requests, None);

            let exchange = HttpFixture::new(200, "", ORDERS_ONLY, None, false).await;
            client(config.clone(), &exchange, &clock)
                .exchange_information(&rest_requests::ExchangeInformation::new(), deadline())
                .await
                .unwrap();
            assert_window(
                &config.pool_usage().unwrap().request_weight,
                Duration::from_mins(1),
                2400,
                LimitSource::Documented,
                1,
                Duration::from_secs(45),
            );
            exchange.finish().await;
        }

        #[tokio::test]
        async fn a_stated_limit_replaces_the_fallback_in_the_report() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let exchange = HttpFixture::new(
                200,
                "",
                r#"{"rateLimits":[
                    {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1200}
                ],"symbols":[]}"#,
                None,
                false,
            )
            .await;
            client(config(&pools, &clock), &exchange, &clock)
                .exchange_information(&rest_requests::ExchangeInformation::new(), deadline())
                .await
                .unwrap();
            let report = config(&pools, &clock).pool_usage().unwrap();
            assert_window(
                &report.request_weight,
                Duration::from_mins(1),
                1200,
                LimitSource::Stated,
                1,
                Duration::from_secs(45),
            );
            assert_eq!(report.raw_requests, None);
            exchange.finish().await;
        }

        #[tokio::test]
        async fn usage_follows_the_used_weight_header() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let venue = used_weight(1500).await;
            let client = client(config(&pools, &clock), &venue, &clock);
            coinm_time(&client).await.unwrap();
            assert_window(
                &client.pool_usage().unwrap().request_weight,
                Duration::from_mins(1),
                2400,
                LimitSource::Documented,
                1500,
                Duration::from_secs(45),
            );
            clock.set(START + 45_000);
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 0);
            venue.finish().await;
        }

        #[tokio::test]
        async fn a_requests_reported_weight_is_the_weight_it_is_charged() {
            let pools = WeightPools::new();
            let clock = ManualClock::at(START);
            let venue = HttpFixture::new(200, "", "[]", None, false).await;
            let client = client(config(&pools, &clock), &venue, &clock);
            let bars = rest_requests::KlineCandlestickData::new()
                .symbol(Symbol::new("BTCUSD_PERP").unwrap())
                .interval("1m")
                .limit(99);
            let more = bars.clone().limit(500);
            assert_eq!(bars.weight().unwrap(), 1);
            assert_eq!(more.weight().unwrap(), 5);

            client
                .kline_candlestick_data(&bars, deadline())
                .await
                .unwrap();
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 1);
            client
                .kline_candlestick_data(&more, deadline())
                .await
                .unwrap();
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 6);

            let odd = rest_requests::OrderBook::new()
                .symbol(Symbol::new("BTCUSD_PERP").unwrap())
                .limit(7);
            assert!(matches!(odd.weight(), Err(Error::Validation(_))));
            assert!(matches!(
                client.order_book(&odd, deadline()).await,
                Err(Error::Validation(_))
            ));
            assert_eq!(client.pool_usage().unwrap().request_weight.used, 6);
            assert_eq!(venue.connections_accepted(), 2);
            venue.finish().await;
        }
    }
}

#[tokio::test]
async fn sapi_products_share_ip_cooldown_before_any_later_send() {
    use binance_client::{convert, margin, wallet};
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let banned = HttpFixture::new(
        418,
        "Retry-After: 60\r\n",
        "{\"code\":-1003,\"msg\":\"synthetic ban\"}",
        None,
        false,
    )
    .await;
    let unused = server_time().await;
    let wallet = wallet::RestClient::new(
        wallet::Config::with_pools(&pools)
            .unwrap()
            .clock(clock.clone())
            .rest_url(&banned.url)
            .unwrap(),
    )
    .unwrap();
    assert!(
        wallet
            .system_status(&wallet::rest_requests::SystemStatus::new(), deadline())
            .await
            .is_err()
    );
    let convert = convert::RestClient::new(
        convert::Config::with_pools(&pools)
            .unwrap()
            .clock(clock.clone())
            .rest_url(&unused.url)
            .unwrap(),
    )
    .unwrap();
    let error = convert
        .list_all_convert_pairs(
            &convert::rest_requests::ListAllConvertPairs::new(),
            deadline(),
        )
        .await
        .unwrap_err();
    assert_refused(&error, Duration::from_mins(1));
    let margin = margin::RestClient::new(
        margin::Config::with_pools(&pools)
            .unwrap()
            .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
            .clock(clock)
            .rest_url(&unused.url)
            .unwrap(),
    )
    .unwrap();
    let error = margin
        .get_all_margin_assets(
            &margin::rest_requests::GetAllMarginAssets::new(),
            deadline(),
        )
        .await
        .unwrap_err();
    assert_refused(&error, Duration::from_mins(1));
    assert_eq!(unused.connections_accepted(), 0);
    banned.finish().await;
    unused.finish().await;
}

fn options_client(
    config: options::Config,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> options::RestClient {
    options::RestClient::new(
        config
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap()
}

async fn options_time(client: &options::RestClient) -> Result<(), Error> {
    client
        .check_server_time(&options::rest_requests::CheckServerTime::new(), deadline())
        .await
        .map(drop)
}

#[tokio::test]
async fn independent_options_clients_share_the_process_environment_pool() {
    let clock = ManualClock::at(MINUTE_START);
    let spent = futures_pool_spent().await;
    let unsent = server_time().await;
    let first_config = options::Config::new(options::Environment::Production)
        .unwrap()
        .clock(clock.clone());
    let second_config = options::Config::new(options::Environment::Production)
        .unwrap()
        .clock(clock.clone());
    let first = options_client(first_config.clone(), &spent, &clock);
    let second = options_client(second_config.clone(), &unsent, &clock);
    options_time(&first).await.unwrap();
    assert_refused(
        &options_time(&second).await.unwrap_err(),
        Duration::from_mins(1),
    );
    assert_eq!(
        first_config.pool_usage().unwrap(),
        second_config.pool_usage().unwrap()
    );
    assert_eq!(second.pool_usage().unwrap().request_weight.used, 2400);
    assert_eq!(spent.connections_accepted(), 1);
    assert_eq!(unsent.connections_accepted(), 0);
    spent.finish().await;
    unsent.finish().await;
}

#[tokio::test]
async fn options_pool_isolated_by_registry_environment_and_product() {
    let clock = ManualClock::at(MINUTE_START);
    let pools = WeightPools::new();
    let independent_pools = WeightPools::new();
    let spent = futures_pool_spent().await;
    let unsent = server_time().await;
    let available = server_time().await;
    let spender = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools).unwrap(),
        &spent,
        &clock,
    );
    options_time(&spender).await.unwrap();
    let same = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools).unwrap(),
        &unsent,
        &clock,
    );
    assert_refused(
        &options_time(&same).await.unwrap_err(),
        Duration::from_mins(1),
    );
    let separate_environment = options_client(
        options::Config::with_pools(options::Environment::Production, &pools).unwrap(),
        &available,
        &clock,
    );
    let separate_registry = options_client(
        options::Config::with_pools(options::Environment::Demo, &independent_pools).unwrap(),
        &available,
        &clock,
    );
    options_time(&separate_environment).await.unwrap();
    options_time(&separate_registry).await.unwrap();
    let futures = usdm_client(
        usdm::Config::with_pools(usdm::Environment::Demo, &pools).unwrap(),
        &available,
        &clock,
    );
    let spot = spot_client(
        spot::Config::with_pools(spot::Environment::Demo, &pools).unwrap(),
        &available,
        &clock,
    );
    usdm_time(&futures).await.unwrap();
    spot_time(&spot).await.unwrap();
    assert_eq!(unsent.connections_accepted(), 0);
    assert_eq!(available.connections_accepted(), 4);
    spent.finish().await;
    unsent.finish().await;
    available.finish().await;
}

fn options_order() -> options::rest_requests::NewOrder {
    options::rest_requests::NewOrder::new()
        .symbol(options::Symbol::new("BTC-261030-90000-C").unwrap())
        .side("BUY")
        .type_value("LIMIT")
        .quantity(binance_client::Decimal::ONE)
        .price(binance_client::Decimal::ONE)
        .client_order_id(options::ClientOrderId::new("pool-contract").unwrap())
        .build()
        .unwrap()
}

#[tokio::test]
async fn options_account_owners_are_separate_until_explicitly_shared() {
    let clock = ManualClock::at(MINUTE_START);
    let pools = WeightPools::new();
    let body = r#"{"orderId":7,"symbol":"BTC-261030-90000-C","clientOrderId":"pool-contract"}"#;
    let exhausted =
        HttpFixture::new(200, "X-MBX-ORDER-COUNT-1M: 1200\r\n", body, None, false).await;
    let accepted = HttpFixture::new(200, "", body, None, false).await;
    let config = options::Config::with_pools(options::Environment::Demo, &pools).unwrap();
    let first = options_client(config.clone(), &exhausted, &clock);
    let clone = options_client(config, &accepted, &clock);
    let separate = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools).unwrap(),
        &accepted,
        &clock,
    );
    first.new_order(&options_order(), deadline()).await.unwrap();
    assert_refused(
        &clone
            .new_order(&options_order(), deadline())
            .await
            .unwrap_err(),
        Duration::from_mins(1),
    );
    separate
        .new_order(&options_order(), deadline())
        .await
        .unwrap();
    let owner = Budgets::new(BudgetLimits::options()).unwrap();
    let explicit = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools)
            .unwrap()
            .budgets(owner.clone()),
        &exhausted,
        &clock,
    );
    let shared = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools)
            .unwrap()
            .budgets(owner),
        &accepted,
        &clock,
    );
    explicit
        .new_order(&options_order(), deadline())
        .await
        .unwrap();
    assert_refused(
        &shared
            .new_order(&options_order(), deadline())
            .await
            .unwrap_err(),
        Duration::from_mins(1),
    );
    assert_eq!(exhausted.connections_accepted(), 2);
    assert_eq!(accepted.connections_accepted(), 1);
    exhausted.finish().await;
    accepted.finish().await;
}

#[tokio::test]
async fn options_has_no_borrowed_futures_ten_second_order_ceiling() {
    let limits = BudgetLimits::options();
    assert_eq!(limits.orders_per_ten_seconds, u64::MAX);
    assert_eq!(limits.orders_per_minute, 1200);
    let clock = ManualClock::at(MINUTE_START);
    let pools = WeightPools::new();
    let body = r#"{"orderId":7,"symbol":"BTC-261030-90000-C","clientOrderId":"pool-contract"}"#;
    let venue = HttpFixture::new(
        200,
        "X-MBX-ORDER-COUNT-10S: 300\r\nX-MBX-ORDER-COUNT-1M: 1\r\n",
        body,
        None,
        false,
    )
    .await;
    let client = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools).unwrap(),
        &venue,
        &clock,
    );
    client
        .new_order(&options_order(), deadline())
        .await
        .unwrap();
    client
        .new_order(&options_order(), deadline())
        .await
        .unwrap();
    assert_eq!(venue.connections_accepted(), 2);
    venue.finish().await;
}

#[tokio::test]
async fn options_exchange_information_updates_the_shared_ip_pool() {
    let clock = ManualClock::at(MINUTE_START);
    let pools = WeightPools::new();
    let exchange = HttpFixture::new(200, "", r#"{"serverTime":1,"optionContracts":[],"optionAssets":[],"optionSymbols":[],"rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1}]}"#, None, false).await;
    let unsent = server_time().await;
    let first = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools).unwrap(),
        &exchange,
        &clock,
    );
    let second = options_client(
        options::Config::with_pools(options::Environment::Demo, &pools).unwrap(),
        &unsent,
        &clock,
    );
    first
        .exchange_information(
            &options::rest_requests::ExchangeInformation::new(),
            deadline(),
        )
        .await
        .unwrap();
    let usage = second.pool_usage().unwrap().request_weight;
    assert_eq!(usage.limit, 1);
    assert_eq!(usage.source, binance_client::LimitSource::Stated);
    assert_refused(
        &options_time(&second).await.unwrap_err(),
        Duration::from_mins(1),
    );
    assert_eq!(unsent.connections_accepted(), 0);
    exchange.finish().await;
    unsent.finish().await;
}
