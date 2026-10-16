// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Every pinned binary template and its documented native response associations.
#![allow(clippy::unwrap_used, clippy::panic, reason = "synthetic wire corpus")]
use binance_client::spot::{fix, sbe};
use serde_json::Value;
#[path = "support/binary_shape_dispatch.rs"]
mod dispatch;

fn bytes(case: &Value) -> Vec<u8> {
    case["hex"]
        .as_str()
        .unwrap()
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn decode(case: &Value) -> Result<Value, binance_client::Error> {
    let wire = bytes(case);
    let role = if case["role"] == "market_data" {
        fix::Role::MarketData
    } else {
        fix::Role::OrderEntry
    };
    match case["protocol"].as_str().unwrap() {
        "api" => sbe::decode_api::<Value>(&wire),
        "market" => sbe::decode_market(&wire).map(|_| Value::Null),
        "fix" => fix::decode_sbe(role, &wire).map(|_| Value::Null),
        "oe" => fix::decode(fix::Role::OrderEntry, &wire).map(|_| Value::Null),
        "md" => fix::decode(fix::Role::MarketData, &wire).map(|_| Value::Null),
        _ => panic!("unknown binary corpus protocol"),
    }
}

fn assertions(case: &Value, value: &Value) {
    for assertion in case["assertions"].as_array().unwrap() {
        let actual = value.pointer(assertion["path"].as_str().unwrap());
        if assertion["absent"] == true {
            assert!(actual.is_none(), "{}: {assertion}", case["id"]);
        } else if let Some(length) = assertion.get("array_length") {
            assert_eq!(
                actual.unwrap().as_array().unwrap().len() as u64,
                length.as_u64().unwrap(),
                "{}",
                case["id"]
            );
        } else {
            assert_eq!(
                actual,
                Some(&assertion["equals"]),
                "{}: {assertion}",
                case["id"]
            );
        }
    }
}

#[test]
fn every_binary_template_and_native_projection_has_the_expected_disposition() {
    let corpus: Value = serde_json::from_str(include_str!("fixtures/binary-shapes.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let result = decode(case);
        if case["generic_expectation"] != "decoded" {
            assert!(result.is_err(), "{} must refuse", case["id"]);
            continue;
        }
        let value = result.unwrap_or_else(|e| panic!("{}: {e}", case["id"]));
        assertions(case, &value);
        for check in case["native_checks"].as_array().unwrap() {
            dispatch::check(
                check["model"].as_str().unwrap(),
                value
                    .pointer(check["value_path"].as_str().unwrap())
                    .unwrap()
                    .clone(),
                check["expectation"].as_str().unwrap(),
            )
            .unwrap_or_else(|e| panic!("{} {}: {e}", case["id"], check["model"]));
        }
    }
}
