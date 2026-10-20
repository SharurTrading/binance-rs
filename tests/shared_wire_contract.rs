// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared wire shapes preserve native sign invariants and redact retained evidence.

#![allow(
    clippy::unwrap_used,
    reason = "synthetic wire fixtures must decode for assertions"
)]

use binance_client::{Decimal, coinm, options, spot, usdm, wallet};
use serde_json::json;

#[test]
fn futures_levels_enforce_quantity_at_construction_and_decode() {
    assert!(usdm::wire::PriceLevel::new(-Decimal::ONE, -Decimal::ONE).is_err());
    assert!(coinm::wire::PriceLevel::new(-Decimal::ONE, -Decimal::ONE).is_err());
    let level = usdm::wire::PriceLevel::new(-Decimal::ONE, Decimal::ZERO).unwrap();
    let same_type: coinm::wire::PriceLevel = level;
    assert_eq!(same_type.price(), -Decimal::ONE);
    assert_eq!(same_type.quantity(), Decimal::ZERO);
    assert_eq!(serde_json::to_value(same_type).unwrap(), json!(["-1", "0"]));
    assert!(serde_json::from_value::<usdm::wire::PriceLevel>(json!(["1", "-1"])).is_err());
    assert!(serde_json::from_value::<coinm::wire::PriceLevel>(json!(["1", "-1"])).is_err());
    assert_eq!(
        spot::wire::PriceLevel::new(-Decimal::ONE, -Decimal::ONE).quantity,
        -Decimal::ONE
    );
    assert_eq!(
        options::wire::PriceLevel::new(-Decimal::ONE, -Decimal::ONE).quantity,
        -Decimal::ONE
    );
}

#[test]
fn retained_evidence_has_one_type_and_redacted_debug_across_products() {
    let raw = json!({"futureSecret":"private-account-evidence"});
    let wallet: wallet::event_payloads::UnknownMessage = raw.clone().into();
    let spot: spot::event_payloads::UnknownMessage = wallet;
    let futures: usdm::event_payloads::UnknownMessage = spot;
    let options: options::event_payloads::UnknownMessage = futures;
    assert_eq!(options.as_value(), &raw);
    assert_eq!(format!("{options:?}"), "UnknownMessage([REDACTED])");
}
