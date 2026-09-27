// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
use serde::{Deserialize, Serialize};

/// Caller-owned Spot order identity, retained exactly without Futures grammar.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct ClientOrderId(String);
impl ClientOrderId {
    /// Validate nonempty, printable provider identity without modifying it.
    /// Venue-specific length/character restrictions remain venue checks.
    ///
    /// # Errors
    /// Refuses empty identities or control characters.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(Error::Validation("Spot client order ID"));
        }
        Ok(Self(value))
    }
    /// The exact caller identity.
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
