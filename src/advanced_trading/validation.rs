// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared syntactic checks, without product order or risk policy.
use crate::Error;
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) type Parameters = BTreeMap<String, Value>;
pub(crate) fn text<'a>(p: &'a Parameters, k: &str) -> Option<&'a str> {
    p.get(k).and_then(Value::as_str)
}
pub(crate) fn truth(p: &Parameters, k: &str) -> bool {
    p.get(k).is_some_and(|v| v == true || v == "true")
}
pub(crate) fn required(p: &Parameters, keys: &[&str]) -> Result<(), Error> {
    if keys.iter().any(|k| !p.contains_key(*k)) {
        return Err(Error::Validation("required native conditional parameter"));
    }
    Ok(())
}
pub(crate) fn either(p: &Parameters, a: &str, b: &str) -> Result<(), Error> {
    if !p.contains_key(a) && !p.contains_key(b) {
        return Err(Error::Validation("native query identity required"));
    }
    Ok(())
}
pub(crate) fn forbidden(p: &Parameters, keys: &[&str]) -> Result<(), Error> {
    if keys.iter().any(|k| p.contains_key(*k)) {
        return Err(Error::Validation("conflicting native order parameter"));
    }
    Ok(())
}
pub(crate) fn range(p: &Parameters, max: u64, strict: bool) -> Result<(), Error> {
    for key in ["startTime", "endTime"] {
        if p.get(key).and_then(Value::as_i64).is_some_and(|v| v < 0) {
            return Err(Error::Validation("negative native timestamp"));
        }
    }
    if let (Some(start), Some(end)) = (
        p.get("startTime").and_then(Value::as_i64),
        p.get("endTime").and_then(Value::as_i64),
    ) {
        let span = u64::try_from(
            end.checked_sub(start)
                .ok_or(Error::Validation("native time range overflow"))?,
        )
        .map_err(|_| Error::Validation("reversed native time range"))?;
        if span > max || (strict && span == max) {
            return Err(Error::Validation("native history interval"));
        }
    }
    Ok(())
}
pub(crate) fn common(p: &Parameters) -> Result<(), Error> {
    for (key, v) in p {
        if v.as_str()
            .is_some_and(|s| s.is_empty() || s.chars().any(char::is_control))
        {
            return Err(Error::Validation(
                "empty or control-bearing native parameter",
            ));
        }
        if key == "recvWindow" && v.as_i64().is_none() {
            return Err(Error::Validation(
                "native receive window uses integer milliseconds",
            ));
        }
    }
    range(p, u64::MAX, false)
}

pub(crate) fn validate(_op: &str, p: &Parameters) -> Result<(), Error> {
    common(p)
}

pub(crate) fn string_response_failed(value: &Value, success_code: &str) -> bool {
    value.get("success") == Some(&Value::Bool(false))
        || value
            .get("code")
            .and_then(Value::as_str)
            .is_some_and(|code| code != success_code)
}

/// Classification accepts the native catalog explicitly; unknown codes and 5xx
/// responses never inherit another product's refusal semantics.
pub(crate) fn definitive(status: u16, value: &Value, codes: &[i64]) -> bool {
    status < 500
        && value
            .get("code")
            .and_then(Value::as_i64)
            .is_some_and(|code| codes.contains(&code))
}
pub(crate) fn validate_time(p: &Parameters, _now: u64) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advanced_trading::{
        algo, alpha, copy_trading, portfolio_margin, portfolio_margin_pro, stocks,
    };

    #[test]
    fn classification_uses_each_pinned_native_catalog() -> Result<(), serde_json::Error> {
        type Classifier = fn(u16, &Value) -> bool;
        let products: &[(&str, Classifier)] = &[
            (
                include_str!("../../schema/algo-error-codes.json"),
                algo::validation::definitive,
            ),
            (
                include_str!("../../schema/alpha-error-codes.json"),
                alpha::validation::definitive,
            ),
            (
                include_str!("../../schema/copy-trading-error-codes.json"),
                copy_trading::validation::definitive,
            ),
            (
                include_str!("../../schema/portfolio-margin-error-codes.json"),
                portfolio_margin::validation::definitive,
            ),
            (
                include_str!("../../schema/portfolio-margin-pro-error-codes.json"),
                portfolio_margin_pro::validation::definitive,
            ),
            (
                include_str!("../../schema/stocks-error-codes.json"),
                stocks::validation::definitive,
            ),
        ];
        for (catalog, classify) in products {
            let catalog: Value = serde_json::from_str(catalog)?;
            let codes: Vec<i64> = serde_json::from_value(catalog["definitive"].clone())?;
            for code in &codes {
                assert!(classify(400, &serde_json::json!({"code": code})));
                assert!(!classify(500, &serde_json::json!({"code": code})));
            }
            assert_eq!(
                classify(400, &serde_json::json!({"code": -1131})),
                codes.contains(&-1131)
            );
            assert!(!classify(400, &serde_json::json!({"code": -999_999})));
            assert!(!classify(400, &serde_json::json!({"code": "-1131"})));
        }
        // Only Portfolio Margin pins these definitive server messages.
        let reply = serde_json::json!({"msg":"Service Unavailable."});
        assert!(portfolio_margin::validation::definitive(503, &reply));
        assert!(!algo::validation::definitive(503, &reply));
        assert!(!portfolio_margin::validation::definitive(
            503,
            &serde_json::json!({"msg":"unknown"})
        ));
        Ok(())
    }
}
