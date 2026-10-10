// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use super::wire::Kline;
use super::wire::PriceLevel;
use crate::Decimal;
use crate::SensitiveString;
use serde::{Deserialize, Serialize};

/// Provider-native `AccountFundingFlowResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountFundingFlowResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::core_trading::options::RecordId>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::core_trading::options::Symbol>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `createDate` wire field.
    #[serde(
        rename = "createDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_date: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `accountFundingFlow`.
pub type AccountFundingFlowResponse = Vec<AccountFundingFlowResponseItem>;

/// Provider-native `OptionMarginAccountInformationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct OptionMarginAccountInformationResponse {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: Vec<OptionMarginAccountInformationResponseAssetItem>,
    /// Exact `greek` wire field.
    #[serde(rename = "greek")]
    pub greek: Vec<OptionMarginAccountInformationResponseGreekItem>,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Exact `canTrade` wire field.
    #[serde(rename = "canTrade", default, skip_serializing_if = "Option::is_none")]
    pub can_trade: Option<bool>,
    /// Exact `canDeposit` wire field.
    #[serde(
        rename = "canDeposit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_deposit: Option<bool>,
    /// Exact `canWithdraw` wire field.
    #[serde(
        rename = "canWithdraw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_withdraw: Option<bool>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `tradeGroupId` wire field.
    #[serde(
        rename = "tradeGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_group_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OptionMarginAccountInformationResponseAssetItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OptionMarginAccountInformationResponseAssetItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `marginBalance` wire field.
    #[serde(rename = "marginBalance", deserialize_with = "super::wire::decimal")]
    pub margin_balance: Decimal,
    /// Exact `equity` wire field.
    #[serde(rename = "equity", deserialize_with = "super::wire::decimal")]
    pub equity: Decimal,
    /// Exact `available` wire field.
    #[serde(rename = "available", deserialize_with = "super::wire::decimal")]
    pub available: Decimal,
    /// Exact `initialMargin` wire field.
    #[serde(rename = "initialMargin", deserialize_with = "super::wire::decimal")]
    pub initial_margin: Decimal,
    /// Exact `maintMargin` wire field.
    #[serde(rename = "maintMargin", deserialize_with = "super::wire::decimal")]
    pub maint_margin: Decimal,
    /// Exact `unrealizedPNL` wire field.
    #[serde(rename = "unrealizedPNL", deserialize_with = "super::wire::decimal")]
    pub unrealized_pnl: Decimal,
    /// Exact `adjustedEquity` wire field.
    #[serde(rename = "adjustedEquity", deserialize_with = "super::wire::decimal")]
    pub adjusted_equity: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OptionMarginAccountInformationResponseGreekItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OptionMarginAccountInformationResponseGreekItem {
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `delta` wire field.
    #[serde(rename = "delta", deserialize_with = "super::wire::decimal")]
    pub delta: Decimal,
    /// Exact `gamma` wire field.
    #[serde(rename = "gamma", deserialize_with = "super::wire::decimal")]
    pub gamma: Decimal,
    /// Exact `theta` wire field.
    #[serde(rename = "theta", deserialize_with = "super::wire::decimal")]
    pub theta: Decimal,
    /// Exact `vega` wire field.
    #[serde(rename = "vega", deserialize_with = "super::wire::decimal")]
    pub vega: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CheckServerTimeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckServerTimeResponse {
    /// Exact `serverTime` wire field.
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponse {
    /// Exact `timezone` wire field.
    #[serde(rename = "timezone", default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Exact `serverTime` wire field.
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    /// Exact `optionContracts` wire field.
    #[serde(rename = "optionContracts")]
    pub option_contracts: Vec<ExchangeInformationResponseOptionContractsItem>,
    /// Exact `optionAssets` wire field.
    #[serde(rename = "optionAssets")]
    pub option_assets: Vec<ExchangeInformationResponseOptionAssetsItem>,
    /// Exact `optionSymbols` wire field.
    #[serde(rename = "optionSymbols")]
    pub option_symbols: Vec<ExchangeInformationResponseOptionSymbolsItem>,
    /// Exact `rateLimits` wire field.
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<ExchangeInformationResponseRateLimitsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponseOptionContractsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseOptionContractsItem {
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset")]
    pub base_asset: crate::Asset,
    /// Exact `quoteAsset` wire field.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: crate::Asset,
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `settleAsset` wire field.
    #[serde(rename = "settleAsset")]
    pub settle_asset: crate::Asset,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponseOptionAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseOptionAssetsItem {
    /// Exact `name` wire field.
    #[serde(rename = "name")]
    pub name: crate::Asset,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponseOptionSymbolsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseOptionSymbolsItem {
    /// Exact `expiryDate` wire field.
    #[serde(rename = "expiryDate")]
    pub expiry_date: i64,
    /// Exact `filters` wire field.
    #[serde(rename = "filters")]
    pub filters: Vec<ExchangeInformationResponseOptionSymbolsItemFiltersItem>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `strikePrice` wire field.
    #[serde(rename = "strikePrice", deserialize_with = "super::wire::decimal")]
    pub strike_price: Decimal,
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `unit` wire field.
    #[serde(rename = "unit", deserialize_with = "super::wire::decimal")]
    pub unit: Decimal,
    /// Exact `liquidationFeeRate` wire field.
    #[serde(
        rename = "liquidationFeeRate",
        deserialize_with = "super::wire::decimal"
    )]
    pub liquidation_fee_rate: Decimal,
    /// Exact `minQty` wire field.
    #[serde(rename = "minQty", deserialize_with = "super::wire::decimal")]
    pub min_qty: Decimal,
    /// Exact `maxQty` wire field.
    #[serde(rename = "maxQty", deserialize_with = "super::wire::decimal")]
    pub max_qty: Decimal,
    /// Exact `initialMargin` wire field.
    #[serde(rename = "initialMargin", deserialize_with = "super::wire::decimal")]
    pub initial_margin: Decimal,
    /// Exact `maintenanceMargin` wire field.
    #[serde(
        rename = "maintenanceMargin",
        deserialize_with = "super::wire::decimal"
    )]
    pub maintenance_margin: Decimal,
    /// Exact `minInitialMargin` wire field.
    #[serde(rename = "minInitialMargin", deserialize_with = "super::wire::decimal")]
    pub min_initial_margin: Decimal,
    /// Exact `minMaintenanceMargin` wire field.
    #[serde(
        rename = "minMaintenanceMargin",
        deserialize_with = "super::wire::decimal"
    )]
    pub min_maintenance_margin: Decimal,
    /// Exact `priceScale` wire field.
    #[serde(rename = "priceScale")]
    pub price_scale: i64,
    /// Exact `quantityScale` wire field.
    #[serde(rename = "quantityScale")]
    pub quantity_scale: i64,
    /// Exact `quoteAsset` wire field.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: crate::Asset,
    /// Exact `contractType` wire field.
    #[serde(rename = "contractType")]
    pub contract_type: String,
    /// Exact `underlyingType` wire field.
    #[serde(rename = "underlyingType")]
    pub underlying_type: String,
    /// Exact `nakedSell` wire field.
    #[serde(rename = "nakedSell")]
    pub naked_sell: bool,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider filters with explicit discriminator dispatch and unknown retention.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ExchangeInformationResponseOptionSymbolsItemFiltersItem {
    /// Provider `PRICE_FILTER` filter.
    PriceFilter(Box<ExchangeInformationResponseOptionSymbolsItemFiltersItemVariant1>),
    /// Future filter facts, retained with redacted Debug.
    Unknown(super::event_payloads::UnknownMessage),
}
impl<'de> Deserialize<'de> for ExchangeInformationResponseOptionSymbolsItemFiltersItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match value.get("filterType").and_then(serde_json::Value::as_str) {
            Some("PRICE_FILTER") => serde_json::from_value(value)
                .map(|v| Self::PriceFilter(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some(_) => Ok(Self::Unknown(value.into())),
            None => Err(serde::de::Error::custom("filter type required")),
        }
    }
}

/// Provider-native `ExchangeInformationResponseOptionSymbolsItemFiltersItemVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseOptionSymbolsItemFiltersItemVariant1 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `minPrice` wire field.
    #[serde(rename = "minPrice", deserialize_with = "super::wire::decimal")]
    pub min_price: Decimal,
    /// Exact `maxPrice` wire field.
    #[serde(rename = "maxPrice", deserialize_with = "super::wire::decimal")]
    pub max_price: Decimal,
    /// Exact `tickSize` wire field.
    #[serde(rename = "tickSize", deserialize_with = "super::wire::decimal")]
    pub tick_size: Decimal,
    /// Exact `minQty` wire field.
    #[serde(rename = "minQty", deserialize_with = "super::wire::decimal")]
    pub min_qty: Decimal,
    /// Exact `maxQty` wire field.
    #[serde(rename = "maxQty", deserialize_with = "super::wire::decimal")]
    pub max_qty: Decimal,
    /// Exact `stepSize` wire field.
    #[serde(rename = "stepSize", deserialize_with = "super::wire::decimal")]
    pub step_size: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponseRateLimitsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseRateLimitsItem {
    /// Exact `rateLimitType` wire field.
    #[serde(
        rename = "rateLimitType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limit_type: Option<String>,
    /// Exact `interval` wire field.
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Exact `intervalNum` wire field.
    #[serde(
        rename = "intervalNum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_num: Option<i64>,
    /// Exact `limit` wire field.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `HistoricalExerciseRecordsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HistoricalExerciseRecordsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `strikePrice` wire field.
    #[serde(rename = "strikePrice", deserialize_with = "super::wire::decimal")]
    pub strike_price: Decimal,
    /// Exact `realStrikePrice` wire field.
    #[serde(rename = "realStrikePrice", deserialize_with = "super::wire::decimal")]
    pub real_strike_price: Decimal,
    /// Exact `expiryDate` wire field.
    #[serde(
        rename = "expiryDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_date: Option<i64>,
    /// Exact `strikeResult` wire field.
    #[serde(
        rename = "strikeResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strike_result: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `historicalExerciseRecords`.
pub type HistoricalExerciseRecordsResponse = Vec<HistoricalExerciseRecordsResponseItem>;

/// Provider-native `IndexPriceResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndexPriceResponse {
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Exact `indexPrice` wire field.
    #[serde(rename = "indexPrice", deserialize_with = "super::wire::decimal")]
    pub index_price: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `klineCandlestickData`.
pub type KlineCandlestickDataResponse = Vec<Kline>;

/// Provider-native `OpenInterestResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenInterestResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `sumOpenInterest` wire field.
    #[serde(rename = "sumOpenInterest", deserialize_with = "super::wire::decimal")]
    pub sum_open_interest: Decimal,
    /// Exact `sumOpenInterestUsd` wire field.
    #[serde(
        rename = "sumOpenInterestUsd",
        deserialize_with = "super::wire::decimal"
    )]
    pub sum_open_interest_usd: Decimal,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `openInterest`.
pub type OpenInterestResponse = Vec<OpenInterestResponseItem>;

/// Provider-native `OptionMarkPriceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OptionMarkPriceResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `markPrice` wire field.
    #[serde(rename = "markPrice", deserialize_with = "super::wire::decimal")]
    pub mark_price: Decimal,
    /// Exact `bidIV` wire field.
    #[serde(rename = "bidIV", deserialize_with = "super::wire::decimal")]
    pub bid_iv: Decimal,
    /// Exact `askIV` wire field.
    #[serde(rename = "askIV", deserialize_with = "super::wire::decimal")]
    pub ask_iv: Decimal,
    /// Exact `markIV` wire field.
    #[serde(rename = "markIV", deserialize_with = "super::wire::decimal")]
    pub mark_iv: Decimal,
    /// Exact `delta` wire field.
    #[serde(rename = "delta", deserialize_with = "super::wire::decimal")]
    pub delta: Decimal,
    /// Exact `theta` wire field.
    #[serde(rename = "theta", deserialize_with = "super::wire::decimal")]
    pub theta: Decimal,
    /// Exact `gamma` wire field.
    #[serde(rename = "gamma", deserialize_with = "super::wire::decimal")]
    pub gamma: Decimal,
    /// Exact `vega` wire field.
    #[serde(rename = "vega", deserialize_with = "super::wire::decimal")]
    pub vega: Decimal,
    /// Exact `highPriceLimit` wire field.
    #[serde(rename = "highPriceLimit", deserialize_with = "super::wire::decimal")]
    pub high_price_limit: Decimal,
    /// Exact `lowPriceLimit` wire field.
    #[serde(rename = "lowPriceLimit", deserialize_with = "super::wire::decimal")]
    pub low_price_limit: Decimal,
    /// Exact `riskFreeInterest` wire field.
    #[serde(rename = "riskFreeInterest", deserialize_with = "super::wire::decimal")]
    pub risk_free_interest: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `optionMarkPrice`.
pub type OptionMarkPriceResponse = Vec<OptionMarkPriceResponseItem>;

/// Provider-native `OrderBookResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderBookResponse {
    /// Exact `bids` wire field.
    #[serde(rename = "bids")]
    pub bids: Vec<PriceLevel>,
    /// Exact `asks` wire field.
    #[serde(rename = "asks")]
    pub asks: Vec<PriceLevel>,
    /// Exact `T` wire field.
    #[serde(rename = "T")]
    pub upper_t: i64,
    /// Exact `lastUpdateId` wire field.
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `RecentBlockTradesListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RecentBlockTradesListResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::core_trading::options::RecordId>,
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<crate::core_trading::options::TradeId>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `qty` wire field.
    #[serde(rename = "qty", deserialize_with = "super::wire::decimal")]
    pub qty: Decimal,
    /// Exact `quoteQty` wire field.
    #[serde(rename = "quoteQty", deserialize_with = "super::wire::decimal")]
    pub quote_qty: Decimal,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<i64>,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `recentBlockTradesList`.
pub type RecentBlockTradesListResponse = Vec<RecentBlockTradesListResponseItem>;

/// Provider-native `RecentTradesListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RecentTradesListResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::core_trading::options::RecordId>,
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<crate::core_trading::options::TradeId>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `qty` wire field.
    #[serde(rename = "qty", deserialize_with = "super::wire::decimal")]
    pub qty: Decimal,
    /// Exact `quoteQty` wire field.
    #[serde(rename = "quoteQty", deserialize_with = "super::wire::decimal")]
    pub quote_qty: Decimal,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<i64>,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `recentTradesList`.
pub type RecentTradesListResponse = Vec<RecentTradesListResponseItem>;

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TestConnectivityResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `Ticker24hrPriceChangeStatisticsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticker24hrPriceChangeStatisticsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `priceChange` wire field.
    #[serde(rename = "priceChange", deserialize_with = "super::wire::decimal")]
    pub price_change: Decimal,
    /// Exact `priceChangePercent` wire field.
    #[serde(
        rename = "priceChangePercent",
        deserialize_with = "super::wire::decimal"
    )]
    pub price_change_percent: Decimal,
    /// Exact `lastPrice` wire field.
    #[serde(rename = "lastPrice", deserialize_with = "super::wire::decimal")]
    pub last_price: Decimal,
    /// Exact `lastQty` wire field.
    #[serde(rename = "lastQty", deserialize_with = "super::wire::decimal")]
    pub last_qty: Decimal,
    /// Exact `open` wire field.
    #[serde(rename = "open", deserialize_with = "super::wire::decimal")]
    pub open: Decimal,
    /// Exact `high` wire field.
    #[serde(rename = "high", deserialize_with = "super::wire::decimal")]
    pub high: Decimal,
    /// Exact `low` wire field.
    #[serde(rename = "low", deserialize_with = "super::wire::decimal")]
    pub low: Decimal,
    /// Exact `volume` wire field.
    #[serde(rename = "volume", deserialize_with = "super::wire::decimal")]
    pub volume: Decimal,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `bidPrice` wire field.
    #[serde(rename = "bidPrice", deserialize_with = "super::wire::decimal")]
    pub bid_price: Decimal,
    /// Exact `askPrice` wire field.
    #[serde(rename = "askPrice", deserialize_with = "super::wire::decimal")]
    pub ask_price: Decimal,
    /// Exact `openTime` wire field.
    #[serde(rename = "openTime", default, skip_serializing_if = "Option::is_none")]
    pub open_time: Option<i64>,
    /// Exact `closeTime` wire field.
    #[serde(rename = "closeTime", default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<i64>,
    /// Exact `firstTradeId` wire field.
    #[serde(
        rename = "firstTradeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub first_trade_id: Option<i64>,
    /// Exact `tradeCount` wire field.
    #[serde(
        rename = "tradeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_count: Option<i64>,
    /// Exact `strikePrice` wire field.
    #[serde(rename = "strikePrice", deserialize_with = "super::wire::decimal")]
    pub strike_price: Decimal,
    /// Exact `exercisePrice` wire field.
    #[serde(rename = "exercisePrice", deserialize_with = "super::wire::decimal")]
    pub exercise_price: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `ticker24hrPriceChangeStatistics`.
pub type Ticker24hrPriceChangeStatisticsResponse = Vec<Ticker24hrPriceChangeStatisticsResponseItem>;

/// Provider-native `AcceptBlockTradeOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcceptBlockTradeOrderResponse {
    /// Exact `blockTradeSettlementKey` wire field.
    #[serde(
        rename = "blockTradeSettlementKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_settlement_key: Option<crate::core_trading::options::BlockTradeSettlementKey>,
    /// Exact `expireTime` wire field.
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_time: Option<i64>,
    /// Exact `liquidity` wire field.
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `legs` wire field.
    #[serde(rename = "legs")]
    pub legs: Vec<AcceptBlockTradeOrderResponseLegsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AcceptBlockTradeOrderResponseLegsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcceptBlockTradeOrderResponseLegsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(rename = "quantity", deserialize_with = "super::wire::decimal")]
    pub quantity: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryBlockTradeDetailsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryBlockTradeDetailsResponse {
    /// Exact `blockTradeSettlementKey` wire field.
    #[serde(
        rename = "blockTradeSettlementKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_settlement_key: Option<crate::core_trading::options::BlockTradeSettlementKey>,
    /// Exact `expireTime` wire field.
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_time: Option<i64>,
    /// Exact `liquidity` wire field.
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `legs` wire field.
    #[serde(rename = "legs")]
    pub legs: Vec<QueryBlockTradeDetailsResponseLegsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryBlockTradeDetailsResponseLegsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryBlockTradeDetailsResponseLegsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(rename = "quantity", deserialize_with = "super::wire::decimal")]
    pub quantity: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountBlockTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountBlockTradeListResponseItem {
    /// Exact `parentOrderId` wire field.
    #[serde(
        rename = "parentOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_order_id: Option<crate::core_trading::options::OrderId>,
    /// Exact `crossType` wire field.
    #[serde(rename = "crossType", default, skip_serializing_if = "Option::is_none")]
    pub cross_type: Option<String>,
    /// Exact `legs` wire field.
    #[serde(rename = "legs")]
    pub legs: Vec<AccountBlockTradeListResponseItemLegsItem>,
    /// Exact `blockTradeSettlementKey` wire field.
    #[serde(
        rename = "blockTradeSettlementKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_settlement_key: Option<crate::core_trading::options::BlockTradeSettlementKey>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountBlockTradeListResponseItemLegsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountBlockTradeListResponseItemLegsItem {
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `orderPrice` wire field.
    #[serde(
        rename = "orderPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_price: Option<Decimal>,
    /// Exact `orderQuantity` wire field.
    #[serde(
        rename = "orderQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_quantity: Option<Decimal>,
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
    /// Exact `executedAmount` wire field.
    #[serde(
        rename = "executedAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_amount: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `orderSide` wire field.
    #[serde(rename = "orderSide", default, skip_serializing_if = "Option::is_none")]
    pub order_side: Option<String>,
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::core_trading::options::RecordId>,
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<crate::core_trading::options::TradeId>,
    /// Exact `tradePrice` wire field.
    #[serde(
        rename = "tradePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_price: Option<Decimal>,
    /// Exact `tradeQty` wire field.
    #[serde(
        rename = "tradeQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_qty: Option<Decimal>,
    /// Exact `tradeTime` wire field.
    #[serde(rename = "tradeTime", default, skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
    /// Exact `liquidity` wire field.
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// Exact `commission` wire field.
    #[serde(
        rename = "commission",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `accountBlockTradeList`.
pub type AccountBlockTradeListResponse = Vec<AccountBlockTradeListResponseItem>;

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelBlockTradeOrderResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExtendBlockTradeOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExtendBlockTradeOrderResponse {
    /// Exact `blockTradeSettlementKey` wire field.
    #[serde(
        rename = "blockTradeSettlementKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_settlement_key: Option<crate::core_trading::options::BlockTradeSettlementKey>,
    /// Exact `expireTime` wire field.
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_time: Option<i64>,
    /// Exact `liquidity` wire field.
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `legs` wire field.
    #[serde(rename = "legs")]
    pub legs: Vec<ExtendBlockTradeOrderResponseLegsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExtendBlockTradeOrderResponseLegsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExtendBlockTradeOrderResponseLegsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(rename = "quantity", deserialize_with = "super::wire::decimal")]
    pub quantity: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewBlockTradeOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewBlockTradeOrderResponse {
    /// Exact `blockTradeSettlementKey` wire field.
    #[serde(
        rename = "blockTradeSettlementKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_settlement_key: Option<crate::core_trading::options::BlockTradeSettlementKey>,
    /// Exact `expireTime` wire field.
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_time: Option<i64>,
    /// Exact `liquidity` wire field.
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `legs` wire field.
    #[serde(rename = "legs")]
    pub legs: Vec<NewBlockTradeOrderResponseLegsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewBlockTradeOrderResponseLegsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewBlockTradeOrderResponseLegsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(rename = "quantity", deserialize_with = "super::wire::decimal")]
    pub quantity: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Validated nested request builder for `NewBlockTradeOrderLegsInputItem`.
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewBlockTradeOrderLegsInputItem {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
}
impl NewBlockTradeOrderLegsInputItem {
    /// Start this nested request builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set `symbol`.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set `side`.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set `type`.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set `quantity`.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set `price`.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Validate required and conditional provider parameters.
    ///
    /// # Errors
    /// Refuses missing or contradictory input.
    pub fn build(self) -> Result<Self, crate::Error> {
        let p = crate::core::parameters(&self)?;
        crate::core::validate_parameters(
            &p,
            &["price", "quantity", "side", "symbol", "type"],
            &[("side", &["BUY", "SELL"]), ("type", &["LIMIT"])],
            &[],
        )?;
        super::validation::validate("nestedInput", &p)?;
        Ok(self)
    }
}

/// Provider-native `QueryBlockTradeOrderResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryBlockTradeOrderResponseItem {
    /// Exact `blockTradeSettlementKey` wire field.
    #[serde(
        rename = "blockTradeSettlementKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_trade_settlement_key: Option<crate::core_trading::options::BlockTradeSettlementKey>,
    /// Exact `expireTime` wire field.
    #[serde(
        rename = "expireTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_time: Option<i64>,
    /// Exact `liquidity` wire field.
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `legs` wire field.
    #[serde(rename = "legs")]
    pub legs: Vec<QueryBlockTradeOrderResponseItemLegsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryBlockTradeOrderResponseItemLegsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryBlockTradeOrderResponseItemLegsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(rename = "quantity", deserialize_with = "super::wire::decimal")]
    pub quantity: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryBlockTradeOrder`.
pub type QueryBlockTradeOrderResponse = Vec<QueryBlockTradeOrderResponseItem>;

/// Provider-native `AutoCancelAllOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutoCancelAllOpenOrdersResponse {
    /// Exact `underlyings` wire field.
    #[serde(rename = "underlyings")]
    pub underlyings: Vec<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetAutoCancelAllOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAutoCancelAllOpenOrdersResponse {
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `countdownTime` wire field.
    #[serde(
        rename = "countdownTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub countdown_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SetAutoCancelAllOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetAutoCancelAllOpenOrdersResponse {
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `countdownTime` wire field.
    #[serde(
        rename = "countdownTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub countdown_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetMarketMakerProtectionConfigResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetMarketMakerProtectionConfigResponse {
    /// Exact `underlyingId` wire field.
    #[serde(
        rename = "underlyingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_id: Option<i64>,
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `windowTimeInMilliseconds` wire field.
    #[serde(
        rename = "windowTimeInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_time_in_milliseconds: Option<i64>,
    /// Exact `frozenTimeInMilliseconds` wire field.
    #[serde(
        rename = "frozenTimeInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frozen_time_in_milliseconds: Option<i64>,
    /// Exact `qtyLimit` wire field.
    #[serde(rename = "qtyLimit", deserialize_with = "super::wire::decimal")]
    pub qty_limit: Decimal,
    /// Exact `deltaLimit` wire field.
    #[serde(rename = "deltaLimit", deserialize_with = "super::wire::decimal")]
    pub delta_limit: Decimal,
    /// Exact `lastTriggerTime` wire field.
    #[serde(
        rename = "lastTriggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_trigger_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ResetMarketMakerProtectionConfigResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResetMarketMakerProtectionConfigResponse {
    /// Exact `underlyingId` wire field.
    #[serde(
        rename = "underlyingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_id: Option<i64>,
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `windowTimeInMilliseconds` wire field.
    #[serde(
        rename = "windowTimeInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_time_in_milliseconds: Option<i64>,
    /// Exact `frozenTimeInMilliseconds` wire field.
    #[serde(
        rename = "frozenTimeInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frozen_time_in_milliseconds: Option<i64>,
    /// Exact `qtyLimit` wire field.
    #[serde(rename = "qtyLimit", deserialize_with = "super::wire::decimal")]
    pub qty_limit: Decimal,
    /// Exact `deltaLimit` wire field.
    #[serde(rename = "deltaLimit", deserialize_with = "super::wire::decimal")]
    pub delta_limit: Decimal,
    /// Exact `lastTriggerTime` wire field.
    #[serde(
        rename = "lastTriggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_trigger_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SetMarketMakerProtectionConfigResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetMarketMakerProtectionConfigResponse {
    /// Exact `underlyingId` wire field.
    #[serde(
        rename = "underlyingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_id: Option<i64>,
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `windowTimeInMilliseconds` wire field.
    #[serde(
        rename = "windowTimeInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_time_in_milliseconds: Option<i64>,
    /// Exact `frozenTimeInMilliseconds` wire field.
    #[serde(
        rename = "frozenTimeInMilliseconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frozen_time_in_milliseconds: Option<i64>,
    /// Exact `qtyLimit` wire field.
    #[serde(rename = "qtyLimit", deserialize_with = "super::wire::decimal")]
    pub qty_limit: Decimal,
    /// Exact `deltaLimit` wire field.
    #[serde(rename = "deltaLimit", deserialize_with = "super::wire::decimal")]
    pub delta_limit: Decimal,
    /// Exact `lastTriggerTime` wire field.
    #[serde(
        rename = "lastTriggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_trigger_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountTradeListResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::core_trading::options::RecordId>,
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<crate::core_trading::options::TradeId>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `realizedProfit` wire field.
    #[serde(
        rename = "realizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_profit: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `liquidity` wire field.
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `accountTradeList`.
pub type AccountTradeListResponse = Vec<AccountTradeListResponseItem>;

/// Provider-native `CancelAllOptionOrdersByUnderlyingResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllOptionOrdersByUnderlyingResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelAllOptionOrdersOnSpecificSymbolResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllOptionOrdersOnSpecificSymbolResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code", deserialize_with = "super::wire::decimal")]
    pub code: Decimal,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelMultipleOptionOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMultipleOptionOrdersResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `mmp` wire field.
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// Exact `source` wire field.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `cancelMultipleOptionOrders`.
pub type CancelMultipleOptionOrdersResponse =
    Vec<super::wire::BatchResult<CancelMultipleOptionOrdersResponseItem>>;

/// Provider-native `PlaceMultipleOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PlaceMultipleOrdersResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `postOnly` wire field.
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `mmp` wire field.
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `source` wire field.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `placeMultipleOrders`.
pub type PlaceMultipleOrdersResponse =
    Vec<super::wire::BatchResult<PlaceMultipleOrdersResponseItem>>;

/// Validated nested request builder for `PlaceMultipleOrdersOrdersInputItem`.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaceMultipleOrdersOrdersInputItem {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    post_only: Option<bool>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    client_order_id: Option<crate::core_trading::options::ClientOrderId>,
    #[serde(rename = "isMmp", skip_serializing_if = "Option::is_none")]
    is_mmp: Option<bool>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
}
impl PlaceMultipleOrdersOrdersInputItem {
    /// Start this nested request builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set `symbol`.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set `side`.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set `type`.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set `quantity`.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set `price`.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set `timeInForce`.
    #[must_use]
    pub fn time_in_force(mut self, value: impl Into<String>) -> Self {
        self.time_in_force = Some(value.into());
        self
    }
    /// Set `reduceOnly`.
    #[must_use]
    pub fn reduce_only(mut self, value: bool) -> Self {
        self.reduce_only = Some(value);
        self
    }
    /// Set `postOnly`.
    #[must_use]
    pub fn post_only(mut self, value: bool) -> Self {
        self.post_only = Some(value);
        self
    }
    /// Set `newOrderRespType`.
    #[must_use]
    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }
    /// Set `clientOrderId`.
    #[must_use]
    pub fn client_order_id(mut self, value: crate::core_trading::options::ClientOrderId) -> Self {
        self.client_order_id = Some(value);
        self
    }
    /// Set `isMmp`.
    #[must_use]
    pub fn is_mmp(mut self, value: bool) -> Self {
        self.is_mmp = Some(value);
        self
    }
    /// Set `selfTradePreventionMode`.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Validate required and conditional provider parameters.
    ///
    /// # Errors
    /// Refuses missing or contradictory input.
    pub fn build(self) -> Result<Self, crate::Error> {
        let p = crate::core::parameters(&self)?;
        crate::core::validate_parameters(
            &p,
            &[
                "clientOrderId",
                "price",
                "quantity",
                "side",
                "symbol",
                "type",
            ],
            &[
                ("side", &["BUY", "SELL"]),
                ("type", &["LIMIT"]),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX"]),
                ("newOrderRespType", &["ACK", "RESULT"]),
                (
                    "selfTradePreventionMode",
                    &["EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH"],
                ),
            ],
            &[],
        )?;
        super::validation::validate("newOrder", &p)?;
        Ok(self)
    }
}

/// Provider-native `CancelOptionOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelOptionOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `createDate` wire field.
    #[serde(
        rename = "createDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_date: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `source` wire field.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `mmp` wire field.
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `fee` wire field.
    #[serde(
        rename = "fee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fee: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `postOnly` wire field.
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `source` wire field.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `mmp` wire field.
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QuerySingleOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySingleOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `postOnly` wire field.
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `mmp` wire field.
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OptionPositionInformationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OptionPositionInformationResponseItem {
    /// Exact `entryPrice` wire field.
    #[serde(rename = "entryPrice", deserialize_with = "super::wire::decimal")]
    pub entry_price: Decimal,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(rename = "quantity", deserialize_with = "super::wire::decimal")]
    pub quantity: Decimal,
    /// Exact `markValue` wire field.
    #[serde(rename = "markValue", deserialize_with = "super::wire::decimal")]
    pub mark_value: Decimal,
    /// Exact `unrealizedPNL` wire field.
    #[serde(rename = "unrealizedPNL", deserialize_with = "super::wire::decimal")]
    pub unrealized_pnl: Decimal,
    /// Exact `markPrice` wire field.
    #[serde(rename = "markPrice", deserialize_with = "super::wire::decimal")]
    pub mark_price: Decimal,
    /// Exact `strikePrice` wire field.
    #[serde(rename = "strikePrice", deserialize_with = "super::wire::decimal")]
    pub strike_price: Decimal,
    /// Exact `expiryDate` wire field.
    #[serde(
        rename = "expiryDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_date: Option<i64>,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: crate::Asset,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Exact `bidQuantity` wire field.
    #[serde(rename = "bidQuantity", deserialize_with = "super::wire::decimal")]
    pub bid_quantity: Decimal,
    /// Exact `askQuantity` wire field.
    #[serde(rename = "askQuantity", deserialize_with = "super::wire::decimal")]
    pub ask_quantity: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `optionPositionInformation`.
pub type OptionPositionInformationResponse = Vec<OptionPositionInformationResponseItem>;

/// Provider-native `QueryCurrentOpenOptionOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentOpenOptionOrdersResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `mmp` wire field.
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryCurrentOpenOptionOrders`.
pub type QueryCurrentOpenOptionOrdersResponse = Vec<QueryCurrentOpenOptionOrdersResponseItem>;

/// Provider-native `QueryOptionOrderHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryOptionOrderHistoryResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::core_trading::options::OrderId,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `mmp` wire field.
    #[serde(rename = "mmp", default, skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryOptionOrderHistory`.
pub type QueryOptionOrderHistoryResponse = Vec<QueryOptionOrderHistoryResponseItem>;

/// Provider-native `TradfiOptionsContractResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradfiOptionsContractResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UserCommissionResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserCommissionResponse {
    /// Exact `commissions` wire field.
    #[serde(rename = "commissions")]
    pub commissions: Vec<UserCommissionResponseCommissionsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UserCommissionResponseCommissionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserCommissionResponseCommissionsItem {
    /// Exact `underlying` wire field.
    #[serde(rename = "underlying")]
    pub underlying: crate::Symbol,
    /// Exact `makerFee` wire field.
    #[serde(rename = "makerFee", deserialize_with = "super::wire::decimal")]
    pub maker_fee: Decimal,
    /// Exact `takerFee` wire field.
    #[serde(rename = "takerFee", deserialize_with = "super::wire::decimal")]
    pub taker_fee: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UserExerciseRecordResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserExerciseRecordResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id")]
    pub id: crate::core_trading::options::RecordId,
    /// Exact `currency` wire field.
    #[serde(rename = "currency")]
    pub currency: crate::Asset,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: crate::core_trading::options::Symbol,
    /// Exact `exercisePrice` wire field.
    #[serde(rename = "exercisePrice", deserialize_with = "super::wire::decimal")]
    pub exercise_price: Decimal,
    /// Exact `quantity` wire field.
    #[serde(rename = "quantity", deserialize_with = "super::wire::decimal")]
    pub quantity: Decimal,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `fee` wire field.
    #[serde(rename = "fee", deserialize_with = "super::wire::decimal")]
    pub fee: Decimal,
    /// Exact `createDate` wire field.
    #[serde(
        rename = "createDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_date: Option<i64>,
    /// Exact `priceScale` wire field.
    #[serde(
        rename = "priceScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_scale: Option<i64>,
    /// Exact `quantityScale` wire field.
    #[serde(
        rename = "quantityScale",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_scale: Option<i64>,
    /// Exact `optionSide` wire field.
    #[serde(
        rename = "optionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub option_side: Option<String>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: crate::Asset,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `userExerciseRecord`.
pub type UserExerciseRecordResponse = Vec<UserExerciseRecordResponseItem>;

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CloseUserDataStreamResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KeepaliveUserDataStreamResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `StartUserDataStreamResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StartUserDataStreamResponse {
    /// Exact `listenKey` wire field.
    #[serde(rename = "listenKey", default, skip_serializing_if = "Option::is_none")]
    pub listen_key: Option<SensitiveString>,
    /// Exact `expiration` wire field.
    #[serde(
        rename = "expiration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
