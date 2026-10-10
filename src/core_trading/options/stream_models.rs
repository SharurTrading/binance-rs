// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated market and user-data event payloads.

use super::wire::PriceLevel;
use crate::Decimal;
use crate::SensitiveString;
use serde::{Deserialize, Serialize};

/// Provider-native `IndexPriceStreamsEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndexPriceStreamsEventItem {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::core_trading::options::Symbol,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
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
    pub s: crate::core_trading::options::Symbol,
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
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::core_trading::options::Symbol,
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
    #[serde(rename = "x", default, skip_serializing_if = "Option::is_none")]
    pub x: Option<bool>,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `V` wire field.
    #[serde(rename = "V", deserialize_with = "super::wire::decimal")]
    pub upper_v: Decimal,
    /// Exact `Q` wire field.
    #[serde(rename = "Q", deserialize_with = "super::wire::decimal")]
    pub upper_q: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OptionMarkPriceEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OptionMarkPriceEventItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::core_trading::options::Symbol,
    /// Exact `mp` wire field.
    #[serde(rename = "mp", deserialize_with = "super::wire::decimal")]
    pub mp: Decimal,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `i` wire field.
    #[serde(rename = "i", deserialize_with = "super::wire::decimal")]
    pub i: Decimal,
    /// Exact `P` wire field.
    #[serde(rename = "P", deserialize_with = "super::wire::decimal")]
    pub upper_p: Decimal,
    /// Exact `bo` wire field.
    #[serde(rename = "bo", deserialize_with = "super::wire::decimal")]
    pub bo: Decimal,
    /// Exact `ao` wire field.
    #[serde(rename = "ao", deserialize_with = "super::wire::decimal")]
    pub ao: Decimal,
    /// Exact `bq` wire field.
    #[serde(rename = "bq", deserialize_with = "super::wire::decimal")]
    pub bq: Decimal,
    /// Exact `aq` wire field.
    #[serde(rename = "aq", deserialize_with = "super::wire::decimal")]
    pub aq: Decimal,
    /// Exact `b` wire field.
    #[serde(rename = "b", deserialize_with = "super::wire::decimal")]
    pub b: Decimal,
    /// Exact `a` wire field.
    #[serde(rename = "a", deserialize_with = "super::wire::decimal")]
    pub a: Decimal,
    /// Exact `hl` wire field.
    #[serde(rename = "hl", deserialize_with = "super::wire::decimal")]
    pub hl: Decimal,
    /// Exact `ll` wire field.
    #[serde(rename = "ll", deserialize_with = "super::wire::decimal")]
    pub ll: Decimal,
    /// Exact `vo` wire field.
    #[serde(rename = "vo", deserialize_with = "super::wire::decimal")]
    pub vo: Decimal,
    /// Exact `rf` wire field.
    #[serde(rename = "rf", deserialize_with = "super::wire::decimal")]
    pub rf: Decimal,
    /// Exact `d` wire field.
    #[serde(rename = "d", deserialize_with = "super::wire::decimal")]
    pub d: Decimal,
    /// Exact `t` wire field.
    #[serde(rename = "t", deserialize_with = "super::wire::decimal")]
    pub t: Decimal,
    /// Exact `g` wire field.
    #[serde(rename = "g", deserialize_with = "super::wire::decimal")]
    pub g: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewSymbolInfoEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewSymbolInfoEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::core_trading::options::Symbol,
    /// Exact `ps` wire field.
    #[serde(rename = "ps")]
    pub ps: crate::Symbol,
    /// Exact `qa` wire field.
    #[serde(rename = "qa")]
    pub qa: crate::Asset,
    /// Exact `d` wire field.
    #[serde(rename = "d", default, skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    /// Exact `sp` wire field.
    #[serde(rename = "sp", deserialize_with = "super::wire::decimal")]
    pub sp: Decimal,
    /// Exact `dt` wire field.
    #[serde(rename = "dt", default, skip_serializing_if = "Option::is_none")]
    pub dt: Option<i64>,
    /// Exact `u` wire field.
    #[serde(rename = "u", deserialize_with = "super::wire::decimal")]
    pub u: Decimal,
    /// Exact `ot` wire field.
    #[serde(rename = "ot", default, skip_serializing_if = "Option::is_none")]
    pub ot: Option<i64>,
    /// Exact `cs` wire field.
    #[serde(rename = "cs", default, skip_serializing_if = "Option::is_none")]
    pub cs: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenInterestEventItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenInterestEventItem {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::core_trading::options::Symbol,
    /// Exact `o` wire field.
    #[serde(rename = "o", deserialize_with = "super::wire::decimal")]
    pub o: Decimal,
    /// Exact `h` wire field.
    #[serde(rename = "h", deserialize_with = "super::wire::decimal")]
    pub h: Decimal,
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
    pub s: crate::core_trading::options::Symbol,
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
    pub s: crate::core_trading::options::Symbol,
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
    pub s: crate::core_trading::options::Symbol,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `Hour24TickerEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Hour24TickerEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::core_trading::options::Symbol,
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

/// Provider-native `TradeStreamsEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeStreamsEvent {
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
    pub s: crate::core_trading::options::Symbol,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `q` wire field.
    #[serde(rename = "q", deserialize_with = "super::wire::decimal")]
    pub q: Decimal,
    /// Exact `X` wire field.
    #[serde(rename = "X", default, skip_serializing_if = "Option::is_none")]
    pub upper_x: Option<String>,
    /// Exact `S` wire field.
    #[serde(rename = "S", default, skip_serializing_if = "Option::is_none")]
    pub upper_s: Option<String>,
    /// Exact `m` wire field.
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
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
    /// Exact `eq` wire field.
    #[serde(rename = "eq", deserialize_with = "super::wire::decimal")]
    pub eq: Decimal,
    /// Exact `aeq` wire field.
    #[serde(rename = "aeq", deserialize_with = "super::wire::decimal")]
    pub aeq: Decimal,
    /// Exact `b` wire field.
    #[serde(rename = "b", deserialize_with = "super::wire::decimal")]
    pub b: Decimal,
    /// Exact `m` wire field.
    #[serde(rename = "m", deserialize_with = "super::wire::decimal")]
    pub m: Decimal,
    /// Exact `u` wire field.
    #[serde(rename = "u", deserialize_with = "super::wire::decimal")]
    pub u: Decimal,
    /// Exact `i` wire field.
    #[serde(rename = "i", deserialize_with = "super::wire::decimal")]
    pub i: Decimal,
    /// Exact `M` wire field.
    #[serde(rename = "M", deserialize_with = "super::wire::decimal")]
    pub upper_m: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BalancePositionUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BalancePositionUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `m` wire field.
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    pub m: Option<String>,
    /// Exact `B` wire field.
    #[serde(rename = "B")]
    pub upper_b: Vec<BalancePositionUpdateEventUpperBItem>,
    /// Exact `P` wire field.
    #[serde(rename = "P")]
    pub upper_p: Vec<BalancePositionUpdateEventUpperPItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BalancePositionUpdateEventUpperBItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BalancePositionUpdateEventUpperBItem {
    /// Exact `a` wire field.
    #[serde(rename = "a")]
    pub a: crate::Asset,
    /// Exact `b` wire field.
    #[serde(rename = "b", deserialize_with = "super::wire::decimal")]
    pub b: Decimal,
    /// Exact `bc` wire field.
    #[serde(rename = "bc", deserialize_with = "super::wire::decimal")]
    pub bc: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BalancePositionUpdateEventUpperPItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BalancePositionUpdateEventUpperPItem {
    /// Exact `s` wire field.
    #[serde(rename = "s")]
    pub s: crate::core_trading::options::Symbol,
    /// Exact `c` wire field.
    #[serde(rename = "c", deserialize_with = "super::wire::decimal")]
    pub c: Decimal,
    /// Exact `p` wire field.
    #[serde(rename = "p", deserialize_with = "super::wire::decimal")]
    pub p: Decimal,
    /// Exact `a` wire field.
    #[serde(rename = "a", deserialize_with = "super::wire::decimal")]
    pub a: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GreekUpdateEvent` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GreekUpdateEvent {
    /// Exact `e` wire field.
    #[serde(rename = "e")]
    pub e: String,
    /// Exact `E` wire field.
    #[serde(rename = "E")]
    pub upper_e: i64,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `G` wire field.
    #[serde(rename = "G")]
    pub upper_g: Vec<GreekUpdateEventUpperGItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GreekUpdateEventUpperGItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GreekUpdateEventUpperGItem {
    /// Exact `u` wire field.
    #[serde(rename = "u")]
    pub u: crate::Symbol,
    /// Exact `d` wire field.
    #[serde(rename = "d", deserialize_with = "super::wire::decimal")]
    pub d: Decimal,
    /// Exact `g` wire field.
    #[serde(rename = "g", deserialize_with = "super::wire::decimal")]
    pub g: Decimal,
    /// Exact `t` wire field.
    #[serde(rename = "t", deserialize_with = "super::wire::decimal")]
    pub t: Decimal,
    /// Exact `v` wire field.
    #[serde(rename = "v", deserialize_with = "super::wire::decimal")]
    pub v: Decimal,
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
    #[serde(rename = "listenKey", default, skip_serializing_if = "Option::is_none")]
    pub listen_key: Option<SensitiveString>,
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
    pub s: crate::core_trading::options::Symbol,
    /// Exact `c` wire field.
    #[serde(rename = "c")]
    pub c: String,
    /// Exact `S` wire field.
    #[serde(rename = "S")]
    pub upper_s: String,
    /// Exact `o` wire field.
    #[serde(rename = "o", default, skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
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
    #[serde(rename = "ap", deserialize_with = "super::wire::decimal")]
    pub ap: Decimal,
    /// Exact `x` wire field.
    #[serde(rename = "x")]
    pub x: String,
    /// Exact `X` wire field.
    #[serde(rename = "X")]
    pub upper_x: String,
    /// Exact `i` wire field.
    #[serde(rename = "i")]
    pub i: crate::core_trading::options::OrderId,
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
    #[serde(rename = "n", deserialize_with = "super::wire::decimal")]
    pub n: Decimal,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `t` wire field.
    #[serde(rename = "t", default, skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    /// Exact `b` wire field.
    #[serde(rename = "b", deserialize_with = "super::wire::decimal")]
    pub b: Decimal,
    /// Exact `a` wire field.
    #[serde(rename = "a", deserialize_with = "super::wire::decimal")]
    pub a: Decimal,
    /// Exact `m` wire field.
    #[serde(rename = "m", default, skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    /// Exact `R` wire field.
    #[serde(rename = "R", default, skip_serializing_if = "Option::is_none")]
    pub upper_r: Option<bool>,
    /// Exact `ot` wire field.
    #[serde(rename = "ot", default, skip_serializing_if = "Option::is_none")]
    pub ot: Option<String>,
    /// Exact `rp` wire field.
    #[serde(rename = "rp", deserialize_with = "super::wire::decimal")]
    pub rp: Decimal,
    /// Exact `V` wire field.
    #[serde(rename = "V", default, skip_serializing_if = "Option::is_none")]
    pub upper_v: Option<String>,
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
    /// Exact `s` wire field.
    #[serde(rename = "s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// Exact `mb` wire field.
    #[serde(rename = "mb", deserialize_with = "super::wire::decimal")]
    pub mb: Decimal,
    /// Exact `mm` wire field.
    #[serde(rename = "mm", deserialize_with = "super::wire::decimal")]
    pub mm: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
