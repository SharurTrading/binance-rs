// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated ws response DTOs; regenerate with scripts/codegen/generate.py.

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
    pub assets: Option<Vec<AccountInformationResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<AccountInformationResponsePositionsItem>>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountInformationResponseAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationResponseAssetsItem {
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

/// Provider-native `AccountInformationResponsePositionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationResponsePositionsItem {
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
    /// Exact `breakEvenPrice` wire field.
    #[serde(
        rename = "breakEvenPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub break_even_price: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// Provider-native `AccountInformationV2Response` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInformationV2Response {
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

/// Exact response for `futuresAccountBalance`.
pub type FuturesAccountBalanceResponse = Vec<FuturesAccountBalanceResponseItem>;

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
    /// Exact `lastUpdateId` wire field.
    #[serde(
        rename = "lastUpdateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_id: Option<i64>,
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
    /// Exact `lastUpdateId` wire field.
    #[serde(
        rename = "lastUpdateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_id: Option<i64>,
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

/// Provider-native `NewOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NewOrderResponse {
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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

/// Provider-native `PositionInformationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionInformationResponseItem {
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

/// Exact response for `positionInformation`.
pub type PositionInformationResponse = Vec<PositionInformationResponseItem>;

/// Provider-native `PositionInformationV2ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionInformationV2ResponseItem {
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
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<String>,
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

/// Exact response for `positionInformationV2`.
pub type PositionInformationV2Response = Vec<PositionInformationV2ResponseItem>;

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
