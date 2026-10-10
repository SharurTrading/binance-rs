// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated native Margin risk and execution payloads.

use crate::Decimal;
use crate::SensitiveString;
use serde::{Deserialize, Serialize};

/// Provider-native `MarginLevelStatusChangeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginLevelStatusChangeEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UserLiabilityChangeEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserLiabilityChangeEvent {
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
    #[serde(rename = "t")]
    pub t: String,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `i` wire field.
    #[serde(rename = "i", deserialize_with = "super::wire::decimal")]
    pub i: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider alternatives for `RiskDataStreamEventsEvent`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum RiskDataStreamEventsEvent {
    /// Wire alternative 1.
    Variant1(Box<RiskDataStreamEventsEventVariant1>),
    /// Wire alternative 2.
    Variant2(Box<RiskDataStreamEventsEventVariant2>),
}

/// Provider-native `RiskDataStreamEventsEventVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RiskDataStreamEventsEventVariant1 {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `RiskDataStreamEventsEventVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RiskDataStreamEventsEventVariant2 {
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
    #[serde(rename = "t")]
    pub t: String,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `i` wire field.
    #[serde(rename = "i", deserialize_with = "super::wire::decimal")]
    pub i: Decimal,
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

/// Provider-native `ExecutionReportEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
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
    pub c: crate::margin::ClientOrderId,
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
    /// Exact `F` wire field.
    #[serde(rename = "F", deserialize_with = "super::wire::decimal")]
    pub upper_f: Decimal,
    /// Exact `g` wire field.
    #[serde(rename = "g")]
    pub g: crate::margin::OrderListId,
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
    pub i: crate::margin::OrderId,
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
    pub t: crate::margin::TradeId,
    /// Exact `I` wire field.
    #[serde(rename = "I")]
    pub upper_i: crate::margin::RecordId,
    /// Exact `w` wire field.
    #[serde(rename = "w")]
    pub w: bool,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `M` wire field.
    #[serde(rename = "M")]
    pub upper_m: bool,
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
    /// Exact `W` wire field.
    #[serde(rename = "W", default, skip_serializing_if = "Option::is_none")]
    pub upper_w: Option<i64>,
    /// Exact `V` wire field.
    #[serde(rename = "V")]
    pub upper_v: String,
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
    /// Exact `v` wire field.
    #[serde(rename = "v", default, skip_serializing_if = "Option::is_none")]
    pub v: Option<i64>,
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
    pub cs: Option<crate::Symbol>,
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
    pub b: Option<String>,
    /// Exact `a` wire field.
    #[serde(rename = "a", default, skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    /// Exact `k` wire field.
    #[serde(rename = "k", default, skip_serializing_if = "Option::is_none")]
    pub k: Option<String>,
    /// Exact `uS` wire field.
    #[serde(rename = "uS", default, skip_serializing_if = "Option::is_none")]
    pub u_s: Option<bool>,
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
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `g` wire field.
    #[serde(rename = "g")]
    pub g: crate::margin::OrderListId,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `L` wire field.
    #[serde(rename = "L", deserialize_with = "super::wire::decimal")]
    pub upper_l: Decimal,
    /// Exact `r` wire field.
    #[serde(rename = "r")]
    pub r: String,
    /// Exact `C` wire field.
    #[serde(rename = "C")]
    pub upper_c: crate::margin::ClientOrderId,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `O` wire field.
    #[serde(rename = "O")]
    pub upper_o: Vec<ListStatusEventUpperOItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ListStatusEventUpperOItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListStatusEventUpperOItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: crate::margin::OrderId,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
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
    pub upper_e: String,
    /// Exact `listenKey` wire field.
    #[serde(rename = "listenKey")]
    pub listen_key: SensitiveString,
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

/// Provider alternatives for `TradeDataStreamEventsEvent`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum TradeDataStreamEventsEvent {
    /// Wire alternative 1.
    Variant1(Box<TradeDataStreamEventsEventVariant1>),
    /// Wire alternative 2.
    Variant2(Box<TradeDataStreamEventsEventVariant2>),
    /// Wire alternative 3.
    Variant3(Box<TradeDataStreamEventsEventVariant3>),
    /// Wire alternative 4.
    Variant4(Box<TradeDataStreamEventsEventVariant4>),
    /// Wire alternative 5.
    Variant5(Box<TradeDataStreamEventsEventVariant5>),
    /// Wire alternative 6.
    Variant6(Box<TradeDataStreamEventsEventVariant6>),
    /// Wire alternative 7.
    Variant7(Box<TradeDataStreamEventsEventVariant7>),
}

/// Provider-native `TradeDataStreamEventsEventVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant1 {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeDataStreamEventsEventVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant2 {
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
    #[serde(rename = "t")]
    pub t: String,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `i` wire field.
    #[serde(rename = "i", deserialize_with = "super::wire::decimal")]
    pub i: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeDataStreamEventsEventVariant3` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant3 {
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

/// Provider-native `TradeDataStreamEventsEventVariant4` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct TradeDataStreamEventsEventVariant4 {
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
    pub c: crate::margin::ClientOrderId,
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
    /// Exact `F` wire field.
    #[serde(rename = "F", deserialize_with = "super::wire::decimal")]
    pub upper_f: Decimal,
    /// Exact `g` wire field.
    #[serde(rename = "g")]
    pub g: crate::margin::OrderListId,
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
    pub i: crate::margin::OrderId,
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
    pub t: crate::margin::TradeId,
    /// Exact `I` wire field.
    #[serde(rename = "I")]
    pub upper_i: crate::margin::RecordId,
    /// Exact `w` wire field.
    #[serde(rename = "w")]
    pub w: bool,
    /// Exact `m` wire field.
    #[serde(rename = "m")]
    pub m: bool,
    /// Exact `M` wire field.
    #[serde(rename = "M")]
    pub upper_m: bool,
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
    /// Exact `W` wire field.
    #[serde(rename = "W", default, skip_serializing_if = "Option::is_none")]
    pub upper_w: Option<i64>,
    /// Exact `V` wire field.
    #[serde(rename = "V")]
    pub upper_v: String,
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
    /// Exact `v` wire field.
    #[serde(rename = "v", default, skip_serializing_if = "Option::is_none")]
    pub v: Option<i64>,
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
    pub cs: Option<crate::Symbol>,
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
    pub b: Option<String>,
    /// Exact `a` wire field.
    #[serde(rename = "a", default, skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    /// Exact `k` wire field.
    #[serde(rename = "k", default, skip_serializing_if = "Option::is_none")]
    pub k: Option<String>,
    /// Exact `uS` wire field.
    #[serde(rename = "uS", default, skip_serializing_if = "Option::is_none")]
    pub u_s: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeDataStreamEventsEventVariant5` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant5 {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::Symbol,
    /// Exact `g` wire field.
    #[serde(rename = "g")]
    pub g: crate::margin::OrderListId,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `l` wire field.
    #[serde(rename = "l", deserialize_with = "super::wire::decimal")]
    pub l: Decimal,
    /// Exact `L` wire field.
    #[serde(rename = "L", deserialize_with = "super::wire::decimal")]
    pub upper_l: Decimal,
    /// Exact `r` wire field.
    #[serde(rename = "r")]
    pub r: String,
    /// Exact `C` wire field.
    #[serde(rename = "C")]
    pub upper_c: crate::margin::ClientOrderId,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `O` wire field.
    #[serde(rename = "O")]
    pub upper_o: Vec<TradeDataStreamEventsEventVariant5UpperOItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeDataStreamEventsEventVariant5UpperOItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant5UpperOItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: String,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: crate::margin::OrderId,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeDataStreamEventsEventVariant6` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant6 {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: String,
    /// Exact `listenKey` wire field.
    #[serde(rename = "listenKey")]
    pub listen_key: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeDataStreamEventsEventVariant7` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant7 {
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
    pub upper_b: Vec<TradeDataStreamEventsEventVariant7UpperBItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeDataStreamEventsEventVariant7UpperBItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeDataStreamEventsEventVariant7UpperBItem {
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
