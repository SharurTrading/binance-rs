// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Product-neutral exact JSON values and retained future evidence.

use crate::{Decimal, Symbol};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::borrow::Cow;

pub(crate) fn parse_decimal(value: &Value) -> Result<Decimal, &'static str> {
    let text = match value {
        Value::String(value) => Cow::Borrowed(value.as_str()),
        Value::Number(value) => Cow::Owned(value.to_string()),
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

pub(crate) fn decimal_rows<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Vec<Decimal>>, D::Error> {
    Vec::<Vec<Value>>::deserialize(d)?
        .into_iter()
        .map(|row| {
            row.iter()
                .map(|value| parse_decimal(value).map_err(serde::de::Error::custom))
                .collect()
        })
        .collect()
}

pub(crate) fn decimal<'de, D: Deserializer<'de>>(d: D) -> Result<Decimal, D::Error> {
    parse_decimal(&Value::deserialize(d)?).map_err(serde::de::Error::custom)
}

pub(crate) fn decimal_option<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Decimal>, D::Error> {
    Option::<Value>::deserialize(d)?
        .map(|v| parse_decimal(&v).map_err(serde::de::Error::custom))
        .transpose()
}

// Only fields with an explicitly documented unavailable sentinel use this parser.
pub(crate) fn decimal_option_empty<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Decimal>, D::Error> {
    match Option::<Value>::deserialize(d)? {
        None => Ok(None),
        Some(Value::String(value)) if value.is_empty() => Ok(None),
        Some(value) => parse_decimal(&value)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

pub(crate) fn decimal_option_null_string<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Decimal>, D::Error> {
    match Option::<Value>::deserialize(d)? {
        None => Ok(None),
        Some(Value::String(value)) if value == "null" => Ok(None),
        Some(value) => parse_decimal(&value)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

// Required COIN-M mark-stream funding evidence uses only the documented empty
// string for delivery contracts. Missing fields and JSON null are malformed.
pub(crate) fn decimal_empty<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Decimal>, D::Error> {
    match Value::deserialize(d)? {
        Value::String(value) if value.is_empty() => Ok(None),
        value => parse_decimal(&value)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

/// Unknown future payloads are retained, with redacted Debug output.
#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UnknownMessage(Value);
impl UnknownMessage {
    /// Explicit provider payload access; do not log sensitive account data.
    #[must_use]
    pub fn as_value(&self) -> &Value {
        &self.0
    }
}
impl std::fmt::Debug for UnknownMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UnknownMessage([REDACTED])")
    }
}
impl From<Value> for UnknownMessage {
    fn from(v: Value) -> Self {
        Self(v)
    }
}

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

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "synthetic wire fixtures must decode for assertions"
)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Ordinary {
        #[serde(default, deserialize_with = "decimal_option")]
        amount: Option<Decimal>,
    }
    #[derive(Debug, Deserialize)]
    struct EmptySentinel {
        #[serde(default, deserialize_with = "decimal_option_empty")]
        amount: Option<Decimal>,
    }
    #[derive(Debug, Deserialize)]
    struct NullSentinel {
        #[serde(default, deserialize_with = "decimal_option_null_string")]
        amount: Option<Decimal>,
    }
    #[derive(Debug, Deserialize)]
    struct RequiredEmptySentinel {
        #[serde(deserialize_with = "decimal_empty")]
        amount: Option<Decimal>,
    }

    #[test]
    fn exact_decimal_rejects_unrepresentable_values_without_rounding() {
        let exact = "0.0000000000000000000000000001";
        let from_string = parse_decimal(&Value::String(exact.into())).unwrap();
        let from_number = parse_decimal(&serde_json::from_str::<Value>(exact).unwrap()).unwrap();
        assert_eq!(from_string, from_number);
        assert_eq!(from_string.to_string(), exact);
        assert!(parse_decimal(&Value::String("0.00000000000000000000000000001".into())).is_err());
        assert!(parse_decimal(&Value::String("1.00000000000000000000000000001e1".into())).is_err());
        assert_eq!(
            parse_decimal(&Value::String("1e-28".into())).unwrap(),
            from_string
        );
        assert!(parse_decimal(&Value::Bool(false)).is_err());
    }

    #[test]
    fn unavailable_sentinels_remain_field_specific() {
        for text in ["{}", r#"{"amount":null}"#] {
            assert_eq!(serde_json::from_str::<Ordinary>(text).unwrap().amount, None);
            assert_eq!(
                serde_json::from_str::<EmptySentinel>(text).unwrap().amount,
                None
            );
            assert_eq!(
                serde_json::from_str::<NullSentinel>(text).unwrap().amount,
                None
            );
        }
        assert_eq!(
            serde_json::from_str::<EmptySentinel>(r#"{"amount":""}"#)
                .unwrap()
                .amount,
            None
        );
        assert_eq!(
            serde_json::from_str::<NullSentinel>(r#"{"amount":"null"}"#)
                .unwrap()
                .amount,
            None
        );
        assert_eq!(
            serde_json::from_str::<RequiredEmptySentinel>(r#"{"amount":""}"#)
                .unwrap()
                .amount,
            None
        );
        for text in [r#"{"amount":""}"#, r#"{"amount":"null"}"#] {
            assert!(serde_json::from_str::<Ordinary>(text).is_err());
        }
        assert!(serde_json::from_str::<NullSentinel>(r#"{"amount":""}"#).is_err());
        assert!(serde_json::from_str::<EmptySentinel>(r#"{"amount":"null"}"#).is_err());
        assert!(serde_json::from_str::<RequiredEmptySentinel>("{}").is_err());
        assert!(serde_json::from_str::<RequiredEmptySentinel>(r#"{"amount":null}"#).is_err());
    }
}
