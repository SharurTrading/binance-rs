// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated native stream DTOs.

use crate::Decimal;
use serde::{Deserialize, Serialize};

/// Provider-native `PmProAccountUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PmProAccountUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u", deserialize_with = "super::wire::decimal")]
    pub u: Decimal,
    /// Exact `eq` wire field.
    #[serde(rename = "eq", deserialize_with = "super::wire::decimal")]
    pub eq: Decimal,
    /// Exact `ae` wire field.
    #[serde(rename = "ae", deserialize_with = "super::wire::decimal")]
    pub ae: Decimal,
    /// Exact `im` wire field.
    #[serde(rename = "im", deserialize_with = "super::wire::decimal")]
    pub im: Decimal,
    /// Exact `mm` wire field.
    #[serde(rename = "mm", deserialize_with = "super::wire::decimal")]
    pub mm: Decimal,
    /// Exact `avb` wire field.
    #[serde(rename = "avb", deserialize_with = "super::wire::decimal")]
    pub avb: Decimal,
    /// Exact `vmw` wire field.
    #[serde(rename = "vmw", deserialize_with = "super::wire::decimal")]
    pub vmw: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `RisklevelchangeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RisklevelchangeEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u", deserialize_with = "super::wire::decimal")]
    pub u: Decimal,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `eq` wire field.
    #[serde(rename = "eq", deserialize_with = "super::wire::decimal")]
    pub eq: Decimal,
    /// Exact `ae` wire field.
    #[serde(rename = "ae", deserialize_with = "super::wire::decimal")]
    pub ae: Decimal,
    /// Exact `m` wire field.
    #[serde(rename = "m", deserialize_with = "super::wire::decimal")]
    pub m: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
