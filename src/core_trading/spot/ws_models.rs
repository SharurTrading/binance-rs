// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated ws response DTOs; regenerate with scripts/codegen/generate.py.

use super::ClientOrderId;
use super::wire::Kline;
use super::wire::PriceLevel;
use crate::Decimal;
use crate::SensitiveString;
use crate::Symbol;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Provider-native `AccountCommissionResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountCommissionResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `standardCommission` wire field.
    #[serde(
        rename = "standardCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub standard_commission: Option<AccountCommissionResponseStandardCommission>,
    /// Exact `specialCommission` wire field.
    #[serde(
        rename = "specialCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_commission: Option<AccountCommissionResponseSpecialCommission>,
    /// Exact `taxCommission` wire field.
    #[serde(
        rename = "taxCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_commission: Option<AccountCommissionResponseTaxCommission>,
    /// Exact `discount` wire field.
    #[serde(rename = "discount", default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<AccountCommissionResponseDiscount>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountCommissionResponseStandardCommission` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountCommissionResponseStandardCommission {
    /// Exact `maker` wire field.
    #[serde(
        rename = "maker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker: Option<Decimal>,
    /// Exact `taker` wire field.
    #[serde(
        rename = "taker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker: Option<Decimal>,
    /// Exact `buyer` wire field.
    #[serde(
        rename = "buyer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer: Option<Decimal>,
    /// Exact `seller` wire field.
    #[serde(
        rename = "seller",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub seller: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountCommissionResponseSpecialCommission` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountCommissionResponseSpecialCommission {
    /// Exact `maker` wire field.
    #[serde(
        rename = "maker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker: Option<Decimal>,
    /// Exact `taker` wire field.
    #[serde(
        rename = "taker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker: Option<Decimal>,
    /// Exact `buyer` wire field.
    #[serde(
        rename = "buyer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer: Option<Decimal>,
    /// Exact `seller` wire field.
    #[serde(
        rename = "seller",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub seller: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountCommissionResponseTaxCommission` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountCommissionResponseTaxCommission {
    /// Exact `maker` wire field.
    #[serde(
        rename = "maker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker: Option<Decimal>,
    /// Exact `taker` wire field.
    #[serde(
        rename = "taker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker: Option<Decimal>,
    /// Exact `buyer` wire field.
    #[serde(
        rename = "buyer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer: Option<Decimal>,
    /// Exact `seller` wire field.
    #[serde(
        rename = "seller",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub seller: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountCommissionResponseDiscount` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountCommissionResponseDiscount {
    /// Exact `enabledForAccount` wire field.
    #[serde(
        rename = "enabledForAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_for_account: Option<bool>,
    /// Exact `enabledForSymbol` wire field.
    #[serde(
        rename = "enabledForSymbol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_for_symbol: Option<bool>,
    /// Exact `discountAsset` wire field.
    #[serde(
        rename = "discountAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discount_asset: Option<crate::Asset>,
    /// Exact `discount` wire field.
    #[serde(
        rename = "discount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub discount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountRateLimitsOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountRateLimitsOrdersResponseItem {
    /// Exact `rateLimitType` wire field.
    #[serde(
        rename = "rateLimitType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limit_type: Option<super::enums::RateLimitType>,
    /// Exact `interval` wire field.
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<super::enums::RateLimitInterval>,
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
    /// Exact `count` wire field.
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `accountRateLimitsOrders`.
pub type AccountRateLimitsOrdersResponse = Vec<AccountRateLimitsOrdersResponseItem>;

/// Provider-native `AccountStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountStatusResponse {
    /// Exact `makerCommission` wire field.
    #[serde(
        rename = "makerCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_commission: Option<i64>,
    /// Exact `takerCommission` wire field.
    #[serde(
        rename = "takerCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_commission: Option<i64>,
    /// Exact `buyerCommission` wire field.
    #[serde(
        rename = "buyerCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_commission: Option<i64>,
    /// Exact `sellerCommission` wire field.
    #[serde(
        rename = "sellerCommission",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub seller_commission: Option<i64>,
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
    /// Exact `canDeposit` wire field.
    #[serde(
        rename = "canDeposit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_deposit: Option<bool>,
    /// Exact `commissionRates` wire field.
    #[serde(
        rename = "commissionRates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission_rates: Option<AccountStatusResponseCommissionRates>,
    /// Exact `brokered` wire field.
    #[serde(rename = "brokered", default, skip_serializing_if = "Option::is_none")]
    pub brokered: Option<bool>,
    /// Exact `requireSelfTradePrevention` wire field.
    #[serde(
        rename = "requireSelfTradePrevention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_self_trade_prevention: Option<bool>,
    /// Exact `preventSor` wire field.
    #[serde(
        rename = "preventSor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevent_sor: Option<bool>,
    /// Exact `updateTime` wire field.
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    /// Exact `accountType` wire field.
    #[serde(
        rename = "accountType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_type: Option<String>,
    /// Exact `balances` wire field.
    #[serde(rename = "balances")]
    pub balances: Vec<AccountStatusResponseBalancesItem>,
    /// Exact `permissions` wire field.
    #[serde(
        rename = "permissions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions: Option<Vec<super::enums::Permission>>,
    /// Exact `uid` wire field.
    #[serde(rename = "uid", default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountStatusResponseCommissionRates` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountStatusResponseCommissionRates {
    /// Exact `maker` wire field.
    #[serde(
        rename = "maker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker: Option<Decimal>,
    /// Exact `taker` wire field.
    #[serde(
        rename = "taker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker: Option<Decimal>,
    /// Exact `buyer` wire field.
    #[serde(
        rename = "buyer",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer: Option<Decimal>,
    /// Exact `seller` wire field.
    #[serde(
        rename = "seller",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub seller: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountStatusResponseBalancesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountStatusResponseBalancesItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllOrdersResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllOrdersResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
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
    /// Exact `isWorking` wire field.
    #[serde(rename = "isWorking", default, skip_serializing_if = "Option::is_none")]
    pub is_working: Option<bool>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `allOrders`.
pub type AllOrdersResponse = Vec<AllOrdersResponseItem>;

/// Provider-native `MyTradesResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyTradesResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
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
    /// Exact `isBuyer` wire field.
    #[serde(rename = "isBuyer", default, skip_serializing_if = "Option::is_none")]
    pub is_buyer: Option<bool>,
    /// Exact `isMaker` wire field.
    #[serde(rename = "isMaker", default, skip_serializing_if = "Option::is_none")]
    pub is_maker: Option<bool>,
    /// Exact `isBestMatch` wire field.
    #[serde(
        rename = "isBestMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_best_match: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `myTrades`.
pub type MyTradesResponse = Vec<MyTradesResponseItem>;

/// Provider-native `OpenOrdersStatusResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrdersStatusResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
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
    /// Exact `isWorking` wire field.
    #[serde(rename = "isWorking", default, skip_serializing_if = "Option::is_none")]
    pub is_working: Option<bool>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `openOrdersStatus`.
pub type OpenOrdersStatusResponse = Vec<OpenOrdersStatusResponseItem>;

/// Provider-native `OrderStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderStatusResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
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
    /// Exact `isWorking` wire field.
    #[serde(rename = "isWorking", default, skip_serializing_if = "Option::is_none")]
    pub is_working: Option<bool>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponse {
    /// Exact `timezone` wire field.
    #[serde(rename = "timezone", default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Exact `serverTime` wire field.
    #[serde(
        rename = "serverTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_time: Option<i64>,
    /// Exact `rateLimits` wire field.
    #[serde(
        rename = "rateLimits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limits: Option<Vec<ExchangeInfoResponseRateLimitsItem>>,
    /// Exact `exchangeFilters` wire field.
    #[serde(
        rename = "exchangeFilters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_filters: Option<Vec<ExchangeInfoResponseExchangeFiltersItem>>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<ExchangeInfoResponseSymbolsItem>>,
    /// Exact `sors` wire field.
    #[serde(rename = "sors", default, skip_serializing_if = "Option::is_none")]
    pub sors: Option<Vec<ExchangeInfoResponseSorsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseRateLimitsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseRateLimitsItem {
    /// Exact `rateLimitType` wire field.
    #[serde(
        rename = "rateLimitType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limit_type: Option<super::enums::RateLimitType>,
    /// Exact `interval` wire field.
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<super::enums::RateLimitInterval>,
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
    /// Exact `count` wire field.
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider filters with explicit discriminator dispatch and unknown retention.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ExchangeInfoResponseExchangeFiltersItem {
    /// Provider `EXCHANGE_MAX_NUM_ORDERS` filter.
    ExchangeMaxNumOrders(Box<ExchangeInfoResponseExchangeFiltersItemVariant1>),
    /// Provider `EXCHANGE_MAX_NUM_ALGO_ORDERS` filter.
    ExchangeMaxNumAlgoOrders(Box<ExchangeInfoResponseExchangeFiltersItemVariant2>),
    /// Provider `EXCHANGE_MAX_NUM_ICEBERG_ORDERS` filter.
    ExchangeMaxNumIcebergOrders(Box<ExchangeInfoResponseExchangeFiltersItemVariant3>),
    /// Provider `EXCHANGE_MAX_NUM_ORDER_LISTS` filter.
    ExchangeMaxNumOrderLists(Box<ExchangeInfoResponseExchangeFiltersItemVariant4>),
    /// Future filter facts, retained with redacted Debug.
    Unknown(super::event_payloads::UnknownMessage),
}
impl<'de> Deserialize<'de> for ExchangeInfoResponseExchangeFiltersItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match value.get("filterType").and_then(serde_json::Value::as_str) {
            Some("EXCHANGE_MAX_NUM_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("EXCHANGE_MAX_NUM_ALGO_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumAlgoOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("EXCHANGE_MAX_NUM_ICEBERG_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumIcebergOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("EXCHANGE_MAX_NUM_ORDER_LISTS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumOrderLists(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some(_) => Ok(Self::Unknown(value.into())),
            None => Err(serde::de::Error::custom("filter type required")),
        }
    }
}

/// Provider-native `ExchangeInfoResponseExchangeFiltersItemVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseExchangeFiltersItemVariant1 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrders` wire field.
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseExchangeFiltersItemVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseExchangeFiltersItemVariant2 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumAlgoOrders` wire field.
    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseExchangeFiltersItemVariant3` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseExchangeFiltersItemVariant3 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumIcebergOrders` wire field.
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseExchangeFiltersItemVariant4` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseExchangeFiltersItemVariant4 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrderLists` wire field.
    #[serde(rename = "maxNumOrderLists")]
    pub max_num_order_lists: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<super::enums::SymbolStatus>,
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<crate::Asset>,
    /// Exact `baseAssetPrecision` wire field.
    #[serde(
        rename = "baseAssetPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_asset_precision: Option<i64>,
    /// Exact `quoteAsset` wire field.
    #[serde(
        rename = "quoteAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset: Option<crate::Asset>,
    /// Exact `quotePrecision` wire field.
    #[serde(
        rename = "quotePrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_precision: Option<i64>,
    /// Exact `quoteAssetPrecision` wire field.
    #[serde(
        rename = "quoteAssetPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset_precision: Option<i64>,
    /// Exact `baseCommissionPrecision` wire field.
    #[serde(
        rename = "baseCommissionPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_commission_precision: Option<i64>,
    /// Exact `quoteCommissionPrecision` wire field.
    #[serde(
        rename = "quoteCommissionPrecision",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_commission_precision: Option<i64>,
    /// Exact `orderTypes` wire field.
    #[serde(
        rename = "orderTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_types: Option<Vec<super::enums::OrderType>>,
    /// Exact `icebergAllowed` wire field.
    #[serde(
        rename = "icebergAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_allowed: Option<bool>,
    /// Exact `ocoAllowed` wire field.
    #[serde(
        rename = "ocoAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub oco_allowed: Option<bool>,
    /// Exact `otoAllowed` wire field.
    #[serde(
        rename = "otoAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub oto_allowed: Option<bool>,
    /// Exact `opoAllowed` wire field.
    #[serde(
        rename = "opoAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub opo_allowed: Option<bool>,
    /// Exact `quoteOrderQtyMarketAllowed` wire field.
    #[serde(
        rename = "quoteOrderQtyMarketAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_order_qty_market_allowed: Option<bool>,
    /// Exact `allowTrailingStop` wire field.
    #[serde(
        rename = "allowTrailingStop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_trailing_stop: Option<bool>,
    /// Exact `cancelReplaceAllowed` wire field.
    #[serde(
        rename = "cancelReplaceAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel_replace_allowed: Option<bool>,
    /// Exact `amendAllowed` wire field.
    #[serde(
        rename = "amendAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub amend_allowed: Option<bool>,
    /// Exact `pegInstructionsAllowed` wire field.
    #[serde(
        rename = "pegInstructionsAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_instructions_allowed: Option<bool>,
    /// Exact `isSpotTradingAllowed` wire field.
    #[serde(
        rename = "isSpotTradingAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_spot_trading_allowed: Option<bool>,
    /// Exact `isMarginTradingAllowed` wire field.
    #[serde(
        rename = "isMarginTradingAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_margin_trading_allowed: Option<bool>,
    /// Exact `filters` wire field.
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExchangeInfoResponseSymbolsItemFiltersItem>>,
    /// Exact `permissions` wire field.
    #[serde(
        rename = "permissions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions: Option<Vec<super::enums::Permission>>,
    /// Exact `permissionSets` wire field.
    #[serde(
        rename = "permissionSets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_sets: Option<Vec<Vec<super::enums::Permission>>>,
    /// Exact `defaultSelfTradePreventionMode` wire field.
    #[serde(
        rename = "defaultSelfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `allowedSelfTradePreventionModes` wire field.
    #[serde(
        rename = "allowedSelfTradePreventionModes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_self_trade_prevention_modes: Option<Vec<super::enums::SelfTradePreventionMode>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider filters with explicit discriminator dispatch and unknown retention.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ExchangeInfoResponseSymbolsItemFiltersItem {
    /// Provider `PRICE_FILTER` filter.
    PriceFilter(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant1>),
    /// Provider `PERCENT_PRICE` filter.
    PercentPrice(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant2>),
    /// Provider `PERCENT_PRICE_BY_SIDE` filter.
    PercentPriceBySide(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant3>),
    /// Provider `LOT_SIZE` filter.
    LotSize(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant4>),
    /// Provider `MIN_NOTIONAL` filter.
    MinNotional(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant5>),
    /// Provider `NOTIONAL` filter.
    Notional(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant6>),
    /// Provider `ICEBERG_PARTS` filter.
    IcebergParts(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant7>),
    /// Provider `MARKET_LOT_SIZE` filter.
    MarketLotSize(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant8>),
    /// Provider `MAX_NUM_ORDERS` filter.
    MaxNumOrders(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant9>),
    /// Provider `MAX_NUM_ALGO_ORDERS` filter.
    MaxNumAlgoOrders(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant10>),
    /// Provider `MAX_NUM_ICEBERG_ORDERS` filter.
    MaxNumIcebergOrders(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant11>),
    /// Provider `MAX_POSITION` filter.
    MaxPosition(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant12>),
    /// Provider `TRAILING_DELTA` filter.
    TrailingDelta(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant13>),
    /// Provider `T_PLUS_SELL` filter.
    TPlusSell(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant14>),
    /// Provider `MAX_NUM_ORDER_LISTS` filter.
    MaxNumOrderLists(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant15>),
    /// Provider `MAX_NUM_ORDER_AMENDS` filter.
    MaxNumOrderAmends(Box<ExchangeInfoResponseSymbolsItemFiltersItemVariant16>),
    /// Future filter facts, retained with redacted Debug.
    Unknown(super::event_payloads::UnknownMessage),
}
impl<'de> Deserialize<'de> for ExchangeInfoResponseSymbolsItemFiltersItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match value.get("filterType").and_then(serde_json::Value::as_str) {
            Some("PRICE_FILTER") => serde_json::from_value(value)
                .map(|v| Self::PriceFilter(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("PERCENT_PRICE") => serde_json::from_value(value)
                .map(|v| Self::PercentPrice(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("PERCENT_PRICE_BY_SIDE") => serde_json::from_value(value)
                .map(|v| Self::PercentPriceBySide(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("LOT_SIZE") => serde_json::from_value(value)
                .map(|v| Self::LotSize(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MIN_NOTIONAL") => serde_json::from_value(value)
                .map(|v| Self::MinNotional(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("NOTIONAL") => serde_json::from_value(value)
                .map(|v| Self::Notional(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("ICEBERG_PARTS") => serde_json::from_value(value)
                .map(|v| Self::IcebergParts(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MARKET_LOT_SIZE") => serde_json::from_value(value)
                .map(|v| Self::MarketLotSize(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ALGO_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumAlgoOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ICEBERG_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumIcebergOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_POSITION") => serde_json::from_value(value)
                .map(|v| Self::MaxPosition(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("TRAILING_DELTA") => serde_json::from_value(value)
                .map(|v| Self::TrailingDelta(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("T_PLUS_SELL") => serde_json::from_value(value)
                .map(|v| Self::TPlusSell(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ORDER_LISTS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumOrderLists(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ORDER_AMENDS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumOrderAmends(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some(_) => Ok(Self::Unknown(value.into())),
            None => Err(serde::de::Error::custom("filter type required")),
        }
    }
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant1 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `priceExponent` wire field.
    #[serde(
        rename = "priceExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_exponent: Option<i64>,
    /// Exact `minPrice` wire field.
    #[serde(rename = "minPrice", deserialize_with = "super::wire::decimal")]
    pub min_price: Decimal,
    /// Exact `maxPrice` wire field.
    #[serde(rename = "maxPrice", deserialize_with = "super::wire::decimal")]
    pub max_price: Decimal,
    /// Exact `tickSize` wire field.
    #[serde(rename = "tickSize", deserialize_with = "super::wire::decimal")]
    pub tick_size: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant2 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `multiplierExponent` wire field.
    #[serde(
        rename = "multiplierExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_exponent: Option<i64>,
    /// Exact `multiplierUp` wire field.
    #[serde(rename = "multiplierUp", deserialize_with = "super::wire::decimal")]
    pub multiplier_up: Decimal,
    /// Exact `multiplierDown` wire field.
    #[serde(rename = "multiplierDown", deserialize_with = "super::wire::decimal")]
    pub multiplier_down: Decimal,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant3` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant3 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `multiplierExponent` wire field.
    #[serde(
        rename = "multiplierExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_exponent: Option<i64>,
    /// Exact `bidMultiplierUp` wire field.
    #[serde(rename = "bidMultiplierUp", deserialize_with = "super::wire::decimal")]
    pub bid_multiplier_up: Decimal,
    /// Exact `bidMultiplierDown` wire field.
    #[serde(
        rename = "bidMultiplierDown",
        deserialize_with = "super::wire::decimal"
    )]
    pub bid_multiplier_down: Decimal,
    /// Exact `askMultiplierUp` wire field.
    #[serde(rename = "askMultiplierUp", deserialize_with = "super::wire::decimal")]
    pub ask_multiplier_up: Decimal,
    /// Exact `askMultiplierDown` wire field.
    #[serde(
        rename = "askMultiplierDown",
        deserialize_with = "super::wire::decimal"
    )]
    pub ask_multiplier_down: Decimal,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant4` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant4 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `qtyExponent` wire field.
    #[serde(
        rename = "qtyExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_exponent: Option<i64>,
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

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant5` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant5 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `priceExponent` wire field.
    #[serde(
        rename = "priceExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_exponent: Option<i64>,
    /// Exact `minNotional` wire field.
    #[serde(rename = "minNotional", deserialize_with = "super::wire::decimal")]
    pub min_notional: Decimal,
    /// Exact `applyToMarket` wire field.
    #[serde(rename = "applyToMarket")]
    pub apply_to_market: bool,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant6` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant6 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `priceExponent` wire field.
    #[serde(
        rename = "priceExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_exponent: Option<i64>,
    /// Exact `minNotional` wire field.
    #[serde(rename = "minNotional", deserialize_with = "super::wire::decimal")]
    pub min_notional: Decimal,
    /// Exact `applyMinToMarket` wire field.
    #[serde(rename = "applyMinToMarket")]
    pub apply_min_to_market: bool,
    /// Exact `maxNotional` wire field.
    #[serde(rename = "maxNotional", deserialize_with = "super::wire::decimal")]
    pub max_notional: Decimal,
    /// Exact `applyMaxToMarket` wire field.
    #[serde(rename = "applyMaxToMarket")]
    pub apply_max_to_market: bool,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant7` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant7 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `limit` wire field.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant8` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant8 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `qtyExponent` wire field.
    #[serde(
        rename = "qtyExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_exponent: Option<i64>,
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

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant9` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant9 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrders` wire field.
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant10` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant10 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumAlgoOrders` wire field.
    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant11` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant11 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumIcebergOrders` wire field.
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant12` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant12 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `qtyExponent` wire field.
    #[serde(
        rename = "qtyExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_exponent: Option<i64>,
    /// Exact `maxPosition` wire field.
    #[serde(rename = "maxPosition", deserialize_with = "super::wire::decimal")]
    pub max_position: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant13` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant13 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `minTrailingAboveDelta` wire field.
    #[serde(rename = "minTrailingAboveDelta")]
    pub min_trailing_above_delta: i64,
    /// Exact `maxTrailingAboveDelta` wire field.
    #[serde(rename = "maxTrailingAboveDelta")]
    pub max_trailing_above_delta: i64,
    /// Exact `minTrailingBelowDelta` wire field.
    #[serde(rename = "minTrailingBelowDelta")]
    pub min_trailing_below_delta: i64,
    /// Exact `maxTrailingBelowDelta` wire field.
    #[serde(rename = "maxTrailingBelowDelta")]
    pub max_trailing_below_delta: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant14` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant14 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime")]
    pub end_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant15` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant15 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrderLists` wire field.
    #[serde(rename = "maxNumOrderLists")]
    pub max_num_order_lists: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSymbolsItemFiltersItemVariant16` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSymbolsItemFiltersItemVariant16 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrderAmends` wire field.
    #[serde(rename = "maxNumOrderAmends")]
    pub max_num_order_amends: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExchangeInfoResponseSorsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExchangeInfoResponseSorsItem {
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<crate::Asset>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExecutionRulesResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExecutionRulesResponse {
    /// Exact `symbolRules` wire field.
    #[serde(
        rename = "symbolRules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub symbol_rules: Option<Vec<ExecutionRulesResponseSymbolRulesItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExecutionRulesResponseSymbolRulesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExecutionRulesResponseSymbolRulesItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `rules` wire field.
    #[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ExecutionRulesResponseSymbolRulesItemRulesItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `ExecutionRulesResponseSymbolRulesItemRulesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ExecutionRulesResponseSymbolRulesItemRulesItem {
    /// Exact `ruleType` wire field.
    #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// Exact `bidLimitMultUp` wire field.
    #[serde(
        rename = "bidLimitMultUp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_limit_mult_up: Option<String>,
    /// Exact `bidLimitMultDown` wire field.
    #[serde(
        rename = "bidLimitMultDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bid_limit_mult_down: Option<String>,
    /// Exact `askLimitMultUp` wire field.
    #[serde(
        rename = "askLimitMultUp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_limit_mult_up: Option<String>,
    /// Exact `askLimitMultDown` wire field.
    #[serde(
        rename = "askLimitMultDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ask_limit_mult_down: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `ping`.
pub type PingResponse = BTreeMap<String, serde_json::Value>;

/// Provider-native `TimeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TimeResponse {
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

/// Provider-native `AvgPriceResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvgPriceResponse {
    /// Exact `mins` wire field.
    #[serde(rename = "mins", default, skip_serializing_if = "Option::is_none")]
    pub mins: Option<i64>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `closeTime` wire field.
    #[serde(rename = "closeTime", default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DepthResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepthResponse {
    /// Exact `lastUpdateId` wire field.
    #[serde(
        rename = "lastUpdateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_id: Option<i64>,
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

/// Exact response for `klines`.
pub type KlinesResponse = Vec<Kline>;

/// Provider alternatives for `TickerResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum TickerResponse {
    /// Wire alternative 1.
    Variant1(Box<TickerResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<TickerResponseVariant2Item>>),
}

/// Provider-native `TickerResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `lastPrice` wire field.
    #[serde(
        rename = "lastPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_price: Option<Decimal>,
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

/// Provider-native `TickerResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `lastPrice` wire field.
    #[serde(
        rename = "lastPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_price: Option<Decimal>,
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

/// Provider alternatives for `Ticker24hrResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Ticker24hrResponse {
    /// Wire alternative 1.
    Variant1(Box<Ticker24hrResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<Ticker24hrResponseVariant2Item>>),
}

/// Provider-native `Ticker24hrResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticker24hrResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `prevClosePrice` wire field.
    #[serde(
        rename = "prevClosePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prev_close_price: Option<Decimal>,
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

/// Provider-native `Ticker24hrResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticker24hrResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `prevClosePrice` wire field.
    #[serde(
        rename = "prevClosePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prev_close_price: Option<Decimal>,
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

/// Provider alternatives for `TickerBookResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum TickerBookResponse {
    /// Wire alternative 1.
    Variant1(Box<TickerBookResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<TickerBookResponseVariant2Item>>),
}

/// Provider-native `TickerBookResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerBookResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TickerBookResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerBookResponseVariant2Item {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider alternatives for `TickerPriceResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum TickerPriceResponse {
    /// Wire alternative 1.
    Variant1(Box<TickerPriceResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<Vec<TickerPriceResponseVariant2Item>>),
}

/// Provider-native `TickerPriceResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerPriceResponseVariant1 {
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TickerPriceResponseVariant2Item` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerPriceResponseVariant2Item {
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TickerTradingDayResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TickerTradingDayResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `lastPrice` wire field.
    #[serde(
        rename = "lastPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_price: Option<Decimal>,
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

/// Exact response for `tickerTradingDay`.
pub type TickerTradingDayResponse = Vec<TickerTradingDayResponseItem>;

/// Provider-native `TradesAggregateResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradesAggregateResponseItem {
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
    /// Exact `M` wire field.
    #[serde(rename = "M", default, skip_serializing_if = "Option::is_none")]
    pub upper_m: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `tradesAggregate`.
pub type TradesAggregateResponse = Vec<TradesAggregateResponseItem>;

/// Provider-native `TradesHistoricalResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradesHistoricalResponseItem {
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
    /// Exact `isBestMatch` wire field.
    #[serde(
        rename = "isBestMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_best_match: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `tradesHistorical`.
pub type TradesHistoricalResponse = Vec<TradesHistoricalResponseItem>;

/// Provider-native `BlockTradesHistoricalResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BlockTradesHistoricalResponseItem {
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `blockTradesHistorical`.
pub type BlockTradesHistoricalResponse = Vec<BlockTradesHistoricalResponseItem>;

/// Provider-native `TradesRecentResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradesRecentResponseItem {
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
    /// Exact `isBestMatch` wire field.
    #[serde(
        rename = "isBestMatch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_best_match: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `tradesRecent`.
pub type TradesRecentResponse = Vec<TradesRecentResponseItem>;

/// Exact response for `uiKlines`.
pub type UiKlinesResponse = Vec<Kline>;

/// Provider-native `ReferencePriceResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReferencePriceResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `referencePrice` wire field.
    #[serde(
        rename = "referencePrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_price: Option<Decimal>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
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

/// Provider-native `ReferencePriceCalculationResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReferencePriceCalculationResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `calculationType` wire field.
    #[serde(
        rename = "calculationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub calculation_type: Option<String>,
    /// Exact `bucketCount` wire field.
    #[serde(
        rename = "bucketCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bucket_count: Option<i64>,
    /// Exact `bucketWidthMs` wire field.
    #[serde(
        rename = "bucketWidthMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bucket_width_ms: Option<i64>,
    /// Exact `externalCalculationId` wire field.
    #[serde(
        rename = "externalCalculationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_calculation_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenOrdersCancelAllResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrdersCancelAllResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
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
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OpenOrdersCancelAllResponseItemOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OpenOrdersCancelAllResponseItemOrderReportsItem>>,
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
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenOrdersCancelAllResponseItemOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrdersCancelAllResponseItemOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenOrdersCancelAllResponseItemOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrdersCancelAllResponseItemOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `openOrdersCancelAll`.
pub type OpenOrdersCancelAllResponse =
    Vec<super::wire::BatchResult<OpenOrdersCancelAllResponseItem>>;

/// Provider alternatives for `OrderCancelResponse`; no member is discarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum OrderCancelResponse {
    /// Wire alternative 1.
    Variant1(Box<OrderCancelResponseVariant1>),
    /// Wire alternative 2.
    Variant2(Box<OrderCancelResponseVariant2>),
}

/// Provider-native `OrderCancelResponseVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelResponseVariant1 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderCancelResponseVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelResponseVariant2 {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: ClientOrderId,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderCancelResponseVariant2OrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<OrderCancelResponseVariant2OrderReportsItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderCancelResponseVariant2OrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelResponseVariant2OrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderCancelResponseVariant2OrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelResponseVariant2OrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderPlaceResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderPlaceResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<crate::Symbol>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Exact `fills` wire field.
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<OrderPlaceResponseFillsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderPlaceResponseFillsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderPlaceResponseFillsItem {
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
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderTestResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTestResponse {
    /// Exact `standardCommissionForOrder` wire field.
    #[serde(
        rename = "standardCommissionForOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub standard_commission_for_order: Option<OrderTestResponseStandardCommissionForOrder>,
    /// Exact `specialCommissionForOrder` wire field.
    #[serde(
        rename = "specialCommissionForOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_commission_for_order: Option<OrderTestResponseSpecialCommissionForOrder>,
    /// Exact `taxCommissionForOrder` wire field.
    #[serde(
        rename = "taxCommissionForOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_commission_for_order: Option<OrderTestResponseTaxCommissionForOrder>,
    /// Exact `discount` wire field.
    #[serde(rename = "discount", default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<OrderTestResponseDiscount>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderTestResponseStandardCommissionForOrder` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTestResponseStandardCommissionForOrder {
    /// Exact `maker` wire field.
    #[serde(
        rename = "maker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker: Option<Decimal>,
    /// Exact `taker` wire field.
    #[serde(
        rename = "taker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderTestResponseSpecialCommissionForOrder` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTestResponseSpecialCommissionForOrder {
    /// Exact `maker` wire field.
    #[serde(
        rename = "maker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker: Option<Decimal>,
    /// Exact `taker` wire field.
    #[serde(
        rename = "taker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderTestResponseTaxCommissionForOrder` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTestResponseTaxCommissionForOrder {
    /// Exact `maker` wire field.
    #[serde(
        rename = "maker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker: Option<Decimal>,
    /// Exact `taker` wire field.
    #[serde(
        rename = "taker",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderTestResponseDiscount` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderTestResponseDiscount {
    /// Exact `enabledForAccount` wire field.
    #[serde(
        rename = "enabledForAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_for_account: Option<bool>,
    /// Exact `enabledForSymbol` wire field.
    #[serde(
        rename = "enabledForSymbol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_for_symbol: Option<bool>,
    /// Exact `discountAsset` wire field.
    #[serde(
        rename = "discountAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discount_asset: Option<crate::Asset>,
    /// Exact `discount` wire field.
    #[serde(
        rename = "discount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub discount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SessionSubscriptionsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SessionSubscriptionsResponseItem {
    /// Exact `subscriptionId` wire field.
    #[serde(
        rename = "subscriptionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `sessionSubscriptions`.
pub type SessionSubscriptionsResponse = Vec<SessionSubscriptionsResponseItem>;

/// Provider-native `UserDataStreamSubscribeResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserDataStreamSubscribeResponse {
    /// Exact `subscriptionId` wire field.
    #[serde(rename = "subscriptionId")]
    pub subscription_id: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UserDataStreamSubscribeSignatureResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserDataStreamSubscribeSignatureResponse {
    /// Exact `subscriptionId` wire field.
    #[serde(rename = "subscriptionId")]
    pub subscription_id: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `userDataStreamUnsubscribe`.
pub type UserDataStreamUnsubscribeResponse = BTreeMap<String, serde_json::Value>;

/// Provider-native `AllOrderListsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllOrderListsResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<AllOrderListsResponseItemOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllOrderListsResponseItemOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllOrderListsResponseItemOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `allOrderLists`.
pub type AllOrderListsResponse = Vec<AllOrderListsResponseItem>;

/// Provider-native `MyAllocationsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyAllocationsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `allocationId` wire field.
    #[serde(
        rename = "allocationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allocation_id: Option<i64>,
    /// Exact `allocationType` wire field.
    #[serde(
        rename = "allocationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allocation_type: Option<super::enums::AllocationType>,
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
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Exact `isBuyer` wire field.
    #[serde(rename = "isBuyer", default, skip_serializing_if = "Option::is_none")]
    pub is_buyer: Option<bool>,
    /// Exact `isMaker` wire field.
    #[serde(rename = "isMaker", default, skip_serializing_if = "Option::is_none")]
    pub is_maker: Option<bool>,
    /// Exact `isAllocator` wire field.
    #[serde(
        rename = "isAllocator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_allocator: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `myAllocations`.
pub type MyAllocationsResponse = Vec<MyAllocationsResponseItem>;

/// Provider-native `MyFiltersResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponse {
    /// Exact `exchangeFilters` wire field.
    #[serde(
        rename = "exchangeFilters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange_filters: Option<Vec<MyFiltersResponseExchangeFiltersItem>>,
    /// Exact `symbolFilters` wire field.
    #[serde(
        rename = "symbolFilters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub symbol_filters: Option<Vec<MyFiltersResponseSymbolFiltersItem>>,
    /// Exact `assetFilters` wire field.
    #[serde(
        rename = "assetFilters",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_filters: Option<Vec<MyFiltersResponseAssetFiltersItem>>,
    /// Exact `rateLimits` wire field.
    #[serde(
        rename = "rateLimits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limits: Option<Vec<MyFiltersResponseRateLimitsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider filters with explicit discriminator dispatch and unknown retention.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum MyFiltersResponseExchangeFiltersItem {
    /// Provider `EXCHANGE_MAX_NUM_ORDERS` filter.
    ExchangeMaxNumOrders(Box<MyFiltersResponseExchangeFiltersItemVariant1>),
    /// Provider `EXCHANGE_MAX_NUM_ALGO_ORDERS` filter.
    ExchangeMaxNumAlgoOrders(Box<MyFiltersResponseExchangeFiltersItemVariant2>),
    /// Provider `EXCHANGE_MAX_NUM_ICEBERG_ORDERS` filter.
    ExchangeMaxNumIcebergOrders(Box<MyFiltersResponseExchangeFiltersItemVariant3>),
    /// Provider `EXCHANGE_MAX_NUM_ORDER_LISTS` filter.
    ExchangeMaxNumOrderLists(Box<MyFiltersResponseExchangeFiltersItemVariant4>),
    /// Future filter facts, retained with redacted Debug.
    Unknown(super::event_payloads::UnknownMessage),
}
impl<'de> Deserialize<'de> for MyFiltersResponseExchangeFiltersItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match value.get("filterType").and_then(serde_json::Value::as_str) {
            Some("EXCHANGE_MAX_NUM_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("EXCHANGE_MAX_NUM_ALGO_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumAlgoOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("EXCHANGE_MAX_NUM_ICEBERG_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumIcebergOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("EXCHANGE_MAX_NUM_ORDER_LISTS") => serde_json::from_value(value)
                .map(|v| Self::ExchangeMaxNumOrderLists(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some(_) => Ok(Self::Unknown(value.into())),
            None => Err(serde::de::Error::custom("filter type required")),
        }
    }
}

/// Provider-native `MyFiltersResponseExchangeFiltersItemVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseExchangeFiltersItemVariant1 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrders` wire field.
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseExchangeFiltersItemVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseExchangeFiltersItemVariant2 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumAlgoOrders` wire field.
    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseExchangeFiltersItemVariant3` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseExchangeFiltersItemVariant3 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumIcebergOrders` wire field.
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseExchangeFiltersItemVariant4` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseExchangeFiltersItemVariant4 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrderLists` wire field.
    #[serde(rename = "maxNumOrderLists")]
    pub max_num_order_lists: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider filters with explicit discriminator dispatch and unknown retention.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum MyFiltersResponseSymbolFiltersItem {
    /// Provider `PRICE_FILTER` filter.
    PriceFilter(Box<MyFiltersResponseSymbolFiltersItemVariant1>),
    /// Provider `PERCENT_PRICE` filter.
    PercentPrice(Box<MyFiltersResponseSymbolFiltersItemVariant2>),
    /// Provider `PERCENT_PRICE_BY_SIDE` filter.
    PercentPriceBySide(Box<MyFiltersResponseSymbolFiltersItemVariant3>),
    /// Provider `LOT_SIZE` filter.
    LotSize(Box<MyFiltersResponseSymbolFiltersItemVariant4>),
    /// Provider `MIN_NOTIONAL` filter.
    MinNotional(Box<MyFiltersResponseSymbolFiltersItemVariant5>),
    /// Provider `NOTIONAL` filter.
    Notional(Box<MyFiltersResponseSymbolFiltersItemVariant6>),
    /// Provider `ICEBERG_PARTS` filter.
    IcebergParts(Box<MyFiltersResponseSymbolFiltersItemVariant7>),
    /// Provider `MARKET_LOT_SIZE` filter.
    MarketLotSize(Box<MyFiltersResponseSymbolFiltersItemVariant8>),
    /// Provider `MAX_NUM_ORDERS` filter.
    MaxNumOrders(Box<MyFiltersResponseSymbolFiltersItemVariant9>),
    /// Provider `MAX_NUM_ALGO_ORDERS` filter.
    MaxNumAlgoOrders(Box<MyFiltersResponseSymbolFiltersItemVariant10>),
    /// Provider `MAX_NUM_ICEBERG_ORDERS` filter.
    MaxNumIcebergOrders(Box<MyFiltersResponseSymbolFiltersItemVariant11>),
    /// Provider `MAX_POSITION` filter.
    MaxPosition(Box<MyFiltersResponseSymbolFiltersItemVariant12>),
    /// Provider `TRAILING_DELTA` filter.
    TrailingDelta(Box<MyFiltersResponseSymbolFiltersItemVariant13>),
    /// Provider `T_PLUS_SELL` filter.
    TPlusSell(Box<MyFiltersResponseSymbolFiltersItemVariant14>),
    /// Provider `MAX_NUM_ORDER_LISTS` filter.
    MaxNumOrderLists(Box<MyFiltersResponseSymbolFiltersItemVariant15>),
    /// Provider `MAX_NUM_ORDER_AMENDS` filter.
    MaxNumOrderAmends(Box<MyFiltersResponseSymbolFiltersItemVariant16>),
    /// Future filter facts, retained with redacted Debug.
    Unknown(super::event_payloads::UnknownMessage),
}
impl<'de> Deserialize<'de> for MyFiltersResponseSymbolFiltersItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match value.get("filterType").and_then(serde_json::Value::as_str) {
            Some("PRICE_FILTER") => serde_json::from_value(value)
                .map(|v| Self::PriceFilter(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("PERCENT_PRICE") => serde_json::from_value(value)
                .map(|v| Self::PercentPrice(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("PERCENT_PRICE_BY_SIDE") => serde_json::from_value(value)
                .map(|v| Self::PercentPriceBySide(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("LOT_SIZE") => serde_json::from_value(value)
                .map(|v| Self::LotSize(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MIN_NOTIONAL") => serde_json::from_value(value)
                .map(|v| Self::MinNotional(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("NOTIONAL") => serde_json::from_value(value)
                .map(|v| Self::Notional(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("ICEBERG_PARTS") => serde_json::from_value(value)
                .map(|v| Self::IcebergParts(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MARKET_LOT_SIZE") => serde_json::from_value(value)
                .map(|v| Self::MarketLotSize(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ALGO_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumAlgoOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ICEBERG_ORDERS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumIcebergOrders(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_POSITION") => serde_json::from_value(value)
                .map(|v| Self::MaxPosition(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("TRAILING_DELTA") => serde_json::from_value(value)
                .map(|v| Self::TrailingDelta(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("T_PLUS_SELL") => serde_json::from_value(value)
                .map(|v| Self::TPlusSell(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ORDER_LISTS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumOrderLists(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some("MAX_NUM_ORDER_AMENDS") => serde_json::from_value(value)
                .map(|v| Self::MaxNumOrderAmends(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some(_) => Ok(Self::Unknown(value.into())),
            None => Err(serde::de::Error::custom("filter type required")),
        }
    }
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant1 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `priceExponent` wire field.
    #[serde(
        rename = "priceExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_exponent: Option<i64>,
    /// Exact `minPrice` wire field.
    #[serde(rename = "minPrice", deserialize_with = "super::wire::decimal")]
    pub min_price: Decimal,
    /// Exact `maxPrice` wire field.
    #[serde(rename = "maxPrice", deserialize_with = "super::wire::decimal")]
    pub max_price: Decimal,
    /// Exact `tickSize` wire field.
    #[serde(rename = "tickSize", deserialize_with = "super::wire::decimal")]
    pub tick_size: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant2` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant2 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `multiplierExponent` wire field.
    #[serde(
        rename = "multiplierExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_exponent: Option<i64>,
    /// Exact `multiplierUp` wire field.
    #[serde(rename = "multiplierUp", deserialize_with = "super::wire::decimal")]
    pub multiplier_up: Decimal,
    /// Exact `multiplierDown` wire field.
    #[serde(rename = "multiplierDown", deserialize_with = "super::wire::decimal")]
    pub multiplier_down: Decimal,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant3` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant3 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `multiplierExponent` wire field.
    #[serde(
        rename = "multiplierExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_exponent: Option<i64>,
    /// Exact `bidMultiplierUp` wire field.
    #[serde(rename = "bidMultiplierUp", deserialize_with = "super::wire::decimal")]
    pub bid_multiplier_up: Decimal,
    /// Exact `bidMultiplierDown` wire field.
    #[serde(
        rename = "bidMultiplierDown",
        deserialize_with = "super::wire::decimal"
    )]
    pub bid_multiplier_down: Decimal,
    /// Exact `askMultiplierUp` wire field.
    #[serde(rename = "askMultiplierUp", deserialize_with = "super::wire::decimal")]
    pub ask_multiplier_up: Decimal,
    /// Exact `askMultiplierDown` wire field.
    #[serde(
        rename = "askMultiplierDown",
        deserialize_with = "super::wire::decimal"
    )]
    pub ask_multiplier_down: Decimal,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant4` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant4 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `qtyExponent` wire field.
    #[serde(
        rename = "qtyExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_exponent: Option<i64>,
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

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant5` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant5 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `priceExponent` wire field.
    #[serde(
        rename = "priceExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_exponent: Option<i64>,
    /// Exact `minNotional` wire field.
    #[serde(rename = "minNotional", deserialize_with = "super::wire::decimal")]
    pub min_notional: Decimal,
    /// Exact `applyToMarket` wire field.
    #[serde(rename = "applyToMarket")]
    pub apply_to_market: bool,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant6` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant6 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `priceExponent` wire field.
    #[serde(
        rename = "priceExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_exponent: Option<i64>,
    /// Exact `minNotional` wire field.
    #[serde(rename = "minNotional", deserialize_with = "super::wire::decimal")]
    pub min_notional: Decimal,
    /// Exact `applyMinToMarket` wire field.
    #[serde(rename = "applyMinToMarket")]
    pub apply_min_to_market: bool,
    /// Exact `maxNotional` wire field.
    #[serde(rename = "maxNotional", deserialize_with = "super::wire::decimal")]
    pub max_notional: Decimal,
    /// Exact `applyMaxToMarket` wire field.
    #[serde(rename = "applyMaxToMarket")]
    pub apply_max_to_market: bool,
    /// Exact `avgPriceMins` wire field.
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant7` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant7 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `limit` wire field.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant8` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant8 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `qtyExponent` wire field.
    #[serde(
        rename = "qtyExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_exponent: Option<i64>,
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

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant9` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant9 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrders` wire field.
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant10` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant10 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumAlgoOrders` wire field.
    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant11` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant11 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumIcebergOrders` wire field.
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant12` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant12 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `qtyExponent` wire field.
    #[serde(
        rename = "qtyExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_exponent: Option<i64>,
    /// Exact `maxPosition` wire field.
    #[serde(rename = "maxPosition", deserialize_with = "super::wire::decimal")]
    pub max_position: Decimal,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant13` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant13 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `minTrailingAboveDelta` wire field.
    #[serde(rename = "minTrailingAboveDelta")]
    pub min_trailing_above_delta: i64,
    /// Exact `maxTrailingAboveDelta` wire field.
    #[serde(rename = "maxTrailingAboveDelta")]
    pub max_trailing_above_delta: i64,
    /// Exact `minTrailingBelowDelta` wire field.
    #[serde(rename = "minTrailingBelowDelta")]
    pub min_trailing_below_delta: i64,
    /// Exact `maxTrailingBelowDelta` wire field.
    #[serde(rename = "maxTrailingBelowDelta")]
    pub max_trailing_below_delta: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant14` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant14 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `endTime` wire field.
    #[serde(rename = "endTime")]
    pub end_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant15` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant15 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrderLists` wire field.
    #[serde(rename = "maxNumOrderLists")]
    pub max_num_order_lists: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseSymbolFiltersItemVariant16` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseSymbolFiltersItemVariant16 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `maxNumOrderAmends` wire field.
    #[serde(rename = "maxNumOrderAmends")]
    pub max_num_order_amends: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider filters with explicit discriminator dispatch and unknown retention.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum MyFiltersResponseAssetFiltersItem {
    /// Provider `MAX_ASSET` filter.
    MaxAsset(Box<MyFiltersResponseAssetFiltersItemVariant1>),
    /// Future filter facts, retained with redacted Debug.
    Unknown(super::event_payloads::UnknownMessage),
}
impl<'de> Deserialize<'de> for MyFiltersResponseAssetFiltersItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match value.get("filterType").and_then(serde_json::Value::as_str) {
            Some("MAX_ASSET") => serde_json::from_value(value)
                .map(|v| Self::MaxAsset(Box::new(v)))
                .map_err(serde::de::Error::custom),
            Some(_) => Ok(Self::Unknown(value.into())),
            None => Err(serde::de::Error::custom("filter type required")),
        }
    }
}

/// Provider-native `MyFiltersResponseAssetFiltersItemVariant1` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseAssetFiltersItemVariant1 {
    /// Exact `filterType` wire field.
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// Exact `qtyExponent` wire field.
    #[serde(
        rename = "qtyExponent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub qty_exponent: Option<i64>,
    /// Exact `limit` wire field.
    #[serde(rename = "limit", deserialize_with = "super::wire::decimal")]
    pub limit: Decimal,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyFiltersResponseRateLimitsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyFiltersResponseRateLimitsItem {
    /// Exact `rateLimitType` wire field.
    #[serde(
        rename = "rateLimitType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_limit_type: Option<super::enums::RateLimitType>,
    /// Exact `interval` wire field.
    #[serde(rename = "interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<super::enums::RateLimitInterval>,
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
    /// Exact `count` wire field.
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `MyPreventedMatchesResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MyPreventedMatchesResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `takerOrderId` wire field.
    #[serde(
        rename = "takerOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_order_id: Option<i64>,
    /// Exact `makerSymbol` wire field.
    #[serde(
        rename = "makerSymbol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_symbol: Option<Symbol>,
    /// Exact `makerOrderId` wire field.
    #[serde(
        rename = "makerOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_order_id: Option<i64>,
    /// Exact `tradeGroupId` wire field.
    #[serde(
        rename = "tradeGroupId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_group_id: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `price` wire field.
    #[serde(
        rename = "price",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub price: Option<Decimal>,
    /// Exact `makerPreventedQuantity` wire field.
    #[serde(
        rename = "makerPreventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_prevented_quantity: Option<Decimal>,
    /// Exact `transactTime` wire field.
    #[serde(
        rename = "transactTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transact_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `myPreventedMatches`.
pub type MyPreventedMatchesResponse = Vec<MyPreventedMatchesResponseItem>;

/// Provider-native `OpenOrderListsStatusResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrderListsStatusResponseItem {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OpenOrderListsStatusResponseItemOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OpenOrderListsStatusResponseItemOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenOrderListsStatusResponseItemOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `openOrderListsStatus`.
pub type OpenOrderListsStatusResponse = Vec<OpenOrderListsStatusResponseItem>;

/// Provider-native `OrderAmendmentsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderAmendmentsResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `executionId` wire field.
    #[serde(
        rename = "executionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_id: Option<i64>,
    /// Exact `origClientOrderId` wire field.
    #[serde(
        rename = "origClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id: Option<super::ClientOrderId>,
    /// Exact `newClientOrderId` wire field.
    #[serde(
        rename = "newClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_client_order_id: Option<super::ClientOrderId>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `newQty` wire field.
    #[serde(
        rename = "newQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_qty: Option<Decimal>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `orderAmendments`.
pub type OrderAmendmentsResponse = Vec<OrderAmendmentsResponseItem>;

/// Provider-native `OrderListStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListStatusResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListStatusResponseOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListStatusResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListStatusResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderAmendKeepPriorityResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderAmendKeepPriorityResponse {
    /// Exact `transactTime` wire field.
    #[serde(
        rename = "transactTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transact_time: Option<i64>,
    /// Exact `executionId` wire field.
    #[serde(
        rename = "executionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_id: Option<i64>,
    /// Exact `amendedOrder` wire field.
    #[serde(
        rename = "amendedOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub amended_order: Option<OrderAmendKeepPriorityResponseAmendedOrder>,
    /// Exact `listStatus` wire field.
    #[serde(
        rename = "listStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status: Option<OrderAmendKeepPriorityResponseListStatus>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderAmendKeepPriorityResponseAmendedOrder` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderAmendKeepPriorityResponseAmendedOrder {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    /// Exact `origClientOrderId` wire field.
    #[serde(
        rename = "origClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id: Option<super::ClientOrderId>,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `preventedQty` wire field.
    #[serde(
        rename = "preventedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_qty: Option<Decimal>,
    /// Exact `quoteOrderQty` wire field.
    #[serde(
        rename = "quoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_order_qty: Option<Decimal>,
    /// Exact `cumulativeQuoteQty` wire field.
    #[serde(
        rename = "cumulativeQuoteQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_quote_qty: Option<Decimal>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderAmendKeepPriorityResponseListStatus` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderAmendKeepPriorityResponseListStatus {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderAmendKeepPriorityResponseListStatusOrdersItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderAmendKeepPriorityResponseListStatusOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderAmendKeepPriorityResponseListStatusOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderCancelReplaceResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelReplaceResponse {
    /// Exact `cancelResult` wire field.
    #[serde(rename = "cancelResult")]
    pub cancel_result: String,
    /// Exact `newOrderResult` wire field.
    #[serde(rename = "newOrderResult")]
    pub new_order_result: String,
    /// Exact `cancelResponse` wire field.
    #[serde(rename = "cancelResponse")]
    pub cancel_response: OrderCancelReplaceResponseCancelResponse,
    /// Exact `newOrderResponse` wire field.
    #[serde(rename = "newOrderResponse")]
    pub new_order_response: OrderCancelReplaceResponseNewOrderResponse,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderCancelReplaceResponseCancelResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelReplaceResponseCancelResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `origClientOrderId` wire field.
    #[serde(
        rename = "origClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_client_order_id: Option<super::ClientOrderId>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderCancelReplaceResponseNewOrderResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelReplaceResponseNewOrderResponse {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListCancelResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListCancelResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListCancelResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OrderListCancelResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListCancelResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListCancelResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListCancelResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListCancelResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListPlaceResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OrderListPlaceResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOcoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOcoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListPlaceOcoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OrderListPlaceOcoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOcoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOcoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOcoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOcoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOpoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOpoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListPlaceOpoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OrderListPlaceOpoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOpoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOpoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOpoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOpoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOpocoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOpocoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListPlaceOpocoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OrderListPlaceOpocoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOpocoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOpocoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOpocoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOpocoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `executedQty` wire field.
    #[serde(
        rename = "executedQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_qty: Option<Decimal>,
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `origQty` wire field.
    #[serde(
        rename = "origQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_qty: Option<Decimal>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOtoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOtoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListPlaceOtoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OrderListPlaceOtoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOtoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOtoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOtoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOtoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOtocoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOtocoResponse {
    /// Exact `orderListId` wire field.
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Exact `contingencyType` wire field.
    #[serde(
        rename = "contingencyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contingency_type: Option<super::enums::ContingencyType>,
    /// Exact `listStatusType` wire field.
    #[serde(
        rename = "listStatusType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_status_type: Option<super::enums::ListStatusType>,
    /// Exact `listOrderStatus` wire field.
    #[serde(
        rename = "listOrderStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_order_status: Option<super::enums::ListOrderStatus>,
    /// Exact `listClientOrderId` wire field.
    #[serde(
        rename = "listClientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub list_client_order_id: Option<super::ClientOrderId>,
    /// Exact `transactionTime` wire field.
    #[serde(
        rename = "transactionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_time: Option<i64>,
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orders` wire field.
    #[serde(rename = "orders")]
    pub orders: Vec<OrderListPlaceOtocoResponseOrdersItem>,
    /// Exact `orderReports` wire field.
    #[serde(
        rename = "orderReports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reports: Option<Vec<OrderListPlaceOtocoResponseOrderReportsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOtocoResponseOrdersItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOtocoResponseOrdersItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `orderId` wire field.
    #[serde(rename = "orderId")]
    pub order_id: i64,
    /// Exact `clientOrderId` wire field.
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OrderListPlaceOtocoResponseOrderReportsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderListPlaceOtocoResponseOrderReportsItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SorOrderPlaceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SorOrderPlaceResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
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
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<super::ClientOrderId>,
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
    /// Exact `origQuoteOrderQty` wire field.
    #[serde(
        rename = "origQuoteOrderQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_quote_order_qty: Option<Decimal>,
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
    pub status: Option<super::enums::OrderStatus>,
    /// Exact `timeInForce` wire field.
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<super::enums::TimeInForce>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<super::enums::OrderType>,
    /// Exact `side` wire field.
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<super::enums::OrderSide>,
    /// Exact `workingTime` wire field.
    #[serde(
        rename = "workingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_time: Option<i64>,
    /// Exact `fills` wire field.
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<SorOrderPlaceResponseItemFillsItem>>,
    /// Exact `workingFloor` wire field.
    #[serde(
        rename = "workingFloor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub working_floor: Option<super::enums::WorkingFloor>,
    /// Exact `selfTradePreventionMode` wire field.
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<super::enums::SelfTradePreventionMode>,
    /// Exact `usedSor` wire field.
    #[serde(rename = "usedSor", default, skip_serializing_if = "Option::is_none")]
    pub used_sor: Option<bool>,
    /// Exact `stopPrice` wire field.
    #[serde(
        rename = "stopPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_price: Option<Decimal>,
    /// Exact `trailingDelta` wire field.
    #[serde(
        rename = "trailingDelta",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_delta: Option<i64>,
    /// Exact `icebergQty` wire field.
    #[serde(
        rename = "icebergQty",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub iceberg_qty: Option<Decimal>,
    /// Exact `strategyId` wire field.
    #[serde(
        rename = "strategyId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_id: Option<i64>,
    /// Exact `strategyType` wire field.
    #[serde(
        rename = "strategyType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_type: Option<i64>,
    /// Exact `preventedMatchId` wire field.
    #[serde(
        rename = "preventedMatchId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_match_id: Option<i64>,
    /// Exact `preventedQuantity` wire field.
    #[serde(
        rename = "preventedQuantity",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevented_quantity: Option<Decimal>,
    /// Exact `trailingTime` wire field.
    #[serde(
        rename = "trailingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_time: Option<i64>,
    /// Exact `pegPriceType` wire field.
    #[serde(
        rename = "pegPriceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_price_type: Option<String>,
    /// Exact `pegOffsetType` wire field.
    #[serde(
        rename = "pegOffsetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_type: Option<String>,
    /// Exact `pegOffsetValue` wire field.
    #[serde(
        rename = "pegOffsetValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub peg_offset_value: Option<i64>,
    /// Exact `peggedPrice` wire field.
    #[serde(
        rename = "peggedPrice",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pegged_price: Option<Decimal>,
    /// Exact `expiryReason` wire field.
    #[serde(
        rename = "expiryReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiry_reason: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SorOrderPlaceResponseItemFillsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SorOrderPlaceResponseItemFillsItem {
    /// Exact `matchType` wire field.
    #[serde(rename = "matchType", default, skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
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
    pub commission_asset: Option<String>,
    /// Exact `tradeId` wire field.
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,
    /// Exact `allocId` wire field.
    #[serde(rename = "allocId", default, skip_serializing_if = "Option::is_none")]
    pub alloc_id: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `sorOrderPlace`.
pub type SorOrderPlaceResponse = Vec<SorOrderPlaceResponseItem>;

/// Provider-native `SorOrderTestResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SorOrderTestResponse {
    /// Exact `standardCommissionForOrder` wire field.
    #[serde(
        rename = "standardCommissionForOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub standard_commission_for_order: Option<SorOrderTestResponseStandardCommissionForOrder>,
    /// Exact `taxCommissionForOrder` wire field.
    #[serde(
        rename = "taxCommissionForOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_commission_for_order: Option<SorOrderTestResponseTaxCommissionForOrder>,
    /// Exact `discount` wire field.
    #[serde(rename = "discount", default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<SorOrderTestResponseDiscount>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SorOrderTestResponseStandardCommissionForOrder` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SorOrderTestResponseStandardCommissionForOrder {
    /// Exact `maker` wire field.
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
    /// Exact `taker` wire field.
    #[serde(rename = "taker", default, skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SorOrderTestResponseTaxCommissionForOrder` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SorOrderTestResponseTaxCommissionForOrder {
    /// Exact `maker` wire field.
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
    /// Exact `taker` wire field.
    #[serde(rename = "taker", default, skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SorOrderTestResponseDiscount` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SorOrderTestResponseDiscount {
    /// Exact `enabledForAccount` wire field.
    #[serde(
        rename = "enabledForAccount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_for_account: Option<bool>,
    /// Exact `enabledForSymbol` wire field.
    #[serde(
        rename = "enabledForSymbol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_for_symbol: Option<bool>,
    /// Exact `discountAsset` wire field.
    #[serde(
        rename = "discountAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discount_asset: Option<String>,
    /// Exact `discount` wire field.
    #[serde(rename = "discount", default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
