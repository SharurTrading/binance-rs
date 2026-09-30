// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use super::wire::Kline;
use super::wire::PriceLevel;
use crate::ClientOrderId;
use crate::Decimal;
use crate::SensitiveString;
use crate::Symbol;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Provider-native `AccountInformationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationResponse {
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AccountInformationResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<AccountInformationResponsePositionsItem>>,
    /// Exact `canDeposit` wire field.
    #[serde(
        rename = "canDeposit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_deposit: Option<bool>,
    /// Exact `canTrade` wire field.
    #[serde(rename = "canTrade", default, skip_serializing_if = "Option::is_none")]
    pub can_trade: Option<bool>,
    /// Exact `canWithdraw` wire field.
    #[serde(
        rename = "canWithdraw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_withdraw: Option<bool>,
    /// Exact `feeTier` wire field.
    #[serde(rename = "feeTier", default, skip_serializing_if = "Option::is_none")]
    pub fee_tier: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountInformationResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationResponseAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<crate::Asset>,
    /// Exact `walletBalance` wire field.
    #[serde(
        rename = "walletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub wallet_balance: Option<Decimal>,
    /// Exact `unrealizedProfit` wire field.
    #[serde(
        rename = "unrealizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_profit: Option<Decimal>,
    /// Exact `marginBalance` wire field.
    #[serde(
        rename = "marginBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_balance: Option<Decimal>,
    /// Exact `maintMargin` wire field.
    #[serde(
        rename = "maintMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maint_margin: Option<Decimal>,
    /// Exact `initialMargin` wire field.
    #[serde(
        rename = "initialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin: Option<Decimal>,
    /// Exact `positionInitialMargin` wire field.
    #[serde(
        rename = "positionInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_initial_margin: Option<Decimal>,
    /// Exact `openOrderInitialMargin` wire field.
    #[serde(
        rename = "openOrderInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_order_initial_margin: Option<Decimal>,
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Exact `crossWalletBalance` wire field.
    #[serde(
        rename = "crossWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_wallet_balance: Option<Decimal>,
    /// Exact `crossUnPnl` wire field.
    #[serde(
        rename = "crossUnPnl",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_un_pnl: Option<Decimal>,
    /// Exact `availableBalance` wire field.
    #[serde(
        rename = "availableBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_balance: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountInformationResponsePositionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationResponsePositionsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `positionAmt` wire field.
    #[serde(
        rename = "positionAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<Decimal>,
    /// Exact `initialMargin` wire field.
    #[serde(
        rename = "initialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin: Option<Decimal>,
    /// Exact `maintMargin` wire field.
    #[serde(
        rename = "maintMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maint_margin: Option<Decimal>,
    /// Exact `unrealizedProfit` wire field.
    #[serde(
        rename = "unrealizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_profit: Option<Decimal>,
    /// Exact `positionInitialMargin` wire field.
    #[serde(
        rename = "positionInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_initial_margin: Option<Decimal>,
    /// Exact `openOrderInitialMargin` wire field.
    #[serde(
        rename = "openOrderInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_order_initial_margin: Option<Decimal>,
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Exact `isolated` wire field.
    #[serde(rename = "isolated", default, skip_serializing_if = "Option::is_none")]
    pub isolated: Option<bool>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `entryPrice` wire field.
    #[serde(
        rename = "entryPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_price: Option<Decimal>,
    /// Exact `breakEvenPrice` wire field.
    #[serde(
        rename = "breakEvenPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub break_even_price: Option<Decimal>,
    /// Exact `maxQty` wire field.
    #[serde(
        rename = "maxQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_qty: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `notionalValue` wire field.
    #[serde(
        rename = "notionalValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional_value: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `FuturesAccountBalanceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesAccountBalanceResponseItem {
    /// Exact `accountAlias` wire field.
    #[serde(
        rename = "accountAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_alias: Option<String>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `balance` wire field.
    #[serde(rename = "balance", deserialize_with = "super::wire::decimal")]
    pub balance: Decimal,
    /// Exact `withdrawAvailable` wire field.
    #[serde(rename = "withdrawAvailable")]
    pub withdraw_available: String,
    /// Exact `crossWalletBalance` wire field.
    #[serde(
        rename = "crossWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_wallet_balance: Option<Decimal>,
    /// Exact `crossUnPnl` wire field.
    #[serde(
        rename = "crossUnPnl",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_un_pnl: Option<Decimal>,
    /// Exact `availableBalance` wire field.
    #[serde(
        rename = "availableBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_balance: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `futuresAccountBalance`.
pub type FuturesAccountBalanceResponse = Vec<FuturesAccountBalanceResponseItem>;

/// Provider-native `GetCurrentPositionModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCurrentPositionModeResponse {
    /// Exact `dualSidePosition` wire field.
    #[serde(
        rename = "dualSidePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dual_side_position: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ChangePositionModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangePositionModeResponse {
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

/// Provider-native `GetDownloadIdForFuturesOrderHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDownloadIdForFuturesOrderHistoryResponse {
    /// Exact `avgCostTimestampOfLast30d` wire field.
    #[serde(
        rename = "avgCostTimestampOfLast30d",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetDownloadIdForFuturesTradeHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDownloadIdForFuturesTradeHistoryResponse {
    /// Exact `avgCostTimestampOfLast30d` wire field.
    #[serde(
        rename = "avgCostTimestampOfLast30d",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetDownloadIdForFuturesTransactionHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDownloadIdForFuturesTransactionHistoryResponse {
    /// Exact `avgCostTimestampOfLast30d` wire field.
    #[serde(
        rename = "avgCostTimestampOfLast30d",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetFuturesOrderHistoryDownloadLinkByIdResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFuturesOrderHistoryDownloadLinkByIdResponse {
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `url` wire field.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<crate::SensitiveString>,
    /// Exact `notified` wire field.
    #[serde(rename = "notified", default, skip_serializing_if = "Option::is_none")]
    pub notified: Option<bool>,
    /// Exact `expirationTimestamp` wire field.
    #[serde(
        rename = "expirationTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    /// Exact `isExpired` wire field.
    #[serde(rename = "isExpired", default, skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetFuturesTradeDownloadLinkByIdResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFuturesTradeDownloadLinkByIdResponse {
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `url` wire field.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<crate::SensitiveString>,
    /// Exact `notified` wire field.
    #[serde(rename = "notified", default, skip_serializing_if = "Option::is_none")]
    pub notified: Option<bool>,
    /// Exact `expirationTimestamp` wire field.
    #[serde(
        rename = "expirationTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    /// Exact `isExpired` wire field.
    #[serde(rename = "isExpired", default, skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetFuturesTransactionHistoryDownloadLinkByIdResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFuturesTransactionHistoryDownloadLinkByIdResponse {
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `url` wire field.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<crate::SensitiveString>,
    /// Exact `notified` wire field.
    #[serde(rename = "notified", default, skip_serializing_if = "Option::is_none")]
    pub notified: Option<bool>,
    /// Exact `expirationTimestamp` wire field.
    #[serde(
        rename = "expirationTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    /// Exact `isExpired` wire field.
    #[serde(rename = "isExpired", default, skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetIncomeHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetIncomeHistoryResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `incomeType` wire field.
    #[serde(
        rename = "incomeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub income_type: Option<String>,
    /// Exact `income` wire field.
    #[serde(
        rename = "income",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub income: Option<Decimal>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<crate::Asset>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<String>,
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getIncomeHistory`.
pub type GetIncomeHistoryResponse = Vec<GetIncomeHistoryResponseItem>;

/// Provider-native `NotionalBracketForPairResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalBracketForPairResponseItem {
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `brackets` wire field.
    #[serde(rename = "brackets", default, skip_serializing_if = "Option::is_none")]
    pub brackets: Option<Vec<NotionalBracketForPairResponseItemBracketsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NotionalBracketForPairResponseItemBracketsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalBracketForPairResponseItemBracketsItem {
    /// Exact `bracket` wire field.
    #[serde(rename = "bracket", default, skip_serializing_if = "Option::is_none")]
    pub bracket: Option<i64>,
    /// Exact `initialLeverage` wire field.
    #[serde(
        rename = "initialLeverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_leverage: Option<i64>,
    /// Exact `qtyCap` wire field.
    #[serde(
        rename = "qtyCap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_cap: Option<Decimal>,
    /// Exact `qtylFloor` wire field.
    #[serde(
        rename = "qtylFloor",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qtyl_floor: Option<Decimal>,
    /// Exact `maintMarginRatio` wire field.
    #[serde(
        rename = "maintMarginRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maint_margin_ratio: Option<Decimal>,
    /// Exact `cum` wire field.
    #[serde(
        rename = "cum",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `notionalBracketForPair`.
pub type NotionalBracketForPairResponse = Vec<NotionalBracketForPairResponseItem>;

/// Provider-native `NotionalBracketForSymbolResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalBracketForSymbolResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `notionalCoef` wire field.
    #[serde(
        rename = "notionalCoef",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional_coef: Option<Decimal>,
    /// Exact `brackets` wire field.
    #[serde(rename = "brackets", default, skip_serializing_if = "Option::is_none")]
    pub brackets: Option<Vec<NotionalBracketForSymbolResponseItemBracketsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NotionalBracketForSymbolResponseItemBracketsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalBracketForSymbolResponseItemBracketsItem {
    /// Exact `bracket` wire field.
    #[serde(rename = "bracket", default, skip_serializing_if = "Option::is_none")]
    pub bracket: Option<i64>,
    /// Exact `initialLeverage` wire field.
    #[serde(
        rename = "initialLeverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_leverage: Option<i64>,
    /// Exact `qtyCap` wire field.
    #[serde(
        rename = "qtyCap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_cap: Option<Decimal>,
    /// Exact `qtylFloor` wire field.
    #[serde(
        rename = "qtylFloor",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qtyl_floor: Option<Decimal>,
    /// Exact `maintMarginRatio` wire field.
    #[serde(
        rename = "maintMarginRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maint_margin_ratio: Option<Decimal>,
    /// Exact `cum` wire field.
    #[serde(
        rename = "cum",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `notionalBracketForSymbol`.
pub type NotionalBracketForSymbolResponse = Vec<NotionalBracketForSymbolResponseItem>;

/// Provider-native `UserCommissionRateResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserCommissionRateResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `makerCommissionRate` wire field.
    #[serde(
        rename = "makerCommissionRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_commission_rate: Option<Decimal>,
    /// Exact `takerCommissionRate` wire field.
    #[serde(
        rename = "takerCommissionRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_commission_rate: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BasisResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BasisResponseItem {
    /// Exact `indexPrice` wire field.
    #[serde(
        rename = "indexPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub index_price: Option<Decimal>,
    /// Exact `contractType` wire field.
    #[serde(
        rename = "contractType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_type: Option<String>,
    /// Exact `basisRate` wire field.
    #[serde(
        rename = "basisRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub basis_rate: Option<Decimal>,
    /// Exact `futuresPrice` wire field.
    #[serde(
        rename = "futuresPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub futures_price: Option<Decimal>,
    /// Exact `annualizedBasisRate` wire field.
    #[serde(
        rename = "annualizedBasisRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_basis_rate: Option<Decimal>,
    /// Exact `basis` wire field.
    #[serde(
        rename = "basis",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub basis: Option<Decimal>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `basis`.
pub type BasisResponse = Vec<BasisResponseItem>;

/// Provider-native `CheckServerTimeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckServerTimeResponse {
    /// Exact `serverTime` wire field.
    #[serde(
        rename = "serverTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CompressedAggregateTradesListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompressedAggregateTradesListResponseItem {
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `compressedAggregateTradesList`.
pub type CompressedAggregateTradesListResponse = Vec<CompressedAggregateTradesListResponseItem>;

/// Exact response for `continuousContractKlineCandlestickData`.
pub type ContinuousContractKlineCandlestickDataResponse = Vec<Kline>;

/// Provider-native `ExchangeInformationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponse {
    /// Exact `exchangeFilters` wire field.
    #[serde(
        rename = "exchangeFilters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_filters: Option<Vec<String>>,
    /// Exact `rateLimits` wire field.
    #[serde(
        rename = "rateLimits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limits: Option<Vec<ExchangeInformationResponseRateLimitsItem>>,
    /// Exact `serverTime` wire field.
    #[serde(
        rename = "serverTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_time: Option<i64>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<ExchangeInformationResponseSymbolsItem>>,
    /// Exact `timezone` wire field.
    #[serde(rename = "timezone", default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponseRateLimitsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseRateLimitsItem {
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
    /// Exact `rateLimitType` wire field.
    #[serde(
        rename = "rateLimitType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limit_type: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponseSymbolsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseSymbolsItem {
    /// Exact `filters` wire field.
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExchangeInformationResponseSymbolsItemFiltersItem>>,
    /// Exact `orderTypes` wire field.
    #[serde(
        rename = "orderTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_types: Option<Vec<String>>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<Vec<String>>,
    /// Exact `liquidationFee` wire field.
    #[serde(
        rename = "liquidationFee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_fee: Option<Decimal>,
    /// Exact `marketTakeBound` wire field.
    #[serde(
        rename = "marketTakeBound",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub market_take_bound: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `contractType` wire field.
    #[serde(
        rename = "contractType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_type: Option<String>,
    /// Exact `deliveryDate` wire field.
    #[serde(
        rename = "deliveryDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_date: Option<i64>,
    /// Exact `onboardDate` wire field.
    #[serde(
        rename = "onboardDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub onboard_date: Option<i64>,
    /// Exact `contractStatus` wire field.
    #[serde(
        rename = "contractStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_status: Option<String>,
    /// Exact `contractSize` wire field.
    #[serde(
        rename = "contractSize",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_size: Option<Decimal>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<crate::Asset>,
    /// Exact `marginAsset` wire field.
    #[serde(
        rename = "marginAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_asset: Option<crate::Asset>,
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
    /// Exact `equalQtyPrecision` wire field.
    #[serde(
        rename = "equalQtyPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub equal_qty_precision: Option<i64>,
    /// Exact `triggerProtect` wire field.
    #[serde(
        rename = "triggerProtect",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_protect: Option<Decimal>,
    /// Exact `maintMarginPercent` wire field.
    #[serde(
        rename = "maintMarginPercent",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maint_margin_percent: Option<Decimal>,
    /// Exact `requiredMarginPercent` wire field.
    #[serde(
        rename = "requiredMarginPercent",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub required_margin_percent: Option<Decimal>,
    /// Exact `underlyingType` wire field.
    #[serde(
        rename = "underlyingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_type: Option<String>,
    /// Exact `underlyingSubType` wire field.
    #[serde(
        rename = "underlyingSubType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub underlying_sub_type: Option<Vec<String>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInformationResponseSymbolsItemFiltersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseSymbolsItemFiltersItem {
    /// Exact `filterType` wire field.
    #[serde(
        rename = "filterType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_type: Option<String>,
    /// Exact `maxPrice` wire field.
    #[serde(
        rename = "maxPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_price: Option<Decimal>,
    /// Exact `minPrice` wire field.
    #[serde(
        rename = "minPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_price: Option<Decimal>,
    /// Exact `tickSize` wire field.
    #[serde(
        rename = "tickSize",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tick_size: Option<Decimal>,
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
    /// Exact `stepSize` wire field.
    #[serde(
        rename = "stepSize",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub step_size: Option<Decimal>,
    /// Exact `limit` wire field.
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
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
    /// Exact `multiplierDecimal` wire field.
    #[serde(
        rename = "multiplierDecimal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_decimal: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetFundingRateHistoryOfPerpetualFuturesResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFundingRateHistoryOfPerpetualFuturesResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `fundingTime` wire field.
    #[serde(
        rename = "fundingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_time: Option<i64>,
    /// Exact `fundingRate` wire field.
    #[serde(
        rename = "fundingRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_rate: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getFundingRateHistoryOfPerpetualFutures`.
pub type GetFundingRateHistoryOfPerpetualFuturesResponse =
    Vec<GetFundingRateHistoryOfPerpetualFuturesResponseItem>;

/// Provider-native `IndexPriceAndMarkPriceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndexPriceAndMarkPriceResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `markPrice` wire field.
    #[serde(
        rename = "markPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_price: Option<Decimal>,
    /// Exact `indexPrice` wire field.
    #[serde(
        rename = "indexPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub index_price: Option<Decimal>,
    /// Exact `estimatedSettlePrice` wire field.
    #[serde(
        rename = "estimatedSettlePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_settle_price: Option<Decimal>,
    /// Exact `lastFundingRate` wire field.
    #[serde(
        rename = "lastFundingRate",
        default,
        deserialize_with = "super::wire::decimal_option_empty",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_funding_rate: Option<Decimal>,
    /// Exact `interestRate` wire field.
    #[serde(
        rename = "interestRate",
        default,
        deserialize_with = "super::wire::decimal_option_empty",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_rate: Option<Decimal>,
    /// Exact `nextFundingTime` wire field.
    #[serde(
        rename = "nextFundingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_funding_time: Option<i64>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `indexPriceAndMarkPrice`.
pub type IndexPriceAndMarkPriceResponse = Vec<IndexPriceAndMarkPriceResponseItem>;

/// Exact response for `indexPriceKlineCandlestickData`.
pub type IndexPriceKlineCandlestickDataResponse = Vec<super::wire::PriceKline>;

/// Exact response for `klineCandlestickData`.
pub type KlineCandlestickDataResponse = Vec<Kline>;

/// Provider-native `LongShortRatioResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LongShortRatioResponseItem {
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `longShortRatio` wire field.
    #[serde(
        rename = "longShortRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_short_ratio: Option<Decimal>,
    /// Exact `longAccount` wire field.
    #[serde(
        rename = "longAccount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_account: Option<Decimal>,
    /// Exact `shortAccount` wire field.
    #[serde(
        rename = "shortAccount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_account: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `longShortRatio`.
pub type LongShortRatioResponse = Vec<LongShortRatioResponseItem>;

/// Exact response for `markPriceKlineCandlestickData`.
pub type MarkPriceKlineCandlestickDataResponse = Vec<super::wire::PriceKline>;

/// Provider-native `OldTradesLookupResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OldTradesLookupResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
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
    /// Exact `baseQty` wire field.
    #[serde(
        rename = "baseQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_qty: Option<Decimal>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `isBuyerMaker` wire field.
    #[serde(
        rename = "isBuyerMaker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_buyer_maker: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `oldTradesLookup`.
pub type OldTradesLookupResponse = Vec<OldTradesLookupResponseItem>;

/// Provider-native `OpenInterestResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenInterestResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `openInterest` wire field.
    #[serde(
        rename = "openInterest",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_interest: Option<Decimal>,
    /// Exact `contractType` wire field.
    #[serde(
        rename = "contractType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_type: Option<String>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenInterestStatisticsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenInterestStatisticsResponseItem {
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `contractType` wire field.
    #[serde(
        rename = "contractType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_type: Option<String>,
    /// Exact `sumOpenInterest` wire field.
    #[serde(
        rename = "sumOpenInterest",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sum_open_interest: Option<Decimal>,
    /// Exact `sumOpenInterestValue` wire field.
    #[serde(
        rename = "sumOpenInterestValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sum_open_interest_value: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `openInterestStatistics`.
pub type OpenInterestStatisticsResponse = Vec<OpenInterestStatisticsResponseItem>;

/// Provider-native `OrderBookResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderBookResponse {
    /// Exact `lastUpdateId` wire field.
    #[serde(
        rename = "lastUpdateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `E` wire field.
    #[serde(rename = "E", default, skip_serializing_if = "Option::is_none")]
    pub upper_e: Option<i64>,
    /// Exact `T` wire field.
    #[serde(rename = "T", default, skip_serializing_if = "Option::is_none")]
    pub upper_t: Option<i64>,
    /// Exact `bids` wire field.
    #[serde(rename = "bids", default, skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<PriceLevel>>,
    /// Exact `asks` wire field.
    #[serde(rename = "asks", default, skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<PriceLevel>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `premiumIndexKlineData`.
pub type PremiumIndexKlineDataResponse = Vec<super::wire::PriceKline>;

/// Provider-native `QueryIndexPriceConstituentsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIndexPriceConstituentsResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `constituents` wire field.
    #[serde(
        rename = "constituents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub constituents: Option<Vec<QueryIndexPriceConstituentsResponseConstituentsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIndexPriceConstituentsResponseConstituentsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIndexPriceConstituentsResponseConstituentsItem {
    /// Exact `exchange` wire field.
    #[serde(rename = "exchange", default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `weight` wire field.
    #[serde(
        rename = "weight",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weight: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `RecentTradesListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RecentTradesListResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
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
    /// Exact `baseQty` wire field.
    #[serde(
        rename = "baseQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_qty: Option<Decimal>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `isBuyerMaker` wire field.
    #[serde(
        rename = "isBuyerMaker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_buyer_maker: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `recentTradesList`.
pub type RecentTradesListResponse = Vec<RecentTradesListResponseItem>;

/// Provider-native `SymbolOrderBookTickerResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolOrderBookTickerResponseItem {
    /// Exact `lastUpdateId` wire field.
    #[serde(
        rename = "lastUpdateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `bidPrice` wire field.
    #[serde(
        rename = "bidPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_price: Option<Decimal>,
    /// Exact `bidQty` wire field.
    #[serde(
        rename = "bidQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_qty: Option<Decimal>,
    /// Exact `askPrice` wire field.
    #[serde(
        rename = "askPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_price: Option<Decimal>,
    /// Exact `askQty` wire field.
    #[serde(
        rename = "askQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_qty: Option<Decimal>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `symbolOrderBookTicker`.
pub type SymbolOrderBookTickerResponse = Vec<SymbolOrderBookTickerResponseItem>;

/// Provider-native `SymbolPriceTickerResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolPriceTickerResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `ps` wire field.
    #[serde(rename = "ps", default, skip_serializing_if = "Option::is_none")]
    pub ps: Option<crate::Symbol>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `symbolPriceTicker`.
pub type SymbolPriceTickerResponse = Vec<SymbolPriceTickerResponseItem>;

/// Provider-native `TakerBuySellVolumeResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TakerBuySellVolumeResponseItem {
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `contractType` wire field.
    #[serde(
        rename = "contractType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_type: Option<String>,
    /// Exact `takerBuyVol` wire field.
    #[serde(
        rename = "takerBuyVol",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_buy_vol: Option<Decimal>,
    /// Exact `takerSellVol` wire field.
    #[serde(
        rename = "takerSellVol",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_sell_vol: Option<Decimal>,
    /// Exact `takerBuyVolValue` wire field.
    #[serde(
        rename = "takerBuyVolValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_buy_vol_value: Option<Decimal>,
    /// Exact `takerSellVolValue` wire field.
    #[serde(
        rename = "takerSellVolValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_sell_vol_value: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `takerBuySellVolume`.
pub type TakerBuySellVolumeResponse = Vec<TakerBuySellVolumeResponseItem>;

/// Exact response for `testConnectivity`.
pub type TestConnectivityResponse = BTreeMap<String, serde_json::Value>;

/// Provider-native `Ticker24hrPriceChangeStatisticsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticker24hrPriceChangeStatisticsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
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
    /// Exact `baseVolume` wire field.
    #[serde(
        rename = "baseVolume",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_volume: Option<Decimal>,
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

/// Exact response for `ticker24hrPriceChangeStatistics`.
pub type Ticker24hrPriceChangeStatisticsResponse = Vec<Ticker24hrPriceChangeStatisticsResponseItem>;

/// Provider-native `TopTraderLongShortRatioAccountsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopTraderLongShortRatioAccountsResponseItem {
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `longShortRatio` wire field.
    #[serde(
        rename = "longShortRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_short_ratio: Option<Decimal>,
    /// Exact `longAccount` wire field.
    #[serde(
        rename = "longAccount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_account: Option<Decimal>,
    /// Exact `shortAccount` wire field.
    #[serde(
        rename = "shortAccount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_account: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `topTraderLongShortRatioAccounts`.
pub type TopTraderLongShortRatioAccountsResponse = Vec<TopTraderLongShortRatioAccountsResponseItem>;

/// Provider-native `TopTraderLongShortRatioPositionsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopTraderLongShortRatioPositionsResponseItem {
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `longShortRatio` wire field.
    #[serde(
        rename = "longShortRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_short_ratio: Option<Decimal>,
    /// Exact `longPosition` wire field.
    #[serde(
        rename = "longPosition",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_position: Option<Decimal>,
    /// Exact `shortPosition` wire field.
    #[serde(
        rename = "shortPosition",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_position: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `topTraderLongShortRatioPositions`.
pub type TopTraderLongShortRatioPositionsResponse =
    Vec<TopTraderLongShortRatioPositionsResponseItem>;

/// Provider-native `AccountTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountTradeListResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
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
    /// Exact `realizedPnl` wire field.
    #[serde(
        rename = "realizedPnl",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pnl: Option<Decimal>,
    /// Exact `marginAsset` wire field.
    #[serde(
        rename = "marginAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_asset: Option<crate::Asset>,
    /// Exact `baseQty` wire field.
    #[serde(
        rename = "baseQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_qty: Option<Decimal>,
    /// Exact `quoteQty` wire field.
    #[serde(
        rename = "quoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_qty: Option<Decimal>,
    /// Exact `commission` wire field.
    #[serde(
        rename = "commission",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub commission: Option<Decimal>,
    /// Exact `commissionAsset` wire field.
    #[serde(
        rename = "commissionAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission_asset: Option<crate::Asset>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `buyer` wire field.
    #[serde(rename = "buyer", default, skip_serializing_if = "Option::is_none")]
    pub buyer: Option<bool>,
    /// Exact `maker` wire field.
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `accountTradeList`.
pub type AccountTradeListResponse = Vec<AccountTradeListResponseItem>;

/// Provider-native `AllOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllOrdersResponseItem {
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `cumBase` wire field.
    #[serde(
        rename = "cumBase",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_base: Option<Decimal>,
    /// Exact `cumQuote` wire field.
    #[serde(
        rename = "cumQuote",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_quote: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `goodTillDate` wire field.
    #[serde(
        rename = "goodTillDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub good_till_date: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `allOrders`.
pub type AllOrdersResponse = Vec<AllOrdersResponseItem>;

/// Provider-native `AutoCancelAllOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutoCancelAllOpenOrdersResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `countdownTime` wire field.
    #[serde(
        rename = "countdownTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub countdown_time: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelAllOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllOpenOrdersResponse {
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

/// Provider-native `CancelMultipleOrdersResponseItemSuccess` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMultipleOrdersResponseItemSuccess {
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `cumQty` wire field.
    #[serde(
        rename = "cumQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `cancelMultipleOrders`.
pub type CancelMultipleOrdersResponse =
    Vec<super::wire::BatchResult<CancelMultipleOrdersResponseItemSuccess>>;

/// Provider-native `ModifyMultipleOrdersResponseItemSuccess` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModifyMultipleOrdersResponseItemSuccess {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `modifyId` wire field.
    #[serde(rename = "modifyId", default, skip_serializing_if = "Option::is_none")]
    pub modify_id: Option<i64>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `cumQty` wire field.
    #[serde(
        rename = "cumQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_qty: Option<Decimal>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
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
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `modifyMultipleOrders`.
pub type ModifyMultipleOrdersResponse =
    Vec<super::wire::BatchResult<ModifyMultipleOrdersResponseItemSuccess>>;

/// Validated nested request builder for `ModifyMultipleOrdersBatchOrdersInputItem`.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ModifyMultipleOrdersBatchOrdersInputItem {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "modifyId", skip_serializing_if = "Option::is_none")]
    modify_id: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    timestamp: Option<i64>,
}
impl ModifyMultipleOrdersBatchOrdersInputItem {
    /// Start this nested request builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set `orderId`.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set `origClientOrderId`.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.orig_client_order_id = Some(value);
        self
    }
    /// Set `symbol`.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set `side`.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
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
    /// Set `modifyId`.
    #[must_use]
    pub fn modify_id(mut self, value: i64) -> Self {
        self.modify_id = Some(value);
        self
    }
    /// Set `recvWindow`.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Set `timestamp`.
    #[must_use]
    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }
    /// Validate required and conditional provider parameters.
    ///
    /// # Errors
    /// Refuses missing or contradictory input.
    pub fn build(self) -> Result<Self, crate::Error> {
        super::validation::validate("modifyOrder", &crate::core::parameters(&self)?)?;
        Ok(self)
    }
}

/// Provider-native `PlaceMultipleOrdersResponseItemSuccess` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PlaceMultipleOrdersResponseItemSuccess {
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `cumQty` wire field.
    #[serde(
        rename = "cumQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
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

/// Exact response for `placeMultipleOrders`.
pub type PlaceMultipleOrdersResponse =
    Vec<super::wire::BatchResult<PlaceMultipleOrdersResponseItemSuccess>>;

/// Validated nested request builder for `PlaceMultipleOrdersBatchOrdersInputItem`.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaceMultipleOrdersBatchOrdersInputItem {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    position_side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "activationPrice", skip_serializing_if = "Option::is_none")]
    activation_price: Option<Decimal>,
    #[serde(rename = "callbackRate", skip_serializing_if = "Option::is_none")]
    callback_rate: Option<Decimal>,
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    working_type: Option<String>,
    #[serde(rename = "priceProtect", skip_serializing_if = "Option::is_none")]
    price_protect: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    price_match: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
}
impl PlaceMultipleOrdersBatchOrdersInputItem {
    /// Start this nested request builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set `symbol`.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set `side`.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set `positionSide`.
    #[must_use]
    pub fn position_side(mut self, value: impl Into<String>) -> Self {
        self.position_side = Some(value.into());
        self
    }
    /// Set `type`.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set `timeInForce`.
    #[must_use]
    pub fn time_in_force(mut self, value: impl Into<String>) -> Self {
        self.time_in_force = Some(value.into());
        self
    }
    /// Set `quantity`.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set `reduceOnly`.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
        self
    }
    /// Set `price`.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set `newClientOrderId`.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
        self
    }
    /// Set `stopPrice`.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }
    /// Set `activationPrice`.
    #[must_use]
    pub fn activation_price(mut self, value: Decimal) -> Self {
        self.activation_price = Some(value);
        self
    }
    /// Set `callbackRate`.
    #[must_use]
    pub fn callback_rate(mut self, value: Decimal) -> Self {
        self.callback_rate = Some(value);
        self
    }
    /// Set `workingType`.
    #[must_use]
    pub fn working_type(mut self, value: impl Into<String>) -> Self {
        self.working_type = Some(value.into());
        self
    }
    /// Set `priceProtect`.
    #[must_use]
    pub fn price_protect(mut self, value: impl Into<String>) -> Self {
        self.price_protect = Some(value.into());
        self
    }
    /// Set `newOrderRespType`.
    #[must_use]
    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }
    /// Set `priceMatch`.
    #[must_use]
    pub fn price_match(mut self, value: impl Into<String>) -> Self {
        self.price_match = Some(value.into());
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
        super::validation::validate("newOrder", &crate::core::parameters(&self)?)?;
        Ok(self)
    }
}

/// Provider-native `CancelOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelOrderResponse {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `cumQty` wire field.
    #[serde(
        rename = "cumQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
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

/// Provider-native `ModifyOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModifyOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `modifyId` wire field.
    #[serde(rename = "modifyId", default, skip_serializing_if = "Option::is_none")]
    pub modify_id: Option<i64>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `cumQty` wire field.
    #[serde(
        rename = "cumQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_qty: Option<Decimal>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
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
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewOrderResponse {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `cumQty` wire field.
    #[serde(
        rename = "cumQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
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

/// Provider-native `QueryOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryOrderResponse {
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
    pub client_order_id: ClientOrderId,
    /// Exact `cumBase` wire field.
    #[serde(
        rename = "cumBase",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_base: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
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

/// Provider-native `ChangeInitialLeverageResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeInitialLeverageResponse {
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i64>,
    /// Exact `maxQty` wire field.
    #[serde(
        rename = "maxQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_qty: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ChangeMarginTypeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeMarginTypeResponse {
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

/// Provider-native `CurrentAllOpenOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CurrentAllOpenOrdersResponseItem {
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `cumBase` wire field.
    #[serde(
        rename = "cumBase",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_base: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
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

/// Exact response for `currentAllOpenOrders`.
pub type CurrentAllOpenOrdersResponse = Vec<CurrentAllOpenOrdersResponseItem>;

/// Provider-native `GetOrderModifyHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetOrderModifyHistoryResponseItem {
    /// Exact `amendmentId` wire field.
    #[serde(
        rename = "amendmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub amendment_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `amendment` wire field.
    #[serde(rename = "amendment", default, skip_serializing_if = "Option::is_none")]
    pub amendment: Option<GetOrderModifyHistoryResponseItemAmendment>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetOrderModifyHistoryResponseItemAmendment` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetOrderModifyHistoryResponseItemAmendment {
    /// Exact `price` wire field.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<GetOrderModifyHistoryResponseItemAmendmentPrice>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", default, skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<GetOrderModifyHistoryResponseItemAmendmentOrigQty>,
    /// Exact `count` wire field.
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Exact `modifyId` wire field.
    #[serde(rename = "modifyId", default, skip_serializing_if = "Option::is_none")]
    pub modify_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetOrderModifyHistoryResponseItemAmendmentPrice` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetOrderModifyHistoryResponseItemAmendmentPrice {
    /// Exact `before` wire field.
    #[serde(rename = "before", default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Exact `after` wire field.
    #[serde(rename = "after", default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetOrderModifyHistoryResponseItemAmendmentOrigQty` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetOrderModifyHistoryResponseItemAmendmentOrigQty {
    /// Exact `before` wire field.
    #[serde(rename = "before", default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Exact `after` wire field.
    #[serde(rename = "after", default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getOrderModifyHistory`.
pub type GetOrderModifyHistoryResponse = Vec<GetOrderModifyHistoryResponseItem>;

/// Provider-native `GetPositionMarginChangeHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPositionMarginChangeHistoryResponseItem {
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<crate::Asset>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<i64>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getPositionMarginChangeHistory`.
pub type GetPositionMarginChangeHistoryResponse = Vec<GetPositionMarginChangeHistoryResponseItem>;

/// Provider-native `ModifyIsolatedPositionMarginResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModifyIsolatedPositionMarginResponse {
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PositionAdlQuantileEstimationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionAdlQuantileEstimationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `adlQuantile` wire field.
    #[serde(
        rename = "adlQuantile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub adl_quantile: Option<PositionAdlQuantileEstimationResponseItemAdlQuantile>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PositionAdlQuantileEstimationResponseItemAdlQuantile` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionAdlQuantileEstimationResponseItemAdlQuantile {
    /// Exact `LONG` wire field.
    #[serde(rename = "LONG", default, skip_serializing_if = "Option::is_none")]
    pub upper_long: Option<i64>,
    /// Exact `SHORT` wire field.
    #[serde(rename = "SHORT", default, skip_serializing_if = "Option::is_none")]
    pub upper_short: Option<i64>,
    /// Exact `HEDGE` wire field.
    #[serde(rename = "HEDGE", default, skip_serializing_if = "Option::is_none")]
    pub upper_hedge: Option<i64>,
    /// Exact `BOTH` wire field.
    #[serde(rename = "BOTH", default, skip_serializing_if = "Option::is_none")]
    pub upper_both: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `positionAdlQuantileEstimation`.
pub type PositionAdlQuantileEstimationResponse = Vec<PositionAdlQuantileEstimationResponseItem>;

/// Provider-native `PositionInformationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionInformationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `positionAmt` wire field.
    #[serde(
        rename = "positionAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<Decimal>,
    /// Exact `entryPrice` wire field.
    #[serde(
        rename = "entryPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_price: Option<Decimal>,
    /// Exact `breakEvenPrice` wire field.
    #[serde(
        rename = "breakEvenPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub break_even_price: Option<Decimal>,
    /// Exact `markPrice` wire field.
    #[serde(
        rename = "markPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_price: Option<Decimal>,
    /// Exact `unRealizedProfit` wire field.
    #[serde(
        rename = "unRealizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub un_realized_profit: Option<Decimal>,
    /// Exact `liquidationPrice` wire field.
    #[serde(
        rename = "liquidationPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_price: Option<Decimal>,
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Exact `maxQty` wire field.
    #[serde(
        rename = "maxQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_qty: Option<Decimal>,
    /// Exact `marginType` wire field.
    #[serde(
        rename = "marginType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_type: Option<String>,
    /// Exact `isolatedMargin` wire field.
    #[serde(
        rename = "isolatedMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_margin: Option<Decimal>,
    /// Exact `isAutoAddMargin` wire field.
    #[serde(
        rename = "isAutoAddMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_auto_add_margin: Option<String>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `positionInformation`.
pub type PositionInformationResponse = Vec<PositionInformationResponseItem>;

/// Provider-native `QueryCurrentOpenOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentOpenOrderResponse {
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `cumBase` wire field.
    #[serde(
        rename = "cumBase",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_base: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `priceRate` wire field.
    #[serde(
        rename = "priceRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price_rate: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
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

/// Provider-native `UsersForceOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UsersForceOrdersResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<crate::Symbol>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `cumBase` wire field.
    #[serde(
        rename = "cumBase",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_base: Option<Decimal>,
    /// Exact `cumQuote` wire field.
    #[serde(
        rename = "cumQuote",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_quote: Option<Decimal>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
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
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `goodTillDate` wire field.
    #[serde(
        rename = "goodTillDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub good_till_date: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `usersForceOrders`.
pub type UsersForceOrdersResponse = Vec<UsersForceOrdersResponseItem>;

/// Exact response for `closeUserDataStream`.
pub type CloseUserDataStreamResponse = BTreeMap<String, serde_json::Value>;

/// Provider-native `KeepaliveUserDataStreamResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KeepaliveUserDataStreamResponse {
    /// Exact `listenKey` wire field.
    #[serde(rename = "listenKey", default, skip_serializing_if = "Option::is_none")]
    pub listen_key: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
