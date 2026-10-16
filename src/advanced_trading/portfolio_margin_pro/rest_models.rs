// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::Decimal;
use crate::SensitiveString;
use serde::{Deserialize, Serialize};

/// Provider-native `BnbTransferResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BnbTransferResponse {
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ChangeAutoRepayFuturesStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeAutoRepayFuturesStatusResponse {
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetAutoRepayFuturesStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAutoRepayFuturesStatusResponse {
    /// Exact `autoRepay` wire field.
    #[serde(rename = "autoRepay", default, skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `FundAutoCollectionResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundAutoCollectionResponse {
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `FundCollectionByAssetResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundCollectionByAssetResponse {
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetDeltaModeStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDeltaModeStatusResponse {
    /// Exact `deltaEnabled` wire field.
    #[serde(
        rename = "deltaEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delta_enabled: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SwitchDeltaModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SwitchDeltaModeResponse {
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetPortfolioMarginProAccountBalanceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPortfolioMarginProAccountBalanceResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `totalWalletBalance` wire field.
    #[serde(
        rename = "totalWalletBalance",
        deserialize_with = "super::wire::decimal"
    )]
    pub total_wallet_balance: Decimal,
    /// Exact `crossMarginAsset` wire field.
    #[serde(
        rename = "crossMarginAsset",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_margin_asset: Option<Decimal>,
    /// Exact `crossMarginBorrowed` wire field.
    #[serde(
        rename = "crossMarginBorrowed",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_margin_borrowed: Option<Decimal>,
    /// Exact `crossMarginFree` wire field.
    #[serde(
        rename = "crossMarginFree",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_margin_free: Option<Decimal>,
    /// Exact `crossMarginInterest` wire field.
    #[serde(
        rename = "crossMarginInterest",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_margin_interest: Option<Decimal>,
    /// Exact `crossMarginLocked` wire field.
    #[serde(
        rename = "crossMarginLocked",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_margin_locked: Option<Decimal>,
    /// Exact `umWalletBalance` wire field.
    #[serde(
        rename = "umWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub um_wallet_balance: Option<Decimal>,
    /// Exact `umUnrealizedPNL` wire field.
    #[serde(
        rename = "umUnrealizedPNL",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub um_unrealized_pnl: Option<Decimal>,
    /// Exact `cmWalletBalance` wire field.
    #[serde(
        rename = "cmWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cm_wallet_balance: Option<Decimal>,
    /// Exact `cmUnrealizedPNL` wire field.
    #[serde(
        rename = "cmUnrealizedPNL",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::wire::decimal_empty_option"
    )]
    pub cm_unrealized_pnl: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `negativeBalance` wire field.
    #[serde(
        rename = "negativeBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub negative_balance: Option<Decimal>,
    /// Exact `optionWalletBalance` wire field.
    #[serde(
        rename = "optionWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_wallet_balance: Option<Decimal>,
    /// Exact `optionEquity` wire field.
    #[serde(
        rename = "optionEquity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_equity: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getPortfolioMarginProAccountBalance`.
pub type GetPortfolioMarginProAccountBalanceResponse =
    Vec<GetPortfolioMarginProAccountBalanceResponseItem>;

/// Provider-native `GetPortfolioMarginProAccountInfoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPortfolioMarginProAccountInfoResponse {
    /// Exact `uniMMR` wire field.
    #[serde(rename = "uniMMR", deserialize_with = "super::wire::decimal")]
    pub uni_mmr: Decimal,
    /// Exact `accountEquity` wire field.
    #[serde(rename = "accountEquity", deserialize_with = "super::wire::decimal")]
    pub account_equity: Decimal,
    /// Exact `actualEquity` wire field.
    #[serde(rename = "actualEquity", deserialize_with = "super::wire::decimal")]
    pub actual_equity: Decimal,
    /// Exact `accountMaintMargin` wire field.
    #[serde(
        rename = "accountMaintMargin",
        deserialize_with = "super::wire::decimal"
    )]
    pub account_maint_margin: Decimal,
    /// Exact `accountInitialMargin` wire field.
    #[serde(
        rename = "accountInitialMargin",
        deserialize_with = "super::wire::decimal"
    )]
    pub account_initial_margin: Decimal,
    /// Exact `accountStatus` wire field.
    #[serde(rename = "accountStatus")]
    pub account_status: String,
    /// Exact `accountType` wire field.
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetPortfolioMarginProSpanAccountInfoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPortfolioMarginProSpanAccountInfoResponse {
    /// Exact `uniMMR` wire field.
    #[serde(rename = "uniMMR", deserialize_with = "super::wire::decimal")]
    pub uni_mmr: Decimal,
    /// Exact `accountEquity` wire field.
    #[serde(rename = "accountEquity", deserialize_with = "super::wire::decimal")]
    pub account_equity: Decimal,
    /// Exact `actualEquity` wire field.
    #[serde(rename = "actualEquity", deserialize_with = "super::wire::decimal")]
    pub actual_equity: Decimal,
    /// Exact `accountMaintMargin` wire field.
    #[serde(
        rename = "accountMaintMargin",
        deserialize_with = "super::wire::decimal"
    )]
    pub account_maint_margin: Decimal,
    /// Exact `riskUnitMMList` wire field.
    #[serde(
        rename = "riskUnitMMList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_unit_mm_list:
        Option<Vec<GetPortfolioMarginProSpanAccountInfoResponseRiskUnitMmListItem>>,
    /// Exact `marginMM` wire field.
    #[serde(
        rename = "marginMM",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_mm: Option<Decimal>,
    /// Exact `otherMM` wire field.
    #[serde(
        rename = "otherMM",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_mm: Option<Decimal>,
    /// Exact `accountStatus` wire field.
    #[serde(rename = "accountStatus")]
    pub account_status: String,
    /// Exact `accountType` wire field.
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetPortfolioMarginProSpanAccountInfoResponseRiskUnitMmListItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPortfolioMarginProSpanAccountInfoResponseRiskUnitMmListItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `uniMaintainUsd` wire field.
    #[serde(
        rename = "uniMaintainUsd",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub uni_maintain_usd: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetTransferableEarnAssetBalanceForPortfolioMarginResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetTransferableEarnAssetBalanceForPortfolioMarginResponse {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PortfolioMarginProBankruptcyLoanRepayResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginProBankruptcyLoanRepayResponse {
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryPortfolioMarginProBankruptcyLoanAmountResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPortfolioMarginProBankruptcyLoanAmountResponse {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponseRowsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponseRowsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
    /// Exact `repayTime` wire field.
    #[serde(rename = "repayTime", default, skip_serializing_if = "Option::is_none")]
    pub repay_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryPortfolioMarginProNegativeBalanceInterestHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPortfolioMarginProNegativeBalanceInterestHistoryResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `interest` wire field.
    #[serde(
        rename = "interest",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest: Option<Decimal>,
    /// Exact `interestAccruedTime` wire field.
    #[serde(
        rename = "interestAccruedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_accrued_time: Option<i64>,
    /// Exact `interestRate` wire field.
    #[serde(
        rename = "interestRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_rate: Option<Decimal>,
    /// Exact `principal` wire field.
    #[serde(
        rename = "principal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub principal: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryPortfolioMarginProNegativeBalanceInterestHistory`.
pub type QueryPortfolioMarginProNegativeBalanceInterestHistoryResponse =
    Vec<QueryPortfolioMarginProNegativeBalanceInterestHistoryResponseItem>;

/// Provider-native `RepayFuturesNegativeBalanceResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepayFuturesNegativeBalanceResponse {
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TransferLdusdtRwusdForPortfolioMarginResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransferLdusdtRwusdForPortfolioMarginResponse {
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetMarginCallLevelResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetMarginCallLevelResponse {
    /// Exact `marginCallLevel` wire field.
    #[serde(
        rename = "marginCallLevel",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_level: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SetMarginCallLevelResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetMarginCallLevelResponse {
    /// Exact `marginCallLevel` wire field.
    #[serde(
        rename = "marginCallLevel",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_level: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DeleteMarginCallLevelResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeleteMarginCallLevelResponse {
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetPortfolioMarginAssetLeverageResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPortfolioMarginAssetLeverageResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getPortfolioMarginAssetLeverage`.
pub type GetPortfolioMarginAssetLeverageResponse = Vec<GetPortfolioMarginAssetLeverageResponseItem>;

/// Provider-native `PortfolioMarginCollateralRateResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginCollateralRateResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `collateralRate` wire field.
    #[serde(
        rename = "collateralRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub collateral_rate: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `portfolioMarginCollateralRate`.
pub type PortfolioMarginCollateralRateResponse = Vec<PortfolioMarginCollateralRateResponseItem>;

/// Provider-native `PortfolioMarginProTieredCollateralRateResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginProTieredCollateralRateResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `collateralInfo` wire field.
    #[serde(
        rename = "collateralInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collateral_info:
        Option<Vec<PortfolioMarginProTieredCollateralRateResponseItemCollateralInfoItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PortfolioMarginProTieredCollateralRateResponseItemCollateralInfoItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginProTieredCollateralRateResponseItemCollateralInfoItem {
    /// Exact `tierFloor` wire field.
    #[serde(
        rename = "tierFloor",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tier_floor: Option<Decimal>,
    /// Exact `tierCap` wire field.
    #[serde(
        rename = "tierCap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tier_cap: Option<Decimal>,
    /// Exact `collateralRate` wire field.
    #[serde(
        rename = "collateralRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub collateral_rate: Option<Decimal>,
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

/// Exact response for `portfolioMarginProTieredCollateralRate`.
pub type PortfolioMarginProTieredCollateralRateResponse =
    Vec<PortfolioMarginProTieredCollateralRateResponseItem>;

/// Provider-native `QueryPortfolioMarginAssetIndexPriceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPortfolioMarginAssetIndexPriceResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `assetIndexPrice` wire field.
    #[serde(
        rename = "assetIndexPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_index_price: Option<Decimal>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryPortfolioMarginAssetIndexPrice`.
pub type QueryPortfolioMarginAssetIndexPriceResponse =
    Vec<QueryPortfolioMarginAssetIndexPriceResponseItem>;
