// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Market-stream fields the venue documents as conditional decode when absent,
//! while the fields it always sends remain required evidence.

#![allow(clippy::unwrap_used, reason = "synthetic contract assertions")]

use binance_client::{Decimal, coinm, spot, usdm};
use serde_json::json;

/// A COIN-M settlement push: the venue sends `bks` only on a bracket update.
const SETTLEMENT_WITHOUT_BRACKETS: &str = r#"{"e":"contractInfo","E":1,"s":"BTCUSD_261225","ps":"BTCUSD","ct":"CURRENT_QUARTER","dt":1798185600000,"ot":1782979200000,"cs":"SETTLING","st":2}"#;

#[test]
fn coinm_contract_info_settlement_without_brackets_decodes() {
    let event: coinm::stream_models::ContractInfoStreamEvent =
        serde_json::from_str(SETTLEMENT_WITHOUT_BRACKETS).unwrap();
    assert_eq!(event.s.as_str(), "BTCUSD_261225");
    assert_eq!(
        event.ps.as_ref().map(binance_client::Symbol::as_str),
        Some("BTCUSD")
    );
    assert_eq!(event.ct, "CURRENT_QUARTER");
    assert_eq!(event.dt, 1_798_185_600_000);
    assert_eq!(event.ot, 1_782_979_200_000);
    assert_eq!(event.cs, "SETTLING");
    assert_eq!(event.st, Some(2));
    assert_eq!(event.bks, None);
}

#[test]
fn coinm_contract_info_bracket_update_keeps_its_brackets() {
    let event: coinm::stream_models::ContractInfoStreamEvent = serde_json::from_value(json!({
        "e":"contractInfo","E":1_669_647_330_375_i64,"s":"APTUSD_PERP","ps":"APTUSD","ct":"PERPETUAL",
        "dt":4_133_404_800_000_i64,"ot":1_666_594_800_000_i64,"cs":"TRADING",
        "bks":[{"bs":1,"bnf":0,"bnc":500_000,"mmr":0.0065,"cf":75,"mi":51,"ma":75}],"st":1
    }))
    .unwrap();
    let brackets = event.bks.unwrap();
    assert_eq!(brackets.len(), 1);
    assert_eq!(brackets[0].bs, Some(1));
    assert_eq!(brackets[0].bnf, Some(0));
    assert_eq!(brackets[0].bnc, Some(500_000));
    assert_eq!(brackets[0].mmr, Some(Decimal::new(65, 4)));
    assert_eq!(brackets[0].cf, Some(75));
    assert_eq!(brackets[0].mi, Some(51));
    assert_eq!(brackets[0].ma, Some(75));
}

#[test]
fn coinm_contract_info_empty_bracket_list_stays_distinct_from_absent() {
    let mut frame: serde_json::Value = serde_json::from_str(SETTLEMENT_WITHOUT_BRACKETS).unwrap();
    frame["bks"] = json!([]);
    let event: coinm::stream_models::ContractInfoStreamEvent =
        serde_json::from_value(frame).unwrap();
    assert_eq!(event.bks, Some(Vec::new()));
}

#[test]
fn usdm_contract_info_listing_without_brackets_decodes() {
    let event: usdm::stream_models::ContractInfoStreamEvent = serde_json::from_value(json!({
        "e":"contractInfo","E":1,"s":"BTCUSDT_261225","ct":"CURRENT_QUARTER",
        "dt":1_798_185_600_000_i64,"ot":1_782_979_200_000_i64,"cs":"PENDING_TRADING","st":1
    }))
    .unwrap();
    assert_eq!(event.s, "BTCUSDT_261225");
    assert_eq!(event.ct, "CURRENT_QUARTER");
    assert_eq!(event.dt, 1_798_185_600_000);
    assert_eq!(event.ot, 1_782_979_200_000);
    assert_eq!(event.cs, "PENDING_TRADING");
    assert_eq!(event.st, Some(1));
    assert_eq!(event.bks, None);
}

#[test]
fn contract_info_without_a_documented_contract_field_is_refused_in_both_markets() {
    let complete = json!({
        "e":"contractInfo","E":1,"s":"BTCUSD_261225","ct":"CURRENT_QUARTER",
        "dt":1_798_185_600_000_i64,"ot":1_782_979_200_000_i64,"cs":"SETTLING"
    });
    for field in ["ct", "dt", "ot", "cs"] {
        let mut frame = complete.clone();
        frame.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<usdm::stream_models::ContractInfoStreamEvent>(frame.clone())
                .is_err(),
            "USD-M accepted a frame without {field}"
        );
        assert!(
            serde_json::from_value::<coinm::stream_models::ContractInfoStreamEvent>(frame).is_err(),
            "COIN-M accepted a frame without {field}"
        );
    }
}

#[test]
fn spot_reference_price_null_decodes_as_absent() {
    let event: spot::stream_models::ReferencePriceEvent = serde_json::from_value(
        json!({"e":"referencePrice","s":"BAZUSD","r":null,"t":1_770_313_263_917_i64}),
    )
    .unwrap();
    assert_eq!(event.s.as_str(), "BAZUSD");
    assert_eq!(event.r, None);
    assert_eq!(event.t, 1_770_313_263_917);
}

#[test]
fn spot_reference_price_value_stays_exact_and_malformed_value_is_refused() {
    let event: spot::stream_models::ReferencePriceEvent = serde_json::from_value(
        json!({"e":"referencePrice","s":"BAZUSD","r":"1.00","t":1_770_313_263_917_i64}),
    )
    .unwrap();
    assert_eq!(event.r, Some(Decimal::new(100, 2)));
    assert!(
        serde_json::from_value::<spot::stream_models::ReferencePriceEvent>(
            json!({"e":"referencePrice","s":"BAZUSD","r":"not-a-price","t":1_770_313_263_917_i64}),
        )
        .is_err()
    );
}
