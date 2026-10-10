// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Options identities retain native spelling; no asset or expiry inference.
use crate::Error;
use serde::{Deserialize, Serialize};
macro_rules! text_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);
        impl $name {
            /// Retain a nonempty provider identity without transforming it.
            ///
            /// # Errors
            /// Refuses empty identities, whitespace and control characters.
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                if value.is_empty() || value.chars().any(|c| c.is_control() || c.is_whitespace()) {
                    return Err(Error::Validation("Options identity"));
                }
                Ok(Self(value))
            }
            /// Exact provider identity.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                Self::new(String::deserialize(d)?).map_err(serde::de::Error::custom)
            }
        }
    };
}
text_id!(
    Symbol,
    "Native option contract symbol; contains dashes and remains distinct from its underlying."
);
text_id!(
    ClientOrderId,
    "Caller order identity, required for every Options order submission."
);

/// Native integer identity representation; string-valued venue IDs retain spelling.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum IntegerIdentity {
    Number(i64),
    Text(String),
}
fn integer_text(value: &str) -> Result<i64, Error> {
    let digits = value.strip_prefix('-').unwrap_or(value);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::Validation("Options decimal-text identity"));
    }
    value
        .parse()
        .map_err(|_| Error::Validation("Options integer identity overflow"))
}
macro_rules! integer_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        #[serde(transparent)]
        pub struct $name {
            #[serde(skip)]
            value: i64,
            wire: IntegerIdentity,
        }
        impl $name {
            /// Preserve a representable native integer without an undocumented positive bound.
            #[must_use]
            pub fn new(value: i64) -> Self {
                Self {
                    value,
                    wire: IntegerIdentity::Number(value),
                }
            }
            /// Preserve exact decimal-text spelling of a native integer identity.
            ///
            /// # Errors
            /// Refuses empty, noninteger or unrepresentable identity evidence.
            pub fn from_text(value: impl Into<String>) -> Result<Self, Error> {
                let text = value.into();
                let value = integer_text(&text)?;
                Ok(Self {
                    value,
                    wire: IntegerIdentity::Text(text),
                })
            }
            /// Exact integer value; string response spelling remains preserved on serialization.
            #[must_use]
            pub fn value(&self) -> i64 {
                self.value
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                match IntegerIdentity::deserialize(d)? {
                    IntegerIdentity::Number(value) => Ok(Self::new(value)),
                    IntegerIdentity::Text(value) => {
                        Self::from_text(value).map_err(serde::de::Error::custom)
                    }
                }
            }
        }
    };
}
integer_id!(
    OrderId,
    "Validated Options order identity, preserving numeric or textual wire representation."
);
integer_id!(
    TradeId,
    "Validated Options trade identity, distinct from an order identity."
);
integer_id!(
    RecordId,
    "Validated Options account/market record identity, distinct from an order or trade."
);
text_id!(
    BlockOrderMatchingKey,
    "Native block-order matching key, supplied by the venue and preserved for reconciliation."
);
text_id!(
    BlockTradeSettlementKey,
    "Native block-trade settlement key, distinct from its matching key."
);
