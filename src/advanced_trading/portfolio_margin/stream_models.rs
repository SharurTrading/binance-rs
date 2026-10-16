// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated native stream DTOs.

use crate::Decimal;
use serde::{Deserialize, Serialize};

/// Provider-native `AccountConfigUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountConfigUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `fs` wire field.
    #[serde(rename = "fs")]
    pub fs: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `ac` wire field.
    #[serde(rename = "ac")]
    pub ac: AccountConfigUpdateEventAc,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountConfigUpdateEventAc` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountConfigUpdateEventAc {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `l` wire field.
    #[serde(rename = "l")]
    pub l: i64,
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
    /// Exact `fs` wire field.
    #[serde(rename = "fs")]
    pub fs: String,
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
    #[serde(rename = "m")]
    pub m: String,
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
    pub s: String,
    /// Exact `pa` wire field.
    #[serde(rename = "pa", deserialize_with = "super::wire::decimal")]
    pub pa: Decimal,
    /// Exact `ep` wire field.
    #[serde(rename = "ep", deserialize_with = "super::wire::decimal")]
    pub ep: Decimal,
    /// Exact `cr` wire field.
    #[serde(rename = "cr", deserialize_with = "super::wire::decimal")]
    pub cr: Decimal,
    /// Exact `up` wire field.
    #[serde(rename = "up", deserialize_with = "super::wire::decimal")]
    pub up: Decimal,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: String,
    /// Exact `bep` wire field.
    #[serde(
        rename = "bep",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bep: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AlgoOrderUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AlgoOrderUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `fs` wire field.
    #[serde(rename = "fs")]
    pub fs: String,
    /// Exact `ao` wire field.
    #[serde(rename = "ao")]
    pub ao: AlgoOrderUpdateEventAo,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AlgoOrderUpdateEventAo` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct AlgoOrderUpdateEventAo {
    /// Exact `caid` wire field.
    #[serde(rename = "caid", default, skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    /// Exact `aid` wire field.
    #[serde(rename = "aid", default, skip_serializing_if = "Option::is_none")]
    pub aid: Option<i64>,
    /// Exact `at` wire field.
    #[serde(rename = "at", default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: String,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: String,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<String>,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: String,
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
    pub act: Option<String>,
    /// Exact `tp` wire field.
    #[serde(
        rename = "tp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tp: Option<Decimal>,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<String>,
    /// Exact `wt` wire field.
    #[serde(rename = "wt", default, skip_serializing_if = "Option::is_none")]
    pub wt: Option<String>,
    /// Exact `pm` wire field.
    #[serde(rename = "pm", default, skip_serializing_if = "Option::is_none")]
    pub pm: Option<String>,
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
    /// Exact `U` wire field.
    #[serde(rename = "U", default, skip_serializing_if = "Option::is_none")]
    pub upper_u: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ConditionalOrderTradeUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConditionalOrderTradeUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `fs` wire field.
    #[serde(rename = "fs")]
    pub fs: String,
    /// Exact `so` wire field.
    #[serde(rename = "so")]
    pub so: ConditionalOrderTradeUpdateEventSo,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ConditionalOrderTradeUpdateEventSo` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConditionalOrderTradeUpdateEventSo {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `si` wire field.
    #[serde(rename = "si")]
    pub si: i64,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: String,
    /// Exact `st` wire field.
    #[serde(rename = "st")]
    pub st: String,
    /// Exact `f` wire field.
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `sp` wire field.
    #[serde(
        rename = "sp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sp: Option<Decimal>,
    /// Exact `os` wire field.
    #[serde(rename = "os")]
    pub os: String,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `ut` wire field.
    #[serde(rename = "ut", default, skip_serializing_if = "Option::is_none")]
    pub ut: Option<i64>,
    /// Exact `R` wire field.
    #[serde(rename = "R", default, skip_serializing_if = "Option::is_none")]
    pub upper_r: Option<bool>,
    /// Exact `wt` wire field.
    #[serde(rename = "wt", default, skip_serializing_if = "Option::is_none")]
    pub wt: Option<String>,
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
    /// Exact `i` wire field.
    #[serde(rename = "i", default, skip_serializing_if = "Option::is_none")]
    pub i: Option<i64>,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<String>,
    /// Exact `gtd` wire field.
    #[serde(rename = "gtd", default, skip_serializing_if = "Option::is_none")]
    pub gtd: Option<i64>,
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
    pub s: String,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: String,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: String,
    /// Exact `f` wire field.
    #[serde(rename = "f")]
    pub f: String,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `P` wire field.
    #[serde(rename = "P", deserialize_with = "super::wire::decimal")]
    pub upper_p: Decimal,
    /// Exact `d` wire field.
    #[serde(rename = "d", default, skip_serializing_if = "Option::is_none")]
    pub d: Option<i64>,
    /// Exact `F` wire field.
    #[serde(rename = "F", deserialize_with = "super::wire::decimal")]
    pub upper_f: Decimal,
    /// Exact `g` wire field.
    #[serde(rename = "g")]
    pub g: i64,
    /// Exact `C` wire field.
    #[serde(rename = "C")]
    pub upper_c: String,
    /// Exact `x` wire field.
    #[serde(rename = "x")]
    pub x: String,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: String,
    /// Exact `r` wire field.
    #[serde(rename = "r")]
    pub r: String,
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
    #[serde(rename = "t")]
    pub t: i64,
    /// Exact `v` wire field.
    #[serde(rename = "v", default, skip_serializing_if = "Option::is_none")]
    pub v: Option<i64>,
    /// Exact `I` wire field.
    #[serde(rename = "I")]
    pub upper_i: i64,
    /// Exact `w` wire field.
    #[serde(rename = "w")]
    pub w: bool,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `O` wire field.
    #[serde(rename = "O")]
    pub upper_o: i64,
    /// Exact `Z` wire field.
    #[serde(rename = "Z", deserialize_with = "super::wire::decimal")]
    pub upper_z: Decimal,
    /// Exact `Y` wire field.
    #[serde(rename = "Y", deserialize_with = "super::wire::decimal")]
    pub upper_y: Decimal,
    /// Exact `Q` wire field.
    #[serde(rename = "Q", deserialize_with = "super::wire::decimal")]
    pub upper_q: Decimal,
    /// Exact `D` wire field.
    #[serde(rename = "D", default, skip_serializing_if = "Option::is_none")]
    pub upper_d: Option<i64>,
    /// Exact `j` wire field.
    #[serde(rename = "j", default, skip_serializing_if = "Option::is_none")]
    pub j: Option<i64>,
    /// Exact `J` wire field.
    #[serde(rename = "J", default, skip_serializing_if = "Option::is_none")]
    pub upper_j: Option<i64>,
    /// Exact `W` wire field.
    #[serde(rename = "W", default, skip_serializing_if = "Option::is_none")]
    pub upper_w: Option<i64>,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<String>,
    /// Exact `u` wire field.
    #[serde(rename = "u", default, skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    /// Exact `U` wire field.
    #[serde(rename = "U", default, skip_serializing_if = "Option::is_none")]
    pub upper_u: Option<i64>,
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
    /// Exact `eR` wire field.
    #[serde(rename = "eR", default, skip_serializing_if = "Option::is_none")]
    pub e_r: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `LiabilityChangeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LiabilityChangeEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: crate::Asset,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `i` wire field.
    #[serde(rename = "i", deserialize_with = "super::wire::decimal")]
    pub i: Decimal,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenOrderLossEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrderLossEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `O` wire field.
    #[serde(rename = "O")]
    pub upper_o: Vec<OpenOrderLossEventUpperOItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenOrderLossEventUpperOItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrderLossEventUpperOItem {
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: String,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
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
    /// Exact `fs` wire field.
    #[serde(rename = "fs")]
    pub fs: String,
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
    pub s: String,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: String,
    /// Exact `o` wire field.
    #[serde(rename = "o")]
    pub o: String,
    /// Exact `f` wire field.
    #[serde(rename = "f")]
    pub f: String,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `ap` wire field.
    #[serde(rename = "ap", deserialize_with = "super::wire::decimal")]
    pub ap: Decimal,
    /// Exact `sp` wire field.
    #[serde(rename = "sp", deserialize_with = "super::wire::decimal")]
    pub sp: Decimal,
    /// Exact `x` wire field.
    #[serde(rename = "x")]
    pub x: String,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: String,
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
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `t` wire field.
    #[serde(rename = "t")]
    pub t: i64,
    /// Exact `b` wire field.
    #[serde(rename = "b", deserialize_with = "super::wire::decimal")]
    pub b: Decimal,
    /// Exact `a` wire field.
    #[serde(rename = "a", deserialize_with = "super::wire::decimal")]
    pub a: Decimal,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `R` wire field.
    #[serde(rename = "R")]
    pub upper_r: bool,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: String,
    /// Exact `rp` wire field.
    #[serde(rename = "rp", deserialize_with = "super::wire::decimal")]
    pub rp: Decimal,
    /// Exact `st` wire field.
    #[serde(rename = "st", default, skip_serializing_if = "Option::is_none")]
    pub st: Option<String>,
    /// Exact `si` wire field.
    #[serde(rename = "si", default, skip_serializing_if = "Option::is_none")]
    pub si: Option<i64>,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<String>,
    /// Exact `gtd` wire field.
    #[serde(rename = "gtd", default, skip_serializing_if = "Option::is_none")]
    pub gtd: Option<i64>,
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
    /// Exact `U` wire field.
    #[serde(rename = "U", default, skip_serializing_if = "Option::is_none")]
    pub upper_u: Option<i64>,
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

/// Provider-native `RiskLevelChangeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RiskLevelChangeEvent {
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
