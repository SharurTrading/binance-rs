// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact financial wire values for SAPI.
use crate::Decimal;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub(crate) fn parse_decimal(value: &Value) -> Result<Decimal, &'static str> {
    let text = match value {
        Value::String(value) => value.clone(),
        Value::Number(value) => value.to_string(),
        _ => return Err("decimal must be a string or exact JSON number"),
    };
    Decimal::from_str_exact(&text)
        .or_else(|_| {
            let (base, _) = text
                .split_once(['e', 'E'])
                .ok_or(rust_decimal::Error::ErrorString(
                    "scientific exponent missing".into(),
                ))?;
            Decimal::from_str_exact(base)?;
            Decimal::from_scientific(&text)
        })
        .map_err(|_| "invalid or unrepresentable decimal")
}

pub(crate) fn decimal<'de, D: Deserializer<'de>>(d: D) -> Result<Decimal, D::Error> {
    parse_decimal(&Value::deserialize(d)?).map_err(serde::de::Error::custom)
}

pub(crate) fn decimal_option<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Decimal>, D::Error> {
    Option::<Value>::deserialize(d)?
        .map(|v| parse_decimal(&v).map_err(serde::de::Error::custom))
        .transpose()
}
