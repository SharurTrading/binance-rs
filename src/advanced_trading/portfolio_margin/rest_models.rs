// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::ClientOrderId;
use crate::Decimal;
use crate::SensitiveString;
use crate::Symbol;
use serde::{Deserialize, Serialize};

/// Provider alternatives for `AccountBalanceResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum AccountBalanceResponse {
    /// Wire alternative 1.
    Variant1(Box<Vec<AccountBalanceResponseVariant1Item>>),
    /// Wire alternative 2.
    Variant2(Box<AccountBalanceResponseVariant2>),
}

/// Provider-native `AccountBalanceResponseVariant1Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountBalanceResponseVariant1Item {
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountBalanceResponseVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountBalanceResponseVariant2 {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `totalWalletBalance` wire field.
    #[serde(
        rename = "totalWalletBalance",
        deserialize_with = "super::wire::decimal"
    )]
    pub total_wallet_balance: Decimal,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountInformationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationResponse {
    /// Exact `uniMMR` wire field.
    #[serde(rename = "uniMMR", deserialize_with = "super::wire::decimal")]
    pub uni_mmr: Decimal,
    /// Exact `accountEquity` wire field.
    #[serde(rename = "accountEquity", deserialize_with = "super::wire::decimal")]
    pub account_equity: Decimal,
    /// Exact `actualEquity` wire field.
    #[serde(rename = "actualEquity", deserialize_with = "super::wire::decimal")]
    pub actual_equity: Decimal,
    /// Exact `accountInitialMargin` wire field.
    #[serde(
        rename = "accountInitialMargin",
        deserialize_with = "super::wire::decimal"
    )]
    pub account_initial_margin: Decimal,
    /// Exact `accountMaintMargin` wire field.
    #[serde(
        rename = "accountMaintMargin",
        deserialize_with = "super::wire::decimal"
    )]
    pub account_maint_margin: Decimal,
    /// Exact `accountStatus` wire field.
    #[serde(rename = "accountStatus")]
    pub account_status: String,
    /// Exact `virtualMaxWithdrawAmount` wire field.
    #[serde(
        rename = "virtualMaxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_max_withdraw_amount: Option<Decimal>,
    /// Exact `totalAvailableBalance` wire field.
    #[serde(
        rename = "totalAvailableBalance",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::wire::decimal_empty_option"
    )]
    pub total_available_balance: Option<Decimal>,
    /// Exact `totalMarginOpenLoss` wire field.
    #[serde(
        rename = "totalMarginOpenLoss",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::wire::decimal_empty_option"
    )]
    pub total_margin_open_loss: Option<Decimal>,
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

/// Provider-native `ChangeCmInitialLeverageResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeCmInitialLeverageResponse {
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
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ChangeCmPositionModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeCmPositionModeResponse {
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

/// Provider-native `GetCmCurrentPositionModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCmCurrentPositionModeResponse {
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

/// Provider-native `ChangeUmInitialLeverageResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeUmInitialLeverageResponse {
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i64>,
    /// Exact `maxNotionalValue` wire field.
    #[serde(
        rename = "maxNotionalValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_notional_value: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ChangeUmPositionModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeUmPositionModeResponse {
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

/// Provider-native `GetUmCurrentPositionModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmCurrentPositionModeResponse {
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

/// Provider-native `CmNotionalAndLeverageBracketsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CmNotionalAndLeverageBracketsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `brackets` wire field.
    #[serde(rename = "brackets", default, skip_serializing_if = "Option::is_none")]
    pub brackets: Option<Vec<CmNotionalAndLeverageBracketsResponseItemBracketsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CmNotionalAndLeverageBracketsResponseItemBracketsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CmNotionalAndLeverageBracketsResponseItemBracketsItem {
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
    /// Exact `qtyFloor` wire field.
    #[serde(
        rename = "qtyFloor",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_floor: Option<Decimal>,
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

/// Exact response for `cmNotionalAndLeverageBrackets`.
pub type CmNotionalAndLeverageBracketsResponse = Vec<CmNotionalAndLeverageBracketsResponseItem>;

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

/// Provider-native `GetCmAccountDetailResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCmAccountDetailResponse {
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<GetCmAccountDetailResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<GetCmAccountDetailResponsePositionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCmAccountDetailResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCmAccountDetailResponseAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
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

/// Provider-native `GetCmAccountDetailResponsePositionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCmAccountDetailResponsePositionsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `positionAmt` wire field.
    #[serde(rename = "positionAmt", deserialize_with = "super::wire::decimal")]
    pub position_amt: Decimal,
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
    /// Exact `positionSide` wire field.
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Exact `entryPrice` wire field.
    #[serde(rename = "entryPrice", deserialize_with = "super::wire::decimal")]
    pub entry_price: Decimal,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCmIncomeHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCmIncomeHistoryResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
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

/// Exact response for `getCmIncomeHistory`.
pub type GetCmIncomeHistoryResponse = Vec<GetCmIncomeHistoryResponseItem>;

/// Provider-native `GetDownloadIdForUmFuturesOrderHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDownloadIdForUmFuturesOrderHistoryResponse {
    /// Exact `avgCostTimestampOfLast30d` wire field.
    #[serde(
        rename = "avgCostTimestampOfLast30d",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    /// Exact `downloadId` wire field.
    #[serde(rename = "downloadId")]
    pub download_id: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetDownloadIdForUmFuturesTradeHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDownloadIdForUmFuturesTradeHistoryResponse {
    /// Exact `avgCostTimestampOfLast30d` wire field.
    #[serde(
        rename = "avgCostTimestampOfLast30d",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    /// Exact `downloadId` wire field.
    #[serde(rename = "downloadId")]
    pub download_id: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetDownloadIdForUmFuturesTransactionHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetDownloadIdForUmFuturesTransactionHistoryResponse {
    /// Exact `avgCostTimestampOfLast30d` wire field.
    #[serde(
        rename = "avgCostTimestampOfLast30d",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    /// Exact `downloadId` wire field.
    #[serde(rename = "downloadId")]
    pub download_id: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetMarginBorrowLoanInterestHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetMarginBorrowLoanInterestHistoryResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<GetMarginBorrowLoanInterestHistoryResponseRowsItem>>,
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetMarginBorrowLoanInterestHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetMarginBorrowLoanInterestHistoryResponseRowsItem {
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<i64>,
    /// Exact `interestAccuredTime` wire field.
    #[serde(
        rename = "interestAccuredTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_accured_time: Option<i64>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `rawAsset` wire field.
    #[serde(rename = "rawAsset", default, skip_serializing_if = "Option::is_none")]
    pub raw_asset: Option<crate::Asset>,
    /// Exact `principal` wire field.
    #[serde(
        rename = "principal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub principal: Option<Decimal>,
    /// Exact `interest` wire field.
    #[serde(
        rename = "interest",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest: Option<Decimal>,
    /// Exact `interestRate` wire field.
    #[serde(
        rename = "interestRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_rate: Option<Decimal>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetUmAccountDetailResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmAccountDetailResponse {
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<GetUmAccountDetailResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<GetUmAccountDetailResponsePositionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetUmAccountDetailResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmAccountDetailResponseAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
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

/// Provider-native `GetUmAccountDetailResponsePositionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmAccountDetailResponsePositionsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `entryPrice` wire field.
    #[serde(rename = "entryPrice", deserialize_with = "super::wire::decimal")]
    pub entry_price: Decimal,
    /// Exact `maxNotional` wire field.
    #[serde(
        rename = "maxNotional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_notional: Option<Decimal>,
    /// Exact `bidNotional` wire field.
    #[serde(
        rename = "bidNotional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_notional: Option<Decimal>,
    /// Exact `askNotional` wire field.
    #[serde(
        rename = "askNotional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_notional: Option<Decimal>,
    /// Exact `positionSide` wire field.
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Exact `positionAmt` wire field.
    #[serde(rename = "positionAmt", deserialize_with = "super::wire::decimal")]
    pub position_amt: Decimal,
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

/// Provider-native `GetUmAccountDetailV2Response` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmAccountDetailV2Response {
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<GetUmAccountDetailV2ResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<GetUmAccountDetailV2ResponsePositionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetUmAccountDetailV2ResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmAccountDetailV2ResponseAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
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

/// Provider-native `GetUmAccountDetailV2ResponsePositionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmAccountDetailV2ResponsePositionsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `positionSide` wire field.
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Exact `positionAmt` wire field.
    #[serde(rename = "positionAmt", deserialize_with = "super::wire::decimal")]
    pub position_amt: Decimal,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetUmFuturesOrderDownloadLinkByIdResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmFuturesOrderDownloadLinkByIdResponse {
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `url` wire field.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<crate::SensitiveString>,
    /// Exact `s3Link` wire field.
    #[serde(rename = "s3Link", default, skip_serializing_if = "Option::is_none")]
    pub s3_link: Option<crate::SensitiveString>,
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

/// Provider-native `GetUmFuturesTradeDownloadLinkByIdResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmFuturesTradeDownloadLinkByIdResponse {
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `url` wire field.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<crate::SensitiveString>,
    /// Exact `s3Link` wire field.
    #[serde(rename = "s3Link", default, skip_serializing_if = "Option::is_none")]
    pub s3_link: Option<crate::SensitiveString>,
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

/// Provider-native `GetUmFuturesTransactionDownloadLinkByIdResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmFuturesTransactionDownloadLinkByIdResponse {
    /// Exact `downloadId` wire field.
    #[serde(
        rename = "downloadId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub download_id: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `url` wire field.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<crate::SensitiveString>,
    /// Exact `s3Link` wire field.
    #[serde(rename = "s3Link", default, skip_serializing_if = "Option::is_none")]
    pub s3_link: Option<crate::SensitiveString>,
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

/// Provider-native `GetUmIncomeHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmIncomeHistoryResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
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

/// Exact response for `getUmIncomeHistory`.
pub type GetUmIncomeHistoryResponse = Vec<GetUmIncomeHistoryResponseItem>;

/// Provider-native `GetUserCommissionRateForCmResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUserCommissionRateForCmResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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

/// Provider-native `GetUserCommissionRateForUmResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUserCommissionRateForUmResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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

/// Provider-native `MarginMaxBorrowResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginMaxBorrowResponse {
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
    /// Exact `borrowLimit` wire field.
    #[serde(
        rename = "borrowLimit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub borrow_limit: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponse {
    /// Exact `indicators` wire field.
    #[serde(
        rename = "indicators",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub indicators: Option<PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicators>,
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

/// Provider-native `PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicators` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicators {
    /// Exact `BTCUSDT` wire field.
    #[serde(rename = "BTCUSDT", default, skip_serializing_if = "Option::is_none")]
    pub upper_btcusdt: Option<
        Vec<PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperBtcusdtItem>,
    >,
    /// Exact `ACCOUNT` wire field.
    #[serde(rename = "ACCOUNT", default, skip_serializing_if = "Option::is_none")]
    pub upper_account: Option<
        Vec<PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperAccountItem>,
    >,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperBtcusdtItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperBtcusdtItem {
    /// Exact `isLocked` wire field.
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// Exact `plannedRecoverTime` wire field.
    #[serde(
        rename = "plannedRecoverTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub planned_recover_time: Option<i64>,
    /// Exact `indicator` wire field.
    #[serde(rename = "indicator", default, skip_serializing_if = "Option::is_none")]
    pub indicator: Option<String>,
    /// Exact `value` wire field.
    #[serde(
        rename = "value",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Decimal>,
    /// Exact `triggerValue` wire field.
    #[serde(
        rename = "triggerValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_value: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperAccountItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperAccountItem {
    /// Exact `indicator` wire field.
    #[serde(rename = "indicator", default, skip_serializing_if = "Option::is_none")]
    pub indicator: Option<String>,
    /// Exact `value` wire field.
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// Exact `triggerValue` wire field.
    #[serde(
        rename = "triggerValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_value: Option<i64>,
    /// Exact `plannedRecoverTime` wire field.
    #[serde(
        rename = "plannedRecoverTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub planned_recover_time: Option<i64>,
    /// Exact `isLocked` wire field.
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCmPositionInformationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCmPositionInformationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `positionAmt` wire field.
    #[serde(rename = "positionAmt", deserialize_with = "super::wire::decimal")]
    pub position_amt: Decimal,
    /// Exact `entryPrice` wire field.
    #[serde(rename = "entryPrice", deserialize_with = "super::wire::decimal")]
    pub entry_price: Decimal,
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
    /// Exact `positionSide` wire field.
    #[serde(rename = "positionSide")]
    pub position_side: String,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `maxQty` wire field.
    #[serde(
        rename = "maxQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_qty: Option<Decimal>,
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

/// Exact response for `queryCmPositionInformation`.
pub type QueryCmPositionInformationResponse = Vec<QueryCmPositionInformationResponseItem>;

/// Provider-native `QueryMarginLoanRecordResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginLoanRecordResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<QueryMarginLoanRecordResponseRowsItem>>,
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginLoanRecordResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginLoanRecordResponseRowsItem {
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<i64>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `principal` wire field.
    #[serde(
        rename = "principal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub principal: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginMaxWithdrawResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginMaxWithdrawResponse {
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

/// Provider-native `QueryMarginRepayRecordResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginRepayRecordResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<QueryMarginRepayRecordResponseRowsItem>>,
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginRepayRecordResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginRepayRecordResponseRowsItem {
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
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
    /// Exact `principal` wire field.
    #[serde(
        rename = "principal",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub principal: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryPortfolioMarginNegativeBalanceInterestHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryPortfolioMarginNegativeBalanceInterestHistoryResponseItem {
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
    /// Exact `interestAccuredTime` wire field.
    #[serde(
        rename = "interestAccuredTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_accured_time: Option<i64>,
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

/// Exact response for `queryPortfolioMarginNegativeBalanceInterestHistory`.
pub type QueryPortfolioMarginNegativeBalanceInterestHistoryResponse =
    Vec<QueryPortfolioMarginNegativeBalanceInterestHistoryResponseItem>;

/// Provider-native `QueryUmPositionInformationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUmPositionInformationResponseItem {
    /// Exact `entryPrice` wire field.
    #[serde(rename = "entryPrice", deserialize_with = "super::wire::decimal")]
    pub entry_price: Decimal,
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Exact `markPrice` wire field.
    #[serde(
        rename = "markPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_price: Option<Decimal>,
    /// Exact `maxNotionalValue` wire field.
    #[serde(
        rename = "maxNotionalValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_notional_value: Option<Decimal>,
    /// Exact `positionAmt` wire field.
    #[serde(rename = "positionAmt", deserialize_with = "super::wire::decimal")]
    pub position_amt: Decimal,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `positionSide` wire field.
    #[serde(rename = "positionSide")]
    pub position_side: String,
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

/// Exact response for `queryUmPositionInformation`.
pub type QueryUmPositionInformationResponse = Vec<QueryUmPositionInformationResponseItem>;

/// Provider-native `QueryUserNegativeBalanceAutoExchangeRecordResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserNegativeBalanceAutoExchangeRecordResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<QueryUserNegativeBalanceAutoExchangeRecordResponseRowsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserNegativeBalanceAutoExchangeRecordResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserNegativeBalanceAutoExchangeRecordResponseRowsItem {
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `details` wire field.
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<QueryUserNegativeBalanceAutoExchangeRecordResponseRowsItemDetailsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserNegativeBalanceAutoExchangeRecordResponseRowsItemDetailsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserNegativeBalanceAutoExchangeRecordResponseRowsItemDetailsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `negativeBalance` wire field.
    #[serde(
        rename = "negativeBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub negative_balance: Option<Decimal>,
    /// Exact `negativeMaxThreshold` wire field.
    #[serde(
        rename = "negativeMaxThreshold",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub negative_max_threshold: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserRateLimitResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserRateLimitResponseItem {
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

/// Exact response for `queryUserRateLimit`.
pub type QueryUserRateLimitResponse = Vec<QueryUserRateLimitResponseItem>;

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

/// Provider-native `UmFuturesAccountConfigurationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent provider wire flags must retain their native meaning"
)]
pub struct UmFuturesAccountConfigurationResponse {
    /// Exact `feeTier` wire field.
    #[serde(rename = "feeTier", default, skip_serializing_if = "Option::is_none")]
    pub fee_tier: Option<i64>,
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
    /// Exact `dualSidePosition` wire field.
    #[serde(
        rename = "dualSidePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dual_side_position: Option<bool>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `multiAssetsMargin` wire field.
    #[serde(
        rename = "multiAssetsMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multi_assets_margin: Option<bool>,
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

/// Provider-native `UmFuturesSymbolConfigurationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UmFuturesSymbolConfigurationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `marginType` wire field.
    #[serde(
        rename = "marginType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_type: Option<String>,
    /// Exact `isAutoAddMargin` wire field.
    #[serde(
        rename = "isAutoAddMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_auto_add_margin: Option<String>,
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i64>,
    /// Exact `maxNotionalValue` wire field.
    #[serde(
        rename = "maxNotionalValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_notional_value: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `umFuturesSymbolConfiguration`.
pub type UmFuturesSymbolConfigurationResponse = Vec<UmFuturesSymbolConfigurationResponseItem>;

/// Provider-native `UmNotionalAndLeverageBracketsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UmNotionalAndLeverageBracketsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    pub brackets: Option<Vec<UmNotionalAndLeverageBracketsResponseItemBracketsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UmNotionalAndLeverageBracketsResponseItemBracketsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UmNotionalAndLeverageBracketsResponseItemBracketsItem {
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
    /// Exact `notionalCap` wire field.
    #[serde(
        rename = "notionalCap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional_cap: Option<Decimal>,
    /// Exact `notionalFloor` wire field.
    #[serde(
        rename = "notionalFloor",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional_floor: Option<Decimal>,
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

/// Exact response for `umNotionalAndLeverageBrackets`.
pub type UmNotionalAndLeverageBracketsResponse = Vec<UmNotionalAndLeverageBracketsResponseItem>;

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TestConnectivityResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelAllCmOpenConditionalOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllCmOpenConditionalOrdersResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `msg` wire field.
    #[serde(rename = "msg")]
    pub msg: SensitiveString,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelAllCmOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllCmOpenOrdersResponse {
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

/// Provider-native `CancelAllUmOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllUmOpenOrdersResponse {
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

/// Provider-native `CancelCmConditionalOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelCmConditionalOrderResponse {
    /// Exact `newClientStrategyId` wire field.
    #[serde(
        rename = "newClientStrategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_client_strategy_id: Option<crate::ClientOrderId>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyStatus` wire field.
    #[serde(
        rename = "strategyStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_status: Option<String>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
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
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewCmConditionalOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewCmConditionalOrderResponse {
    /// Exact `newClientStrategyId` wire field.
    #[serde(
        rename = "newClientStrategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_client_strategy_id: Option<crate::ClientOrderId>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyStatus` wire field.
    #[serde(
        rename = "strategyStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_status: Option<String>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
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
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelCmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelCmOrderResponse {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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

/// Provider-native `ModifyCmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModifyCmOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
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
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
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

/// Provider-native `NewCmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewCmOrderResponse {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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

/// Provider-native `QueryCmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCmOrderResponse {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
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

/// Provider-native `CancelMarginAccountAllOpenOrdersOnASymbolResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMarginAccountAllOpenOrdersOnASymbolResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `origClientOrderId` wire field.
    #[serde(
        rename = "origClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id: Option<ClientOrderId>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<String>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<String>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<String>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<CancelMarginAccountAllOpenOrdersOnASymbolResponseItemOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports:
        Option<Vec<CancelMarginAccountAllOpenOrdersOnASymbolResponseItemOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelMarginAccountAllOpenOrdersOnASymbolResponseItemOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMarginAccountAllOpenOrdersOnASymbolResponseItemOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelMarginAccountAllOpenOrdersOnASymbolResponseItemOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMarginAccountAllOpenOrdersOnASymbolResponseItemOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `origClientOrderId` wire field.
    #[serde(
        rename = "origClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id: Option<ClientOrderId>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `orderListId` wire field.
    #[serde(
        rename = "orderListId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_list_id: Option<i64>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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

/// Exact response for `cancelMarginAccountAllOpenOrdersOnASymbol`.
pub type CancelMarginAccountAllOpenOrdersOnASymbolResponse =
    Vec<CancelMarginAccountAllOpenOrdersOnASymbolResponseItem>;

/// Provider-native `CancelMarginAccountOcoOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMarginAccountOcoOrdersResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<String>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<String>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<String>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<CancelMarginAccountOcoOrdersResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<CancelMarginAccountOcoOrdersResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelMarginAccountOcoOrdersResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMarginAccountOcoOrdersResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelMarginAccountOcoOrdersResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMarginAccountOcoOrdersResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `origClientOrderId` wire field.
    #[serde(
        rename = "origClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id: Option<ClientOrderId>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `orderListId` wire field.
    #[serde(
        rename = "orderListId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_list_id: Option<i64>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<String>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<String>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<String>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelMarginAccountOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelMarginAccountOrderResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origClientOrderId` wire field.
    #[serde(
        rename = "origClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id: Option<ClientOrderId>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewMarginOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewMarginOrderResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `transactTime` wire field.
    #[serde(
        rename = "transactTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transact_time: Option<i64>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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
    pub fills: Option<Vec<NewMarginOrderResponseFillsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewMarginOrderResponseFillsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewMarginOrderResponseFillsItem {
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginAccountOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountOrderResponse {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
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
    #[serde(rename = "isWorking", default, skip_serializing_if = "Option::is_none")]
    pub is_working: Option<bool>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `accountId` wire field.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<String>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option_null_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelUmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelUmOrderResponse {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
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
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ModifyUmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModifyUmOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
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
    /// Exact `origType` wire field.
    #[serde(rename = "origType", default, skip_serializing_if = "Option::is_none")]
    pub orig_type: Option<String>,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `NewUmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewUmOrderResponse {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUmOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUmOrderResponse {
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
    /// Exact `cumQuote` wire field.
    #[serde(
        rename = "cumQuote",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_quote: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
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
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CmAccountTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CmAccountTradeListResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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

/// Exact response for `cmAccountTradeList`.
pub type CmAccountTradeListResponse = Vec<CmAccountTradeListResponseItem>;

/// Provider-native `CmPositionAdlQuantileEstimationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CmPositionAdlQuantileEstimationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `adlQuantile` wire field.
    #[serde(
        rename = "adlQuantile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub adl_quantile: Option<CmPositionAdlQuantileEstimationResponseItemAdlQuantile>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CmPositionAdlQuantileEstimationResponseItemAdlQuantile` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CmPositionAdlQuantileEstimationResponseItemAdlQuantile {
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

/// Exact response for `cmPositionAdlQuantileEstimation`.
pub type CmPositionAdlQuantileEstimationResponse = Vec<CmPositionAdlQuantileEstimationResponseItem>;

/// Provider-native `GetUmFuturesBnbBurnStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetUmFuturesBnbBurnStatusResponse {
    /// Exact `feeBurn` wire field.
    #[serde(rename = "feeBurn", default, skip_serializing_if = "Option::is_none")]
    pub fee_burn: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ToggleBnbBurnOnUmFuturesTradeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ToggleBnbBurnOnUmFuturesTradeResponse {
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

/// Provider-native `MarginAccountBorrowResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountBorrowResponse {
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId")]
    pub tran_id: i64,
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
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<String>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<String>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<String>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
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
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
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
    pub order_id: i64,
    /// Exact `orderListId` wire field.
    #[serde(
        rename = "orderListId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_list_id: Option<i64>,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `transactTime` wire field.
    #[serde(
        rename = "transactTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transact_time: Option<i64>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountRepayResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountRepayResponse {
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountRepayDebtResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountRepayDebtResponse {
    /// Exact `amount` wire field.
    #[serde(
        rename = "amount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount: Option<Decimal>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `specifyRepayAssets` wire field.
    #[serde(
        rename = "specifyRepayAssets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub specify_repay_assets: Option<Vec<String>>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MarginAccountTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginAccountTradeListResponseItem {
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
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Exact `isBestMatch` wire field.
    #[serde(
        rename = "isBestMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_best_match: Option<bool>,
    /// Exact `isBuyer` wire field.
    #[serde(rename = "isBuyer", default, skip_serializing_if = "Option::is_none")]
    pub is_buyer: Option<bool>,
    /// Exact `isMaker` wire field.
    #[serde(rename = "isMaker", default, skip_serializing_if = "Option::is_none")]
    pub is_maker: Option<bool>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
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
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `marginAccountTradeList`.
pub type MarginAccountTradeListResponse = Vec<MarginAccountTradeListResponseItem>;

/// Provider-native `QueryAllCmConditionalOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllCmConditionalOrdersResponseItem {
    /// Exact `newClientStrategyId` wire field.
    #[serde(
        rename = "newClientStrategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_client_strategy_id: Option<crate::ClientOrderId>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyStatus` wire field.
    #[serde(
        rename = "strategyStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_status: Option<String>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `triggerTime` wire field.
    #[serde(
        rename = "triggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_time: Option<i64>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryAllCmConditionalOrders`.
pub type QueryAllCmConditionalOrdersResponse = Vec<QueryAllCmConditionalOrdersResponseItem>;

/// Provider-native `QueryAllCmOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllCmOrdersResponseItem {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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

/// Exact response for `queryAllCmOrders`.
pub type QueryAllCmOrdersResponse = Vec<QueryAllCmOrdersResponseItem>;

/// Provider-native `QueryAllCurrentCmOpenConditionalOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllCurrentCmOpenConditionalOrdersResponseItem {
    /// Exact `newClientStrategyId` wire field.
    #[serde(
        rename = "newClientStrategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_client_strategy_id: Option<crate::ClientOrderId>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyStatus` wire field.
    #[serde(
        rename = "strategyStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_status: Option<String>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryAllCurrentCmOpenConditionalOrders`.
pub type QueryAllCurrentCmOpenConditionalOrdersResponse =
    Vec<QueryAllCurrentCmOpenConditionalOrdersResponseItem>;

/// Provider-native `QueryAllCurrentCmOpenOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllCurrentCmOpenOrdersResponseItem {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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

/// Exact response for `queryAllCurrentCmOpenOrders`.
pub type QueryAllCurrentCmOpenOrdersResponse = Vec<QueryAllCurrentCmOpenOrdersResponseItem>;

/// Provider-native `QueryAllCurrentUmOpenOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllCurrentUmOpenOrdersResponseItem {
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
    /// Exact `cumQuote` wire field.
    #[serde(
        rename = "cumQuote",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_quote: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
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
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryAllCurrentUmOpenOrders`.
pub type QueryAllCurrentUmOpenOrdersResponse = Vec<QueryAllCurrentUmOpenOrdersResponseItem>;

/// Provider-native `QueryAllMarginAccountOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllMarginAccountOrdersResponseItem {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
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
    #[serde(rename = "isWorking", default, skip_serializing_if = "Option::is_none")]
    pub is_working: Option<bool>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `accountId` wire field.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<String>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option_null_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryAllMarginAccountOrders`.
pub type QueryAllMarginAccountOrdersResponse = Vec<QueryAllMarginAccountOrdersResponseItem>;

/// Provider-native `QueryAllUmOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllUmOrdersResponseItem {
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
    /// Exact `cumQuote` wire field.
    #[serde(
        rename = "cumQuote",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_quote: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
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
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryAllUmOrders`.
pub type QueryAllUmOrdersResponse = Vec<QueryAllUmOrdersResponseItem>;

/// Provider-native `QueryCmConditionalOrderHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCmConditionalOrderHistoryResponse {
    /// Exact `newClientStrategyId` wire field.
    #[serde(
        rename = "newClientStrategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_client_strategy_id: Option<crate::ClientOrderId>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyStatus` wire field.
    #[serde(
        rename = "strategyStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_status: Option<String>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `triggerTime` wire field.
    #[serde(
        rename = "triggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_time: Option<i64>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCmModifyOrderHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCmModifyOrderHistoryResponseItem {
    /// Exact `amendmentId` wire field.
    #[serde(
        rename = "amendmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub amendment_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `amendment` wire field.
    #[serde(rename = "amendment", default, skip_serializing_if = "Option::is_none")]
    pub amendment: Option<QueryCmModifyOrderHistoryResponseItemAmendment>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCmModifyOrderHistoryResponseItemAmendment` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCmModifyOrderHistoryResponseItemAmendment {
    /// Exact `price` wire field.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<QueryCmModifyOrderHistoryResponseItemAmendmentPrice>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty")]
    pub orig_qty: QueryCmModifyOrderHistoryResponseItemAmendmentOrigQty,
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

/// Provider-native `QueryCmModifyOrderHistoryResponseItemAmendmentPrice` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCmModifyOrderHistoryResponseItemAmendmentPrice {
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

/// Provider-native `QueryCmModifyOrderHistoryResponseItemAmendmentOrigQty` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCmModifyOrderHistoryResponseItemAmendmentOrigQty {
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

/// Exact response for `queryCmModifyOrderHistory`.
pub type QueryCmModifyOrderHistoryResponse = Vec<QueryCmModifyOrderHistoryResponseItem>;

/// Provider-native `QueryCurrentCmOpenConditionalOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentCmOpenConditionalOrderResponse {
    /// Exact `newClientStrategyId` wire field.
    #[serde(
        rename = "newClientStrategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_client_strategy_id: Option<crate::ClientOrderId>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyStatus` wire field.
    #[serde(
        rename = "strategyStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_status: Option<String>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<String>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    /// Exact `bookTime` wire field.
    #[serde(rename = "bookTime", default, skip_serializing_if = "Option::is_none")]
    pub book_time: Option<i64>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryCurrentCmOpenOrderResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentCmOpenOrderResponseItem {
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
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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

/// Exact response for `queryCurrentCmOpenOrder`.
pub type QueryCurrentCmOpenOrderResponse = Vec<QueryCurrentCmOpenOrderResponseItem>;

/// Provider-native `QueryCurrentMarginOpenOrderResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentMarginOpenOrderResponseItem {
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `cummulativeQuoteQty` wire field.
    #[serde(
        rename = "cummulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cummulative_quote_qty: Option<Decimal>,
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
    #[serde(rename = "isWorking", default, skip_serializing_if = "Option::is_none")]
    pub is_working: Option<bool>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `accountId` wire field.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<String>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option_null_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryCurrentMarginOpenOrder`.
pub type QueryCurrentMarginOpenOrderResponse = Vec<QueryCurrentMarginOpenOrderResponseItem>;

/// Provider-native `QueryCurrentUmOpenOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentUmOpenOrderResponse {
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
    /// Exact `cumQuote` wire field.
    #[serde(
        rename = "cumQuote",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_quote: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
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
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
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
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryMarginAccountsAllOcoResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsAllOcoResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<String>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<String>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<String>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsAllOco`.
pub type QueryMarginAccountsAllOcoResponse = Vec<QueryMarginAccountsAllOcoResponseItem>;

/// Provider-native `QueryMarginAccountsOpenOcoResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryMarginAccountsOpenOcoResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<String>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<String>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<String>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
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
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryMarginAccountsOpenOco`.
pub type QueryMarginAccountsOpenOcoResponse = Vec<QueryMarginAccountsOpenOcoResponseItem>;

/// Provider-native `QueryUmModifyOrderHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUmModifyOrderHistoryResponseItem {
    /// Exact `amendmentId` wire field.
    #[serde(
        rename = "amendmentId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub amendment_id: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `amendment` wire field.
    #[serde(rename = "amendment", default, skip_serializing_if = "Option::is_none")]
    pub amendment: Option<QueryUmModifyOrderHistoryResponseItemAmendment>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUmModifyOrderHistoryResponseItemAmendment` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUmModifyOrderHistoryResponseItemAmendment {
    /// Exact `price` wire field.
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<QueryUmModifyOrderHistoryResponseItemAmendmentPrice>,
    /// Exact `origQty` wire field.
    #[serde(rename = "origQty")]
    pub orig_qty: QueryUmModifyOrderHistoryResponseItemAmendmentOrigQty,
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

/// Provider-native `QueryUmModifyOrderHistoryResponseItemAmendmentPrice` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUmModifyOrderHistoryResponseItemAmendmentPrice {
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

/// Provider-native `QueryUmModifyOrderHistoryResponseItemAmendmentOrigQty` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUmModifyOrderHistoryResponseItemAmendmentOrigQty {
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

/// Exact response for `queryUmModifyOrderHistory`.
pub type QueryUmModifyOrderHistoryResponse = Vec<QueryUmModifyOrderHistoryResponseItem>;

/// Provider-native `QueryUsersCmForceOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUsersCmForceOrdersResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
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
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `cumBase` wire field.
    #[serde(
        rename = "cumBase",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_base: Option<Decimal>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryUsersCmForceOrders`.
pub type QueryUsersCmForceOrdersResponse = Vec<QueryUsersCmForceOrdersResponseItem>;

/// Provider-native `QueryUsersMarginForceOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUsersMarginForceOrdersResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<QueryUsersMarginForceOrdersResponseRowsItem>>,
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUsersMarginForceOrdersResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUsersMarginForceOrdersResponseRowsItem {
    /// Exact `avgPrice` wire field.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_price: Option<Decimal>,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
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
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `updatedTime` wire field.
    #[serde(
        rename = "updatedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUsersUmForceOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUsersUmForceOrdersResponseItem {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `status` wire field.
    #[serde(rename = "status")]
    pub status: String,
    /// Exact `clientOrderId` wire field.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: ClientOrderId,
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
    #[serde(rename = "origQty", deserialize_with = "super::wire::decimal")]
    pub orig_qty: Decimal,
    /// Exact `executedQty` wire field.
    #[serde(rename = "executedQty", deserialize_with = "super::wire::decimal")]
    pub executed_qty: Decimal,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryUsersUmForceOrders`.
pub type QueryUsersUmForceOrdersResponse = Vec<QueryUsersUmForceOrdersResponseItem>;

/// Provider-native `UmAccountTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UmAccountTradeListResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
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
    /// Exact `buyer` wire field.
    #[serde(rename = "buyer", default, skip_serializing_if = "Option::is_none")]
    pub buyer: Option<bool>,
    /// Exact `maker` wire field.
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<bool>,
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

/// Exact response for `umAccountTradeList`.
pub type UmAccountTradeListResponse = Vec<UmAccountTradeListResponseItem>;

/// Provider-native `UmPositionAdlQuantileEstimationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UmPositionAdlQuantileEstimationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `adlQuantile` wire field.
    #[serde(
        rename = "adlQuantile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub adl_quantile: Option<UmPositionAdlQuantileEstimationResponseItemAdlQuantile>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UmPositionAdlQuantileEstimationResponseItemAdlQuantile` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UmPositionAdlQuantileEstimationResponseItemAdlQuantile {
    /// Exact `LONG` wire field.
    #[serde(rename = "LONG", default, skip_serializing_if = "Option::is_none")]
    pub upper_long: Option<i64>,
    /// Exact `SHORT` wire field.
    #[serde(rename = "SHORT", default, skip_serializing_if = "Option::is_none")]
    pub upper_short: Option<i64>,
    /// Exact `BOTH` wire field.
    #[serde(rename = "BOTH", default, skip_serializing_if = "Option::is_none")]
    pub upper_both: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `umPositionAdlQuantileEstimation`.
pub type UmPositionAdlQuantileEstimationResponse = Vec<UmPositionAdlQuantileEstimationResponseItem>;

/// Provider-native `NewUmAlgoOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewUmAlgoOrderResponse {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: ClientOrderId,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
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
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `triggerPrice` wire field.
    #[serde(
        rename = "triggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_price: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
    /// Exact `reduceOnly` wire field.
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    /// Exact `activatePrice` wire field.
    #[serde(
        rename = "activatePrice",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::wire::decimal_empty_option"
    )]
    pub activate_price: Option<Decimal>,
    /// Exact `callbackRate` wire field.
    #[serde(
        rename = "callbackRate",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::wire::decimal_empty_option"
    )]
    pub callback_rate: Option<Decimal>,
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
    /// Exact `triggerTime` wire field.
    #[serde(
        rename = "triggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_time: Option<i64>,
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

/// Provider-native `CancelUmAlgoOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelUmAlgoOrderResponse {
    /// Exact `complete` wire field.
    #[serde(rename = "complete", default, skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CancelAllUmAlgoOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllUmAlgoOpenOrdersResponse {
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

/// Provider-native `QueryCurrentUmOpenAlgoOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryCurrentUmOpenAlgoOrderResponse {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: ClientOrderId,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
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
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `actualOrderId` wire field.
    #[serde(
        rename = "actualOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_order_id: Option<String>,
    /// Exact `actualPrice` wire field.
    #[serde(
        rename = "actualPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_price: Option<Decimal>,
    /// Exact `triggerPrice` wire field.
    #[serde(
        rename = "triggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_price: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
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
    /// Exact `triggerTime` wire field.
    #[serde(
        rename = "triggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_time: Option<i64>,
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

/// Provider-native `QueryAllCurrentUmOpenAlgoOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllCurrentUmOpenAlgoOrdersResponseItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: ClientOrderId,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
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
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `triggerPrice` wire field.
    #[serde(
        rename = "triggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_price: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
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
    /// Exact `triggerTime` wire field.
    #[serde(
        rename = "triggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_time: Option<i64>,
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

/// Exact response for `queryAllCurrentUmOpenAlgoOrders`.
pub type QueryAllCurrentUmOpenAlgoOrdersResponse = Vec<QueryAllCurrentUmOpenAlgoOrdersResponseItem>;

/// Provider-native `QueryUmAlgoOrderHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUmAlgoOrderHistoryResponseItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId", default, skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<i64>,
    /// Exact `clientAlgoId` wire field.
    #[serde(rename = "clientAlgoId")]
    pub client_algo_id: ClientOrderId,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
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
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    /// Exact `quantity` wire field.
    #[serde(
        rename = "quantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity: Option<Decimal>,
    /// Exact `algoStatus` wire field.
    #[serde(
        rename = "algoStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algo_status: Option<String>,
    /// Exact `actualOrderId` wire field.
    #[serde(
        rename = "actualOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_order_id: Option<String>,
    /// Exact `actualPrice` wire field.
    #[serde(
        rename = "actualPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_price: Option<Decimal>,
    /// Exact `triggerPrice` wire field.
    #[serde(
        rename = "triggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_price: Option<Decimal>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    /// Exact `workingType` wire field.
    #[serde(
        rename = "workingType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_type: Option<String>,
    /// Exact `priceMatch` wire field.
    #[serde(
        rename = "priceMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_match: Option<String>,
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `priceProtect` wire field.
    #[serde(
        rename = "priceProtect",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_protect: Option<bool>,
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
    /// Exact `triggerTime` wire field.
    #[serde(
        rename = "triggerTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_time: Option<i64>,
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

/// Exact response for `queryUmAlgoOrderHistory`.
pub type QueryUmAlgoOrderHistoryResponse = Vec<QueryUmAlgoOrderHistoryResponseItem>;

/// Provider-native `FuturesTradfiPerpsContractResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesTradfiPerpsContractResponse {
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
