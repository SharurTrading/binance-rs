// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact financial wire values for SAPI.
use serde::{Deserialize, Deserializer};

/// Validate issuance evidence during response decoding, before an accepted receipt.
pub(crate) fn listen_token<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<crate::SensitiveString, D::Error> {
    let token = crate::SensitiveString::deserialize(deserializer)?;
    if token.as_str().is_empty() {
        return Err(serde::de::Error::custom("empty Margin listen token"));
    }
    Ok(token)
}

/// Native expiration is a nonnegative Unix timestamp in milliseconds.
pub(crate) fn expiration_time<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    let expiration = i64::deserialize(deserializer)?;
    if expiration < 0 {
        return Err(serde::de::Error::custom("negative Margin token expiry"));
    }
    Ok(expiration)
}

pub(crate) use crate::core::wire::{decimal, decimal_option, parse_decimal};
