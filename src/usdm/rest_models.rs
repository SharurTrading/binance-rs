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

/// Provider-native `AccountInformationV2Response` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationV2Response {
    /// Exact `feeTier` wire field.
    #[serde(rename = "feeTier", default, skip_serializing_if = "Option::is_none")]
    pub fee_tier: Option<i64>,
    /// Exact `feeBurn` wire field.
    #[serde(rename = "feeBurn", default, skip_serializing_if = "Option::is_none")]
    pub fee_burn: Option<bool>,
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
    /// Exact `totalInitialMargin` wire field.
    #[serde(
        rename = "totalInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_initial_margin: Option<Decimal>,
    /// Exact `totalMaintMargin` wire field.
    #[serde(
        rename = "totalMaintMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_maint_margin: Option<Decimal>,
    /// Exact `totalWalletBalance` wire field.
    #[serde(
        rename = "totalWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_wallet_balance: Option<Decimal>,
    /// Exact `totalUnrealizedProfit` wire field.
    #[serde(
        rename = "totalUnrealizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_unrealized_profit: Option<Decimal>,
    /// Exact `totalMarginBalance` wire field.
    #[serde(
        rename = "totalMarginBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_margin_balance: Option<Decimal>,
    /// Exact `totalPositionInitialMargin` wire field.
    #[serde(
        rename = "totalPositionInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_position_initial_margin: Option<Decimal>,
    /// Exact `totalOpenOrderInitialMargin` wire field.
    #[serde(
        rename = "totalOpenOrderInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_open_order_initial_margin: Option<Decimal>,
    /// Exact `totalCrossWalletBalance` wire field.
    #[serde(
        rename = "totalCrossWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cross_wallet_balance: Option<Decimal>,
    /// Exact `totalCrossUnPnl` wire field.
    #[serde(
        rename = "totalCrossUnPnl",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cross_un_pnl: Option<Decimal>,
    /// Exact `availableBalance` wire field.
    #[serde(
        rename = "availableBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_balance: Option<Decimal>,
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AccountInformationV2ResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<AccountInformationV2ResponsePositionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountInformationV2ResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationV2ResponseAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
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
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Exact `marginAvailable` wire field.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<bool>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountInformationV2ResponsePositionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationV2ResponsePositionsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `entryPrice` wire field.
    #[serde(
        rename = "entryPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_price: Option<Decimal>,
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
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `positionAmt` wire field.
    #[serde(
        rename = "positionAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountInformationV3Response` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationV3Response {
    /// Exact `totalInitialMargin` wire field.
    #[serde(
        rename = "totalInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_initial_margin: Option<Decimal>,
    /// Exact `totalMaintMargin` wire field.
    #[serde(
        rename = "totalMaintMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_maint_margin: Option<Decimal>,
    /// Exact `totalWalletBalance` wire field.
    #[serde(
        rename = "totalWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_wallet_balance: Option<Decimal>,
    /// Exact `totalUnrealizedProfit` wire field.
    #[serde(
        rename = "totalUnrealizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_unrealized_profit: Option<Decimal>,
    /// Exact `totalMarginBalance` wire field.
    #[serde(
        rename = "totalMarginBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_margin_balance: Option<Decimal>,
    /// Exact `totalPositionInitialMargin` wire field.
    #[serde(
        rename = "totalPositionInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_position_initial_margin: Option<Decimal>,
    /// Exact `totalOpenOrderInitialMargin` wire field.
    #[serde(
        rename = "totalOpenOrderInitialMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_open_order_initial_margin: Option<Decimal>,
    /// Exact `totalCrossWalletBalance` wire field.
    #[serde(
        rename = "totalCrossWalletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cross_wallet_balance: Option<Decimal>,
    /// Exact `totalCrossUnPnl` wire field.
    #[serde(
        rename = "totalCrossUnPnl",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cross_un_pnl: Option<Decimal>,
    /// Exact `availableBalance` wire field.
    #[serde(
        rename = "availableBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_balance: Option<Decimal>,
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AccountInformationV3ResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<AccountInformationV3ResponsePositionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountInformationV3ResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationV3ResponseAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
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
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountInformationV3ResponsePositionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationV3ResponsePositionsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `positionAmt` wire field.
    #[serde(
        rename = "positionAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<Decimal>,
    /// Exact `unrealizedProfit` wire field.
    #[serde(
        rename = "unrealizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_profit: Option<Decimal>,
    /// Exact `isolatedMargin` wire field.
    #[serde(
        rename = "isolatedMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_margin: Option<Decimal>,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Exact `isolatedWallet` wire field.
    #[serde(
        rename = "isolatedWallet",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_wallet: Option<Decimal>,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `FuturesAccountBalanceV2ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesAccountBalanceV2ResponseItem {
    /// Exact `accountAlias` wire field.
    #[serde(
        rename = "accountAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_alias: Option<String>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Exact `balance` wire field.
    #[serde(
        rename = "balance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance: Option<Decimal>,
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
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Exact `marginAvailable` wire field.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<bool>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `futuresAccountBalanceV2`.
pub type FuturesAccountBalanceV2Response = Vec<FuturesAccountBalanceV2ResponseItem>;

/// Provider-native `FuturesAccountBalanceV3ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesAccountBalanceV3ResponseItem {
    /// Exact `accountAlias` wire field.
    #[serde(
        rename = "accountAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_alias: Option<String>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Exact `balance` wire field.
    #[serde(
        rename = "balance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance: Option<Decimal>,
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
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Exact `marginAvailable` wire field.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<bool>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `futuresAccountBalanceV3`.
pub type FuturesAccountBalanceV3Response = Vec<FuturesAccountBalanceV3ResponseItem>;

/// Provider-native `FuturesAccountConfigurationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesAccountConfigurationResponse {
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `FuturesTradingQuantitativeRulesIndicatorsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponse {
    /// Exact `indicators` wire field.
    #[serde(
        rename = "indicators",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub indicators: Option<FuturesTradingQuantitativeRulesIndicatorsResponseIndicators>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `FuturesTradingQuantitativeRulesIndicatorsResponseIndicators` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponseIndicators {
    /// Exact `BTCUSDT` wire field.
    #[serde(rename = "BTCUSDT", default, skip_serializing_if = "Option::is_none")]
    pub upper_btcusdt:
        Option<Vec<FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperBtcusdtItem>>,
    /// Exact `ETHUSDT` wire field.
    #[serde(rename = "ETHUSDT", default, skip_serializing_if = "Option::is_none")]
    pub upper_ethusdt:
        Option<Vec<FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperEthusdtItem>>,
    /// Exact `ACCOUNT` wire field.
    #[serde(rename = "ACCOUNT", default, skip_serializing_if = "Option::is_none")]
    pub upper_account:
        Option<Vec<FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperAccountItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperBtcusdtItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperBtcusdtItem {
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperEthusdtItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperEthusdtItem {
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperAccountItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsUpperAccountItem {
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `GetBnbBurnStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetBnbBurnStatusResponse {
    /// Exact `feeBurn` wire field.
    #[serde(rename = "feeBurn", default, skip_serializing_if = "Option::is_none")]
    pub fee_burn: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ToggleBnbBurnOnFuturesTradeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ToggleBnbBurnOnFuturesTradeResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `GetCurrentMultiAssetsModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCurrentMultiAssetsModeResponse {
    /// Exact `multiAssetsMargin` wire field.
    #[serde(
        rename = "multiAssetsMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multi_assets_margin: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ChangeMultiAssetsModeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeMultiAssetsModeResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub url: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub url: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub url: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `GetIncomeHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetIncomeHistoryResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    pub asset: Option<String>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `getIncomeHistory`.
pub type GetIncomeHistoryResponse = Vec<GetIncomeHistoryResponseItem>;

/// Provider alternatives for `NotionalAndLeverageBracketsResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum NotionalAndLeverageBracketsResponse {
    /// Wire alternative 1.
    Variant1(Box<Vec<NotionalAndLeverageBracketsResponseVariant1Item>>),
    /// Wire alternative 2.
    Variant2(Box<NotionalAndLeverageBracketsResponseVariant2>),
}

/// Provider-native `NotionalAndLeverageBracketsResponseVariant1Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalAndLeverageBracketsResponseVariant1Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    pub brackets: Option<Vec<NotionalAndLeverageBracketsResponseVariant1ItemBracketsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `NotionalAndLeverageBracketsResponseVariant1ItemBracketsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalAndLeverageBracketsResponseVariant1ItemBracketsItem {
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `NotionalAndLeverageBracketsResponseVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalAndLeverageBracketsResponseVariant2 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    pub brackets: Option<Vec<NotionalAndLeverageBracketsResponseVariant2BracketsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `NotionalAndLeverageBracketsResponseVariant2BracketsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotionalAndLeverageBracketsResponseVariant2BracketsItem {
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
    #[serde(rename = "cum", default, skip_serializing_if = "Option::is_none")]
    pub cum: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `queryUserRateLimit`.
pub type QueryUserRateLimitResponse = Vec<QueryUserRateLimitResponseItem>;

/// Provider-native `SymbolConfigurationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolConfigurationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    pub is_auto_add_margin: Option<bool>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `symbolConfiguration`.
pub type SymbolConfigurationResponse = Vec<SymbolConfigurationResponseItem>;

/// Provider-native `UserCommissionRateResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserCommissionRateResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `rpiCommissionRate` wire field.
    #[serde(
        rename = "rpiCommissionRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rpi_commission_rate: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AcceptTheOfferedQuoteResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcceptTheOfferedQuoteResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `orderStatus` wire field.
    #[serde(
        rename = "orderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_status: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ListAllConvertPairsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListAllConvertPairsResponseItem {
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset", default, skip_serializing_if = "Option::is_none")]
    pub from_asset: Option<String>,
    /// Exact `toAsset` wire field.
    #[serde(rename = "toAsset", default, skip_serializing_if = "Option::is_none")]
    pub to_asset: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `listAllConvertPairs`.
pub type ListAllConvertPairsResponse = Vec<ListAllConvertPairsResponseItem>;

/// Provider-native `OrderStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderStatusResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `orderStatus` wire field.
    #[serde(
        rename = "orderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_status: Option<String>,
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset", default, skip_serializing_if = "Option::is_none")]
    pub from_asset: Option<String>,
    /// Exact `fromAmount` wire field.
    #[serde(
        rename = "fromAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_amount: Option<Decimal>,
    /// Exact `toAsset` wire field.
    #[serde(rename = "toAsset", default, skip_serializing_if = "Option::is_none")]
    pub to_asset: Option<String>,
    /// Exact `toAmount` wire field.
    #[serde(
        rename = "toAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_amount: Option<Decimal>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `SendQuoteRequestResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SendQuoteRequestResponse {
    /// Exact `quoteId` wire field.
    #[serde(rename = "quoteId", default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
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
    #[serde(
        rename = "toAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_amount: Option<Decimal>,
    /// Exact `fromAmount` wire field.
    #[serde(
        rename = "fromAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_amount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider alternatives for `AdlRiskResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum AdlRiskResponse {
    /// Wire alternative 1.
    Variant1(Box<AdlRiskResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<AdlRiskResponseVariant2Item>>),
}

/// Provider-native `AdlRiskResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AdlRiskResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `adlRisk` wire field.
    #[serde(rename = "adlRisk", default, skip_serializing_if = "Option::is_none")]
    pub adl_risk: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AdlRiskResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AdlRiskResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `adlRisk` wire field.
    #[serde(rename = "adlRisk", default, skip_serializing_if = "Option::is_none")]
    pub adl_risk: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
        skip_serializing_if = "Option::is_none"
    )]
    pub annualized_basis_rate: Option<String>,
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
    pub pair: Option<String>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `CompositeIndexSymbolInformationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompositeIndexSymbolInformationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `component` wire field.
    #[serde(rename = "component", default, skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    /// Exact `baseAssetList` wire field.
    #[serde(
        rename = "baseAssetList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_asset_list: Option<Vec<CompositeIndexSymbolInformationResponseItemBaseAssetListItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `CompositeIndexSymbolInformationResponseItemBaseAssetListItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CompositeIndexSymbolInformationResponseItemBaseAssetListItem {
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<String>,
    /// Exact `weightInQuantity` wire field.
    #[serde(
        rename = "weightInQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weight_in_quantity: Option<Decimal>,
    /// Exact `weightInPercentage` wire field.
    #[serde(
        rename = "weightInPercentage",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weight_in_percentage: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `compositeIndexSymbolInformation`.
pub type CompositeIndexSymbolInformationResponse = Vec<CompositeIndexSymbolInformationResponseItem>;

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
    /// Exact `nq` wire field.
    #[serde(
        rename = "nq",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub nq: Option<Decimal>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<ExchangeInformationResponseAssetsItem>>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<ExchangeInformationResponseSymbolsItem>>,
    /// Exact `timezone` wire field.
    #[serde(rename = "timezone", default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ExchangeInformationResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Exact `marginAvailable` wire field.
    #[serde(
        rename = "marginAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_available: Option<bool>,
    /// Exact `autoAssetExchange` wire field.
    #[serde(
        rename = "autoAssetExchange",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_asset_exchange: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ExchangeInformationResponseSymbolsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInformationResponseSymbolsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<String>,
    /// Exact `marginAsset` wire field.
    #[serde(
        rename = "marginAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_asset: Option<String>,
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
    /// Exact `settlePlan` wire field.
    #[serde(
        rename = "settlePlan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub settle_plan: Option<i64>,
    /// Exact `triggerProtect` wire field.
    #[serde(
        rename = "triggerProtect",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_protect: Option<Decimal>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
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
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_decimal: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `GetFundingRateHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFundingRateHistoryResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `fundingRate` wire field.
    #[serde(
        rename = "fundingRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_rate: Option<Decimal>,
    /// Exact `fundingTime` wire field.
    #[serde(
        rename = "fundingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_time: Option<i64>,
    /// Exact `markPrice` wire field.
    #[serde(
        rename = "markPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mark_price: Option<Decimal>,
    /// Exact `rateType` wire field.
    #[serde(rename = "rateType", default, skip_serializing_if = "Option::is_none")]
    pub rate_type: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `getFundingRateHistory`.
pub type GetFundingRateHistoryResponse = Vec<GetFundingRateHistoryResponseItem>;

/// Provider-native `GetFundingRateInfoResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFundingRateInfoResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `adjustedFundingRateCap` wire field.
    #[serde(
        rename = "adjustedFundingRateCap",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjusted_funding_rate_cap: Option<Decimal>,
    /// Exact `adjustedFundingRateFloor` wire field.
    #[serde(
        rename = "adjustedFundingRateFloor",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub adjusted_funding_rate_floor: Option<Decimal>,
    /// Exact `fundingIntervalHours` wire field.
    #[serde(
        rename = "fundingIntervalHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_interval_hours: Option<i64>,
    /// Exact `disclaimer` wire field.
    #[serde(
        rename = "disclaimer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disclaimer: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `getFundingRateInfo`.
pub type GetFundingRateInfoResponse = Vec<GetFundingRateInfoResponseItem>;

/// Exact response for `indexPriceKlineCandlestickData`.
pub type IndexPriceKlineCandlestickDataResponse = Vec<Kline>;

/// Exact response for `klineCandlestickData`.
pub type KlineCandlestickDataResponse = Vec<Kline>;

/// Provider-native `LongShortRatioResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LongShortRatioResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
        skip_serializing_if = "Option::is_none"
    )]
    pub long_account: Option<String>,
    /// Exact `shortAccount` wire field.
    #[serde(
        rename = "shortAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_account: Option<String>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `longShortRatio`.
pub type LongShortRatioResponse = Vec<LongShortRatioResponseItem>;

/// Provider alternatives for `MarkPriceResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum MarkPriceResponse {
    /// Wire alternative 1.
    Variant1(Box<MarkPriceResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<MarkPriceResponseVariant2Item>>),
}

/// Provider-native `MarkPriceResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarkPriceResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_funding_rate: Option<Decimal>,
    /// Exact `interestRate` wire field.
    #[serde(
        rename = "interestRate",
        default,
        deserialize_with = "super::wire::decimal_option",
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `MarkPriceResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarkPriceResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_funding_rate: Option<Decimal>,
    /// Exact `interestRate` wire field.
    #[serde(
        rename = "interestRate",
        default,
        deserialize_with = "super::wire::decimal_option",
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `markPriceKlineCandlestickData`.
pub type MarkPriceKlineCandlestickDataResponse = Vec<Kline>;

/// Provider alternatives for `AssetIndexResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum AssetIndexResponse {
    /// Wire alternative 1.
    Variant1(Box<AssetIndexResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<AssetIndexResponseVariant2Item>>),
}

/// Provider-native `AssetIndexResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetIndexResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `index` wire field.
    #[serde(
        rename = "index",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub index: Option<Decimal>,
    /// Exact `bidBuffer` wire field.
    #[serde(
        rename = "bidBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_buffer: Option<Decimal>,
    /// Exact `askBuffer` wire field.
    #[serde(
        rename = "askBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_buffer: Option<Decimal>,
    /// Exact `bidRate` wire field.
    #[serde(
        rename = "bidRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_rate: Option<Decimal>,
    /// Exact `askRate` wire field.
    #[serde(
        rename = "askRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_rate: Option<Decimal>,
    /// Exact `autoExchangeBidBuffer` wire field.
    #[serde(
        rename = "autoExchangeBidBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_bid_buffer: Option<Decimal>,
    /// Exact `autoExchangeAskBuffer` wire field.
    #[serde(
        rename = "autoExchangeAskBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_ask_buffer: Option<Decimal>,
    /// Exact `autoExchangeBidRate` wire field.
    #[serde(
        rename = "autoExchangeBidRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_bid_rate: Option<Decimal>,
    /// Exact `autoExchangeAskRate` wire field.
    #[serde(
        rename = "autoExchangeAskRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_ask_rate: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AssetIndexResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetIndexResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `index` wire field.
    #[serde(
        rename = "index",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub index: Option<Decimal>,
    /// Exact `bidBuffer` wire field.
    #[serde(
        rename = "bidBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_buffer: Option<Decimal>,
    /// Exact `askBuffer` wire field.
    #[serde(
        rename = "askBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_buffer: Option<Decimal>,
    /// Exact `bidRate` wire field.
    #[serde(
        rename = "bidRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_rate: Option<Decimal>,
    /// Exact `askRate` wire field.
    #[serde(
        rename = "askRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_rate: Option<Decimal>,
    /// Exact `autoExchangeBidBuffer` wire field.
    #[serde(
        rename = "autoExchangeBidBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_bid_buffer: Option<Decimal>,
    /// Exact `autoExchangeAskBuffer` wire field.
    #[serde(
        rename = "autoExchangeAskBuffer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_ask_buffer: Option<Decimal>,
    /// Exact `autoExchangeBidRate` wire field.
    #[serde(
        rename = "autoExchangeBidRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_bid_rate: Option<Decimal>,
    /// Exact `autoExchangeAskRate` wire field.
    #[serde(
        rename = "autoExchangeAskRate",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_exchange_ask_rate: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

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
    /// Exact `quoteQty` wire field.
    #[serde(
        rename = "quoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_qty: Option<Decimal>,
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
    /// Exact `isRPITrade` wire field.
    #[serde(
        rename = "isRPITrade",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_rpi_trade: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `oldTradesLookup`.
pub type OldTradesLookupResponse = Vec<OldTradesLookupResponseItem>;

/// Provider-native `OpenInterestResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenInterestResponse {
    /// Exact `openInterest` wire field.
    #[serde(
        rename = "openInterest",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_interest: Option<Decimal>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `OpenInterestStatisticsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenInterestStatisticsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `CMCCirculatingSupply` wire field.
    #[serde(
        rename = "CMCCirculatingSupply",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cmc_circulating_supply: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `premiumIndexKlineData`.
pub type PremiumIndexKlineDataResponse = Vec<Kline>;

/// Provider-native `QuarterlyContractSettlementPriceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuarterlyContractSettlementPriceResponseItem {
    /// Exact `deliveryTime` wire field.
    #[serde(
        rename = "deliveryTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_time: Option<i64>,
    /// Exact `deliveryPrice` wire field.
    #[serde(
        rename = "deliveryPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_price: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `quarterlyContractSettlementPrice`.
pub type QuarterlyContractSettlementPriceResponse =
    Vec<QuarterlyContractSettlementPriceResponseItem>;

/// Provider-native `QueryIndexPriceConstituentsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryIndexPriceConstituentsResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider alternatives for `QueryInsuranceFundBalanceSnapshotResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum QueryInsuranceFundBalanceSnapshotResponse {
    /// Wire alternative 1.
    Variant1(Box<QueryInsuranceFundBalanceSnapshotResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<QueryInsuranceFundBalanceSnapshotResponseVariant2Item>>),
}

/// Provider-native `QueryInsuranceFundBalanceSnapshotResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryInsuranceFundBalanceSnapshotResponseVariant1 {
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<QueryInsuranceFundBalanceSnapshotResponseVariant1AssetsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `QueryInsuranceFundBalanceSnapshotResponseVariant1AssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryInsuranceFundBalanceSnapshotResponseVariant1AssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Exact `marginBalance` wire field.
    #[serde(
        rename = "marginBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_balance: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `QueryInsuranceFundBalanceSnapshotResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryInsuranceFundBalanceSnapshotResponseVariant2Item {
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<QueryInsuranceFundBalanceSnapshotResponseVariant2ItemAssetsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `QueryInsuranceFundBalanceSnapshotResponseVariant2ItemAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryInsuranceFundBalanceSnapshotResponseVariant2ItemAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Exact `marginBalance` wire field.
    #[serde(
        rename = "marginBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_balance: Option<Decimal>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `quoteQty` wire field.
    #[serde(
        rename = "quoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_qty: Option<Decimal>,
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
    /// Exact `isRPITrade` wire field.
    #[serde(
        rename = "isRPITrade",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_rpi_trade: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `recentTradesList`.
pub type RecentTradesListResponse = Vec<RecentTradesListResponseItem>;

/// Provider-native `RpiOrderBookResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RpiOrderBookResponse {
    /// Exact `lastUpdateId` wire field.
    #[serde(
        rename = "lastUpdateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_id: Option<i64>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider alternatives for `SymbolOrderBookTickerResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum SymbolOrderBookTickerResponse {
    /// Wire alternative 1.
    Variant1(Box<SymbolOrderBookTickerResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<SymbolOrderBookTickerResponseVariant2Item>>),
}

/// Provider-native `SymbolOrderBookTickerResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolOrderBookTickerResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `SymbolOrderBookTickerResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolOrderBookTickerResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider alternatives for `SymbolPriceTickerResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum SymbolPriceTickerResponse {
    /// Wire alternative 1.
    Variant1(Box<SymbolPriceTickerResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<SymbolPriceTickerResponseVariant2Item>>),
}

/// Provider-native `SymbolPriceTickerResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolPriceTickerResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `SymbolPriceTickerResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolPriceTickerResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider alternatives for `SymbolPriceTickerV2Response`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum SymbolPriceTickerV2Response {
    /// Wire alternative 1.
    Variant1(Box<SymbolPriceTickerV2ResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<SymbolPriceTickerV2ResponseVariant2Item>>),
}

/// Provider-native `SymbolPriceTickerV2ResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolPriceTickerV2ResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `SymbolPriceTickerV2ResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SymbolPriceTickerV2ResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TakerBuySellVolumeResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TakerBuySellVolumeResponseItem {
    /// Exact `buySellRatio` wire field.
    #[serde(
        rename = "buySellRatio",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub buy_sell_ratio: Option<Decimal>,
    /// Exact `buyVol` wire field.
    #[serde(
        rename = "buyVol",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub buy_vol: Option<Decimal>,
    /// Exact `sellVol` wire field.
    #[serde(
        rename = "sellVol",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sell_vol: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `takerBuySellVolume`.
pub type TakerBuySellVolumeResponse = Vec<TakerBuySellVolumeResponseItem>;

/// Exact response for `testConnectivity`.
pub type TestConnectivityResponse = BTreeMap<String, serde_json::Value>;

/// Provider alternatives for `Ticker24hrPriceChangeStatisticsResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Ticker24hrPriceChangeStatisticsResponse {
    /// Wire alternative 1.
    Variant1(Box<Ticker24hrPriceChangeStatisticsResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<Ticker24hrPriceChangeStatisticsResponseVariant2Item>>),
}

/// Provider-native `Ticker24hrPriceChangeStatisticsResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticker24hrPriceChangeStatisticsResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `Ticker24hrPriceChangeStatisticsResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticker24hrPriceChangeStatisticsResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TopTraderLongShortRatioAccountsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopTraderLongShortRatioAccountsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
        skip_serializing_if = "Option::is_none"
    )]
    pub long_account: Option<String>,
    /// Exact `shortAccount` wire field.
    #[serde(
        rename = "shortAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_account: Option<String>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `topTraderLongShortRatioAccounts`.
pub type TopTraderLongShortRatioAccountsResponse = Vec<TopTraderLongShortRatioAccountsResponseItem>;

/// Provider-native `TopTraderLongShortRatioPositionsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TopTraderLongShortRatioPositionsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
        skip_serializing_if = "Option::is_none"
    )]
    pub long_account: Option<String>,
    /// Exact `shortAccount` wire field.
    #[serde(
        rename = "shortAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_account: Option<String>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `topTraderLongShortRatioPositions`.
pub type TopTraderLongShortRatioPositionsResponse =
    Vec<TopTraderLongShortRatioPositionsResponseItem>;

/// Provider-native `TradingScheduleResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponse {
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `marketSchedules` wire field.
    #[serde(
        rename = "marketSchedules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub market_schedules: Option<TradingScheduleResponseMarketSchedules>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedules` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedules {
    /// Exact `EQUITY` wire field.
    #[serde(rename = "EQUITY", default, skip_serializing_if = "Option::is_none")]
    pub upper_equity: Option<TradingScheduleResponseMarketSchedulesUpperEquity>,
    /// Exact `COMMODITY` wire field.
    #[serde(rename = "COMMODITY", default, skip_serializing_if = "Option::is_none")]
    pub upper_commodity: Option<TradingScheduleResponseMarketSchedulesUpperCommodity>,
    /// Exact `KR_EQUITY` wire field.
    #[serde(rename = "KR_EQUITY", default, skip_serializing_if = "Option::is_none")]
    pub upper_kr_equity: Option<TradingScheduleResponseMarketSchedulesUpperKrEquity>,
    /// Exact `HK_EQUITY` wire field.
    #[serde(rename = "HK_EQUITY", default, skip_serializing_if = "Option::is_none")]
    pub upper_hk_equity: Option<TradingScheduleResponseMarketSchedulesUpperHkEquity>,
    /// Exact `CN_EQUITY` wire field.
    #[serde(rename = "CN_EQUITY", default, skip_serializing_if = "Option::is_none")]
    pub upper_cn_equity: Option<TradingScheduleResponseMarketSchedulesUpperCnEquity>,
    /// Exact `FX` wire field.
    #[serde(rename = "FX", default, skip_serializing_if = "Option::is_none")]
    pub upper_fx: Option<TradingScheduleResponseMarketSchedulesUpperFx>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperEquity` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperEquity {
    /// Exact `sessions` wire field.
    #[serde(rename = "sessions", default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<TradingScheduleResponseMarketSchedulesUpperEquitySessionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperEquitySessionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperEquitySessionsItem {
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperCommodity` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperCommodity {
    /// Exact `sessions` wire field.
    #[serde(rename = "sessions", default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<TradingScheduleResponseMarketSchedulesUpperCommoditySessionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperCommoditySessionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperCommoditySessionsItem {
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperKrEquity` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperKrEquity {
    /// Exact `sessions` wire field.
    #[serde(rename = "sessions", default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<TradingScheduleResponseMarketSchedulesUpperKrEquitySessionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperKrEquitySessionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperKrEquitySessionsItem {
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperHkEquity` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperHkEquity {
    /// Exact `sessions` wire field.
    #[serde(rename = "sessions", default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<TradingScheduleResponseMarketSchedulesUpperHkEquitySessionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperHkEquitySessionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperHkEquitySessionsItem {
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperCnEquity` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperCnEquity {
    /// Exact `sessions` wire field.
    #[serde(rename = "sessions", default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<TradingScheduleResponseMarketSchedulesUpperCnEquitySessionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperCnEquitySessionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperCnEquitySessionsItem {
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperFx` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperFx {
    /// Exact `sessions` wire field.
    #[serde(rename = "sessions", default, skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<TradingScheduleResponseMarketSchedulesUpperFxSessionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TradingScheduleResponseMarketSchedulesUpperFxSessionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradingScheduleResponseMarketSchedulesUpperFxSessionsItem {
    /// Exact `startTime` wire field.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ClassicPortfolioMarginAccountInformationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClassicPortfolioMarginAccountInformationResponse {
    /// Exact `maxWithdrawAmountUSD` wire field.
    #[serde(
        rename = "maxWithdrawAmountUSD",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount_usd: Option<Decimal>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset", default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Exact `maxWithdrawAmount` wire field.
    #[serde(
        rename = "maxWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_withdraw_amount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountTradeListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountTradeListResponseItem {
    /// Exact `buyer` wire field.
    #[serde(rename = "buyer", default, skip_serializing_if = "Option::is_none")]
    pub buyer: Option<bool>,
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
    pub commission_asset: Option<String>,
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Exact `maker` wire field.
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<bool>,
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
    /// Exact `quoteQty` wire field.
    #[serde(
        rename = "quoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_qty: Option<Decimal>,
    /// Exact `baseQty` wire field.
    #[serde(
        rename = "baseQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_qty: Option<Decimal>,
    /// Exact `marginAsset` wire field.
    #[serde(
        rename = "marginAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_asset: Option<String>,
    /// Exact `realizedPnl` wire field.
    #[serde(
        rename = "realizedPnl",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pnl: Option<Decimal>,
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
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Exact `cumQuote` wire field.
    #[serde(rename = "cumQuote", default, skip_serializing_if = "Option::is_none")]
    pub cum_quote: Option<String>,
    /// Exact `cumBase` wire field.
    #[serde(rename = "cumBase", default, skip_serializing_if = "Option::is_none")]
    pub cum_base: Option<String>,
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
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `allOrders`.
pub type AllOrdersResponse = Vec<AllOrdersResponseItem>;

/// Provider-native `AutoCancelAllOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutoCancelAllOpenOrdersResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `countdownTime` wire field.
    #[serde(
        rename = "countdownTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub countdown_time: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `CancelAlgoOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAlgoOrderResponse {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `clientAlgoId` wire field.
    #[serde(
        rename = "clientAlgoId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_algo_id: Option<ClientOrderId>,
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `NewAlgoOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewAlgoOrderResponse {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `clientAlgoId` wire field.
    #[serde(
        rename = "clientAlgoId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_algo_id: Option<ClientOrderId>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `icebergQuantity` wire field.
    #[serde(
        rename = "icebergQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_quantity: Option<String>,
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
        skip_serializing_if = "Option::is_none"
    )]
    pub activate_price: Option<String>,
    /// Exact `callbackRate` wire field.
    #[serde(
        rename = "callbackRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub callback_rate: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `QueryAlgoOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAlgoOrderResponse {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `clientAlgoId` wire field.
    #[serde(
        rename = "clientAlgoId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_algo_id: Option<ClientOrderId>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `actualType` wire field.
    #[serde(
        rename = "actualType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_type: Option<String>,
    /// Exact `actualQty` wire field.
    #[serde(
        rename = "actualQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_qty: Option<Decimal>,
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
    /// Exact `icebergQuantity` wire field.
    #[serde(
        rename = "icebergQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_quantity: Option<String>,
    /// Exact `tpOrderType` wire field.
    #[serde(
        rename = "tpOrderType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tp_order_type: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `CancelAllAlgoOpenOrdersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelAllAlgoOpenOrdersResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub symbol: Option<Symbol>,
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
    /// Exact `goodTillDate` wire field.
    #[serde(
        rename = "goodTillDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub good_till_date: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub symbol: Option<Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    price_match: Option<String>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
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
    /// Set `priceMatch`.
    #[must_use]
    pub fn price_match(mut self, value: impl Into<String>) -> Self {
        self.price_match = Some(value.into());
        self
    }
    /// Set `stopPrice`.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
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
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    price_match: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "goodTillDate", skip_serializing_if = "Option::is_none")]
    good_till_date: Option<i64>,
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
    /// Set `goodTillDate`.
    #[must_use]
    pub fn good_till_date(mut self, value: i64) -> Self {
        self.good_till_date = Some(value);
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
    pub symbol: Option<Symbol>,
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
    /// Exact `goodTillDate` wire field.
    #[serde(
        rename = "goodTillDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub good_till_date: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub symbol: Option<Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `NewOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewOrderResponse {
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
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
    /// Exact `cumQuote` wire field.
    #[serde(rename = "cumQuote", default, skip_serializing_if = "Option::is_none")]
    pub cum_quote: Option<String>,
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
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `ChangeInitialLeverageResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeInitialLeverageResponse {
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
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `CurrentAllAlgoOpenOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CurrentAllAlgoOpenOrdersResponseItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `clientAlgoId` wire field.
    #[serde(
        rename = "clientAlgoId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_algo_id: Option<ClientOrderId>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `icebergQuantity` wire field.
    #[serde(
        rename = "icebergQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_quantity: Option<String>,
    /// Exact `tpTriggerPrice` wire field.
    #[serde(
        rename = "tpTriggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tp_trigger_price: Option<Decimal>,
    /// Exact `tpPrice` wire field.
    #[serde(
        rename = "tpPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tp_price: Option<Decimal>,
    /// Exact `slTriggerPrice` wire field.
    #[serde(
        rename = "slTriggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sl_trigger_price: Option<Decimal>,
    /// Exact `slPrice` wire field.
    #[serde(
        rename = "slPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sl_price: Option<Decimal>,
    /// Exact `tpOrderType` wire field.
    #[serde(
        rename = "tpOrderType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tp_order_type: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `currentAllAlgoOpenOrders`.
pub type CurrentAllAlgoOpenOrdersResponse = Vec<CurrentAllAlgoOpenOrdersResponseItem>;

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
    /// Exact `cumQuote` wire field.
    #[serde(rename = "cumQuote", default, skip_serializing_if = "Option::is_none")]
    pub cum_quote: Option<String>,
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
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `currentAllOpenOrders`.
pub type CurrentAllOpenOrdersResponse = Vec<CurrentAllOpenOrdersResponseItem>;

/// Provider-native `FuturesTradfiPerpsContractResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FuturesTradfiPerpsContractResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

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
    pub symbol: Option<Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `GetOrderModifyHistoryResponseItemAmendmentPrice` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetOrderModifyHistoryResponseItemAmendmentPrice {
    /// Exact `before` wire field.
    #[serde(rename = "before", default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Exact `after` wire field.
    #[serde(
        rename = "after",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub after: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `getOrderModifyHistory`.
pub type GetOrderModifyHistoryResponse = Vec<GetOrderModifyHistoryResponseItem>;

/// Provider-native `GetPositionMarginChangeHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPositionMarginChangeHistoryResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<i64>,
    /// Exact `deltaType` wire field.
    #[serde(rename = "deltaType", default, skip_serializing_if = "Option::is_none")]
    pub delta_type: Option<String>,
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
    pub asset: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `PositionAdlQuantileEstimationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionAdlQuantileEstimationResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `adlQuantile` wire field.
    #[serde(
        rename = "adlQuantile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub adl_quantile: Option<PositionAdlQuantileEstimationResponseItemAdlQuantile>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `positionAdlQuantileEstimation`.
pub type PositionAdlQuantileEstimationResponse = Vec<PositionAdlQuantileEstimationResponseItem>;

/// Provider-native `PositionInformationV2ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionInformationV2ResponseItem {
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
    /// Exact `isolatedMargin` wire field.
    #[serde(
        rename = "isolatedMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_margin: Option<Decimal>,
    /// Exact `leverage` wire field.
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Exact `liquidationPrice` wire field.
    #[serde(
        rename = "liquidationPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_price: Option<Decimal>,
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
    #[serde(
        rename = "positionAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<Decimal>,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Exact `isolatedWallet` wire field.
    #[serde(
        rename = "isolatedWallet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_wallet: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `unRealizedProfit` wire field.
    #[serde(
        rename = "unRealizedProfit",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub un_realized_profit: Option<Decimal>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `positionInformationV2`.
pub type PositionInformationV2Response = Vec<PositionInformationV2ResponseItem>;

/// Provider-native `PositionInformationV3ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionInformationV3ResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
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
    /// Exact `isolatedMargin` wire field.
    #[serde(
        rename = "isolatedMargin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_margin: Option<Decimal>,
    /// Exact `notional` wire field.
    #[serde(
        rename = "notional",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional: Option<Decimal>,
    /// Exact `marginAsset` wire field.
    #[serde(
        rename = "marginAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_asset: Option<String>,
    /// Exact `isolatedWallet` wire field.
    #[serde(
        rename = "isolatedWallet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_wallet: Option<String>,
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
    /// Exact `adl` wire field.
    #[serde(rename = "adl", default, skip_serializing_if = "Option::is_none")]
    pub adl: Option<i64>,
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
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `positionInformationV3`.
pub type PositionInformationV3Response = Vec<PositionInformationV3ResponseItem>;

/// Provider-native `QueryAllAlgoOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryAllAlgoOrdersResponseItem {
    /// Exact `algoId` wire field.
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    /// Exact `clientAlgoId` wire field.
    #[serde(
        rename = "clientAlgoId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_algo_id: Option<ClientOrderId>,
    /// Exact `algoType` wire field.
    #[serde(rename = "algoType", default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<String>,
    /// Exact `orderType` wire field.
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `icebergQuantity` wire field.
    #[serde(
        rename = "icebergQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_quantity: Option<String>,
    /// Exact `tpTriggerPrice` wire field.
    #[serde(
        rename = "tpTriggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tp_trigger_price: Option<Decimal>,
    /// Exact `tpPrice` wire field.
    #[serde(
        rename = "tpPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tp_price: Option<Decimal>,
    /// Exact `slTriggerPrice` wire field.
    #[serde(
        rename = "slTriggerPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sl_trigger_price: Option<Decimal>,
    /// Exact `slPrice` wire field.
    #[serde(
        rename = "slPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sl_price: Option<Decimal>,
    /// Exact `tpOrderType` wire field.
    #[serde(
        rename = "tpOrderType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tp_order_type: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Exact response for `queryAllAlgoOrders`.
pub type QueryAllAlgoOrdersResponse = Vec<QueryAllAlgoOrdersResponseItem>;

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
    /// Exact `cumQuote` wire field.
    #[serde(rename = "cumQuote", default, skip_serializing_if = "Option::is_none")]
    pub cum_quote: Option<String>,
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
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    /// Exact `closePosition` wire field.
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `TestOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TestOrderResponse {
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
    /// Exact `cumQuote` wire field.
    #[serde(rename = "cumQuote", default, skip_serializing_if = "Option::is_none")]
    pub cum_quote: Option<String>,
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
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
    pub symbol: Option<Symbol>,
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
    /// Exact `goodTillDate` wire field.
    #[serde(
        rename = "goodTillDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub good_till_date: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    pub symbol: Option<Symbol>,
    /// Exact `pair` wire field.
    #[serde(rename = "pair", default, skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
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
    /// Exact `cumQuote` wire field.
    #[serde(
        rename = "cumQuote",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_quote: Option<Decimal>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
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
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `StartUserDataStreamResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StartUserDataStreamResponse {
    /// Exact `listenKey` wire field.
    #[serde(rename = "listenKey", default, skip_serializing_if = "Option::is_none")]
    pub listen_key: Option<SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}
