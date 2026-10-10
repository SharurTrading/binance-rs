// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated market and user-data event payloads.

use super::wire::PriceLevel;
use crate::Decimal;
use crate::SensitiveString;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `a` wire field.
    #[serde(rename = "a", default, skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `nq` wire field.
    #[serde(
        rename = "nq",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub nq: Option<Decimal>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    /// Exact `l` wire field.
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    pub l: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `m` wire field.
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<AllMarketLiquidationOrderStreamsEventO>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AllMarketLiquidationOrderStreamsEventO` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllMarketLiquidationOrderStreamsEventO {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<super::enums::OrderSide>,
    /// Exact `o` wire field.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<super::enums::OrderType>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<super::enums::TimeInForce>,
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
    pub upper_x: Option<super::enums::OrderStatus>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: String,
    /// Exact `c` wire field.
    #[serde(
        rename = "c",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub c: Option<Decimal>,
    /// Exact `o` wire field.
    #[serde(
        rename = "o",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub o: Option<Decimal>,
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
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: String,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `P` wire field.
    #[serde(
        rename = "P",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_p: Option<Decimal>,
    /// Exact `w` wire field.
    #[serde(
        rename = "w",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub w: Option<Decimal>,
    /// Exact `c` wire field.
    #[serde(
        rename = "c",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub c: Option<Decimal>,
    /// Exact `Q` wire field.
    #[serde(
        rename = "Q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_q: Option<Decimal>,
    /// Exact `o` wire field.
    #[serde(
        rename = "o",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub o: Option<Decimal>,
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
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
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
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `CompositeIndexSymbolInformationStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompositeIndexSymbolInformationStreamsEvent {
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
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `C` wire field.
    #[serde(rename = "C", default, skip_serializing_if = "Option::is_none")]
    pub upper_c: Option<String>,
    /// Exact `c` wire field.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<Vec<CompositeIndexSymbolInformationStreamsEventCItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `CompositeIndexSymbolInformationStreamsEventCItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompositeIndexSymbolInformationStreamsEventCItem {
    /// Exact `b` wire field.
    #[serde(rename = "b", default, skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    /// Exact `q` wire field.
    #[serde(rename = "q", default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Exact `w` wire field.
    #[serde(
        rename = "w",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub w: Option<Decimal>,
    /// Exact `W` wire field.
    #[serde(
        rename = "W",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_w: Option<Decimal>,
    /// Exact `i` wire field.
    #[serde(
        rename = "i",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub i: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub ps: Option<String>,
    /// Exact `ct` wire field.
    #[serde(rename = "ct", default, skip_serializing_if = "Option::is_none")]
    pub ct: Option<super::enums::ContractType>,
    /// Exact `k` wire field.
    #[serde(rename = "k", default, skip_serializing_if = "Option::is_none")]
    pub k: Option<ContinuousContractKlineCandlestickStreamsEventK>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(
        rename = "c",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub c: Option<Decimal>,
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
    #[serde(rename = "B", default, skip_serializing_if = "Option::is_none")]
    pub upper_b: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: String,
    /// Exact `ct` wire field.
    #[serde(rename = "ct")]
    pub ct: super::enums::ContractType,
    /// Exact `dt` wire field.
    #[serde(rename = "dt")]
    pub dt: i64,
    /// Exact `ot` wire field.
    #[serde(rename = "ot")]
    pub ot: i64,
    /// Exact `cs` wire field.
    #[serde(rename = "cs")]
    pub cs: super::enums::ContractStatus,
    /// Exact `bks` wire field.
    #[serde(rename = "bks", default, skip_serializing_if = "Option::is_none")]
    pub bks: Option<Vec<ContractInfoStreamEventBksItem>>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: String,
    /// Exact `c` wire field.
    #[serde(
        rename = "c",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub c: Option<Decimal>,
    /// Exact `o` wire field.
    #[serde(
        rename = "o",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub o: Option<Decimal>,
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
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: String,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `P` wire field.
    #[serde(
        rename = "P",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_p: Option<Decimal>,
    /// Exact `w` wire field.
    #[serde(
        rename = "w",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub w: Option<Decimal>,
    /// Exact `c` wire field.
    #[serde(
        rename = "c",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub c: Option<Decimal>,
    /// Exact `Q` wire field.
    #[serde(
        rename = "Q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_q: Option<Decimal>,
    /// Exact `o` wire field.
    #[serde(
        rename = "o",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub o: Option<Decimal>,
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
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
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
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: String,
    /// Exact `k` wire field.
    #[serde(rename = "k", default, skip_serializing_if = "Option::is_none")]
    pub k: Option<KlineCandlestickStreamsEventK>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: Option<String>,
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
    #[serde(
        rename = "c",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub c: Option<Decimal>,
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
    #[serde(rename = "B", default, skip_serializing_if = "Option::is_none")]
    pub upper_b: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `LiquidationOrderStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LiquidationOrderStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `o` wire field.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<LiquidationOrderStreamsEventO>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `LiquidationOrderStreamsEventO` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LiquidationOrderStreamsEventO {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<super::enums::OrderSide>,
    /// Exact `o` wire field.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<super::enums::OrderType>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<super::enums::TimeInForce>,
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
    pub upper_x: Option<super::enums::OrderStatus>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub s: String,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `i` wire field.
    #[serde(
        rename = "i",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub i: Option<Decimal>,
    /// Exact `P` wire field.
    #[serde(
        rename = "P",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_p: Option<Decimal>,
    /// Exact `r` wire field.
    #[serde(
        rename = "r",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r: Option<Decimal>,
    /// Exact `ap` wire field.
    #[serde(
        rename = "ap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ap: Option<Decimal>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `MarkPriceStreamForAllMarketEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarkPriceStreamForAllMarketEventItem {
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
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `i` wire field.
    #[serde(
        rename = "i",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub i: Option<Decimal>,
    /// Exact `P` wire field.
    #[serde(
        rename = "P",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_p: Option<Decimal>,
    /// Exact `r` wire field.
    #[serde(
        rename = "r",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub r: Option<Decimal>,
    /// Exact `ap` wire field.
    #[serde(
        rename = "ap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ap: Option<Decimal>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AssetIndexEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetIndexEventItem {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `i` wire field.
    #[serde(
        rename = "i",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub i: Option<Decimal>,
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
    /// Exact `B` wire field.
    #[serde(
        rename = "B",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_b: Option<Decimal>,
    /// Exact `A` wire field.
    #[serde(
        rename = "A",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_a: Option<Decimal>,
    /// Exact `q` wire field.
    #[serde(
        rename = "q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub q: Option<Decimal>,
    /// Exact `g` wire field.
    #[serde(
        rename = "g",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub g: Option<Decimal>,
    /// Exact `Q` wire field.
    #[serde(
        rename = "Q",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_q: Option<Decimal>,
    /// Exact `G` wire field.
    #[serde(
        rename = "G",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_g: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingSessionStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingSessionStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AllBookTickersStreamEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllBookTickersStreamEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `u` wire field.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `IndividualSymbolBookTickerStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndividualSymbolBookTickerStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `u` wire field.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
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
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    #[serde(rename = "b", deserialize_with = "super::wire::decimal_rows")]
    pub b: Vec<Vec<Decimal>>,
    /// Exact `a` wire field.
    #[serde(rename = "a", deserialize_with = "super::wire::decimal_rows")]
    pub a: Vec<Vec<Decimal>>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `RpiDiffBookDepthStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RpiDiffBookDepthStreamsEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `ai` wire field.
    #[serde(rename = "ai", default, skip_serializing_if = "Option::is_none")]
    pub ai: Option<AccountConfigUpdateEventAi>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountConfigUpdateEventAc` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountConfigUpdateEventAc {
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// Exact `l` wire field.
    #[serde(rename = "l", default, skip_serializing_if = "Option::is_none")]
    pub l: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountConfigUpdateEventAi` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountConfigUpdateEventAi {
    /// Exact `j` wire field.
    #[serde(rename = "j", default, skip_serializing_if = "Option::is_none")]
    pub j: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: AccountUpdateEventA,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountUpdateEventA` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountUpdateEventA {
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: super::enums::AccountUpdateReason,
    /// Exact `B` wire field.
    #[serde(rename = "B", default, skip_serializing_if = "Option::is_none")]
    pub upper_b: Option<Vec<AccountUpdateEventAUpperBItem>>,
    /// Exact `P` wire field.
    #[serde(rename = "P", default, skip_serializing_if = "Option::is_none")]
    pub upper_p: Option<Vec<AccountUpdateEventAUpperPItem>>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountUpdateEventAUpperBItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountUpdateEventAUpperBItem {
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: String,
    /// Exact `wb` wire field.
    #[serde(rename = "wb", deserialize_with = "super::wire::decimal")]
    pub wb: Decimal,
    /// Exact `cw` wire field.
    #[serde(rename = "cw", deserialize_with = "super::wire::decimal")]
    pub cw: Decimal,
    /// Exact `bc` wire field.
    #[serde(rename = "bc", deserialize_with = "super::wire::decimal")]
    pub bc: Decimal,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountUpdateEventAUpperPItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountUpdateEventAUpperPItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `pa` wire field.
    #[serde(rename = "pa", deserialize_with = "super::wire::decimal")]
    pub pa: Decimal,
    /// Exact `ep` wire field.
    #[serde(rename = "ep", deserialize_with = "super::wire::decimal")]
    pub ep: Decimal,
    /// Exact `bep` wire field.
    #[serde(
        rename = "bep",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bep: Option<Decimal>,
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
    #[serde(
        rename = "iw",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iw: Option<Decimal>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: super::enums::PositionSide,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AlgoUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AlgoUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: AlgoUpdateEventO,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AlgoUpdateEventO` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AlgoUpdateEventO {
    /// Exact `caid` wire field.
    #[serde(rename = "caid")]
    pub caid: String,
    /// Exact `aid` wire field.
    #[serde(rename = "aid")]
    pub aid: i64,
    /// Exact `at` wire field.
    #[serde(rename = "at", default, skip_serializing_if = "Option::is_none")]
    pub at: Option<super::enums::AlgoType>,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: super::enums::OrderType,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: super::enums::OrderSide,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: super::enums::PositionSide,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<super::enums::TimeInForce>,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: super::enums::AlgoStatus,
    /// Exact `ai` wire field.
    #[serde(rename = "ai", default, skip_serializing_if = "Option::is_none")]
    pub ai: Option<String>,
    /// Exact `ap` wire field.
    #[serde(
        rename = "ap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ap: Option<Decimal>,
    /// Exact `aq` wire field.
    #[serde(
        rename = "aq",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub aq: Option<Decimal>,
    /// Exact `act` wire field.
    #[serde(rename = "act", default, skip_serializing_if = "Option::is_none")]
    pub act: Option<super::enums::OrderType>,
    /// Exact `tp` wire field.
    #[serde(
        rename = "tp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tp: Option<Decimal>,
    /// Exact `p` wire field.
    #[serde(
        rename = "p",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub p: Option<Decimal>,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `wt` wire field.
    #[serde(rename = "wt", default, skip_serializing_if = "Option::is_none")]
    pub wt: Option<super::enums::WorkingType>,
    /// Exact `pm` wire field.
    #[serde(rename = "pm", default, skip_serializing_if = "Option::is_none")]
    pub pm: Option<super::enums::PriceMatch>,
    /// Exact `cp` wire field.
    #[serde(rename = "cp", default, skip_serializing_if = "Option::is_none")]
    pub cp: Option<bool>,
    /// Exact `pP` wire field.
    #[serde(rename = "pP", default, skip_serializing_if = "Option::is_none")]
    pub p_p: Option<bool>,
    /// Exact `R` wire field.
    #[serde(rename = "R", default, skip_serializing_if = "Option::is_none")]
    pub upper_r: Option<bool>,
    /// Exact `tt` wire field.
    #[serde(rename = "tt", default, skip_serializing_if = "Option::is_none")]
    pub tt: Option<i64>,
    /// Exact `gtd` wire field.
    #[serde(rename = "gtd", default, skip_serializing_if = "Option::is_none")]
    pub gtd: Option<i64>,
    /// Exact `rm` wire field.
    #[serde(rename = "rm", default, skip_serializing_if = "Option::is_none")]
    pub rm: Option<String>,
    /// Exact `ia` wire field.
    #[serde(rename = "ia", default, skip_serializing_if = "Option::is_none")]
    pub ia: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ConditionalOrderTriggerRejectEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConditionalOrderTriggerRejectEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `or` wire field.
    #[serde(rename = "or")]
    pub or: ConditionalOrderTriggerRejectEventOr,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ConditionalOrderTriggerRejectEventOr` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConditionalOrderTriggerRejectEventOr {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: i64,
    /// Exact `r` wire field.
    #[serde(rename = "r")]
    pub r: String,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(rename = "gu")]
    pub gu: GridUpdateEventGu,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `GridUpdateEventGu` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GridUpdateEventGu {
    /// Exact `si` wire field.
    #[serde(rename = "si")]
    pub si: i64,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<String>,
    /// Exact `ss` wire field.
    #[serde(rename = "ss")]
    pub ss: String,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `cw` wire field.
    #[serde(
        rename = "cw",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cw: Option<Decimal>,
    /// Exact `p` wire field.
    #[serde(rename = "p")]
    pub p: Vec<MarginCallEventPItem>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `MarginCallEventPItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginCallEventPItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: super::enums::PositionSide,
    /// Exact `pa` wire field.
    #[serde(rename = "pa", deserialize_with = "super::wire::decimal")]
    pub pa: Decimal,
    /// Exact `mt` wire field.
    #[serde(rename = "mt")]
    pub mt: String,
    /// Exact `iw` wire field.
    #[serde(
        rename = "iw",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iw: Option<Decimal>,
    /// Exact `mp` wire field.
    #[serde(rename = "mp", deserialize_with = "super::wire::decimal")]
    pub mp: Decimal,
    /// Exact `up` wire field.
    #[serde(rename = "up", deserialize_with = "super::wire::decimal")]
    pub up: Decimal,
    /// Exact `mm` wire field.
    #[serde(rename = "mm", deserialize_with = "super::wire::decimal")]
    pub mm: Decimal,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: OrderTradeUpdateEventO,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `OrderTradeUpdateEventO` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTradeUpdateEventO {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
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
    #[serde(rename = "x")]
    pub x: super::enums::ExecutionType,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: super::enums::OrderStatus,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: i64,
    /// Exact `M` wire field.
    #[serde(rename = "M", default, skip_serializing_if = "Option::is_none")]
    pub upper_m: Option<String>,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `z` wire field.
    #[serde(rename = "z", deserialize_with = "super::wire::decimal")]
    pub z: Decimal,
    /// Exact `L` wire field.
    #[serde(rename = "L", deserialize_with = "super::wire::decimal")]
    pub upper_l: Decimal,
    /// Exact `N` wire field.
    #[serde(rename = "N", default, skip_serializing_if = "Option::is_none")]
    pub upper_n: Option<String>,
    /// Exact `n` wire field.
    #[serde(
        rename = "n",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub n: Option<Decimal>,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
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
    pub wt: Option<super::enums::WorkingType>,
    /// Exact `ot` wire field.
    #[serde(rename = "ot", default, skip_serializing_if = "Option::is_none")]
    pub ot: Option<super::enums::OrderType>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: super::enums::PositionSide,
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
    /// Exact `si` wire field.
    #[serde(rename = "si", default, skip_serializing_if = "Option::is_none")]
    pub si: Option<i64>,
    /// Exact `ss` wire field.
    #[serde(rename = "ss", default, skip_serializing_if = "Option::is_none")]
    pub ss: Option<i64>,
    /// Exact `rp` wire field.
    #[serde(
        rename = "rp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rp: Option<Decimal>,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `pm` wire field.
    #[serde(rename = "pm", default, skip_serializing_if = "Option::is_none")]
    pub pm: Option<super::enums::PriceMatch>,
    /// Exact `gtd` wire field.
    #[serde(rename = "gtd", default, skip_serializing_if = "Option::is_none")]
    pub gtd: Option<i64>,
    /// Exact `er` wire field.
    #[serde(rename = "er", default, skip_serializing_if = "Option::is_none")]
    pub er: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(rename = "su")]
    pub su: StrategyUpdateEventSu,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `StrategyUpdateEventSu` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StrategyUpdateEventSu {
    /// Exact `si` wire field.
    #[serde(rename = "si")]
    pub si: i64,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<String>,
    /// Exact `ss` wire field.
    #[serde(rename = "ss")]
    pub ss: String,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `ut` wire field.
    #[serde(rename = "ut", default, skip_serializing_if = "Option::is_none")]
    pub ut: Option<i64>,
    /// Exact `c` wire field.
    #[serde(rename = "c", default, skip_serializing_if = "Option::is_none")]
    pub c: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradeLiteEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeLiteEvent {
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
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: super::enums::OrderSide,
    /// Exact `L` wire field.
    #[serde(rename = "L", deserialize_with = "super::wire::decimal")]
    pub upper_l: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: i64,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
