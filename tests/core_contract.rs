// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Input identity and credential redaction contracts.

use binance_client::{ClientOrderId, Credentials, Symbol};

#[test]
fn identifiers_validate_without_restricting_unicode_symbols() {
    assert!(Symbol::new("BTCUSDT").is_ok());
    assert!(Symbol::new("１２３USDT").is_ok());
    assert!(Symbol::new("").is_err());
    assert!(ClientOrderId::new("strategy/order:1").is_ok());
    assert!(ClientOrderId::new("a".repeat(37)).is_err());
    assert!(ClientOrderId::new("bad id").is_err());
}

#[test]
fn credentials_debug_is_redacted() {
    let credentials = Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap();
    let debug = format!("{credentials:?}");
    assert!(!debug.contains("synthetic-api-key"));
    assert!(!debug.contains("synthetic-secret"));
}

#[test]
fn unrepresentable_configured_timeouts_are_refused_before_io() {
    use binance_client::{coinm, convert, spot, usdm, wallet};
    let huge = std::time::Duration::MAX;
    assert!(
        usdm::Config::with_pools(usdm::Environment::Demo, &binance_client::WeightPools::new())
            .unwrap()
            .timeout(huge)
            .is_err()
    );
    assert!(
        coinm::Config::with_pools(coinm::Environment::Demo, &binance_client::WeightPools::new())
            .unwrap()
            .timeout(huge)
            .is_err()
    );
    assert!(
        spot::Config::with_pools(spot::Environment::Demo, &binance_client::WeightPools::new())
            .unwrap()
            .timeout(huge)
            .is_err()
    );
    assert!(wallet::Config::new().unwrap().timeout(huge).is_err());
    assert!(convert::Config::new().unwrap().timeout(huge).is_err());
}
