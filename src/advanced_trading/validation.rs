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
