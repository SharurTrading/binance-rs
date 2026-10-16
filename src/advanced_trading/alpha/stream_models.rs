// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated native stream DTOs.

use super::wire::PriceLevel;
use crate::Decimal;
use serde::{Deserialize, Serialize};

/// Provider-native `AggregateTradeStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AggregateTradeStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: i64,
    /// Exact `f` wire field.
    #[serde(rename = "f")]
    pub f: i64,
    /// Exact `l` wire field.
    #[serde(rename = "l")]
    pub l: i64,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllBookTickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllBookTickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `u` wire field.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllMiniTickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMiniTickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `c` wire field.
    #[serde(rename = "c", deserialize_with = "super::wire::decimal")]
    pub c: Decimal,
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

/// Provider-native `AllTickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllTickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    #[serde(rename = "c", deserialize_with = "super::wire::decimal")]
    pub c: Decimal,
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
    #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
    pub upper_o: Option<i64>,
    /// Exact `C` wire field.
    #[serde(rename = "C", default, skip_serializing_if = "Option::is_none")]
    pub upper_c: Option<i64>,
    /// Exact `F` wire field.
    #[serde(rename = "F", default, skip_serializing_if = "Option::is_none")]
    pub upper_f: Option<i64>,
    /// Exact `L` wire field.
    #[serde(rename = "L", default, skip_serializing_if = "Option::is_none")]
    pub upper_l: Option<i64>,
    /// Exact `n` wire field.
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllTokens24hTickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllTokens24hTickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `d` wire field.
    #[serde(rename = "d")]
    pub d: Vec<AllTokens24hTickerStreamEventDItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllTokens24hTickerStreamEventDItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllTokens24hTickerStreamEventDItem {
    /// Exact `ca` wire field.
    #[serde(rename = "ca")]
    pub ca: String,
    /// Exact `cnt24` wire field.
    #[serde(rename = "cnt24", default, skip_serializing_if = "Option::is_none")]
    pub cnt24: Option<i64>,
    /// Exact `fdv` wire field.
    #[serde(
        rename = "fdv",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fdv: Option<Decimal>,
    /// Exact `hc` wire field.
    #[serde(rename = "hc", default, skip_serializing_if = "Option::is_none")]
    pub hc: Option<String>,
    /// Exact `liq` wire field.
    #[serde(
        rename = "liq",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub liq: Option<Decimal>,
    /// Exact `mc` wire field.
    #[serde(
        rename = "mc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mc: Option<Decimal>,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `pc24` wire field.
    #[serde(
        rename = "pc24",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pc24: Option<Decimal>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `vol24` wire field.
    #[serde(
        rename = "vol24",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub vol24: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BookTickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BookTickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `u` wire field.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PartialDepthStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartialDepthStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `U` wire field.
    #[serde(rename = "U")]
    pub upper_u: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `pu` wire field.
    #[serde(rename = "pu")]
    pub pu: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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

/// Provider-native `FullDepthStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FullDepthStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `U` wire field.
    #[serde(rename = "U")]
    pub upper_u: i64,
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: i64,
    /// Exact `pu` wire field.
    #[serde(rename = "pu")]
    pub pu: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: String,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    /// Exact `L` wire field.
    #[serde(rename = "L", default, skip_serializing_if = "Option::is_none")]
    pub upper_l: Option<i64>,
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
    #[serde(rename = "n")]
    pub n: i64,
    /// Exact `x` wire field.
    #[serde(rename = "x")]
    pub x: bool,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `V` wire field.
    #[serde(rename = "V", deserialize_with = "super::wire::decimal")]
    pub upper_v: Decimal,
    /// Exact `Q` wire field.
    #[serde(rename = "Q", deserialize_with = "super::wire::decimal")]
    pub upper_q: Decimal,
    /// Exact `B` wire field.
    #[serde(rename = "B", default, skip_serializing_if = "Option::is_none")]
    pub upper_b: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ContractKlineStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContractKlineStreamEvent {
    /// Exact `ca` wire field.
    #[serde(rename = "ca", default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<String>,
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `k` wire field.
    #[serde(rename = "k")]
    pub k: ContractKlineStreamEventK,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ContractKlineStreamEventK` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContractKlineStreamEventK {
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
    /// Exact `ot` wire field.
    #[serde(rename = "ot", default, skip_serializing_if = "Option::is_none")]
    pub ot: Option<i64>,
    /// Exact `ct` wire field.
    #[serde(rename = "ct", default, skip_serializing_if = "Option::is_none")]
    pub ct: Option<i64>,
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MiniTickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MiniTickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `c` wire field.
    #[serde(rename = "c", deserialize_with = "super::wire::decimal")]
    pub c: Decimal,
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

/// Provider-native `TickerStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    #[serde(rename = "c", deserialize_with = "super::wire::decimal")]
    pub c: Decimal,
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
    #[serde(rename = "O", default, skip_serializing_if = "Option::is_none")]
    pub upper_o: Option<i64>,
    /// Exact `C` wire field.
    #[serde(rename = "C", default, skip_serializing_if = "Option::is_none")]
    pub upper_c: Option<i64>,
    /// Exact `F` wire field.
    #[serde(rename = "F", default, skip_serializing_if = "Option::is_none")]
    pub upper_f: Option<i64>,
    /// Exact `L` wire field.
    #[serde(rename = "L", default, skip_serializing_if = "Option::is_none")]
    pub upper_l: Option<i64>,
    /// Exact `n` wire field.
    #[serde(rename = "n", default, skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeStreamEvent {
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
    pub s: String,
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
