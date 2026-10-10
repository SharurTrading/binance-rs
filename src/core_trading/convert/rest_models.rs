// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::Decimal;
use serde::{Deserialize, Serialize};

/// Provider-native `ListAllConvertPairsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListAllConvertPairsResponseItem {
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset")]
    pub from_asset: crate::Asset,
    /// Exact `toAsset` wire field.
    #[serde(rename = "toAsset")]
    pub to_asset: crate::Asset,
    /// Exact `fromAssetMinAmount` wire field.
    #[serde(
        rename = "fromAssetMinAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_asset_min_amount: Option<Decimal>,
    /// Exact `fromAssetMaxAmount` wire field.
    #[serde(
        rename = "fromAssetMaxAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_asset_max_amount: Option<Decimal>,
    /// Exact `toAssetMinAmount` wire field.
    #[serde(
        rename = "toAssetMinAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_asset_min_amount: Option<Decimal>,
    /// Exact `toAssetMaxAmount` wire field.
    #[serde(
        rename = "toAssetMaxAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_asset_max_amount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `listAllConvertPairs`.
pub type ListAllConvertPairsResponse = Vec<ListAllConvertPairsResponseItem>;

/// Provider-native `QueryOrderQuantityPrecisionPerAssetResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryOrderQuantityPrecisionPerAssetResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `fraction` wire field.
    #[serde(rename = "fraction", default, skip_serializing_if = "Option::is_none")]
    pub fraction: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryOrderQuantityPrecisionPerAsset`.
pub type QueryOrderQuantityPrecisionPerAssetResponse =
    Vec<QueryOrderQuantityPrecisionPerAssetResponseItem>;

/// Provider-native `AcceptQuoteResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcceptQuoteResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: super::AcceptanceOrderId,
    /// Exact `createTime` wire field.
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Exact `orderStatus` wire field.
    #[serde(rename = "orderStatus")]
    pub order_status: super::enums::OrderStatus,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelLimitOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelLimitOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: super::OrderId,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetConvertTradeHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetConvertTradeHistoryResponse {
    /// Exact `list` wire field.
    #[serde(rename = "list", default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<GetConvertTradeHistoryResponseListItem>>,
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `limit` wire field.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Exact `moreData` wire field.
    #[serde(rename = "moreData", default, skip_serializing_if = "Option::is_none")]
    pub more_data: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetConvertTradeHistoryResponseListItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetConvertTradeHistoryResponseListItem {
    /// Exact `quoteId` wire field.
    #[serde(rename = "quoteId", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<super::QuoteId>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: super::OrderId,
    /// Exact `orderStatus` wire field.
    #[serde(
        rename = "orderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_status: Option<super::enums::OrderStatus>,
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset")]
    pub from_asset: crate::Asset,
    /// Exact `fromAmount` wire field.
    #[serde(rename = "fromAmount", deserialize_with = "super::wire::decimal")]
    pub from_amount: Decimal,
    /// Exact `toAsset` wire field.
    #[serde(rename = "toAsset")]
    pub to_asset: crate::Asset,
    /// Exact `toAmount` wire field.
    #[serde(rename = "toAmount", deserialize_with = "super::wire::decimal")]
    pub to_amount: Decimal,
    /// Exact `ratio` wire field.
    #[serde(
        rename = "ratio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ratio: Option<Decimal>,
    /// Exact `inverseRatio` wire field.
    #[serde(
        rename = "inverseRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub inverse_ratio: Option<Decimal>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderStatusResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: super::OrderId,
    /// Exact `orderStatus` wire field.
    #[serde(rename = "orderStatus")]
    pub order_status: super::enums::OrderStatus,
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset")]
    pub from_asset: crate::Asset,
    /// Exact `fromAmount` wire field.
    #[serde(rename = "fromAmount", deserialize_with = "super::wire::decimal")]
    pub from_amount: Decimal,
    /// Exact `toAsset` wire field.
    #[serde(rename = "toAsset")]
    pub to_asset: crate::Asset,
    /// Exact `toAmount` wire field.
    #[serde(rename = "toAmount", deserialize_with = "super::wire::decimal")]
    pub to_amount: Decimal,
    /// Exact `ratio` wire field.
    #[serde(rename = "ratio", deserialize_with = "super::wire::decimal")]
    pub ratio: Decimal,
    /// Exact `inverseRatio` wire field.
    #[serde(rename = "inverseRatio", deserialize_with = "super::wire::decimal")]
    pub inverse_ratio: Decimal,
    /// Exact `createTime` wire field.
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PlaceLimitOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PlaceLimitOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: super::OrderId,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryLimitOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryLimitOpenOrdersResponse {
    /// Exact `list` wire field.
    #[serde(rename = "list", default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<QueryLimitOpenOrdersResponseListItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryLimitOpenOrdersResponseListItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryLimitOpenOrdersResponseListItem {
    /// Exact `quoteId` wire field.
    #[serde(rename = "quoteId", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<super::QuoteId>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: super::OrderId,
    /// Exact `orderStatus` wire field.
    #[serde(
        rename = "orderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_status: Option<super::enums::OrderStatus>,
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset")]
    pub from_asset: crate::Asset,
    /// Exact `fromAmount` wire field.
    #[serde(rename = "fromAmount", deserialize_with = "super::wire::decimal")]
    pub from_amount: Decimal,
    /// Exact `toAsset` wire field.
    #[serde(rename = "toAsset")]
    pub to_asset: crate::Asset,
    /// Exact `toAmount` wire field.
    #[serde(rename = "toAmount", deserialize_with = "super::wire::decimal")]
    pub to_amount: Decimal,
    /// Exact `ratio` wire field.
    #[serde(
        rename = "ratio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ratio: Option<Decimal>,
    /// Exact `inverseRatio` wire field.
    #[serde(
        rename = "inverseRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub inverse_ratio: Option<Decimal>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `expiredTimestamp` wire field.
    #[serde(
        rename = "expiredTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expired_timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SendQuoteRequestResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SendQuoteRequestResponse {
    /// Exact `quoteId` wire field.
    #[serde(rename = "quoteId", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<super::QuoteId>,
    /// Exact `ratio` wire field.
    #[serde(
        rename = "ratio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ratio: Option<Decimal>,
    /// Exact `inverseRatio` wire field.
    #[serde(
        rename = "inverseRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub inverse_ratio: Option<Decimal>,
    /// Exact `validTimestamp` wire field.
    #[serde(
        rename = "validTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub valid_timestamp: Option<i64>,
    /// Exact `toAmount` wire field.
    #[serde(rename = "toAmount", deserialize_with = "super::wire::decimal")]
    pub to_amount: Decimal,
    /// Exact `fromAmount` wire field.
    #[serde(rename = "fromAmount", deserialize_with = "super::wire::decimal")]
    pub from_amount: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
