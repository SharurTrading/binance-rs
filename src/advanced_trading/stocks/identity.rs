// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
use serde::{Deserialize, Serialize};
/// Caller-owned Stocks order/conversion ID with the native 32–36 character grammar.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ClientOrderId(String);
impl ClientOrderId {
    /// Retain an exact native identity; never generate or transform one.
    ///
    /// # Errors
    /// Refuses lengths outside 32–36 ASCII characters and characters other than
    /// letters, digits, `_` and `-`.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if !(32..=36).contains(&value.len())
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
        {
            return Err(Error::Validation("Stocks client order identity"));
        }
        Ok(Self(value))
    }
    /// Exact caller-supplied identity.
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
