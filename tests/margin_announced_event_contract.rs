// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native announced Margin payloads remain distinct from legacy risk events.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic contract assertions"
)]

use binance_client::{Decimal, Error, margin};
use margin::event_payloads::{UserPayload, user_payload};
use serde_json::{Value, json};

// The official 2026-09-25 changelog supplies these payload shapes, announced for
// 2026-10-14. Decoder coverage does not establish the new stream's transport.
// https://developers.binance.com/en/docs/products/margin-trading/change-log#2026-09-25
fn level_change() -> Value {
    json!({"e":"marginLevelChange","E":1_789_055_758_837_i64,"l":"1.2","s":"MARGIN_CALL"})
}

fn liability_change() -> Value {
    json!({"e":"liabilityChange","E":1_789_024_038_062_i64,"t":"BORROW","L":[
        {"a":"USDT","p":"520.00054700","i":"0.00000000"},
        {"a":"BNB","p":"0.00000000","i":"0.00000568"}
    ]})
}

#[test]
fn announced_margin_level_change_routes_as_a_known_native_event() {
    let UserPayload::MarginLevelChange(event) = user_payload(level_change()).unwrap() else {
        panic!("announced native margin level event required")
    };
    assert_eq!(event.upper_e, 1_789_055_758_837);
    assert_eq!(event.l, Decimal::new(12, 1));
    assert_eq!(event.s, "MARGIN_CALL");
}

#[test]
fn announced_liability_change_routes_as_a_known_native_event() {
    let UserPayload::LiabilityChange(event) = user_payload(liability_change()).unwrap() else {
        panic!("announced native liability event required")
    };
    assert_eq!(event.upper_e, 1_789_024_038_062);
    assert_eq!(event.t, margin::enums::LiabilityChangeType::Borrow);
    assert_eq!(event.upper_l.len(), 2);
    assert_eq!(event.upper_l[0].a.as_str(), "USDT");
    assert_eq!(event.upper_l[0].p, Decimal::new(52_000_054_700, 8));
    assert_eq!(event.upper_l[0].i, Decimal::new(0, 8));
    assert_eq!(event.upper_l[1].a.as_str(), "BNB");
    assert_eq!(event.upper_l[1].p, Decimal::new(0, 8));
    assert_eq!(event.upper_l[1].i, Decimal::new(568, 8));
}

#[test]
fn liability_change_kind_retains_every_documented_value_and_future_spelling() {
    use margin::enums::LiabilityChangeType;
    for (kind, expected) in [
        ("BORROW", LiabilityChangeType::Borrow),
        ("REPAY", LiabilityChangeType::Repay),
        ("INTEREST", LiabilityChangeType::Interest),
        ("DEBT_CHANGE", LiabilityChangeType::DebtChange),
        (
            "futureDebtKind",
            LiabilityChangeType::Unknown("futureDebtKind".to_owned()),
        ),
    ] {
        let mut value = liability_change();
        value["t"] = json!(kind);
        let UserPayload::LiabilityChange(event) = user_payload(value).unwrap() else {
            panic!("native liability event required")
        };
        assert_eq!(event.t, expected);
        assert_eq!(serde_json::to_value(event.t).unwrap(), json!(kind));
    }
}

#[test]
fn liability_array_preserves_empty_evidence_and_refuses_invalid_assets() {
    let mut value = liability_change();
    value["L"] = json!([]);
    let UserPayload::LiabilityChange(event) = user_payload(value).unwrap() else {
        panic!("native liability event required")
    };
    assert!(event.upper_l.is_empty());
    for asset in ["", "US\nDT"] {
        let mut value = liability_change();
        value["L"][1]["a"] = json!(asset);
        assert!(matches!(user_payload(value), Err(Error::Gap(_))));
    }
}

#[test]
fn future_fields_stay_accessible_with_redacted_diagnostics() {
    for mut value in [level_change(), liability_change()] {
        value["futureEvidence"] = json!("private-future-field");
        let payload = user_payload(value).unwrap();
        assert!(!format!("{payload:?}").contains("private-future-field"));
        match payload {
            UserPayload::MarginLevelChange(event) => {
                assert_eq!(
                    event.extra.as_value()["futureEvidence"],
                    "private-future-field"
                );
            }
            UserPayload::LiabilityChange(event) => {
                assert_eq!(
                    event.extra.as_value()["futureEvidence"],
                    "private-future-field"
                );
            }
            _ => panic!("announced native event required"),
        }
    }
}

#[test]
fn malformed_announced_margin_level_change_is_an_explicit_gap() {
    for field in ["e", "E", "l", "s"] {
        let mut value = level_change();
        value.as_object_mut().unwrap().remove(field);
        assert!(
            matches!(user_payload(value), Err(Error::Gap(_))),
            "missing {field}"
        );
    }
    for level in ["not-a-level", "", "0.12345678901234567890123456789"] {
        let mut value = level_change();
        value["l"] = json!(level);
        assert!(matches!(user_payload(value), Err(Error::Gap(_))));
    }
}

#[test]
fn malformed_announced_liability_change_is_an_explicit_gap() {
    for field in ["E", "t", "L"] {
        let mut value = liability_change();
        value.as_object_mut().unwrap().remove(field);
        assert!(
            matches!(user_payload(value), Err(Error::Gap(_))),
            "missing {field}"
        );
    }
    for field in ["a", "p", "i"] {
        let mut value = liability_change();
        value["L"][1].as_object_mut().unwrap().remove(field);
        assert!(
            matches!(user_payload(value), Err(Error::Gap(_))),
            "missing liability {field}"
        );
    }
    for field in ["p", "i"] {
        for amount in ["not-an-amount", "", "0.12345678901234567890123456789"] {
            let mut value = liability_change();
            value["L"][1][field] = json!(amount);
            assert!(matches!(user_payload(value), Err(Error::Gap(_))));
        }
    }
}

#[test]
fn legacy_uppercase_margin_risk_events_keep_their_native_routes() {
    assert!(matches!(
        user_payload(json!({"e":"MARGIN_LEVEL_STATUS_CHANGE","E":1,
        "l":"1.2","s":"MARGIN_CALL"}))
        .unwrap(),
        UserPayload::MarginLevelStatusChange(_)
    ));
    assert!(matches!(
        user_payload(json!({"e":"USER_LIABILITY_CHANGE","E":1,
        "a":"USDT","t":"BORROW","p":"1","i":"0"}))
        .unwrap(),
        UserPayload::UserLiabilityChange(_)
    ));
}
