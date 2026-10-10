// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Every configuration names the venue IP limit its clients draw on.
//!
//! USDⓈ-M and COIN-M share one IP weight limit and one `X-MBX-USED-WEIGHT-1M`
//! counter (<https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice>,
//! section A.3, checked 2026-10-10); Spot's limits are its own
//! (<https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#ip-limits>);
//! Options states its own (<https://developers.binance.com/en/docs/products/derivatives-trading-options/common-definition>);
//! Wallet, Convert and Margin REST count SAPI endpoint scopes
//! (<https://developers.binance.com/en/docs/products/wallet/general-info>); Margin's
//! WebSocket API counts against Spot's
//! (<https://developers.binance.com/legacy-docs/binance-spot-api-docs/websocket-api/rate-limits>).
//! Every test builds its own registry, except the SBE market configuration, which
//! only draws on the process registry and spends nothing from it.

#![allow(clippy::unwrap_used, reason = "synthetic fixture assertions")]

use binance_client::{
    BudgetLimits, Budgets, Clock, Credentials, Error, PoolEnvironment, PoolKey, Signer, VenuePool,
    WeightPools, coinm, convert, margin, options, spot, usdm, wallet,
};
use std::{collections::BTreeMap, sync::Arc};

struct FixedClock;

impl Clock for FixedClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(1_700_000_040_000)
    }
}

fn key(pool: VenuePool, environment: PoolEnvironment) -> PoolKey {
    PoolKey::new(pool, environment)
}

#[test]
fn each_product_configuration_reports_the_pool_it_draws_on() {
    let pools = WeightPools::new();
    let production = PoolEnvironment::Production;
    let demo = PoolEnvironment::Demo;
    let cases = [
        (
            spot::Config::with_pools(spot::Environment::Production, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Spot, production)),
        ),
        (
            spot::Config::with_pools(spot::Environment::Demo, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Spot, demo)),
        ),
        (
            usdm::Config::with_pools(usdm::Environment::Production, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Futures, production)),
        ),
        (
            usdm::Config::with_pools(usdm::Environment::Demo, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Futures, demo)),
        ),
        (
            coinm::Config::with_pools(coinm::Environment::Production, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Futures, production)),
        ),
        (
            coinm::Config::with_pools(coinm::Environment::Demo, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Futures, demo)),
        ),
        (
            options::Config::with_pools(options::Environment::Production, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Options, production)),
        ),
        (
            options::Config::with_pools(options::Environment::Demo, &pools)
                .unwrap()
                .pool_key(),
            Some(key(VenuePool::Options, demo)),
        ),
        (
            wallet::Config::with_pools(&pools).unwrap().pool_key(),
            Some(key(VenuePool::Sapi, production)),
        ),
        (
            convert::Config::with_pools(&pools).unwrap().pool_key(),
            Some(key(VenuePool::Sapi, production)),
        ),
        (
            margin::Config::with_pools(&pools).unwrap().pool_key(),
            Some(key(VenuePool::Sapi, production)),
        ),
        (
            margin::WsConfig::with_pools(&pools).unwrap().pool_key(),
            Some(key(VenuePool::Spot, production)),
        ),
    ];
    for (index, (reported, expected)) in cases.into_iter().enumerate() {
        assert_eq!(reported, expected, "case {index}");
    }
}

#[test]
fn sbe_market_configuration_reports_the_spot_production_pool() {
    struct NoSigning;
    impl Signer for NoSigning {
        fn sign(&self, _: &[u8]) -> Result<String, Error> {
            Err(Error::Signing)
        }
    }
    let credentials =
        Credentials::external_ed25519("synthetic-sbe-key", Arc::new(NoSigning)).unwrap();
    let config = spot::sbe::MarketConfig::production(credentials).unwrap();
    assert_eq!(
        config.pool_key(),
        Some(key(VenuePool::Spot, PoolEnvironment::Production))
    );
}

#[test]
fn usdm_and_coinm_configurations_group_under_one_futures_key() {
    let pools = WeightPools::new();
    let mut clients = BTreeMap::<Option<PoolKey>, Vec<&str>>::new();
    for (name, reported) in [
        (
            "usdm",
            usdm::Config::with_pools(usdm::Environment::Production, &pools)
                .unwrap()
                .pool_key(),
        ),
        (
            "coinm",
            coinm::Config::with_pools(coinm::Environment::Production, &pools)
                .unwrap()
                .pool_key(),
        ),
        (
            "spot",
            spot::Config::with_pools(spot::Environment::Production, &pools)
                .unwrap()
                .pool_key(),
        ),
    ] {
        clients.entry(reported).or_default().push(name);
    }
    assert_eq!(
        clients,
        BTreeMap::from([
            (
                Some(key(VenuePool::Spot, PoolEnvironment::Production)),
                vec!["spot"]
            ),
            (
                Some(key(VenuePool::Futures, PoolEnvironment::Production)),
                vec!["usdm", "coinm"]
            ),
        ])
    );
}

#[test]
fn demo_and_production_keys_differ() {
    let pools = WeightPools::new();
    let demo = usdm::Config::with_pools(usdm::Environment::Demo, &pools)
        .unwrap()
        .pool_key();
    let production = usdm::Config::with_pools(usdm::Environment::Production, &pools)
        .unwrap()
        .pool_key();
    assert_ne!(demo, production);
}

#[test]
fn spot_and_futures_keys_differ() {
    let pools = WeightPools::new();
    let spot = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .pool_key();
    let futures = usdm::Config::with_pools(usdm::Environment::Production, &pools)
        .unwrap()
        .pool_key();
    assert_ne!(spot, futures);
}

#[test]
fn an_explicit_budget_owner_reports_no_pool() {
    let pools = WeightPools::new();
    let owner = || Budgets::new(BudgetLimits::usdm()).unwrap();
    let reported = [
        spot::Config::with_pools(spot::Environment::Production, &pools)
            .unwrap()
            .budgets(owner())
            .pool_key(),
        usdm::Config::with_pools(usdm::Environment::Production, &pools)
            .unwrap()
            .budgets(owner())
            .pool_key(),
        coinm::Config::with_pools(coinm::Environment::Production, &pools)
            .unwrap()
            .budgets(owner())
            .pool_key(),
        options::Config::with_pools(options::Environment::Production, &pools)
            .unwrap()
            .budgets(owner())
            .pool_key(),
        wallet::Config::with_pools(&pools)
            .unwrap()
            .budgets(Budgets::sapi().unwrap())
            .pool_key(),
        margin::Config::with_pools(&pools)
            .unwrap()
            .budgets(Budgets::sapi().unwrap().for_account())
            .pool_key(),
        margin::WsConfig::with_pools(&pools)
            .unwrap()
            .budgets(owner())
            .pool_key(),
    ];
    assert_eq!(reported, [None; 7]);
}

#[test]
fn pool_usage_names_the_same_pool_as_its_configuration_and_client() {
    let pools = WeightPools::new();
    let clock: Arc<dyn Clock> = Arc::new(FixedClock);
    let spot = spot::Config::with_pools(spot::Environment::Demo, &pools)
        .unwrap()
        .clock(clock.clone());
    let usdm = usdm::Config::with_pools(usdm::Environment::Production, &pools)
        .unwrap()
        .clock(clock.clone());
    let coinm = coinm::Config::with_pools(coinm::Environment::Production, &pools)
        .unwrap()
        .clock(clock.clone());
    let options = options::Config::with_pools(options::Environment::Production, &pools)
        .unwrap()
        .clock(clock.clone());
    let explicit = usdm::Config::with_pools(usdm::Environment::Production, &pools)
        .unwrap()
        .clock(clock)
        .budgets(Budgets::new(BudgetLimits::usdm()).unwrap());
    let reports = [
        (
            spot.pool_usage().unwrap().key,
            spot::RestClient::new(spot.clone())
                .unwrap()
                .pool_usage()
                .unwrap()
                .key,
            spot::RestClient::new(spot.clone()).unwrap().pool_key(),
            spot.pool_key(),
        ),
        (
            usdm.pool_usage().unwrap().key,
            usdm::RestClient::new(usdm.clone())
                .unwrap()
                .pool_usage()
                .unwrap()
                .key,
            usdm::RestClient::new(usdm.clone()).unwrap().pool_key(),
            usdm.pool_key(),
        ),
        (
            coinm.pool_usage().unwrap().key,
            coinm::RestClient::new(coinm.clone())
                .unwrap()
                .pool_usage()
                .unwrap()
                .key,
            coinm::RestClient::new(coinm.clone()).unwrap().pool_key(),
            coinm.pool_key(),
        ),
        (
            options.pool_usage().unwrap().key,
            options::RestClient::new(options.clone())
                .unwrap()
                .pool_usage()
                .unwrap()
                .key,
            options::RestClient::new(options.clone())
                .unwrap()
                .pool_key(),
            options.pool_key(),
        ),
        (
            explicit.pool_usage().unwrap().key,
            usdm::RestClient::new(explicit.clone())
                .unwrap()
                .pool_usage()
                .unwrap()
                .key,
            usdm::RestClient::new(explicit.clone()).unwrap().pool_key(),
            explicit.pool_key(),
        ),
    ];
    let expected = [
        Some(key(VenuePool::Spot, PoolEnvironment::Demo)),
        Some(key(VenuePool::Futures, PoolEnvironment::Production)),
        Some(key(VenuePool::Futures, PoolEnvironment::Production)),
        Some(key(VenuePool::Options, PoolEnvironment::Production)),
        None,
    ];
    for (index, (report, expected)) in reports.into_iter().zip(expected).enumerate() {
        assert_eq!(
            report,
            (expected, expected, expected, expected),
            "case {index}"
        );
    }
}
