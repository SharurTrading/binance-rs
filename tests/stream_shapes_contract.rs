// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Whole documented raw and combined event bodies for every supported family.
#![allow(clippy::unwrap_used, clippy::panic, reason = "synthetic stream corpus")]
#[path = "support/stream_shape_dispatch.rs"]
mod dispatch;

#[test]
fn all_documented_market_and_private_event_shapes_decode() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/stream-shapes.json")).unwrap();
    let mut failures = Vec::new();
    for case in corpus["cases"].as_array().unwrap() {
        let value = case["unwrap"].as_str().map_or_else(
            || case["response"].clone(),
            |field| case["response"][field].clone(),
        );
        if let Err(error) = dispatch::check(case["model"].as_str().unwrap(), value) {
            failures.push(format!(
                "{} {}/{}: {error}",
                case["id"], case["product"], case["operation"]
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "native stream failures:\n{}",
        failures.join("\n")
    );
}
