// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
use serde::{Deserialize, Serialize};
macro_rules! text_id {
    ($name:ident,$doc:literal) => {
        #[doc=$doc]
        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);
        impl $name {
            /// Retain a nonempty provider identity without transforming it.
            ///
            /// # Errors
            /// Refuses empty identities or control characters.
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                if value.is_empty() || value.chars().any(char::is_control) {
                    return Err(Error::Validation("Convert identity"));
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
    QuoteId,
    "Venue quote identity; retain it for acceptance and uncertain-outcome reads."
);
text_id!(
    AcceptanceOrderId,
    "Native string order identity returned by quote acceptance and accepted by status queries."
);
/// Native integer order identity returned by Convert queries and used by limit operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct OrderId(i64);
impl OrderId {
    /// Validate a positive venue order identity.
    ///
    /// # Errors
    /// Refuses non-positive identities.
    pub fn new(value: i64) -> Result<Self, Error> {
        if value <= 0 {
            return Err(Error::Validation("Convert order identity"));
        }
        Ok(Self(value))
    }
    /// Exact provider integer identity.
    #[must_use]
    pub fn value(self) -> i64 {
        self.0
    }
}
impl<'de> Deserialize<'de> for OrderId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::new(i64::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}

impl From<OrderId> for AcceptanceOrderId {
    /// Explicitly render an integer order identity for the native string status query.
    fn from(value: OrderId) -> Self {
        Self(value.value().to_string())
    }
}
