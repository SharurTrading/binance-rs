// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated market and user-data event payloads.

use super::ClientOrderId;
use super::wire::PriceLevel;
use crate::Decimal;
use serde::{Deserialize, Serialize};

/// Provider-native `AggTradeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AggTradeEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: i64,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `f` wire field.
    #[serde(rename = "f")]
    pub f: i64,
    /// Exact `l` wire field.
    #[serde(rename = "l")]
    pub l: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `M` wire field.
    #[serde(rename = "M")]
    pub upper_m: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllMarketRollingWindowTickerEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMarketRollingWindowTickerEventItem {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `P` wire field.
    #[serde(rename = "P", deserialize_with = "super::wire::decimal")]
    pub upper_p: Decimal,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
    /// Exact `h` wire field.
    #[serde(rename = "h", deserialize_with = "super::wire::decimal")]
    pub h: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `w` wire field.
    #[serde(rename = "w", deserialize_with = "super::wire::decimal")]
    pub w: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `O` wire field.
    #[serde(rename = "O")]
    pub upper_o: i64,
    /// Exact `C` wire field.
    #[serde(rename = "C")]
    pub upper_c: i64,
    /// Exact `F` wire field.
    #[serde(rename = "F")]
    pub upper_f: i64,
    /// Exact `L` wire field.
    #[serde(rename = "L")]
    pub upper_l: i64,
    /// Exact `n` wire field.
    #[serde(rename = "n")]
    pub n: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllMiniTickerEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMiniTickerEventItem {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
    /// Exact `h` wire field.
    #[serde(rename = "h", deserialize_with = "super::wire::decimal")]
    pub h: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AvgPriceEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvgPriceEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: String,
    /// Exact `w` wire field.
    #[serde(rename = "w", deserialize_with = "super::wire::decimal")]
    pub w: Decimal,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BookTickerEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BookTickerEvent {
    /// Exact `u` wire field.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `b` wire field.
    #[serde(
        rename = "b",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub b: Option<Decimal>,
    /// Exact `B` wire field.
    #[serde(
        rename = "B",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_b: Option<Decimal>,
    /// Exact `a` wire field.
    #[serde(
        rename = "a",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub a: Option<Decimal>,
    /// Exact `A` wire field.
    #[serde(
        rename = "A",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_a: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DiffBookDepthEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DiffBookDepthEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `U` wire field.
    #[serde(rename = "U")]
    pub upper_u: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `b` wire field.
    #[serde(rename = "b")]
    pub b: Vec<PriceLevel>,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: Vec<PriceLevel>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `k` wire field.
    #[serde(rename = "k")]
    pub k: KlineEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineEventK {
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<super::enums::KlineInterval>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    /// Exact `L` wire field.
    #[serde(rename = "L", default, skip_serializing_if = "Option::is_none")]
    pub upper_l: Option<i64>,
    /// Exact `o` wire field.
    #[serde(
        rename = "o",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub o: Option<Decimal>,
    /// Exact `c` wire field.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    /// Exact `h` wire field.
    #[serde(
        rename = "h",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub h: Option<Decimal>,
    /// Exact `l` wire field.
    #[serde(
        rename = "l",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub l: Option<Decimal>,
    /// Exact `v` wire field.
    #[serde(
        rename = "v",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub v: Option<Decimal>,
    /// Exact `n` wire field.
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Exact `x` wire field.
    #[serde(rename = "x", default, skip_serializing_if = "Option::is_none")]
    pub x: Option<bool>,
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `V` wire field.
    #[serde(
        rename = "V",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_v: Option<Decimal>,
    /// Exact `Q` wire field.
    #[serde(
        rename = "Q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_q: Option<Decimal>,
    /// Exact `B` wire field.
    #[serde(
        rename = "B",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_b: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineOffsetEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineOffsetEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `k` wire field.
    #[serde(rename = "k")]
    pub k: KlineOffsetEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineOffsetEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineOffsetEventK {
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<super::enums::KlineInterval>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    /// Exact `L` wire field.
    #[serde(rename = "L", default, skip_serializing_if = "Option::is_none")]
    pub upper_l: Option<i64>,
    /// Exact `o` wire field.
    #[serde(
        rename = "o",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub o: Option<Decimal>,
    /// Exact `c` wire field.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    /// Exact `h` wire field.
    #[serde(
        rename = "h",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub h: Option<Decimal>,
    /// Exact `l` wire field.
    #[serde(
        rename = "l",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub l: Option<Decimal>,
    /// Exact `v` wire field.
    #[serde(
        rename = "v",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub v: Option<Decimal>,
    /// Exact `n` wire field.
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Exact `x` wire field.
    #[serde(rename = "x", default, skip_serializing_if = "Option::is_none")]
    pub x: Option<bool>,
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `V` wire field.
    #[serde(
        rename = "V",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_v: Option<Decimal>,
    /// Exact `Q` wire field.
    #[serde(
        rename = "Q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_q: Option<Decimal>,
    /// Exact `B` wire field.
    #[serde(
        rename = "B",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_b: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MiniTickerEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MiniTickerEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
    /// Exact `h` wire field.
    #[serde(rename = "h", deserialize_with = "super::wire::decimal")]
    pub h: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PartialBookDepthEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartialBookDepthEvent {
    /// Exact `lastUpdateId` wire field.
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    /// Exact `bids` wire field.
    #[serde(rename = "bids")]
    pub bids: Vec<Vec<serde_json::Value>>,
    /// Exact `asks` wire field.
    #[serde(rename = "asks")]
    pub asks: Vec<Vec<serde_json::Value>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ReferencePriceEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReferencePriceEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `r` wire field.
    #[serde(rename = "r", deserialize_with = "super::wire::decimal_option")]
    pub r: Option<Decimal>,
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `RollingWindowTickerEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RollingWindowTickerEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `P` wire field.
    #[serde(rename = "P", deserialize_with = "super::wire::decimal")]
    pub upper_p: Decimal,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
    /// Exact `h` wire field.
    #[serde(rename = "h", deserialize_with = "super::wire::decimal")]
    pub h: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `w` wire field.
    #[serde(rename = "w", deserialize_with = "super::wire::decimal")]
    pub w: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `O` wire field.
    #[serde(rename = "O")]
    pub upper_o: i64,
    /// Exact `C` wire field.
    #[serde(rename = "C")]
    pub upper_c: i64,
    /// Exact `F` wire field.
    #[serde(rename = "F")]
    pub upper_f: i64,
    /// Exact `L` wire field.
    #[serde(rename = "L")]
    pub upper_l: i64,
    /// Exact `n` wire field.
    #[serde(rename = "n")]
    pub n: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TickerEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `P` wire field.
    #[serde(rename = "P", deserialize_with = "super::wire::decimal")]
    pub upper_p: Decimal,
    /// Exact `w` wire field.
    #[serde(rename = "w", deserialize_with = "super::wire::decimal")]
    pub w: Decimal,
    /// Exact `x` wire field.
    #[serde(rename = "x", deserialize_with = "super::wire::decimal")]
    pub x: Decimal,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `Q` wire field.
    #[serde(rename = "Q", deserialize_with = "super::wire::decimal")]
    pub upper_q: Decimal,
    /// Exact `b` wire field.
    #[serde(rename = "b", deserialize_with = "super::wire::decimal")]
    pub b: Decimal,
    /// Exact `B` wire field.
    #[serde(rename = "B", deserialize_with = "super::wire::decimal")]
    pub upper_b: Decimal,
    /// Exact `a` wire field.
    #[serde(rename = "a", deserialize_with = "super::wire::decimal")]
    pub a: Decimal,
    /// Exact `A` wire field.
    #[serde(rename = "A", deserialize_with = "super::wire::decimal")]
    pub upper_a: Decimal,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
    /// Exact `h` wire field.
    #[serde(rename = "h", deserialize_with = "super::wire::decimal")]
    pub h: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `O` wire field.
    #[serde(rename = "O")]
    pub upper_o: i64,
    /// Exact `C` wire field.
    #[serde(rename = "C")]
    pub upper_c: i64,
    /// Exact `F` wire field.
    #[serde(rename = "F")]
    pub upper_f: i64,
    /// Exact `L` wire field.
    #[serde(rename = "L")]
    pub upper_l: i64,
    /// Exact `n` wire field.
    #[serde(rename = "n")]
    pub n: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `M` wire field.
    #[serde(rename = "M")]
    pub upper_m: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BlockTradeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BlockTradeEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BalanceUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BalanceUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: crate::Asset,
    /// Exact `d` wire field.
    #[serde(rename = "d", deserialize_with = "super::wire::decimal")]
    pub d: Decimal,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `EventStreamTerminatedEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventStreamTerminatedEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExecutionReportEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExecutionReportEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: ClientOrderId,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: super::enums::OrderSide,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: super::enums::OrderType,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<super::enums::TimeInForce>,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `P` wire field.
    #[serde(
        rename = "P",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_p: Option<Decimal>,
    /// Exact `F` wire field.
    #[serde(
        rename = "F",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_f: Option<Decimal>,
    /// Exact `g` wire field.
    #[serde(rename = "g", default, skip_serializing_if = "Option::is_none")]
    pub g: Option<i64>,
    /// Exact `C` wire field.
    #[serde(rename = "C", default, skip_serializing_if = "Option::is_none")]
    pub upper_c: Option<String>,
    /// Exact `x` wire field.
    #[serde(rename = "x")]
    pub x: super::enums::ExecutionType,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: super::enums::OrderStatus,
    /// Exact `r` wire field.
    #[serde(rename = "r", default, skip_serializing_if = "Option::is_none")]
    pub r: Option<String>,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: i64,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `z` wire field.
    #[serde(rename = "z", deserialize_with = "super::wire::decimal")]
    pub z: Decimal,
    /// Exact `L` wire field.
    #[serde(rename = "L", deserialize_with = "super::wire::decimal")]
    pub upper_l: Decimal,
    /// Exact `n` wire field.
    #[serde(rename = "n", deserialize_with = "super::wire::decimal")]
    pub n: Decimal,
    /// Exact `N` wire field.
    #[serde(rename = "N", default, skip_serializing_if = "Option::is_none")]
    pub upper_n: Option<crate::Asset>,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `v` wire field.
    #[serde(rename = "v", default, skip_serializing_if = "Option::is_none")]
    pub v: Option<i64>,
    /// Exact `I` wire field.
    #[serde(rename = "I", default, skip_serializing_if = "Option::is_none")]
    pub upper_i: Option<i64>,
    /// Exact `w` wire field.
    #[serde(rename = "w", default, skip_serializing_if = "Option::is_none")]
    pub w: Option<bool>,
    /// Exact `m` wire field.
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    /// Exact `M` wire field.
    #[serde(rename = "M", default, skip_serializing_if = "Option::is_none")]
    pub upper_m: Option<bool>,
    /// Exact `O` wire field.
    #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
    pub upper_o: Option<i64>,
    /// Exact `Z` wire field.
    #[serde(
        rename = "Z",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_z: Option<Decimal>,
    /// Exact `Y` wire field.
    #[serde(
        rename = "Y",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_y: Option<Decimal>,
    /// Exact `Q` wire field.
    #[serde(
        rename = "Q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_q: Option<Decimal>,
    /// Exact `W` wire field.
    #[serde(rename = "W", default, skip_serializing_if = "Option::is_none")]
    pub upper_w: Option<i64>,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `d` wire field.
    #[serde(rename = "d", default, skip_serializing_if = "Option::is_none")]
    pub d: Option<i64>,
    /// Exact `D` wire field.
    #[serde(rename = "D", default, skip_serializing_if = "Option::is_none")]
    pub upper_d: Option<i64>,
    /// Exact `j` wire field.
    #[serde(rename = "j", default, skip_serializing_if = "Option::is_none")]
    pub j: Option<i64>,
    /// Exact `J` wire field.
    #[serde(rename = "J", default, skip_serializing_if = "Option::is_none")]
    pub upper_j: Option<i64>,
    /// Exact `A` wire field.
    #[serde(
        rename = "A",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_a: Option<Decimal>,
    /// Exact `B` wire field.
    #[serde(
        rename = "B",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_b: Option<Decimal>,
    /// Exact `u` wire field.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    /// Exact `U` wire field.
    #[serde(rename = "U", default, skip_serializing_if = "Option::is_none")]
    pub upper_u: Option<i64>,
    /// Exact `Cs` wire field.
    #[serde(rename = "Cs", default, skip_serializing_if = "Option::is_none")]
    pub cs: Option<String>,
    /// Exact `pl` wire field.
    #[serde(
        rename = "pl",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pl: Option<Decimal>,
    /// Exact `pL` wire field.
    #[serde(
        rename = "pL",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p_l: Option<Decimal>,
    /// Exact `pY` wire field.
    #[serde(
        rename = "pY",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p_y: Option<Decimal>,
    /// Exact `b` wire field.
    #[serde(rename = "b", default, skip_serializing_if = "Option::is_none")]
    pub b: Option<super::enums::AllocationType>,
    /// Exact `a` wire field.
    #[serde(rename = "a", default, skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    /// Exact `k` wire field.
    #[serde(rename = "k", default, skip_serializing_if = "Option::is_none")]
    pub k: Option<super::enums::WorkingFloor>,
    /// Exact `uS` wire field.
    #[serde(rename = "uS", default, skip_serializing_if = "Option::is_none")]
    pub u_s: Option<bool>,
    /// Exact `gP` wire field.
    #[serde(rename = "gP", default, skip_serializing_if = "Option::is_none")]
    pub g_p: Option<String>,
    /// Exact `gOT` wire field.
    #[serde(rename = "gOT", default, skip_serializing_if = "Option::is_none")]
    pub g_ot: Option<String>,
    /// Exact `gOV` wire field.
    #[serde(rename = "gOV", default, skip_serializing_if = "Option::is_none")]
    pub g_ov: Option<i64>,
    /// Exact `gp` wire field.
    #[serde(
        rename = "gp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub gp: Option<Decimal>,
    /// Exact `eR` wire field.
    #[serde(rename = "eR", default, skip_serializing_if = "Option::is_none")]
    pub e_r: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExternalLockUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExternalLockUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: crate::Asset,
    /// Exact `d` wire field.
    #[serde(
        rename = "d",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub d: Option<Decimal>,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ListStatusEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListStatusEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `g` wire field.
    #[serde(rename = "g", default, skip_serializing_if = "Option::is_none")]
    pub g: Option<i64>,
    /// Exact `c` wire field.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<super::enums::ContingencyType>,
    /// Exact `l` wire field.
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    pub l: Option<super::enums::ListStatusType>,
    /// Exact `L` wire field.
    #[serde(rename = "L", default, skip_serializing_if = "Option::is_none")]
    pub upper_l: Option<super::enums::ListOrderStatus>,
    /// Exact `r` wire field.
    #[serde(rename = "r", default, skip_serializing_if = "Option::is_none")]
    pub r: Option<String>,
    /// Exact `C` wire field.
    #[serde(rename = "C", default, skip_serializing_if = "Option::is_none")]
    pub upper_c: Option<String>,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `O` wire field.
    #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
    pub upper_o: Option<Vec<ListStatusEventUpperOItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ListStatusEventUpperOItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListStatusEventUpperOItem {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<i64>,
    /// Exact `c` wire field.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<super::enums::ContingencyType>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OutboundAccountPositionEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OutboundAccountPositionEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `B` wire field.
    #[serde(rename = "B")]
    pub upper_b: Vec<OutboundAccountPositionEventUpperBItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OutboundAccountPositionEventUpperBItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OutboundAccountPositionEventUpperBItem {
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: crate::Asset,
    /// Exact `f` wire field.
    #[serde(rename = "f", deserialize_with = "super::wire::decimal")]
    pub f: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
