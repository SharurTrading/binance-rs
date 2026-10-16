// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use super::wire::Kline;
use super::wire::PriceLevel;
use crate::Decimal;
use crate::Symbol;
use serde::{Deserialize, Serialize};

/// Provider-native `AggregatedTradesResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AggregatedTradesResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `messageDetail` wire field.
    #[serde(
        rename = "messageDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_detail: Option<String>,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: Vec<AggregatedTradesResponseDataItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AggregatedTradesResponseDataItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AggregatedTradesResponseDataItem {
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `FullDepthResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FullDepthResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `messageDetail` wire field.
    #[serde(
        rename = "messageDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_detail: Option<String>,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: FullDepthResponseData,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `FullDepthResponseData` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FullDepthResponseData {
    /// Exact `lastUpdateId` wire field.
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `bids` wire field.
    #[serde(rename = "bids")]
    pub bids: Vec<PriceLevel>,
    /// Exact `asks` wire field.
    #[serde(rename = "asks")]
    pub asks: Vec<PriceLevel>,
    /// Exact `E` wire field.
    #[serde(rename = "E", default, skip_serializing_if = "Option::is_none")]
    pub upper_e: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetExchangeInfoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetExchangeInfoResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `messageDetail` wire field.
    #[serde(
        rename = "messageDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_detail: Option<String>,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: GetExchangeInfoResponseData,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetExchangeInfoResponseData` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetExchangeInfoResponseData {
    /// Exact `timezone` wire field.
    #[serde(rename = "timezone", default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<GetExchangeInfoResponseDataAssetsItem>>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<GetExchangeInfoResponseDataSymbolsItem>>,
    /// Exact `orderTypes` wire field.
    #[serde(
        rename = "orderTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_types: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetExchangeInfoResponseDataAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetExchangeInfoResponseDataAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetExchangeInfoResponseDataSymbolsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetExchangeInfoResponseDataSymbolsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset")]
    pub base_asset: crate::Asset,
    /// Exact `quoteAsset` wire field.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: crate::Asset,
    /// Exact `pricePrecision` wire field.
    #[serde(
        rename = "pricePrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_precision: Option<i64>,
    /// Exact `quantityPrecision` wire field.
    #[serde(
        rename = "quantityPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_precision: Option<i64>,
    /// Exact `baseAssetPrecision` wire field.
    #[serde(
        rename = "baseAssetPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_asset_precision: Option<i64>,
    /// Exact `quotePrecision` wire field.
    #[serde(
        rename = "quotePrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_precision: Option<i64>,
    /// Exact `filters` wire field.
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GetExchangeInfoResponseDataSymbolsItemFiltersItem>>,
    /// Exact `orderTypes` wire field.
    #[serde(
        rename = "orderTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_types: Option<Vec<String>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetExchangeInfoResponseDataSymbolsItemFiltersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetExchangeInfoResponseDataSymbolsItemFiltersItem {
    /// Exact `filterType` wire field.
    #[serde(
        rename = "filterType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_type: Option<String>,
    /// Exact `minPrice` wire field.
    #[serde(
        rename = "minPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_price: Option<Decimal>,
    /// Exact `maxPrice` wire field.
    #[serde(
        rename = "maxPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_price: Option<Decimal>,
    /// Exact `tickSize` wire field.
    #[serde(
        rename = "tickSize",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tick_size: Option<Decimal>,
    /// Exact `stepSize` wire field.
    #[serde(
        rename = "stepSize",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub step_size: Option<Decimal>,
    /// Exact `maxQty` wire field.
    #[serde(
        rename = "maxQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_qty: Option<Decimal>,
    /// Exact `minQty` wire field.
    #[serde(
        rename = "minQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_qty: Option<Decimal>,
    /// Exact `limit` wire field.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
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
    /// Exact `multiplierDown` wire field.
    #[serde(
        rename = "multiplierDown",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_down: Option<Decimal>,
    /// Exact `multiplierUp` wire field.
    #[serde(
        rename = "multiplierUp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_up: Option<Decimal>,
    /// Exact `bidMultiplierUp` wire field.
    #[serde(
        rename = "bidMultiplierUp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_multiplier_up: Option<Decimal>,
    /// Exact `askMultiplierUp` wire field.
    #[serde(
        rename = "askMultiplierUp",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_multiplier_up: Option<Decimal>,
    /// Exact `bidMultiplierDown` wire field.
    #[serde(
        rename = "bidMultiplierDown",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_multiplier_down: Option<Decimal>,
    /// Exact `askMultiplierDown` wire field.
    #[serde(
        rename = "askMultiplierDown",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_multiplier_down: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `KlinesResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KlinesResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `messageDetail` wire field.
    #[serde(
        rename = "messageDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_detail: Option<String>,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: Vec<Kline>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TickerResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `messageDetail` wire field.
    #[serde(
        rename = "messageDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_detail: Option<String>,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: TickerResponseData,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TickerResponseData` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerResponseData {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `priceChange` wire field.
    #[serde(
        rename = "priceChange",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change: Option<Decimal>,
    /// Exact `priceChangePercent` wire field.
    #[serde(
        rename = "priceChangePercent",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change_percent: Option<Decimal>,
    /// Exact `weightedAvgPrice` wire field.
    #[serde(
        rename = "weightedAvgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weighted_avg_price: Option<Decimal>,
    /// Exact `lastPrice` wire field.
    #[serde(
        rename = "lastPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_price: Option<Decimal>,
    /// Exact `lastQty` wire field.
    #[serde(
        rename = "lastQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_qty: Option<Decimal>,
    /// Exact `openPrice` wire field.
    #[serde(
        rename = "openPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_price: Option<Decimal>,
    /// Exact `highPrice` wire field.
    #[serde(
        rename = "highPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_price: Option<Decimal>,
    /// Exact `lowPrice` wire field.
    #[serde(
        rename = "lowPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub low_price: Option<Decimal>,
    /// Exact `volume` wire field.
    #[serde(
        rename = "volume",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume: Option<Decimal>,
    /// Exact `quoteVolume` wire field.
    #[serde(
        rename = "quoteVolume",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_volume: Option<Decimal>,
    /// Exact `openTime` wire field.
    #[serde(rename = "openTime", default, skip_serializing_if = "Option::is_none")]
    pub open_time: Option<i64>,
    /// Exact `closeTime` wire field.
    #[serde(rename = "closeTime", default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<i64>,
    /// Exact `firstId` wire field.
    #[serde(rename = "firstId", default, skip_serializing_if = "Option::is_none")]
    pub first_id: Option<i64>,
    /// Exact `lastId` wire field.
    #[serde(rename = "lastId", default, skip_serializing_if = "Option::is_none")]
    pub last_id: Option<i64>,
    /// Exact `count` wire field.
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenListResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TokenListResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `messageDetail` wire field.
    #[serde(
        rename = "messageDetail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_detail: Option<String>,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: Vec<TokenListResponseDataItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TokenListResponseDataItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct TokenListResponseDataItem {
    /// Exact `tokenId` wire field.
    #[serde(rename = "tokenId", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// Exact `chainId` wire field.
    #[serde(rename = "chainId", default, skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    /// Exact `chainIconUrl` wire field.
    #[serde(
        rename = "chainIconUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chain_icon_url: Option<String>,
    /// Exact `chainName` wire field.
    #[serde(rename = "chainName", default, skip_serializing_if = "Option::is_none")]
    pub chain_name: Option<String>,
    /// Exact `contractAddress` wire field.
    #[serde(
        rename = "contractAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_address: Option<String>,
    /// Exact `name` wire field.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `iconUrl` wire field.
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `percentChange24h` wire field.
    #[serde(
        rename = "percentChange24h",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub percent_change24h: Option<Decimal>,
    /// Exact `volume24h` wire field.
    #[serde(
        rename = "volume24h",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume24h: Option<Decimal>,
    /// Exact `marketCap` wire field.
    #[serde(
        rename = "marketCap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub market_cap: Option<Decimal>,
    /// Exact `fdv` wire field.
    #[serde(
        rename = "fdv",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fdv: Option<Decimal>,
    /// Exact `liquidity` wire field.
    #[serde(
        rename = "liquidity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidity: Option<Decimal>,
    /// Exact `totalSupply` wire field.
    #[serde(
        rename = "totalSupply",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_supply: Option<Decimal>,
    /// Exact `circulatingSupply` wire field.
    #[serde(
        rename = "circulatingSupply",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub circulating_supply: Option<Decimal>,
    /// Exact `holders` wire field.
    #[serde(rename = "holders", default, skip_serializing_if = "Option::is_none")]
    pub holders: Option<String>,
    /// Exact `decimals` wire field.
    #[serde(rename = "decimals", default, skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i64>,
    /// Exact `listingCex` wire field.
    #[serde(
        rename = "listingCex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_cex: Option<bool>,
    /// Exact `hotTag` wire field.
    #[serde(rename = "hotTag", default, skip_serializing_if = "Option::is_none")]
    pub hot_tag: Option<bool>,
    /// Exact `cexCoinName` wire field.
    #[serde(
        rename = "cexCoinName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cex_coin_name: Option<String>,
    /// Exact `canTransfer` wire field.
    #[serde(
        rename = "canTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_transfer: Option<bool>,
    /// Exact `denomination` wire field.
    #[serde(
        rename = "denomination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub denomination: Option<i64>,
    /// Exact `offline` wire field.
    #[serde(rename = "offline", default, skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    /// Exact `tradeDecimal` wire field.
    #[serde(
        rename = "tradeDecimal",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_decimal: Option<i64>,
    /// Exact `alphaId` wire field.
    #[serde(rename = "alphaId", default, skip_serializing_if = "Option::is_none")]
    pub alpha_id: Option<String>,
    /// Exact `offsell` wire field.
    #[serde(rename = "offsell", default, skip_serializing_if = "Option::is_none")]
    pub offsell: Option<bool>,
    /// Exact `priceHigh24h` wire field.
    #[serde(
        rename = "priceHigh24h",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_high24h: Option<Decimal>,
    /// Exact `priceLow24h` wire field.
    #[serde(
        rename = "priceLow24h",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_low24h: Option<Decimal>,
    /// Exact `count24h` wire field.
    #[serde(rename = "count24h", default, skip_serializing_if = "Option::is_none")]
    pub count24h: Option<String>,
    /// Exact `onlineTge` wire field.
    #[serde(rename = "onlineTge", default, skip_serializing_if = "Option::is_none")]
    pub online_tge: Option<bool>,
    /// Exact `onlineAirdrop` wire field.
    #[serde(
        rename = "onlineAirdrop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub online_airdrop: Option<bool>,
    /// Exact `score` wire field.
    #[serde(rename = "score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    /// Exact `cexOffDisplay` wire field.
    #[serde(
        rename = "cexOffDisplay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cex_off_display: Option<bool>,
    /// Exact `stockState` wire field.
    #[serde(
        rename = "stockState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stock_state: Option<bool>,
    /// Exact `listingTime` wire field.
    #[serde(
        rename = "listingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub listing_time: Option<i64>,
    /// Exact `mulPoint` wire field.
    #[serde(rename = "mulPoint", default, skip_serializing_if = "Option::is_none")]
    pub mul_point: Option<i64>,
    /// Exact `bnExclusiveState` wire field.
    #[serde(
        rename = "bnExclusiveState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bn_exclusive_state: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
