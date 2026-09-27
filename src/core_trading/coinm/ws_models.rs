// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated ws response DTOs; regenerate with scripts/codegen/generate.py.

use crate::ClientOrderId;
use crate::Decimal;
use crate::SensitiveString;
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
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AccountInformationResponseAssetsItem>>,
    /// Exact `positions` wire field.
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<AccountInformationResponsePositionsItem>>,
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
    /// Exact `isolatedWallet` wire field.
    #[serde(
        rename = "isolatedWallet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_wallet: Option<String>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `positionAmt` wire field.
    #[serde(
        rename = "positionAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<Decimal>,
    /// Exact `breakEvenPrice` wire field.
    #[serde(
        rename = "breakEvenPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub break_even_price: Option<Decimal>,
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

/// Provider-native `CancelOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelOrderResponse {
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
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_type: Option<Decimal>,
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
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_auto_add_margin: Option<Decimal>,
    /// Exact `positionSide` wire field.
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    /// Exact `notionalValue` wire field.
    #[serde(
        rename = "notionalValue",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub notional_value: Option<Decimal>,
    /// Exact `isolatedWallet` wire field.
    #[serde(
        rename = "isolatedWallet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_wallet: Option<String>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `positionInformation`.
pub type PositionInformationResponse = Vec<PositionInformationResponseItem>;

/// Provider-native `QueryOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryOrderResponse {
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
    /// Exact `cumQty` wire field.
    #[serde(
        rename = "cumQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cum_qty: Option<Decimal>,
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
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
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
