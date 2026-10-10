// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Margin identities preserve exact wire representation and caller spelling.
use crate::Error;
use serde::{Deserialize, Serialize};
/// Caller-selected Margin identity. The Margin catalog does not define Futures grammar.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct ClientOrderId(String);
impl ClientOrderId {
    /// Retain native spelling without normalization or an inferred product grammar.
    ///
    /// # Errors
    /// Refuses empty identities and control characters.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(Error::Validation("Margin client order identity"));
        }
        Ok(Self(value))
    }
    /// Exact caller identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for ClientOrderId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::new(String::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum WireInteger {
    Number(i64),
    Text(String),
}
fn integer_text(value: &str) -> Result<i64, Error> {
    let digits = match value.strip_prefix('-') {
        Some(digits) => digits,
        None => value,
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::Validation("Margin decimal-text identity"));
    }
    value
        .parse()
        .map_err(|_| Error::Validation("Margin integer identity overflow"))
}
macro_rules! integer_identity {
    ($name:ident,$doc:literal) => {
        #[doc=$doc]
        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        #[serde(transparent)]
        pub struct $name {
            #[serde(skip)]
            value: i64,
            wire: WireInteger,
        }
        impl $name {
            /// Preserve a representable native integer; no undocumented positive bound is imposed.
            #[must_use]
            pub fn new(value: i64) -> Self {
                Self {
                    value,
                    wire: WireInteger::Number(value),
                }
            }
            /// Preserve a decimal-text native identity, including its exact spelling.
            ///
            /// # Errors
            /// Refuses empty, noninteger or unrepresentable identity evidence.
            pub fn from_text(value: impl Into<String>) -> Result<Self, Error> {
                let text = value.into();
                let value = integer_text(&text)?;
                Ok(Self {
                    value,
                    wire: WireInteger::Text(text),
                })
            }
            /// Exact numerical identity; serialization retains the original wire representation.
            #[must_use]
            pub fn value(&self) -> i64 {
                self.value
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                match WireInteger::deserialize(d)? {
                    WireInteger::Number(value) => Ok(Self::new(value)),
                    WireInteger::Text(value) => {
                        Self::from_text(value).map_err(serde::de::Error::custom)
                    }
                }
            }
        }
    };
}
integer_identity!(
    OrderId,
    "Native Margin order identity, including documented textual response alternatives."
);
integer_identity!(
    OrderListId,
    "Native Margin order-list identity; -1 explicitly identifies an individual order."
);
integer_identity!(
    TradeId,
    "Native Margin trade identity, distinct from an order identity."
);
integer_identity!(
    TransactionId,
    "Native Margin transaction or repayment identity, retained for reconciliation."
);
integer_identity!(
    PreventedMatchId,
    "Native Margin prevented-match identity, distinct from executed trades."
);
integer_identity!(
    RecordId,
    "Native Margin record or cursor identity, without inferred economic meaning."
);
/// Native order-list membership evidence; individual orders use the documented -1 sentinel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OrderListKind {
    /// Venue explicitly identifies an individual order with -1.
    IndividualOrder,
    /// Venue returned a nonnegative list identity, retained exactly.
    List,
}
/// An unrecognized negative order-list sentinel; the accepted identity is retained.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("unrecognized Margin order-list identity kind")]
#[non_exhaustive]
pub struct OrderListKindError {
    identity: OrderListId,
}
impl OrderListKindError {
    /// Original accepted identity with its numeric or textual wire form preserved.
    #[must_use]
    pub fn identity(&self) -> &OrderListId {
        &self.identity
    }
}
impl OrderListId {
    /// Interpret only the documented sentinel, without fabricating missing membership.
    ///
    /// # Errors
    /// Retains an unknown negative sentinel in a typed invariant failure. The
    /// original identity remains unchanged and queryable by the owning caller.
    pub fn kind(&self) -> Result<OrderListKind, OrderListKindError> {
        match self.value {
            -1 => Ok(OrderListKind::IndividualOrder),
            0.. => Ok(OrderListKind::List),
            _ => Err(OrderListKindError {
                identity: self.clone(),
            }),
        }
    }
}
