// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::Decimal;
use crate::Symbol;
use serde::{Deserialize, Serialize};

/// Provider-native `AdjustCrossMarginMaxLeverageResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AdjustCrossMarginMaxLeverageResponse {
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DisableIsolatedMarginAccountResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DisableIsolatedMarginAccountResponse {
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `EnableIsolatedMarginAccountResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnableIsolatedMarginAccountResponse {
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIsolatedMarginAccountInfoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIsolatedMarginAccountInfoResponse {
    /// Exact `assets` wire field.
    #[serde(rename = "assets")]
    pub assets: Vec<QueryIsolatedMarginAccountInfoResponseAssetsItem>,
    /// Exact `totalAssetOfBtc` wire field.
    #[serde(rename = "totalAssetOfBtc", deserialize_with = "super::wire::decimal")]
    pub total_asset_of_btc: Decimal,
    /// Exact `totalLiabilityOfBtc` wire field.
    #[serde(
        rename = "totalLiabilityOfBtc",
        deserialize_with = "super::wire::decimal"
    )]
    pub total_liability_of_btc: Decimal,
    /// Exact `totalNetAssetOfBtc` wire field.
    #[serde(
        rename = "totalNetAssetOfBtc",
        deserialize_with = "super::wire::decimal"
    )]
    pub total_net_asset_of_btc: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIsolatedMarginAccountInfoResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIsolatedMarginAccountInfoResponseAssetsItem {
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset")]
    pub base_asset: QueryIsolatedMarginAccountInfoResponseAssetsItemBaseAsset,
    /// Exact `quoteAsset` wire field.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: QueryIsolatedMarginAccountInfoResponseAssetsItemQuoteAsset,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isolatedCreated` wire field.
    #[serde(rename = "isolatedCreated")]
    pub isolated_created: bool,
    /// Exact `enabled` wire field.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Exact `marginLevel` wire field.
    #[serde(rename = "marginLevel", deserialize_with = "super::wire::decimal")]
    pub margin_level: Decimal,
    /// Exact `marginLevelStatus` wire field.
    #[serde(rename = "marginLevelStatus")]
    pub margin_level_status: String,
    /// Exact `marginRatio` wire field.
    #[serde(rename = "marginRatio", deserialize_with = "super::wire::decimal")]
    pub margin_ratio: Decimal,
    /// Exact `indexPrice` wire field.
    #[serde(rename = "indexPrice", deserialize_with = "super::wire::decimal")]
    pub index_price: Decimal,
    /// Exact `liquidatePrice` wire field.
    #[serde(rename = "liquidatePrice", deserialize_with = "super::wire::decimal")]
    pub liquidate_price: Decimal,
    /// Exact `liquidateRate` wire field.
    #[serde(rename = "liquidateRate", deserialize_with = "super::wire::decimal")]
    pub liquidate_rate: Decimal,
    /// Exact `tradeEnabled` wire field.
    #[serde(rename = "tradeEnabled")]
    pub trade_enabled: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIsolatedMarginAccountInfoResponseAssetsItemBaseAsset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIsolatedMarginAccountInfoResponseAssetsItemBaseAsset {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `borrowEnabled` wire field.
    #[serde(rename = "borrowEnabled")]
    pub borrow_enabled: bool,
    /// Exact `borrowed` wire field.
    #[serde(rename = "borrowed", deserialize_with = "super::wire::decimal")]
    pub borrowed: Decimal,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `interest` wire field.
    #[serde(rename = "interest", deserialize_with = "super::wire::decimal")]
    pub interest: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `netAsset` wire field.
    #[serde(rename = "netAsset", deserialize_with = "super::wire::decimal")]
    pub net_asset: Decimal,
    /// Exact `netAssetOfBtc` wire field.
    #[serde(rename = "netAssetOfBtc", deserialize_with = "super::wire::decimal")]
    pub net_asset_of_btc: Decimal,
    /// Exact `repayEnabled` wire field.
    #[serde(rename = "repayEnabled")]
    pub repay_enabled: bool,
    /// Exact `totalAsset` wire field.
    #[serde(rename = "totalAsset", deserialize_with = "super::wire::decimal")]
    pub total_asset: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIsolatedMarginAccountInfoResponseAssetsItemQuoteAsset` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIsolatedMarginAccountInfoResponseAssetsItemQuoteAsset {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `borrowEnabled` wire field.
    #[serde(rename = "borrowEnabled")]
    pub borrow_enabled: bool,
    /// Exact `borrowed` wire field.
    #[serde(rename = "borrowed", deserialize_with = "super::wire::decimal")]
    pub borrowed: Decimal,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `interest` wire field.
    #[serde(rename = "interest", deserialize_with = "super::wire::decimal")]
    pub interest: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `netAsset` wire field.
    #[serde(rename = "netAsset", deserialize_with = "super::wire::decimal")]
    pub net_asset: Decimal,
    /// Exact `netAssetOfBtc` wire field.
    #[serde(rename = "netAssetOfBtc", deserialize_with = "super::wire::decimal")]
    pub net_asset_of_btc: Decimal,
    /// Exact `repayEnabled` wire field.
    #[serde(rename = "repayEnabled")]
    pub repay_enabled: bool,
    /// Exact `totalAsset` wire field.
    #[serde(rename = "totalAsset", deserialize_with = "super::wire::decimal")]
    pub total_asset: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetBnbBurnStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetBnbBurnStatusResponse {
    /// Exact `spotBNBBurn` wire field.
    #[serde(rename = "spotBNBBurn")]
    pub spot_bnb_burn: bool,
    /// Exact `interestBNBBurn` wire field.
    #[serde(rename = "interestBNBBurn")]
    pub interest_bnb_burn: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetSummaryOfMarginAccountResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetSummaryOfMarginAccountResponse {
    /// Exact `normalBar` wire field.
    #[serde(rename = "normalBar", deserialize_with = "super::wire::decimal")]
    pub normal_bar: Decimal,
    /// Exact `marginCallBar` wire field.
    #[serde(rename = "marginCallBar", deserialize_with = "super::wire::decimal")]
    pub margin_call_bar: Decimal,
    /// Exact `forceLiquidationBar` wire field.
    #[serde(
        rename = "forceLiquidationBar",
        deserialize_with = "super::wire::decimal"
    )]
    pub force_liquidation_bar: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCrossIsolatedMarginCapitalFlowResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCrossIsolatedMarginCapitalFlowResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id")]
    pub id: crate::margin::RecordId,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId")]
    pub tran_id: crate::margin::TransactionId,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `note` wire field.
    #[serde(rename = "note")]
    pub note: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryCrossIsolatedMarginCapitalFlow`.
pub type QueryCrossIsolatedMarginCapitalFlowResponse =
    Vec<QueryCrossIsolatedMarginCapitalFlowResponseItem>;

/// Provider-native `QueryCrossMarginAccountDetailsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct QueryCrossMarginAccountDetailsResponse {
    /// Exact `created` wire field.
    #[serde(rename = "created")]
    pub created: bool,
    /// Exact `borrowEnabled` wire field.
    #[serde(rename = "borrowEnabled")]
    pub borrow_enabled: bool,
    /// Exact `marginLevel` wire field.
    #[serde(rename = "marginLevel", deserialize_with = "super::wire::decimal")]
    pub margin_level: Decimal,
    /// Exact `collateralMarginLevel` wire field.
    #[serde(
        rename = "collateralMarginLevel",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub collateral_margin_level: Option<Decimal>,
    /// Exact `totalAssetOfBtc` wire field.
    #[serde(rename = "totalAssetOfBtc", deserialize_with = "super::wire::decimal")]
    pub total_asset_of_btc: Decimal,
    /// Exact `totalLiabilityOfBtc` wire field.
    #[serde(
        rename = "totalLiabilityOfBtc",
        deserialize_with = "super::wire::decimal"
    )]
    pub total_liability_of_btc: Decimal,
    /// Exact `totalNetAssetOfBtc` wire field.
    #[serde(
        rename = "totalNetAssetOfBtc",
        deserialize_with = "super::wire::decimal"
    )]
    pub total_net_asset_of_btc: Decimal,
    /// Exact `TotalCollateralValueInUSDT` wire field.
    #[serde(
        rename = "TotalCollateralValueInUSDT",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_collateral_value_in_usdt: Option<Decimal>,
    /// Exact `tradeEnabled` wire field.
    #[serde(rename = "tradeEnabled")]
    pub trade_enabled: bool,
    /// Exact `transferInEnabled` wire field.
    #[serde(rename = "transferInEnabled")]
    pub transfer_in_enabled: bool,
    /// Exact `transferOutEnabled` wire field.
    #[serde(rename = "transferOutEnabled")]
    pub transfer_out_enabled: bool,
    /// Exact `accountType` wire field.
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Exact `userAssets` wire field.
    #[serde(rename = "userAssets")]
    pub user_assets: Vec<QueryCrossMarginAccountDetailsResponseUserAssetsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCrossMarginAccountDetailsResponseUserAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCrossMarginAccountDetailsResponseUserAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `borrowed` wire field.
    #[serde(rename = "borrowed", deserialize_with = "super::wire::decimal")]
    pub borrowed: Decimal,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `interest` wire field.
    #[serde(rename = "interest", deserialize_with = "super::wire::decimal")]
    pub interest: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `netAsset` wire field.
    #[serde(rename = "netAsset", deserialize_with = "super::wire::decimal")]
    pub net_asset: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCrossMarginFeeDataResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCrossMarginFeeDataResponseItem {
    /// Exact `vipLevel` wire field.
    #[serde(rename = "vipLevel")]
    pub vip_level: i64,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `transferIn` wire field.
    #[serde(rename = "transferIn")]
    pub transfer_in: bool,
    /// Exact `borrowable` wire field.
    #[serde(rename = "borrowable")]
    pub borrowable: bool,
    /// Exact `dailyInterest` wire field.
    #[serde(rename = "dailyInterest", deserialize_with = "super::wire::decimal")]
    pub daily_interest: Decimal,
    /// Exact `yearlyInterest` wire field.
    #[serde(rename = "yearlyInterest", deserialize_with = "super::wire::decimal")]
    pub yearly_interest: Decimal,
    /// Exact `borrowLimit` wire field.
    #[serde(rename = "borrowLimit", deserialize_with = "super::wire::decimal")]
    pub borrow_limit: Decimal,
    /// Exact `marginablePairs` wire field.
    #[serde(rename = "marginablePairs")]
    pub marginable_pairs: Vec<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryCrossMarginFeeData`.
pub type QueryCrossMarginFeeDataResponse = Vec<QueryCrossMarginFeeDataResponseItem>;

/// Provider-native `QueryEnabledIsolatedMarginAccountLimitResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryEnabledIsolatedMarginAccountLimitResponse {
    /// Exact `enabledAccount` wire field.
    #[serde(rename = "enabledAccount")]
    pub enabled_account: i64,
    /// Exact `maxAccount` wire field.
    #[serde(rename = "maxAccount")]
    pub max_account: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIsolatedMarginFeeDataResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIsolatedMarginFeeDataResponseItem {
    /// Exact `vipLevel` wire field.
    #[serde(rename = "vipLevel")]
    pub vip_level: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", deserialize_with = "super::wire::decimal")]
    pub leverage: Decimal,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: Vec<QueryIsolatedMarginFeeDataResponseItemDataItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIsolatedMarginFeeDataResponseItemDataItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIsolatedMarginFeeDataResponseItemDataItem {
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `dailyInterest` wire field.
    #[serde(rename = "dailyInterest", deserialize_with = "super::wire::decimal")]
    pub daily_interest: Decimal,
    /// Exact `borrowLimit` wire field.
    #[serde(rename = "borrowLimit", deserialize_with = "super::wire::decimal")]
    pub borrow_limit: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryIsolatedMarginFeeData`.
pub type QueryIsolatedMarginFeeDataResponse = Vec<QueryIsolatedMarginFeeDataResponseItem>;

/// Provider-native `GetFutureHourlyInterestRateResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFutureHourlyInterestRateResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `nextHourlyInterestRate` wire field.
    #[serde(
        rename = "nextHourlyInterestRate",
        deserialize_with = "super::wire::decimal"
    )]
    pub next_hourly_interest_rate: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getFutureHourlyInterestRate`.
pub type GetFutureHourlyInterestRateResponse = Vec<GetFutureHourlyInterestRateResponseItem>;

/// Provider-native `GetInterestHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetInterestHistoryResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows")]
    pub rows: Vec<GetInterestHistoryResponseRowsItem>,
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetInterestHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetInterestHistoryResponseRowsItem {
    /// Exact `txId` wire field.
    #[serde(rename = "txId")]
    pub tx_id: crate::margin::TransactionId,
    /// Exact `interestAccuredTime` wire field.
    #[serde(rename = "interestAccuredTime")]
    pub interest_accured_time: i64,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `rawAsset` wire field.
    #[serde(rename = "rawAsset")]
    pub raw_asset: crate::Asset,
    /// Exact `principal` wire field.
    #[serde(rename = "principal", deserialize_with = "super::wire::decimal")]
    pub principal: Decimal,
    /// Exact `interest` wire field.
    #[serde(rename = "interest", deserialize_with = "super::wire::decimal")]
    pub interest: Decimal,
    /// Exact `interestRate` wire field.
    #[serde(rename = "interestRate", deserialize_with = "super::wire::decimal")]
    pub interest_rate: Decimal,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `isolatedSymbol` wire field.
    #[serde(rename = "isolatedSymbol")]
    pub isolated_symbol: crate::Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountBorrowRepayResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountBorrowRepayResponse {
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId")]
    pub tran_id: crate::margin::TransactionId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryBorrowRepayRecordsInMarginAccountResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryBorrowRepayRecordsInMarginAccountResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows")]
    pub rows: Vec<QueryBorrowRepayRecordsInMarginAccountResponseRowsItem>,
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryBorrowRepayRecordsInMarginAccountResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryBorrowRepayRecordsInMarginAccountResponseRowsItem {
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `isolatedSymbol` wire field.
    #[serde(rename = "isolatedSymbol")]
    pub isolated_symbol: crate::Symbol,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `interest` wire field.
    #[serde(rename = "interest", deserialize_with = "super::wire::decimal")]
    pub interest: Decimal,
    /// Exact `principal` wire field.
    #[serde(rename = "principal", deserialize_with = "super::wire::decimal")]
    pub principal: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Exact `txId` wire field.
    #[serde(rename = "txId")]
    pub tx_id: crate::margin::TransactionId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginInterestRateHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginInterestRateHistoryResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `dailyInterestRate` wire field.
    #[serde(
        rename = "dailyInterestRate",
        deserialize_with = "super::wire::decimal"
    )]
    pub daily_interest_rate: Decimal,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Exact `vipLevel` wire field.
    #[serde(rename = "vipLevel")]
    pub vip_level: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginInterestRateHistory`.
pub type QueryMarginInterestRateHistoryResponse = Vec<QueryMarginInterestRateHistoryResponseItem>;

/// Provider-native `QueryMaxBorrowResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMaxBorrowResponse {
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `borrowLimit` wire field.
    #[serde(rename = "borrowLimit", deserialize_with = "super::wire::decimal")]
    pub borrow_limit: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CrossMarginCollateralRatioResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CrossMarginCollateralRatioResponseItem {
    /// Exact `collaterals` wire field.
    #[serde(rename = "collaterals")]
    pub collaterals: Vec<CrossMarginCollateralRatioResponseItemCollateralsItem>,
    /// Exact `assetNames` wire field.
    #[serde(rename = "assetNames")]
    pub asset_names: Vec<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CrossMarginCollateralRatioResponseItemCollateralsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CrossMarginCollateralRatioResponseItemCollateralsItem {
    /// Exact `minUsdValue` wire field.
    #[serde(rename = "minUsdValue", deserialize_with = "super::wire::decimal")]
    pub min_usd_value: Decimal,
    /// Exact `maxUsdValue` wire field.
    #[serde(rename = "maxUsdValue", deserialize_with = "super::wire::decimal")]
    pub max_usd_value: Decimal,
    /// Exact `discountRate` wire field.
    #[serde(rename = "discountRate", deserialize_with = "super::wire::decimal")]
    pub discount_rate: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `crossMarginCollateralRatio`.
pub type CrossMarginCollateralRatioResponse = Vec<CrossMarginCollateralRatioResponseItem>;

/// Provider-native `GetAllCrossMarginPairsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAllCrossMarginPairsResponseItem {
    /// Exact `base` wire field.
    #[serde(rename = "base")]
    pub base: crate::Asset,
    /// Exact `id` wire field.
    #[serde(rename = "id")]
    pub id: crate::margin::RecordId,
    /// Exact `isBuyAllowed` wire field.
    #[serde(rename = "isBuyAllowed")]
    pub is_buy_allowed: bool,
    /// Exact `isMarginTrade` wire field.
    #[serde(rename = "isMarginTrade")]
    pub is_margin_trade: bool,
    /// Exact `isSellAllowed` wire field.
    #[serde(rename = "isSellAllowed")]
    pub is_sell_allowed: bool,
    /// Exact `quote` wire field.
    #[serde(rename = "quote")]
    pub quote: crate::Asset,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `delistTime` wire field.
    #[serde(rename = "delistTime")]
    pub delist_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getAllCrossMarginPairs`.
pub type GetAllCrossMarginPairsResponse = Vec<GetAllCrossMarginPairsResponseItem>;

/// Provider-native `GetAllIsolatedMarginSymbolResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAllIsolatedMarginSymbolResponseItem {
    /// Exact `base` wire field.
    #[serde(rename = "base")]
    pub base: crate::Asset,
    /// Exact `isBuyAllowed` wire field.
    #[serde(rename = "isBuyAllowed")]
    pub is_buy_allowed: bool,
    /// Exact `isMarginTrade` wire field.
    #[serde(rename = "isMarginTrade")]
    pub is_margin_trade: bool,
    /// Exact `isSellAllowed` wire field.
    #[serde(rename = "isSellAllowed")]
    pub is_sell_allowed: bool,
    /// Exact `quote` wire field.
    #[serde(rename = "quote")]
    pub quote: crate::Asset,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getAllIsolatedMarginSymbol`.
pub type GetAllIsolatedMarginSymbolResponse = Vec<GetAllIsolatedMarginSymbolResponseItem>;

/// Provider-native `GetAllMarginAssetsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAllMarginAssetsResponseItem {
    /// Exact `assetFullName` wire field.
    #[serde(rename = "assetFullName")]
    pub asset_full_name: String,
    /// Exact `assetName` wire field.
    #[serde(rename = "assetName")]
    pub asset_name: String,
    /// Exact `isBorrowable` wire field.
    #[serde(rename = "isBorrowable")]
    pub is_borrowable: bool,
    /// Exact `isMortgageable` wire field.
    #[serde(rename = "isMortgageable")]
    pub is_mortgageable: bool,
    /// Exact `userMinBorrow` wire field.
    #[serde(rename = "userMinBorrow", deserialize_with = "super::wire::decimal")]
    pub user_min_borrow: Decimal,
    /// Exact `userMinRepay` wire field.
    #[serde(rename = "userMinRepay", deserialize_with = "super::wire::decimal")]
    pub user_min_repay: Decimal,
    /// Exact `delistTime` wire field.
    #[serde(rename = "delistTime")]
    pub delist_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getAllMarginAssets`.
pub type GetAllMarginAssetsResponse = Vec<GetAllMarginAssetsResponseItem>;

/// Provider-native `GetDelistScheduleResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDelistScheduleResponseItem {
    /// Exact `delistTime` wire field.
    #[serde(rename = "delistTime")]
    pub delist_time: i64,
    /// Exact `crossMarginAssets` wire field.
    #[serde(rename = "crossMarginAssets")]
    pub cross_margin_assets: Vec<String>,
    /// Exact `isolatedMarginSymbols` wire field.
    #[serde(rename = "isolatedMarginSymbols")]
    pub isolated_margin_symbols: Vec<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getDelistSchedule`.
pub type GetDelistScheduleResponse = Vec<GetDelistScheduleResponseItem>;

/// Provider-native `GetLimitPricePairsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetLimitPricePairsResponse {
    /// Exact `crossMarginSymbols` wire field.
    #[serde(rename = "crossMarginSymbols")]
    pub cross_margin_symbols: Vec<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetListScheduleResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetListScheduleResponseItem {
    /// Exact `listTime` wire field.
    #[serde(rename = "listTime")]
    pub list_time: i64,
    /// Exact `crossMarginAssets` wire field.
    #[serde(rename = "crossMarginAssets")]
    pub cross_margin_assets: Vec<String>,
    /// Exact `isolatedMarginSymbols` wire field.
    #[serde(rename = "isolatedMarginSymbols")]
    pub isolated_margin_symbols: Vec<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getListSchedule`.
pub type GetListScheduleResponse = Vec<GetListScheduleResponseItem>;

/// Provider-native `GetMarginAssetRiskBasedLiquidationRatioResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetMarginAssetRiskBasedLiquidationRatioResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `riskBasedLiquidationRatio` wire field.
    #[serde(
        rename = "riskBasedLiquidationRatio",
        deserialize_with = "super::wire::decimal"
    )]
    pub risk_based_liquidation_ratio: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getMarginAssetRiskBasedLiquidationRatio`.
pub type GetMarginAssetRiskBasedLiquidationRatioResponse =
    Vec<GetMarginAssetRiskBasedLiquidationRatioResponseItem>;

/// Provider-native `GetMarginRestrictedAssetsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetMarginRestrictedAssetsResponse {
    /// Exact `openLongRestrictedAsset` wire field.
    #[serde(rename = "openLongRestrictedAsset")]
    pub open_long_restricted_asset: Vec<String>,
    /// Exact `maxCollateralExceededAsset` wire field.
    #[serde(rename = "maxCollateralExceededAsset")]
    pub max_collateral_exceeded_asset: Vec<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryIsolatedMarginTierDataResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIsolatedMarginTierDataResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `tier` wire field.
    #[serde(rename = "tier")]
    pub tier: i64,
    /// Exact `effectiveMultiple` wire field.
    #[serde(
        rename = "effectiveMultiple",
        deserialize_with = "super::wire::decimal"
    )]
    pub effective_multiple: Decimal,
    /// Exact `initialRiskRatio` wire field.
    #[serde(rename = "initialRiskRatio", deserialize_with = "super::wire::decimal")]
    pub initial_risk_ratio: Decimal,
    /// Exact `liquidationRiskRatio` wire field.
    #[serde(
        rename = "liquidationRiskRatio",
        deserialize_with = "super::wire::decimal"
    )]
    pub liquidation_risk_ratio: Decimal,
    /// Exact `baseAssetMaxBorrowable` wire field.
    #[serde(
        rename = "baseAssetMaxBorrowable",
        deserialize_with = "super::wire::decimal"
    )]
    pub base_asset_max_borrowable: Decimal,
    /// Exact `quoteAssetMaxBorrowable` wire field.
    #[serde(
        rename = "quoteAssetMaxBorrowable",
        deserialize_with = "super::wire::decimal"
    )]
    pub quote_asset_max_borrowable: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryIsolatedMarginTierData`.
pub type QueryIsolatedMarginTierDataResponse = Vec<QueryIsolatedMarginTierDataResponseItem>;

/// Provider-native `QueryMarginAvailableInventoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAvailableInventoryResponse {
    /// Exact `assets` wire field.
    #[serde(rename = "assets")]
    pub assets: super::AssetInventory,
    /// Exact `updateTime` wire field.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginPriceindexResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginPriceindexResponse {
    /// Exact `calcTime` wire field.
    #[serde(rename = "calcTime")]
    pub calc_time: i64,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

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
    #[serde(rename = "listenKey")]
    pub listen_key: crate::SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CreateSpecialKeyResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateSpecialKeyResponse {
    /// Exact `apiKey` wire field.
    #[serde(rename = "apiKey")]
    pub api_key: crate::SensitiveString,
    /// Exact `secretKey` wire field.
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<crate::SensitiveString>,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeleteSpecialKeyResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QuerySpecialKeyResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySpecialKeyResponse {
    /// Exact `apiKey` wire field.
    #[serde(rename = "apiKey")]
    pub api_key: crate::SensitiveString,
    /// Exact `ip` wire field.
    #[serde(rename = "ip")]
    pub ip: String,
    /// Exact `apiName` wire field.
    #[serde(rename = "apiName")]
    pub api_name: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `permissionMode` wire field.
    #[serde(rename = "permissionMode")]
    pub permission_mode: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EditIpForSpecialKeyResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExitSpecialKeyModeResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetForceLiquidationRecordResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetForceLiquidationRecordResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows")]
    pub rows: Vec<GetForceLiquidationRecordResponseRowsItem>,
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetForceLiquidationRecordResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetForceLiquidationRecordResponseRowsItem {
    /// Exact `avgPrice` wire field.
    #[serde(rename = "avgPrice", deserialize_with = "super::wire::decimal")]
    pub avg_price: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `qty` wire field.
    #[serde(rename = "qty", deserialize_with = "super::wire::decimal")]
    pub qty: Decimal,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `updatedTime` wire field.
    #[serde(rename = "updatedTime")]
    pub updated_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetSmallLiabilityExchangeCoinListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetSmallLiabilityExchangeCoinListResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `interest` wire field.
    #[serde(rename = "interest", deserialize_with = "super::wire::decimal")]
    pub interest: Decimal,
    /// Exact `principal` wire field.
    #[serde(rename = "principal", deserialize_with = "super::wire::decimal")]
    pub principal: Decimal,
    /// Exact `liabilityAsset` wire field.
    #[serde(rename = "liabilityAsset")]
    pub liability_asset: crate::Asset,
    /// Exact `liabilityQty` wire field.
    #[serde(rename = "liabilityQty", deserialize_with = "super::wire::decimal")]
    pub liability_qty: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getSmallLiabilityExchangeCoinList`.
pub type GetSmallLiabilityExchangeCoinListResponse =
    Vec<GetSmallLiabilityExchangeCoinListResponseItem>;

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SmallLiabilityExchangeResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetSmallLiabilityExchangeHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetSmallLiabilityExchangeHistoryResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `rows` wire field.
    #[serde(rename = "rows")]
    pub rows: Vec<GetSmallLiabilityExchangeHistoryResponseRowsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetSmallLiabilityExchangeHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetSmallLiabilityExchangeHistoryResponseRowsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `targetAsset` wire field.
    #[serde(rename = "targetAsset")]
    pub target_asset: crate::Asset,
    /// Exact `targetAmount` wire field.
    #[serde(rename = "targetAmount", deserialize_with = "super::wire::decimal")]
    pub target_amount: Decimal,
    /// Exact `bizType` wire field.
    #[serde(rename = "bizType")]
    pub biz_type: String,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider alternatives for `MarginAccountCancelAllOpenOrdersOnASymbolResponseItem`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum MarginAccountCancelAllOpenOrdersOnASymbolResponseItem {
    /// Wire alternative 1.
    Variant1(Box<MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant1>),
    /// Wire alternative 2.
    Variant2(Box<MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2>),
}

/// Provider-native `MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `origClientOrderId` wire field.
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: crate::margin::ClientOrderId,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        deserialize_with = "super::wire::decimal"
    )]
    pub cummulative_quote_qty: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
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

/// Provider-native `MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2OrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(rename = "orderReports")]
    pub order_reports:
        Vec<MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2OrderReportsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2OrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2OrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2OrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolResponseItemVariant2OrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `origClientOrderId` wire field.
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: crate::margin::ClientOrderId,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        deserialize_with = "super::wire::decimal"
    )]
    pub cummulative_quote_qty: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `marginAccountCancelAllOpenOrdersOnASymbol`.
pub type MarginAccountCancelAllOpenOrdersOnASymbolResponse =
    Vec<MarginAccountCancelAllOpenOrdersOnASymbolResponseItem>;

/// Provider-native `QueryMarginAccountsOpenOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOpenOrdersResponseItem {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        deserialize_with = "super::wire::decimal"
    )]
    pub cummulative_quote_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `isWorking` wire field.
    #[serde(rename = "isWorking")]
    pub is_working: bool,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsOpenOrders`.
pub type QueryMarginAccountsOpenOrdersResponse = Vec<QueryMarginAccountsOpenOrdersResponseItem>;

/// Provider-native `MarginAccountCancelOcoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelOcoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<MarginAccountCancelOcoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<MarginAccountCancelOcoResponseOrderReportsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountCancelOcoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelOcoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountCancelOcoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelOcoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `origClientOrderId` wire field.
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: crate::margin::ClientOrderId,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        deserialize_with = "super::wire::decimal"
    )]
    pub cummulative_quote_qty: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
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

/// Provider-native `QueryMarginAccountsOcoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOcoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryMarginAccountsOcoResponseOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginAccountsOcoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOcoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountCancelOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountCancelOrderResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `origClientOrderId` wire field.
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: crate::margin::ClientOrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        deserialize_with = "super::wire::decimal"
    )]
    pub cummulative_quote_qty: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOrderResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `transactTime` wire field.
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
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
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `marginBuyBorrowAmount` wire field.
    #[serde(
        rename = "marginBuyBorrowAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_buy_borrow_amount: Option<Decimal>,
    /// Exact `marginBuyBorrowAsset` wire field.
    #[serde(
        rename = "marginBuyBorrowAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_buy_borrow_asset: Option<crate::Asset>,
    /// Exact `fills` wire field.
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<MarginAccountNewOrderResponseFillsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOrderResponseFillsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOrderResponseFillsItem {
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `qty` wire field.
    #[serde(rename = "qty", deserialize_with = "super::wire::decimal")]
    pub qty: Decimal,
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
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId")]
    pub trade_id: crate::margin::TradeId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginAccountsOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOrderResponse {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        deserialize_with = "super::wire::decimal"
    )]
    pub cummulative_quote_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `isWorking` wire field.
    #[serde(rename = "isWorking")]
    pub is_working: bool,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOcoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOcoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `marginBuyBorrowAmount` wire field.
    #[serde(
        rename = "marginBuyBorrowAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_buy_borrow_amount: Option<Decimal>,
    /// Exact `marginBuyBorrowAsset` wire field.
    #[serde(
        rename = "marginBuyBorrowAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_buy_borrow_asset: Option<crate::Asset>,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<MarginAccountNewOcoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<MarginAccountNewOcoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOcoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOcoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOcoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOcoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactTime` wire field.
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
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
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
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

/// Provider-native `MarginAccountNewOtoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOtoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<MarginAccountNewOtoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<MarginAccountNewOtoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOtoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOtoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOtoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOtoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactTime` wire field.
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
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
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
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

/// Provider-native `MarginAccountNewOtocoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOtocoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<MarginAccountNewOtocoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<MarginAccountNewOtocoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOtocoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOtocoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountNewOtocoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountNewOtocoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactTime` wire field.
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
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
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginManualLiquidationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginManualLiquidationResponse {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `interest` wire field.
    #[serde(rename = "interest", deserialize_with = "super::wire::decimal")]
    pub interest: Decimal,
    /// Exact `principal` wire field.
    #[serde(rename = "principal", deserialize_with = "super::wire::decimal")]
    pub principal: Decimal,
    /// Exact `liabilityAsset` wire field.
    #[serde(rename = "liabilityAsset")]
    pub liability_asset: crate::Asset,
    /// Exact `liabilityQty` wire field.
    #[serde(rename = "liabilityQty", deserialize_with = "super::wire::decimal")]
    pub liability_qty: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCurrentMarginOrderCountUsageResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentMarginOrderCountUsageResponseItem {
    /// Exact `rateLimitType` wire field.
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,
    /// Exact `interval` wire field.
    #[serde(rename = "interval")]
    pub interval: String,
    /// Exact `intervalNum` wire field.
    #[serde(rename = "intervalNum")]
    pub interval_num: i64,
    /// Exact `limit` wire field.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Exact `count` wire field.
    #[serde(rename = "count")]
    pub count: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryCurrentMarginOrderCountUsage`.
pub type QueryCurrentMarginOrderCountUsageResponse =
    Vec<QueryCurrentMarginOrderCountUsageResponseItem>;

/// Provider-native `QueryMarginAccountsAllOcoResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsAllOcoResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryMarginAccountsAllOcoResponseItemOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginAccountsAllOcoResponseItemOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsAllOcoResponseItemOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsAllOco`.
pub type QueryMarginAccountsAllOcoResponse = Vec<QueryMarginAccountsAllOcoResponseItem>;

/// Provider-native `QueryMarginAccountsAllOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsAllOrdersResponseItem {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        deserialize_with = "super::wire::decimal"
    )]
    pub cummulative_quote_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `isWorking` wire field.
    #[serde(rename = "isWorking")]
    pub is_working: bool,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `side` wire field.
    #[serde(rename = "side")]
    pub side: String,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Exact `timeInForce` wire field.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsAllOrders`.
pub type QueryMarginAccountsAllOrdersResponse = Vec<QueryMarginAccountsAllOrdersResponseItem>;

/// Provider-native `QueryMarginAccountsOpenOcoResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOpenOcoResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryMarginAccountsOpenOcoResponseItemOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginAccountsOpenOcoResponseItemOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOpenOcoResponseItemOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsOpenOco`.
pub type QueryMarginAccountsOpenOcoResponse = Vec<QueryMarginAccountsOpenOcoResponseItem>;

/// Provider-native `QueryMarginAccountsOpenOtootocoOrderListsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOpenOtootocoOrderListsResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: crate::margin::OrderListId,
    /// Exact `contingencyType` wire field.
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    /// Exact `listStatusType` wire field.
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    /// Exact `listOrderStatus` wire field.
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: crate::margin::ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<QueryMarginAccountsOpenOtootocoOrderListsResponseItemOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginAccountsOpenOtootocoOrderListsResponseItemOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOpenOtootocoOrderListsResponseItemOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<crate::margin::OrderId>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: crate::margin::ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsOpenOtootocoOrderLists`.
pub type QueryMarginAccountsOpenOtootocoOrderListsResponse =
    Vec<QueryMarginAccountsOpenOtootocoOrderListsResponseItem>;

/// Provider-native `QueryMarginAccountsTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct QueryMarginAccountsTradeListResponseItem {
    /// Exact `commission` wire field.
    #[serde(rename = "commission", deserialize_with = "super::wire::decimal")]
    pub commission: Decimal,
    /// Exact `commissionAsset` wire field.
    #[serde(rename = "commissionAsset")]
    pub commission_asset: crate::Asset,
    /// Exact `id` wire field.
    #[serde(rename = "id")]
    pub id: crate::margin::TradeId,
    /// Exact `isBestMatch` wire field.
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
    /// Exact `isBuyer` wire field.
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,
    /// Exact `isMaker` wire field.
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: crate::margin::OrderId,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `qty` wire field.
    #[serde(rename = "qty", deserialize_with = "super::wire::decimal")]
    pub qty: Decimal,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `isIsolated` wire field.
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    /// Exact `time` wire field.
    #[serde(rename = "time")]
    pub time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsTradeList`.
pub type QueryMarginAccountsTradeListResponse = Vec<QueryMarginAccountsTradeListResponseItem>;

/// Provider-native `QueryPreventedMatchesResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPreventedMatchesResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<crate::margin::PreventedMatchId>,
    /// Exact `takerOrderId` wire field.
    #[serde(rename = "takerOrderId")]
    pub taker_order_id: crate::margin::OrderId,
    /// Exact `makerSymbol` wire field.
    #[serde(rename = "makerSymbol")]
    pub maker_symbol: crate::Symbol,
    /// Exact `makerOrderId` wire field.
    #[serde(rename = "makerOrderId")]
    pub maker_order_id: crate::margin::OrderId,
    /// Exact `tradeGroupId` wire field.
    #[serde(rename = "tradeGroupId")]
    pub trade_group_id: crate::margin::RecordId,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `price` wire field.
    #[serde(rename = "price", deserialize_with = "super::wire::decimal")]
    pub price: Decimal,
    /// Exact `makerPreventedQuantity` wire field.
    #[serde(
        rename = "makerPreventedQuantity",
        deserialize_with = "super::wire::decimal"
    )]
    pub maker_prevented_quantity: Decimal,
    /// Exact `transactTime` wire field.
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryPreventedMatches`.
pub type QueryPreventedMatchesResponse = Vec<QueryPreventedMatchesResponseItem>;

/// Provider-native `QuerySpecialKeyListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuerySpecialKeyListResponseItem {
    /// Exact `apiName` wire field.
    #[serde(rename = "apiName")]
    pub api_name: String,
    /// Exact `apiKey` wire field.
    #[serde(rename = "apiKey")]
    pub api_key: crate::SensitiveString,
    /// Exact `ip` wire field.
    #[serde(rename = "ip")]
    pub ip: String,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `permissionMode` wire field.
    #[serde(rename = "permissionMode")]
    pub permission_mode: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `querySpecialKeyList`.
pub type QuerySpecialKeyListResponse = Vec<QuerySpecialKeyListResponseItem>;

/// Provider-native `QueryLiquidationLoanResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryLiquidationLoanResponse {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `repaidAmount` wire field.
    #[serde(rename = "repaidAmount", deserialize_with = "super::wire::decimal")]
    pub repaid_amount: Decimal,
    /// Exact `remainingAmount` wire field.
    #[serde(rename = "remainingAmount", deserialize_with = "super::wire::decimal")]
    pub remaining_amount: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `LiquidationLoanRepayResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LiquidationLoanRepayResponse {
    /// Exact `repayId` wire field.
    #[serde(rename = "repayId")]
    pub repay_id: crate::margin::TransactionId,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `createTime` wire field.
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryLiquidationLoanRepayHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryLiquidationLoanRepayHistoryResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Exact `rows` wire field.
    #[serde(rename = "rows")]
    pub rows: Vec<QueryLiquidationLoanRepayHistoryResponseRowsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryLiquidationLoanRepayHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryLiquidationLoanRepayHistoryResponseRowsItem {
    /// Exact `repayId` wire field.
    #[serde(rename = "repayId")]
    pub repay_id: crate::margin::TransactionId,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `createTime` wire field.
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCrossMarginTransferHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCrossMarginTransferHistoryResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows")]
    pub rows: Vec<GetCrossMarginTransferHistoryResponseRowsItem>,
    /// Exact `total` wire field.
    #[serde(rename = "total")]
    pub total: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCrossMarginTransferHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCrossMarginTransferHistoryResponseRowsItem {
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    /// Exact `txId` wire field.
    #[serde(rename = "txId")]
    pub tx_id: crate::margin::TransactionId,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `transFrom` wire field.
    #[serde(rename = "transFrom")]
    pub trans_from: String,
    /// Exact `transTo` wire field.
    #[serde(rename = "transTo")]
    pub trans_to: String,
    /// Exact `fromSymbol` wire field.
    #[serde(rename = "fromSymbol")]
    pub from_symbol: Symbol,
    /// Exact `toSymbol` wire field.
    #[serde(rename = "toSymbol")]
    pub to_symbol: Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMaxTransferOutAmountResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMaxTransferOutAmountResponse {
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CreateUserListenTokenResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateUserListenTokenResponse {
    /// Exact `token` wire field.
    #[serde(rename = "token", deserialize_with = "super::wire::listen_token")]
    pub token: crate::SensitiveString,
    /// Exact `expirationTime` wire field.
    #[serde(
        rename = "expirationTime",
        deserialize_with = "super::wire::expiration_time"
    )]
    pub expiration_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
