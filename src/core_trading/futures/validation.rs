// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) fn str_field<'a>(p: &'a BTreeMap<String, Value>, name: &str) -> Option<&'a str> {
    p.get(name).and_then(Value::as_str)
}
pub(crate) fn truth(p: &BTreeMap<String, Value>, name: &str) -> bool {
    p.get(name)
        .is_some_and(|v| v == &Value::Bool(true) || v.as_str() == Some("true"))
}
pub(crate) fn required(p: &BTreeMap<String, Value>, names: &[&str]) -> Result<(), Error> {
    if names.iter().any(|n| !p.contains_key(*n)) {
        return Err(Error::Validation("conditional required parameter"));
    }
    Ok(())
}
