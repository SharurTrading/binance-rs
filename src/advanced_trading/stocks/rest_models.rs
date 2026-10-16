// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::ClientOrderId;
use crate::Decimal;
use crate::Symbol;
use serde::{Deserialize, Serialize};

/// Provider-native `ExchangeInfoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponse {
    /// Exact `timezone` wire field.
    #[serde(rename = "timezone", default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<ExchangeInfoResponseSymbolsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct ExchangeInfoResponseSymbolsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `tradability` wire field.
    #[serde(
        rename = "tradability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tradability: Option<String>,
    /// Exact `tradabilityUpdateTime` wire field.
    #[serde(
        rename = "tradabilityUpdateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tradability_update_time: Option<i64>,
    /// Exact `overnightSupported` wire field.
    #[serde(
        rename = "overnightSupported",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overnight_supported: Option<bool>,
    /// Exact `fractionable` wire field.
    #[serde(
        rename = "fractionable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fractionable: Option<bool>,
    /// Exact `fractionableEh` wire field.
    #[serde(
        rename = "fractionableEh",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fractionable_eh: Option<bool>,
    /// Exact `extendedSession` wire field.
    #[serde(
        rename = "extendedSession",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub extended_session: Option<bool>,
    /// Exact `maxNumOrders` wire field.
    #[serde(
        rename = "maxNumOrders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_num_orders: Option<i64>,
    /// Exact `stepSize` wire field.
    #[serde(
        rename = "stepSize",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub step_size: Option<Decimal>,
    /// Exact `multiplierUp` wire field.
    #[serde(
        rename = "multiplierUp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_up: Option<Decimal>,
    /// Exact `multiplierDown` wire field.
    #[serde(
        rename = "multiplierDown",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_down: Option<Decimal>,
    /// Exact `minQty` wire field.
    #[serde(
        rename = "minQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_qty: Option<Decimal>,
    /// Exact `maxQty` wire field.
    #[serde(
        rename = "maxQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_qty: Option<Decimal>,
    /// Exact `minNotional` wire field.
    #[serde(
        rename = "minNotional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_notional: Option<Decimal>,
    /// Exact `maxNotional` wire field.
    #[serde(
        rename = "maxNotional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_notional: Option<Decimal>,
    /// Exact `listingTime` wire field.
    #[serde(
        rename = "listingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_time: Option<i64>,
    /// Exact `delistingTime` wire field.
    #[serde(
        rename = "delistingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delisting_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenizedAssetsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenizedAssetsResponseItem {
    /// Exact `assetCode` wire field.
    #[serde(rename = "assetCode", default, skip_serializing_if = "Option::is_none")]
    pub asset_code: Option<String>,
    /// Exact `assetName` wire field.
    #[serde(rename = "assetName", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    /// Exact `underlyingEquitySymbol` wire field.
    #[serde(
        rename = "underlyingEquitySymbol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_equity_symbol: Option<Symbol>,
    /// Exact `multiplier` wire field.
    #[serde(
        rename = "multiplier",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier: Option<Decimal>,
    /// Exact `multiplierValid` wire field.
    #[serde(
        rename = "multiplierValid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_valid: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `tokenizedAssets`.
pub type TokenizedAssetsResponse = Vec<TokenizedAssetsResponseItem>;

/// Provider-native `LatestQuoteResponseValue` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LatestQuoteResponseValue {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `bidPrice` wire field.
    #[serde(rename = "bidPrice", deserialize_with = "super::wire::decimal")]
    pub bid_price: Decimal,
    /// Exact `askPrice` wire field.
    #[serde(rename = "askPrice", deserialize_with = "super::wire::decimal")]
    pub ask_price: Decimal,
    /// Exact `bidSize` wire field.
    #[serde(rename = "bidSize", deserialize_with = "super::wire::decimal")]
    pub bid_size: Decimal,
    /// Exact `askSize` wire field.
    #[serde(rename = "askSize", deserialize_with = "super::wire::decimal")]
    pub ask_size: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `latestQuote`.
pub type LatestQuoteResponse = Option<LatestQuoteResponseValue>;

/// Provider-native `PlaceEquityOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PlaceEquityOrderResponse {
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelEquityOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelEquityOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelAllEquityOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllEquityOrdersResponse {
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CurrentOpenOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CurrentOpenOrdersResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `quote` wire field.
    #[serde(rename = "quote", default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<crate::Asset>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `limitPrice` wire field.
    #[serde(
        rename = "limitPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_price: Option<Decimal>,
    /// Exact `avgFilledPrice` wire field.
    #[serde(
        rename = "avgFilledPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_filled_price: Option<Decimal>,
    /// Exact `qty` wire field.
    #[serde(
        rename = "qty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty: Option<Decimal>,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Exact `filledQty` wire field.
    #[serde(
        rename = "filledQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_qty: Option<Decimal>,
    /// Exact `filledTotal` wire field.
    #[serde(
        rename = "filledTotal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_total: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `session` wire field.
    #[serde(rename = "session", default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createdAt` wire field.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Exact `updatedAt` wire field.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `currentOpenOrders`.
pub type CurrentOpenOrdersResponse = Vec<CurrentOpenOrdersResponseItem>;

/// Provider-native `EquityOrderHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityOrderHistoryResponse {
    /// Exact `total` wire field.
    #[serde(
        rename = "total",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total: Option<Decimal>,
    /// Exact `page` wire field.
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Exact `size` wire field.
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<EquityOrderHistoryResponseRowsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `EquityOrderHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityOrderHistoryResponseRowsItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `quote` wire field.
    #[serde(rename = "quote", default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<crate::Asset>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `limitPrice` wire field.
    #[serde(
        rename = "limitPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_price: Option<Decimal>,
    /// Exact `avgFilledPrice` wire field.
    #[serde(
        rename = "avgFilledPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_filled_price: Option<Decimal>,
    /// Exact `qty` wire field.
    #[serde(
        rename = "qty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty: Option<Decimal>,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Exact `filledQty` wire field.
    #[serde(
        rename = "filledQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_qty: Option<Decimal>,
    /// Exact `filledTotal` wire field.
    #[serde(
        rename = "filledTotal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_total: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `session` wire field.
    #[serde(rename = "session", default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createdAt` wire field.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Exact `updatedAt` wire field.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `EquityOrderDetailResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityOrderDetailResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `quote` wire field.
    #[serde(rename = "quote", default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<crate::Asset>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `limitPrice` wire field.
    #[serde(
        rename = "limitPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_price: Option<Decimal>,
    /// Exact `avgFilledPrice` wire field.
    #[serde(
        rename = "avgFilledPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_filled_price: Option<Decimal>,
    /// Exact `qty` wire field.
    #[serde(
        rename = "qty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty: Option<Decimal>,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Exact `filledQty` wire field.
    #[serde(
        rename = "filledQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_qty: Option<Decimal>,
    /// Exact `filledTotal` wire field.
    #[serde(
        rename = "filledTotal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_total: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `session` wire field.
    #[serde(rename = "session", default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createdAt` wire field.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Exact `updatedAt` wire field.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Exact `trades` wire field.
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<EquityOrderDetailResponseTradesItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `EquityOrderDetailResponseTradesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityOrderDetailResponseTradesItem {
    /// Exact `executionId` wire field.
    #[serde(
        rename = "executionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_id: Option<String>,
    /// Exact `executionAt` wire field.
    #[serde(
        rename = "executionAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_at: Option<i64>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `qty` wire field.
    #[serde(
        rename = "qty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `EquityTradeHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityTradeHistoryResponse {
    /// Exact `total` wire field.
    #[serde(
        rename = "total",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total: Option<Decimal>,
    /// Exact `page` wire field.
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Exact `size` wire field.
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<EquityTradeHistoryResponseRowsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `EquityTradeHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EquityTradeHistoryResponseRowsItem {
    /// Exact `executionId` wire field.
    #[serde(
        rename = "executionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_id: Option<String>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `quote` wire field.
    #[serde(rename = "quote", default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<crate::Asset>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `qty` wire field.
    #[serde(
        rename = "qty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty: Option<Decimal>,
    /// Exact `total` wire field.
    #[serde(
        rename = "total",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total: Option<Decimal>,
    /// Exact `executionAt` wire field.
    #[serde(
        rename = "executionAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_at: Option<i64>,
    /// Exact `updatedAt` wire field.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenizedMintResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenizedMintResponse {
    /// Exact `issuerRequestId` wire field.
    #[serde(rename = "issuerRequestId")]
    pub issuer_request_id: String,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenizedRedeemResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenizedRedeemResponse {
    /// Exact `issuerRequestId` wire field.
    #[serde(rename = "issuerRequestId")]
    pub issuer_request_id: String,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenizedConvertStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenizedConvertStatusResponse {
    /// Exact `underlyingAsset` wire field.
    #[serde(
        rename = "underlyingAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_asset: Option<crate::Asset>,
    /// Exact `underlyingAssetAmount` wire field.
    #[serde(
        rename = "underlyingAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_asset_amount: Option<Decimal>,
    /// Exact `tokenizedAsset` wire field.
    #[serde(
        rename = "tokenizedAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tokenized_asset: Option<crate::Asset>,
    /// Exact `tokenizedAssetAmount` wire field.
    #[serde(
        rename = "tokenizedAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tokenized_asset_amount: Option<Decimal>,
    /// Exact `issuerRequestId` wire field.
    #[serde(
        rename = "issuerRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub issuer_request_id: Option<String>,
    /// Exact `convertType` wire field.
    #[serde(
        rename = "convertType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub convert_type: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createdAt` wire field.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Exact `updatedAt` wire field.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenizedConvertHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenizedConvertHistoryResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<TokenizedConvertHistoryResponseRowsItem>>,
    /// Exact `hasMore` wire field.
    #[serde(rename = "hasMore", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// Exact `nextLastId` wire field.
    #[serde(
        rename = "nextLastId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_last_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenizedConvertHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenizedConvertHistoryResponseRowsItem {
    /// Exact `underlyingAsset` wire field.
    #[serde(
        rename = "underlyingAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_asset: Option<crate::Asset>,
    /// Exact `underlyingAssetAmount` wire field.
    #[serde(
        rename = "underlyingAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_asset_amount: Option<Decimal>,
    /// Exact `tokenizedAsset` wire field.
    #[serde(
        rename = "tokenizedAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tokenized_asset: Option<crate::Asset>,
    /// Exact `tokenizedAssetAmount` wire field.
    #[serde(
        rename = "tokenizedAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tokenized_asset_amount: Option<Decimal>,
    /// Exact `issuerRequestId` wire field.
    #[serde(
        rename = "issuerRequestId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub issuer_request_id: Option<String>,
    /// Exact `convertType` wire field.
    #[serde(
        rename = "convertType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub convert_type: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createdAt` wire field.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Exact `updatedAt` wire field.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SignUsEquityDisclaimerResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SignUsEquityDisclaimerResponse {
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CreateRenewListenKeyResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateRenewListenKeyResponse {
    /// Exact `listenKey` wire field.
    #[serde(rename = "listenKey")]
    pub listen_key: crate::SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
