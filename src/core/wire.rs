// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact protocol scalars shared without product-specific value normalization.
use crate::Symbol;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub(crate) fn identity_string_option<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<String>, D::Error> {
    Option::<Value>::deserialize(d)?
        .map(|v| match v {
            Value::String(s) => Ok(s),
            Value::Number(n) if n.is_i64() || n.is_u64() => Ok(n.to_string()),
            _ => Err(serde::de::Error::custom(
                "integer identity or string required",
            )),
        })
        .transpose()
}

pub(crate) fn integer_option<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
    Option::<Value>::deserialize(d)?
        .map(|v| match v {
            Value::String(s) => s.parse().map_err(serde::de::Error::custom),
            Value::Number(n) => n
                .as_i64()
                .ok_or_else(|| serde::de::Error::custom("integer outside signed 64-bit range")),
            _ => Err(serde::de::Error::custom(
                "integer or integer string required",
            )),
        })
        .transpose()
}

pub(crate) fn symbol_option_empty<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Symbol>, D::Error> {
    Option::<String>::deserialize(d)?
        .filter(|s| !s.is_empty())
        .map(|s| Symbol::new(s).map_err(serde::de::Error::custom))
        .transpose()
}
