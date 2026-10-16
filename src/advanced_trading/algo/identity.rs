// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
use serde::{Deserialize, Serialize};

/// Caller-owned Algo service identity, distinct from Futures conditional-order IDs.
/// The Algo service documents exactly 32 characters for Spot and Futures placements.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ClientAlgoId(String);
impl ClientAlgoId {
    /// Retain the exact caller identity without truncation or normalization.
    ///
    /// # Errors
    /// Refuses lengths other than 32 Unicode characters and control characters.
    pub fn new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        if value.chars().count() != 32 || value.chars().any(char::is_control) {
            return Err(Error::Validation(
                "Algo client identity must have 32 characters",
            ));
        }
        Ok(Self(value))
    }
    /// Exact caller-supplied identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for ClientAlgoId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::new(String::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
