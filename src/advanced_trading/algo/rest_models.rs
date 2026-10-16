// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::Decimal;
use crate::SensitiveString;
use crate::Symbol;
use serde::{Deserialize, Serialize};

/// Provider-native `CancelAlgoOrderFutureAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAlgoOrderFutureAlgoResponse {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCurrentAlgoOpenOrdersFutureAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentAlgoOpenOrdersFutureAlgoResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryCurrentAlgoOpenOrdersFutureAlgoResponseOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCurrentAlgoOpenOrdersFutureAlgoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentAlgoOpenOrdersFutureAlgoResponseOrdersItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `totalQty` wire field.
    #[serde(
        rename = "totalQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `urgency` wire field.
    #[serde(rename = "urgency", default, skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryHistoricalAlgoOrdersFutureAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryHistoricalAlgoOrdersFutureAlgoResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryHistoricalAlgoOrdersFutureAlgoResponseOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryHistoricalAlgoOrdersFutureAlgoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryHistoricalAlgoOrdersFutureAlgoResponseOrdersItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `totalQty` wire field.
    #[serde(
        rename = "totalQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `urgency` wire field.
    #[serde(rename = "urgency", default, skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QuerySubOrdersFutureAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySubOrdersFutureAlgoResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `subOrders` wire field.
    #[serde(rename = "subOrders")]
    pub sub_orders: Vec<QuerySubOrdersFutureAlgoResponseSubOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QuerySubOrdersFutureAlgoResponseSubOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySubOrdersFutureAlgoResponseSubOrdersItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Exact `orderStatus` wire field.
    #[serde(
        rename = "orderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_status: Option<String>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `feeAmt` wire field.
    #[serde(
        rename = "feeAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee_amt: Option<Decimal>,
    /// Exact `feeAsset` wire field.
    #[serde(rename = "feeAsset", default, skip_serializing_if = "Option::is_none")]
    pub fee_asset: Option<crate::Asset>,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `subId` wire field.
    #[serde(rename = "subId", default, skip_serializing_if = "Option::is_none")]
    pub sub_id: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TimeWeightedAveragePriceFutureAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TimeWeightedAveragePriceFutureAlgoResponse {
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `VolumeParticipationFutureAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VolumeParticipationFutureAlgoResponse {
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelAlgoOrderSpotAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAlgoOrderSpotAlgoResponse {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCurrentAlgoOpenOrdersSpotAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentAlgoOpenOrdersSpotAlgoResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryCurrentAlgoOpenOrdersSpotAlgoResponseOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCurrentAlgoOpenOrdersSpotAlgoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentAlgoOpenOrdersSpotAlgoResponseOrdersItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `totalQty` wire field.
    #[serde(
        rename = "totalQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `urgency` wire field.
    #[serde(rename = "urgency", default, skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
    /// Exact `duration` wire field.
    #[serde(rename = "duration", default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryHistoricalAlgoOrdersSpotAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryHistoricalAlgoOrdersSpotAlgoResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryHistoricalAlgoOrdersSpotAlgoResponseOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryHistoricalAlgoOrdersSpotAlgoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryHistoricalAlgoOrdersSpotAlgoResponseOrdersItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `totalQty` wire field.
    #[serde(
        rename = "totalQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `urgency` wire field.
    #[serde(rename = "urgency", default, skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
    /// Exact `duration` wire field.
    #[serde(rename = "duration", default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QuerySubOrdersSpotAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySubOrdersSpotAlgoResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `subOrders` wire field.
    #[serde(rename = "subOrders")]
    pub sub_orders: Vec<QuerySubOrdersSpotAlgoResponseSubOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QuerySubOrdersSpotAlgoResponseSubOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySubOrdersSpotAlgoResponseSubOrdersItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Exact `orderStatus` wire field.
    #[serde(
        rename = "orderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_status: Option<String>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `executedAmt` wire field.
    #[serde(
        rename = "executedAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amt: Option<Decimal>,
    /// Exact `feeAmt` wire field.
    #[serde(
        rename = "feeAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee_amt: Option<Decimal>,
    /// Exact `feeAsset` wire field.
    #[serde(rename = "feeAsset", default, skip_serializing_if = "Option::is_none")]
    pub fee_asset: Option<crate::Asset>,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `subId` wire field.
    #[serde(rename = "subId", default, skip_serializing_if = "Option::is_none")]
    pub sub_id: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TimeWeightedAveragePriceSpotAlgoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TimeWeightedAveragePriceSpotAlgoResponse {
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `VolumeParticipationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VolumeParticipationResponse {
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: super::ClientAlgoId,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
