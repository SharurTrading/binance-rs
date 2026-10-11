// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Spot FIX connection attempts are limited per account and per IP address,
//! independently; FIX and FIX SBE share the limits. Sources (checked 2026-10-11):
//! <https://github.com/binance/binance-spot-api-docs/blob/master/fix-api.md#connection-limits>
//! and its 2025-04-21 changelog notice.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]

use binance_client::{
    AccountKey, Credentials, Error, Outcome, Signer, WeightPools,
    spot::fix::{AccountBudgets, CompId, Config, Role, Session},
};
use std::{sync::Arc, time::Duration};

const PRODUCTION: binance_client::spot::Environment = binance_client::spot::Environment::Production;

struct Scratch;
impl Signer for Scratch {
    fn sign(&self, _: &[u8]) -> Result<String, Error> {
        Ok("scratch-signature".into())
    }
}

/// One connection attempt on an unreachable loopback endpoint. Admission runs
/// before transport, so a passed attempt is charged and then fails at TCP.
async fn attempt(budgets: &AccountBudgets, component: &str) -> Error {
    let config = Config::new(
        Role::OrderEntry,
        CompId::new(component).unwrap(),
        Credentials::external_ed25519("synthetic-key", Arc::new(Scratch)).unwrap(),
        budgets.clone(),
    )
    .unwrap()
    .endpoint("tcp://127.0.0.1:1")
    .unwrap()
    .heartbeat(5)
    .unwrap();
    match Session::connect(config).await {
        Ok(..) => panic!("unreachable fixture connected"),
        Err(error) => error,
    }
}

#[tokio::test]
async fn fix_scopes_of_two_keys_on_one_registry_share_the_ip_attempt_window() {
    let pools = WeightPools::new();
    let first =
        AccountBudgets::with_pools_for_account(PRODUCTION, &pools, &AccountKey::new("account-one"))
            .unwrap();
    let second =
        AccountBudgets::with_pools_for_account(PRODUCTION, &pools, &AccountKey::new("account-two"))
            .unwrap();
    for n in 0..10 {
        assert!(
            matches!(
                attempt(&first, &format!("ONE{n}")).await,
                Error::Gap("FIX TCP connection failed")
            ),
            "attempt {n} must pass admission, charging it"
        );
    }
    for n in 0..5 {
        assert!(matches!(
            attempt(&second, &format!("TWO{n}")).await,
            Error::Gap("FIX TCP connection failed")
        ));
    }
    // Neither account's own windows are full; the IP window the registry shares is.
    let error = attempt(&second, "TW10").await;
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert!(
        matches!(&error, Error::Admission { retry_after }
            if !retry_after.is_zero() && *retry_after <= Duration::from_secs(30)),
        "{error:?}"
    );
}

#[tokio::test]
async fn fix_scopes_of_two_registries_do_not_share_the_ip_attempt_window() {
    let first = AccountBudgets::with_pools_for_account(
        PRODUCTION,
        &WeightPools::new(),
        &AccountKey::new("account-one"),
    )
    .unwrap();
    let second = AccountBudgets::with_pools_for_account(
        PRODUCTION,
        &WeightPools::new(),
        &AccountKey::new("account-two"),
    )
    .unwrap();
    for n in 0..10 {
        assert!(matches!(
            attempt(&first, &format!("ONE{n}")).await,
            Error::Gap("FIX TCP connection failed")
        ));
    }
    for n in 0..6 {
        // The sixteenth attempt overall still passes: each registry counts its own IP.
        assert!(matches!(
            attempt(&second, &format!("TWO{n}")).await,
            Error::Gap("FIX TCP connection failed")
        ));
    }
}
