// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Every supported JSON method has independently sourced native response fixtures.
#![allow(clippy::unwrap_used, reason = "synthetic protocol assertions")]
#[path = "support/response_shape_dispatch.rs"]
mod dispatch;

#[test]
fn every_documented_response_shape_decodes_without_unknown_success_members() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/response-shapes.json")).unwrap();
    let mut failures = Vec::new();
    for case in corpus["cases"].as_array().unwrap() {
        if let Err(error) = dispatch::check(
            case["product"].as_str().unwrap(),
            case["transport"].as_str().unwrap(),
            case["operation"].as_str().unwrap(),
            case["response"].clone(),
        ) {
            failures.push(format!(
                "{} {}/{}/{}: {error}",
                case["id"], case["product"], case["transport"], case["operation"]
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} native response failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
