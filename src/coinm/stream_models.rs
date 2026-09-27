// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated market and user-data event payloads.

use super::wire::PriceLevel;
use crate::ClientOrderId;
use crate::Decimal;
use crate::SensitiveString;
use serde::{Deserialize, Serialize};

/// Provider-native `AggregateTradeStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AggregateTradeStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
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
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllBookTickersStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllBookTickersStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
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
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllMarketLiquidationOrderStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMarketLiquidationOrderStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: AllMarketLiquidationOrderStreamsEventO,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllMarketLiquidationOrderStreamsEventO` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMarketLiquidationOrderStreamsEventO {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<String>,
    /// Exact `o` wire field.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `ap` wire field.
    #[serde(
        rename = "ap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ap: Option<Decimal>,
    /// Exact `X` wire field.
    #[serde(rename = "X", default, skip_serializing_if = "Option::is_none")]
    pub upper_x: Option<String>,
    /// Exact `l` wire field.
    #[serde(
        rename = "l",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub l: Option<Decimal>,
    /// Exact `z` wire field.
    #[serde(
        rename = "z",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub z: Option<Decimal>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllMarketMiniTickersStreamEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMarketMiniTickersStreamEventItem {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
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
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllMarketTickersStreamsEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMarketTickersStreamsEventItem {
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
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `Q` wire field.
    #[serde(rename = "Q", deserialize_with = "super::wire::decimal")]
    pub upper_q: Decimal,
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
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ContinuousContractKlineCandlestickStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContinuousContractKlineCandlestickStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `ct` wire field.
    #[serde(rename = "ct")]
    pub ct: String,
    /// Exact `k` wire field.
    #[serde(rename = "k")]
    pub k: ContinuousContractKlineCandlestickStreamsEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ContinuousContractKlineCandlestickStreamsEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContinuousContractKlineCandlestickStreamsEventK {
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
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

/// Provider-native `ContractInfoStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContractInfoStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `ct` wire field.
    #[serde(rename = "ct")]
    pub ct: String,
    /// Exact `dt` wire field.
    #[serde(rename = "dt")]
    pub dt: i64,
    /// Exact `ot` wire field.
    #[serde(rename = "ot")]
    pub ot: i64,
    /// Exact `cs` wire field.
    #[serde(rename = "cs")]
    pub cs: String,
    /// Exact `bks` wire field.
    #[serde(rename = "bks")]
    pub bks: Vec<ContractInfoStreamEventBksItem>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ContractInfoStreamEventBksItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContractInfoStreamEventBksItem {
    /// Exact `bs` wire field.
    #[serde(rename = "bs", default, skip_serializing_if = "Option::is_none")]
    pub bs: Option<i64>,
    /// Exact `bnf` wire field.
    #[serde(rename = "bnf", default, skip_serializing_if = "Option::is_none")]
    pub bnf: Option<i64>,
    /// Exact `bnc` wire field.
    #[serde(rename = "bnc", default, skip_serializing_if = "Option::is_none")]
    pub bnc: Option<i64>,
    /// Exact `mmr` wire field.
    #[serde(
        rename = "mmr",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mmr: Option<Decimal>,
    /// Exact `cf` wire field.
    #[serde(rename = "cf", default, skip_serializing_if = "Option::is_none")]
    pub cf: Option<i64>,
    /// Exact `mi` wire field.
    #[serde(rename = "mi", default, skip_serializing_if = "Option::is_none")]
    pub mi: Option<i64>,
    /// Exact `ma` wire field.
    #[serde(rename = "ma", default, skip_serializing_if = "Option::is_none")]
    pub ma: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DiffBookDepthStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DiffBookDepthStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `U` wire field.
    #[serde(rename = "U")]
    pub upper_u: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `pu` wire field.
    #[serde(rename = "pu")]
    pub pu: i64,
    /// Exact `b` wire field.
    #[serde(rename = "b")]
    pub b: Vec<PriceLevel>,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: Vec<PriceLevel>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `IndexKlineCandlestickStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndexKlineCandlestickStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `k` wire field.
    #[serde(rename = "k")]
    pub k: IndexKlineCandlestickStreamsEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `IndexKlineCandlestickStreamsEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndexKlineCandlestickStreamsEventK {
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
    pub i: Option<String>,
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

/// Provider-native `IndexPriceStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndexPriceStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: crate::Symbol,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `IndividualSymbolBookTickerStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndividualSymbolBookTickerStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
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
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `IndividualSymbolMiniTickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndividualSymbolMiniTickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
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
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `IndividualSymbolTickerStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndividualSymbolTickerStreamsEvent {
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
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `Q` wire field.
    #[serde(rename = "Q", deserialize_with = "super::wire::decimal")]
    pub upper_q: Decimal,
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
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineCandlestickStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineCandlestickStreamsEvent {
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
    pub k: KlineCandlestickStreamsEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlineCandlestickStreamsEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlineCandlestickStreamsEventK {
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
    pub i: Option<String>,
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

/// Provider-native `MarketLiquidationOrderStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketLiquidationOrderStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: MarketLiquidationOrderStreamsEventO,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarketLiquidationOrderStreamsEventO` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketLiquidationOrderStreamsEventO {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<String>,
    /// Exact `o` wire field.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `ap` wire field.
    #[serde(
        rename = "ap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ap: Option<Decimal>,
    /// Exact `X` wire field.
    #[serde(rename = "X", default, skip_serializing_if = "Option::is_none")]
    pub upper_x: Option<String>,
    /// Exact `l` wire field.
    #[serde(
        rename = "l",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub l: Option<Decimal>,
    /// Exact `z` wire field.
    #[serde(
        rename = "z",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub z: Option<Decimal>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarkPriceKlineCandlestickStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarkPriceKlineCandlestickStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `k` wire field.
    #[serde(rename = "k")]
    pub k: MarkPriceKlineCandlestickStreamsEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarkPriceKlineCandlestickStreamsEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarkPriceKlineCandlestickStreamsEventK {
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
    pub i: Option<String>,
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

/// Provider-native `MarkPriceOfAllSymbolsOfAPairEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarkPriceOfAllSymbolsOfAPairEventItem {
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
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: String,
    /// Exact `r` wire field.
    #[serde(rename = "r")]
    pub r: String,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarkPriceStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarkPriceStreamEvent {
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
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: String,
    /// Exact `r` wire field.
    #[serde(rename = "r")]
    pub r: String,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PartialBookDepthStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartialBookDepthStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `U` wire field.
    #[serde(rename = "U")]
    pub upper_u: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `pu` wire field.
    #[serde(rename = "pu")]
    pub pu: i64,
    /// Exact `b` wire field.
    #[serde(rename = "b")]
    pub b: Vec<Vec<Decimal>>,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: Vec<Vec<Decimal>>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountConfigUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountConfigUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `ac` wire field.
    #[serde(rename = "ac", default, skip_serializing_if = "Option::is_none")]
    pub ac: Option<AccountConfigUpdateEventAc>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountConfigUpdateEventAc` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountConfigUpdateEventAc {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `l` wire field.
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    pub l: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: AccountUpdateEventA,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountUpdateEventA` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountUpdateEventA {
    /// Exact `m` wire field.
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    pub m: Option<String>,
    /// Exact `B` wire field.
    #[serde(rename = "B", default, skip_serializing_if = "Option::is_none")]
    pub upper_b: Option<Vec<AccountUpdateEventAUpperBItem>>,
    /// Exact `P` wire field.
    #[serde(rename = "P", default, skip_serializing_if = "Option::is_none")]
    pub upper_p: Option<Vec<AccountUpdateEventAUpperPItem>>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountUpdateEventAUpperBItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountUpdateEventAUpperBItem {
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: crate::Asset,
    /// Exact `wb` wire field.
    #[serde(rename = "wb", deserialize_with = "super::wire::decimal")]
    pub wb: Decimal,
    /// Exact `cw` wire field.
    #[serde(rename = "cw", deserialize_with = "super::wire::decimal")]
    pub cw: Decimal,
    /// Exact `bc` wire field.
    #[serde(rename = "bc", deserialize_with = "super::wire::decimal")]
    pub bc: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountUpdateEventAUpperPItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountUpdateEventAUpperPItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `pa` wire field.
    #[serde(rename = "pa", deserialize_with = "super::wire::decimal")]
    pub pa: Decimal,
    /// Exact `ep` wire field.
    #[serde(rename = "ep", deserialize_with = "super::wire::decimal")]
    pub ep: Decimal,
    /// Exact `bep` wire field.
    #[serde(rename = "bep", default, skip_serializing_if = "Option::is_none")]
    pub bep: Option<String>,
    /// Exact `cr` wire field.
    #[serde(rename = "cr", deserialize_with = "super::wire::decimal")]
    pub cr: Decimal,
    /// Exact `up` wire field.
    #[serde(rename = "up", deserialize_with = "super::wire::decimal")]
    pub up: Decimal,
    /// Exact `mt` wire field.
    #[serde(rename = "mt")]
    pub mt: String,
    /// Exact `iw` wire field.
    #[serde(rename = "iw", deserialize_with = "super::wire::decimal")]
    pub iw: Decimal,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GridUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GridUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `gu` wire field.
    #[serde(rename = "gu", default, skip_serializing_if = "Option::is_none")]
    pub gu: Option<GridUpdateEventGu>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GridUpdateEventGu` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GridUpdateEventGu {
    /// Exact `si` wire field.
    #[serde(rename = "si", default, skip_serializing_if = "Option::is_none")]
    pub si: Option<i64>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<String>,
    /// Exact `ss` wire field.
    #[serde(rename = "ss", default, skip_serializing_if = "Option::is_none")]
    pub ss: Option<String>,
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `r` wire field.
    #[serde(
        rename = "r",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r: Option<Decimal>,
    /// Exact `up` wire field.
    #[serde(
        rename = "up",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub up: Option<Decimal>,
    /// Exact `uq` wire field.
    #[serde(
        rename = "uq",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uq: Option<Decimal>,
    /// Exact `uf` wire field.
    #[serde(
        rename = "uf",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uf: Option<Decimal>,
    /// Exact `mp` wire field.
    #[serde(
        rename = "mp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mp: Option<Decimal>,
    /// Exact `ut` wire field.
    #[serde(rename = "ut", default, skip_serializing_if = "Option::is_none")]
    pub ut: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ListenKeyExpiredEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListenKeyExpiredEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `listenKey` wire field.
    #[serde(rename = "listenKey", default, skip_serializing_if = "Option::is_none")]
    pub listen_key: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginCallEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginCallEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    /// Exact `cw` wire field.
    #[serde(
        rename = "cw",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cw: Option<Decimal>,
    /// Exact `p` wire field.
    #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
    pub p: Option<Vec<MarginCallEventPItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginCallEventPItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginCallEventPItem {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `pa` wire field.
    #[serde(
        rename = "pa",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pa: Option<Decimal>,
    /// Exact `mt` wire field.
    #[serde(rename = "mt", default, skip_serializing_if = "Option::is_none")]
    pub mt: Option<String>,
    /// Exact `iw` wire field.
    #[serde(
        rename = "iw",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iw: Option<Decimal>,
    /// Exact `mp` wire field.
    #[serde(
        rename = "mp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mp: Option<Decimal>,
    /// Exact `up` wire field.
    #[serde(
        rename = "up",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub up: Option<Decimal>,
    /// Exact `mm` wire field.
    #[serde(
        rename = "mm",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mm: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderTradeUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTradeUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: OrderTradeUpdateEventO,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderTradeUpdateEventO` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTradeUpdateEventO {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: ClientOrderId,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: String,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: String,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `ap` wire field.
    #[serde(
        rename = "ap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ap: Option<Decimal>,
    /// Exact `sp` wire field.
    #[serde(
        rename = "sp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sp: Option<Decimal>,
    /// Exact `x` wire field.
    #[serde(rename = "x", default, skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: String,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: i64,
    /// Exact `M` wire field.
    #[serde(
        rename = "M",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_m: Option<Decimal>,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `z` wire field.
    #[serde(rename = "z", deserialize_with = "super::wire::decimal")]
    pub z: Decimal,
    /// Exact `L` wire field.
    #[serde(
        rename = "L",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_l: Option<Decimal>,
    /// Exact `ma` wire field.
    #[serde(rename = "ma", default, skip_serializing_if = "Option::is_none")]
    pub ma: Option<crate::Asset>,
    /// Exact `N` wire field.
    #[serde(rename = "N", default, skip_serializing_if = "Option::is_none")]
    pub upper_n: Option<crate::Asset>,
    /// Exact `n` wire field.
    #[serde(
        rename = "n",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub n: Option<Decimal>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `rp` wire field.
    #[serde(
        rename = "rp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rp: Option<Decimal>,
    /// Exact `b` wire field.
    #[serde(
        rename = "b",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub b: Option<Decimal>,
    /// Exact `a` wire field.
    #[serde(
        rename = "a",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub a: Option<Decimal>,
    /// Exact `m` wire field.
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    /// Exact `R` wire field.
    #[serde(rename = "R", default, skip_serializing_if = "Option::is_none")]
    pub upper_r: Option<bool>,
    /// Exact `wt` wire field.
    #[serde(rename = "wt", default, skip_serializing_if = "Option::is_none")]
    pub wt: Option<String>,
    /// Exact `ot` wire field.
    #[serde(rename = "ot", default, skip_serializing_if = "Option::is_none")]
    pub ot: Option<String>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `cp` wire field.
    #[serde(rename = "cp", default, skip_serializing_if = "Option::is_none")]
    pub cp: Option<bool>,
    /// Exact `AP` wire field.
    #[serde(
        rename = "AP",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_ap: Option<Decimal>,
    /// Exact `cr` wire field.
    #[serde(
        rename = "cr",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cr: Option<Decimal>,
    /// Exact `pP` wire field.
    #[serde(rename = "pP", default, skip_serializing_if = "Option::is_none")]
    pub p_p: Option<bool>,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<String>,
    /// Exact `pm` wire field.
    #[serde(rename = "pm", default, skip_serializing_if = "Option::is_none")]
    pub pm: Option<String>,
    /// Exact `er` wire field.
    #[serde(rename = "er", default, skip_serializing_if = "Option::is_none")]
    pub er: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `StrategyUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StrategyUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `su` wire field.
    #[serde(rename = "su", default, skip_serializing_if = "Option::is_none")]
    pub su: Option<StrategyUpdateEventSu>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `StrategyUpdateEventSu` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StrategyUpdateEventSu {
    /// Exact `si` wire field.
    #[serde(rename = "si", default, skip_serializing_if = "Option::is_none")]
    pub si: Option<i64>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<String>,
    /// Exact `ss` wire field.
    #[serde(rename = "ss", default, skip_serializing_if = "Option::is_none")]
    pub ss: Option<String>,
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<crate::Symbol>,
    /// Exact `ut` wire field.
    #[serde(rename = "ut", default, skip_serializing_if = "Option::is_none")]
    pub ut: Option<i64>,
    /// Exact `c` wire field.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
