// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated native stream DTOs.

use crate::Decimal;
use crate::SensitiveString;
use crate::Symbol;
use serde::{Deserialize, Serialize};

/// Provider-native `PriceStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PriceStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `rates` wire field.
    #[serde(rename = "rates")]
    pub rates: Vec<PriceStreamEventRatesItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PriceStreamEventRatesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PriceStreamEventRatesItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `ac` wire field.
    #[serde(rename = "ac", default, skip_serializing_if = "Option::is_none")]
    pub ac: Option<String>,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `pc` wire field.
    #[serde(
        rename = "pc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pc: Option<Decimal>,
    /// Exact `tc` wire field.
    #[serde(
        rename = "tc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tc: Option<Decimal>,
    /// Exact `mp` wire field.
    #[serde(rename = "mp", default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QuoteStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuoteStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `bp` wire field.
    #[serde(rename = "bp", deserialize_with = "super::wire::decimal")]
    pub bp: Decimal,
    /// Exact `ap` wire field.
    #[serde(rename = "ap", deserialize_with = "super::wire::decimal")]
    pub ap: Decimal,
    /// Exact `bs` wire field.
    #[serde(rename = "bs", deserialize_with = "super::wire::decimal")]
    pub bs: Decimal,
    /// Exact `as` wire field.
    #[serde(rename = "as", deserialize_with = "super::wire::decimal")]
    pub as_value: Decimal,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `k` wire field.
    #[serde(rename = "k")]
    pub k: KlineStreamEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineStreamEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineStreamEventK {
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
    /// Exact `ct` wire field.
    #[serde(rename = "ct", default, skip_serializing_if = "Option::is_none")]
    pub ct: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: String,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
    /// Exact `c` wire field.
    #[serde(rename = "c", deserialize_with = "super::wire::decimal")]
    pub c: Decimal,
    /// Exact `h` wire field.
    #[serde(rename = "h", deserialize_with = "super::wire::decimal")]
    pub h: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
    /// Exact `n` wire field.
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Exact `x` wire field.
    #[serde(rename = "x")]
    pub x: bool,
    /// Exact `vw` wire field.
    #[serde(
        rename = "vw",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub vw: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CalendarStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalendarStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `from` wire field.
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Exact `to` wire field.
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Exact `ts` wire field.
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradabilityStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradabilityStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `assetCode` wire field.
    #[serde(rename = "assetCode", default, skip_serializing_if = "Option::is_none")]
    pub asset_code: Option<String>,
    /// Exact `tradability` wire field.
    #[serde(rename = "tradability")]
    pub tradability: String,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradingStatusStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingStatusStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `assetCode` wire field.
    #[serde(rename = "assetCode", default, skip_serializing_if = "Option::is_none")]
    pub asset_code: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `z` wire field.
    #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
    pub z: Option<String>,
    /// Exact `tradability` wire field.
    #[serde(
        rename = "tradability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tradability: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderReportStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderReportStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `x` wire field.
    #[serde(rename = "x", default, skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: String,
    /// Exact `ai` wire field.
    #[serde(rename = "ai", default, skip_serializing_if = "Option::is_none")]
    pub ai: Option<String>,
    /// Exact `b` wire field.
    #[serde(rename = "b")]
    pub b: crate::Asset,
    /// Exact `q` wire field.
    #[serde(rename = "q")]
    pub q: crate::Asset,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: String,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: String,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `Q` wire field.
    #[serde(
        rename = "Q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_q: Option<Decimal>,
    /// Exact `N` wire field.
    #[serde(
        rename = "N",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_n: Option<Decimal>,
    /// Exact `fq` wire field.
    #[serde(
        rename = "fq",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fq: Option<Decimal>,
    /// Exact `FN` wire field.
    #[serde(
        rename = "FN",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_fn: Option<Decimal>,
    /// Exact `tc` wire field.
    #[serde(
        rename = "tc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tc: Option<Decimal>,
    /// Exact `Z` wire field.
    #[serde(
        rename = "Z",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_z: Option<Decimal>,
    /// Exact `n` wire field.
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `U` wire field.
    #[serde(rename = "U", default, skip_serializing_if = "Option::is_none")]
    pub upper_u: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
