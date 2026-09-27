// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::Decimal;
use crate::Symbol;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Provider-native `AccountApiTradingStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountApiTradingStatusResponse {
    /// Exact `data` wire field.
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<AccountApiTradingStatusResponseData>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountApiTradingStatusResponseData` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountApiTradingStatusResponseData {
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
    /// Exact `triggerCondition` wire field.
    #[serde(
        rename = "triggerCondition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_condition: Option<AccountApiTradingStatusResponseDataTriggerCondition>,
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

/// Provider-native `AccountApiTradingStatusResponseDataTriggerCondition` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountApiTradingStatusResponseDataTriggerCondition {
    /// Exact `GCR` wire field.
    #[serde(rename = "GCR", default, skip_serializing_if = "Option::is_none")]
    pub upper_gcr: Option<i64>,
    /// Exact `IFER` wire field.
    #[serde(rename = "IFER", default, skip_serializing_if = "Option::is_none")]
    pub upper_ifer: Option<i64>,
    /// Exact `UFR` wire field.
    #[serde(rename = "UFR", default, skip_serializing_if = "Option::is_none")]
    pub upper_ufr: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountInfoResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInfoResponse {
    /// Exact `vipLevel` wire field.
    #[serde(rename = "vipLevel", default, skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<i64>,
    /// Exact `isMarginEnabled` wire field.
    #[serde(
        rename = "isMarginEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_margin_enabled: Option<bool>,
    /// Exact `isFutureEnabled` wire field.
    #[serde(
        rename = "isFutureEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_future_enabled: Option<bool>,
    /// Exact `isOptionsEnabled` wire field.
    #[serde(
        rename = "isOptionsEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_options_enabled: Option<bool>,
    /// Exact `isPortfolioMarginRetailEnabled` wire field.
    #[serde(
        rename = "isPortfolioMarginRetailEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_portfolio_margin_retail_enabled: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AccountStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountStatusResponse {
    /// Exact `data` wire field.
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DailyAccountSnapshotResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyAccountSnapshotResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: i64,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<crate::SensitiveString>,
    /// Exact `snapshotVos` wire field.
    #[serde(rename = "snapshotVos")]
    pub snapshot_vos: Vec<DailyAccountSnapshotResponseSnapshotVosItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DailyAccountSnapshotResponseSnapshotVosItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyAccountSnapshotResponseSnapshotVosItem {
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: DailyAccountSnapshotResponseSnapshotVosItemData,
    /// Exact `type` wire field.
    #[serde(rename = "type")]
    pub type_value: String,
    /// Exact `updateTime` wire field.
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DailyAccountSnapshotResponseSnapshotVosItemData` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyAccountSnapshotResponseSnapshotVosItemData {
    /// Exact `balances` wire field.
    #[serde(rename = "balances", default, skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<DailyAccountSnapshotResponseSnapshotVosItemDataBalancesItem>>,
    /// Exact `totalAssetOfBtc` wire field.
    #[serde(
        rename = "totalAssetOfBtc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_asset_of_btc: Option<Decimal>,
    /// Exact `marginLevel` wire field.
    #[serde(
        rename = "marginLevel",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_level: Option<Decimal>,
    /// Exact `totalLiabilityOfBtc` wire field.
    #[serde(
        rename = "totalLiabilityOfBtc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_liability_of_btc: Option<Decimal>,
    /// Exact `totalNetAssetOfBtc` wire field.
    #[serde(
        rename = "totalNetAssetOfBtc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_net_asset_of_btc: Option<Decimal>,
    /// Exact `userAssets` wire field.
    #[serde(
        rename = "userAssets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_assets: Option<Vec<DailyAccountSnapshotResponseSnapshotVosItemDataUserAssetsItem>>,
    /// Exact `assets` wire field.
    #[serde(rename = "assets", default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<DailyAccountSnapshotResponseSnapshotVosItemDataAssetsItem>>,
    /// Exact `position` wire field.
    #[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec<DailyAccountSnapshotResponseSnapshotVosItemDataPositionItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DailyAccountSnapshotResponseSnapshotVosItemDataBalancesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyAccountSnapshotResponseSnapshotVosItemDataBalancesItem {
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

/// Provider-native `DailyAccountSnapshotResponseSnapshotVosItemDataUserAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyAccountSnapshotResponseSnapshotVosItemDataUserAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `borrowed` wire field.
    #[serde(
        rename = "borrowed",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub borrowed: Option<Decimal>,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `interest` wire field.
    #[serde(
        rename = "interest",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest: Option<Decimal>,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `netAsset` wire field.
    #[serde(
        rename = "netAsset",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_asset: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DailyAccountSnapshotResponseSnapshotVosItemDataAssetsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyAccountSnapshotResponseSnapshotVosItemDataAssetsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `marginBalance` wire field.
    #[serde(
        rename = "marginBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_balance: Option<Decimal>,
    /// Exact `walletBalance` wire field.
    #[serde(
        rename = "walletBalance",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub wallet_balance: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DailyAccountSnapshotResponseSnapshotVosItemDataPositionItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyAccountSnapshotResponseSnapshotVosItemDataPositionItem {
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
    /// Exact `positionAmt` wire field.
    #[serde(
        rename = "positionAmt",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_amt: Option<Decimal>,
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
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DisableFastWithdrawSwitchResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider empty object receipt with retained future fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EnableFastWithdrawSwitchResponse {
    /// Future fields; never logged implicitly.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetApiKeyPermissionResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetApiKeyPermissionResponse {
    /// Exact `ipRestrict` wire field.
    #[serde(
        rename = "ipRestrict",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_restrict: Option<bool>,
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `enableReading` wire field.
    #[serde(
        rename = "enableReading",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_reading: Option<bool>,
    /// Exact `enableWithdrawals` wire field.
    #[serde(
        rename = "enableWithdrawals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_withdrawals: Option<bool>,
    /// Exact `enableInternalTransfer` wire field.
    #[serde(
        rename = "enableInternalTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_internal_transfer: Option<bool>,
    /// Exact `enableMargin` wire field.
    #[serde(
        rename = "enableMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_margin: Option<bool>,
    /// Exact `enableFutures` wire field.
    #[serde(
        rename = "enableFutures",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_futures: Option<bool>,
    /// Exact `permitsUniversalTransfer` wire field.
    #[serde(
        rename = "permitsUniversalTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permits_universal_transfer: Option<bool>,
    /// Exact `enableVanillaOptions` wire field.
    #[serde(
        rename = "enableVanillaOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_vanilla_options: Option<bool>,
    /// Exact `enableFixApiTrade` wire field.
    #[serde(
        rename = "enableFixApiTrade",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_fix_api_trade: Option<bool>,
    /// Exact `enableFixReadOnly` wire field.
    #[serde(
        rename = "enableFixReadOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_fix_read_only: Option<bool>,
    /// Exact `enableSpotAndMarginTrading` wire field.
    #[serde(
        rename = "enableSpotAndMarginTrading",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_spot_and_margin_trading: Option<bool>,
    /// Exact `enablePortfolioMarginTrading` wire field.
    #[serde(
        rename = "enablePortfolioMarginTrading",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_portfolio_margin_trading: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AssetDetailResponseValue` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetDetailResponseValue {
    /// Exact `minWithdrawAmount` wire field.
    #[serde(
        rename = "minWithdrawAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_withdraw_amount: Option<Decimal>,
    /// Exact `depositStatus` wire field.
    #[serde(
        rename = "depositStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_status: Option<bool>,
    /// Exact `withdrawFee` wire field.
    #[serde(
        rename = "withdrawFee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_fee: Option<Decimal>,
    /// Exact `withdrawStatus` wire field.
    #[serde(
        rename = "withdrawStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_status: Option<bool>,
    /// Exact `depositTip` wire field.
    #[serde(
        rename = "depositTip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_tip: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `assetDetail`.
pub type AssetDetailResponse = BTreeMap<crate::Asset, AssetDetailResponseValue>;

/// Provider-native `AssetDividendRecordResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetDividendRecordResponse {
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<AssetDividendRecordResponseRowsItem>>,
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AssetDividendRecordResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetDividendRecordResponseRowsItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `divTime` wire field.
    #[serde(rename = "divTime", default, skip_serializing_if = "Option::is_none")]
    pub div_time: Option<i64>,
    /// Exact `enInfo` wire field.
    #[serde(rename = "enInfo", default, skip_serializing_if = "Option::is_none")]
    pub en_info: Option<String>,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Exact `direction` wire field.
    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustConvertResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustConvertResponse {
    /// Exact `totalTransfered` wire field.
    #[serde(
        rename = "totalTransfered",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transfered: Option<Decimal>,
    /// Exact `totalServiceCharge` wire field.
    #[serde(
        rename = "totalServiceCharge",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_service_charge: Option<Decimal>,
    /// Exact `transferResult` wire field.
    #[serde(
        rename = "transferResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_result: Option<Vec<DustConvertResponseTransferResultItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustConvertResponseTransferResultItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustConvertResponseTransferResultItem {
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset")]
    pub from_asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `transferedAmount` wire field.
    #[serde(
        rename = "transferedAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfered_amount: Option<Decimal>,
    /// Exact `serviceChargeAmount` wire field.
    #[serde(
        rename = "serviceChargeAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_charge_amount: Option<Decimal>,
    /// Exact `operateTime` wire field.
    #[serde(
        rename = "operateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operate_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustConvertibleAssetsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustConvertibleAssetsResponse {
    /// Exact `dribbletPercentage` wire field.
    #[serde(
        rename = "dribbletPercentage",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dribblet_percentage: Option<Decimal>,
    /// Exact `totalTransferQuotaAssetAmount` wire field.
    #[serde(
        rename = "totalTransferQuotaAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transfer_quota_asset_amount: Option<Decimal>,
    /// Exact `totalTransferTargetAssetAmount` wire field.
    #[serde(
        rename = "totalTransferTargetAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transfer_target_asset_amount: Option<Decimal>,
    /// Exact `dribbletBase` wire field.
    #[serde(
        rename = "dribbletBase",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dribblet_base: Option<Decimal>,
    /// Exact `details` wire field.
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<DustConvertibleAssetsResponseDetailsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustConvertibleAssetsResponseDetailsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustConvertibleAssetsResponseDetailsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `assetFullName` wire field.
    #[serde(
        rename = "assetFullName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_full_name: Option<String>,
    /// Exact `amountFree` wire field.
    #[serde(
        rename = "amountFree",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount_free: Option<Decimal>,
    /// Exact `exchange` wire field.
    #[serde(
        rename = "exchange",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange: Option<Decimal>,
    /// Exact `toQuotaAssetAmount` wire field.
    #[serde(
        rename = "toQuotaAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_quota_asset_amount: Option<Decimal>,
    /// Exact `toTargetAssetAmount` wire field.
    #[serde(
        rename = "toTargetAssetAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_target_asset_amount: Option<Decimal>,
    /// Exact `toTargetAssetOffExchange` wire field.
    #[serde(
        rename = "toTargetAssetOffExchange",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_target_asset_off_exchange: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustlogResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustlogResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Exact `userAssetDribblets` wire field.
    #[serde(
        rename = "userAssetDribblets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_asset_dribblets: Option<Vec<DustlogResponseUserAssetDribbletsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustlogResponseUserAssetDribbletsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustlogResponseUserAssetDribbletsItem {
    /// Exact `operateTime` wire field.
    #[serde(
        rename = "operateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operate_time: Option<i64>,
    /// Exact `totalTransferedAmount` wire field.
    #[serde(
        rename = "totalTransferedAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transfered_amount: Option<Decimal>,
    /// Exact `totalServiceChargeAmount` wire field.
    #[serde(
        rename = "totalServiceChargeAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_service_charge_amount: Option<Decimal>,
    /// Exact `transId` wire field.
    #[serde(rename = "transId", default, skip_serializing_if = "Option::is_none")]
    pub trans_id: Option<i64>,
    /// Exact `userAssetDribbletDetails` wire field.
    #[serde(
        rename = "userAssetDribbletDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_asset_dribblet_details:
        Option<Vec<DustlogResponseUserAssetDribbletsItemUserAssetDribbletDetailsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustlogResponseUserAssetDribbletsItemUserAssetDribbletDetailsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustlogResponseUserAssetDribbletsItemUserAssetDribbletDetailsItem {
    /// Exact `transId` wire field.
    #[serde(rename = "transId", default, skip_serializing_if = "Option::is_none")]
    pub trans_id: Option<i64>,
    /// Exact `serviceChargeAmount` wire field.
    #[serde(
        rename = "serviceChargeAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_charge_amount: Option<Decimal>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `operateTime` wire field.
    #[serde(
        rename = "operateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operate_time: Option<i64>,
    /// Exact `transferedAmount` wire field.
    #[serde(
        rename = "transferedAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfered_amount: Option<Decimal>,
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset")]
    pub from_asset: crate::Asset,
    /// Exact `targetAsset` wire field.
    #[serde(
        rename = "targetAsset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_asset: Option<crate::Asset>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustTransferResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustTransferResponse {
    /// Exact `totalServiceCharge` wire field.
    #[serde(
        rename = "totalServiceCharge",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_service_charge: Option<Decimal>,
    /// Exact `totalTransfered` wire field.
    #[serde(
        rename = "totalTransfered",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transfered: Option<Decimal>,
    /// Exact `transferResult` wire field.
    #[serde(
        rename = "transferResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_result: Option<Vec<DustTransferResponseTransferResultItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DustTransferResponseTransferResultItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DustTransferResponseTransferResultItem {
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `fromAsset` wire field.
    #[serde(rename = "fromAsset")]
    pub from_asset: crate::Asset,
    /// Exact `operateTime` wire field.
    #[serde(
        rename = "operateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operate_time: Option<i64>,
    /// Exact `serviceChargeAmount` wire field.
    #[serde(
        rename = "serviceChargeAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_charge_amount: Option<Decimal>,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Exact `transferedAmount` wire field.
    #[serde(
        rename = "transferedAmount",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfered_amount: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `FundingWalletResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FundingWalletResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `freeze` wire field.
    #[serde(
        rename = "freeze",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub freeze: Option<Decimal>,
    /// Exact `withdrawing` wire field.
    #[serde(
        rename = "withdrawing",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawing: Option<Decimal>,
    /// Exact `btcValuation` wire field.
    #[serde(
        rename = "btcValuation",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub btc_valuation: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `fundingWallet`.
pub type FundingWalletResponse = Vec<FundingWalletResponseItem>;

/// Provider-native `GetSpotAssetTagsResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetSpotAssetTagsResponseItem {
    /// Exact `assetCode` wire field.
    #[serde(rename = "assetCode", default, skip_serializing_if = "Option::is_none")]
    pub asset_code: Option<crate::Asset>,
    /// Exact `assetName` wire field.
    #[serde(rename = "assetName", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    /// Exact `trading` wire field.
    #[serde(rename = "trading", default, skip_serializing_if = "Option::is_none")]
    pub trading: Option<bool>,
    /// Exact `tags` wire field.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getSpotAssetTags`.
pub type GetSpotAssetTagsResponse = Vec<GetSpotAssetTagsResponseItem>;

/// Provider-native `GetAssetsThatCanBeConvertedIntoBnbResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAssetsThatCanBeConvertedIntoBnbResponse {
    /// Exact `details` wire field.
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<GetAssetsThatCanBeConvertedIntoBnbResponseDetailsItem>>,
    /// Exact `totalTransferBtc` wire field.
    #[serde(
        rename = "totalTransferBtc",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transfer_btc: Option<Decimal>,
    /// Exact `totalTransferBNB` wire field.
    #[serde(
        rename = "totalTransferBNB",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_transfer_bnb: Option<Decimal>,
    /// Exact `dribbletPercentage` wire field.
    #[serde(
        rename = "dribbletPercentage",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dribblet_percentage: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetAssetsThatCanBeConvertedIntoBnbResponseDetailsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAssetsThatCanBeConvertedIntoBnbResponseDetailsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `assetFullName` wire field.
    #[serde(
        rename = "assetFullName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_full_name: Option<String>,
    /// Exact `amountFree` wire field.
    #[serde(
        rename = "amountFree",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub amount_free: Option<Decimal>,
    /// Exact `toBTC` wire field.
    #[serde(
        rename = "toBTC",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_btc: Option<Decimal>,
    /// Exact `toBNB` wire field.
    #[serde(
        rename = "toBNB",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_bnb: Option<Decimal>,
    /// Exact `toBNBOffExchange` wire field.
    #[serde(
        rename = "toBNBOffExchange",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_bnb_off_exchange: Option<Decimal>,
    /// Exact `exchange` wire field.
    #[serde(
        rename = "exchange",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exchange: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCloudMiningPaymentAndRefundHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCloudMiningPaymentAndRefundHistoryResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<GetCloudMiningPaymentAndRefundHistoryResponseRowsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCloudMiningPaymentAndRefundHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCloudMiningPaymentAndRefundHistoryResponseRowsItem {
    /// Exact `createTime` wire field.
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<i64>,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<i64>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetOpenSymbolListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetOpenSymbolListResponseItem {
    /// Exact `openTime` wire field.
    #[serde(rename = "openTime", default, skip_serializing_if = "Option::is_none")]
    pub open_time: Option<i64>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getOpenSymbolList`.
pub type GetOpenSymbolListResponse = Vec<GetOpenSymbolListResponseItem>;

/// Provider-native `QueryUserDelegationHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserDelegationHistoryResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<QueryUserDelegationHistoryResponseRowsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserDelegationHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserDelegationHistoryResponseRowsItem {
    /// Exact `clientTranId` wire field.
    #[serde(
        rename = "clientTranId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_tran_id: Option<String>,
    /// Exact `transferType` wire field.
    #[serde(
        rename = "transferType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_type: Option<String>,
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserUniversalTransferHistoryResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserUniversalTransferHistoryResponse {
    /// Exact `total` wire field.
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Exact `rows` wire field.
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<QueryUserUniversalTransferHistoryResponseRowsItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserUniversalTransferHistoryResponseRowsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserUniversalTransferHistoryResponseRowsItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `type` wire field.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Exact `timestamp` wire field.
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `UserUniversalTransferResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserUniversalTransferResponse {
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserWalletBalanceResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserWalletBalanceResponseItem {
    /// Exact `activate` wire field.
    #[serde(rename = "activate")]
    pub activate: bool,
    /// Exact `balance` wire field.
    #[serde(rename = "balance", deserialize_with = "super::wire::decimal")]
    pub balance: Decimal,
    /// Exact `walletName` wire field.
    #[serde(rename = "walletName")]
    pub wallet_name: String,
    /// Exact `assetBalances` wire field.
    #[serde(
        rename = "assetBalances",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_balances: Option<Vec<QueryUserWalletBalanceResponseItemAssetBalancesItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `QueryUserWalletBalanceResponseItemAssetBalancesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueryUserWalletBalanceResponseItemAssetBalancesItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `assetName` wire field.
    #[serde(rename = "assetName", default, skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `freeze` wire field.
    #[serde(
        rename = "freeze",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub freeze: Option<Decimal>,
    /// Exact `withdrawing` wire field.
    #[serde(
        rename = "withdrawing",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawing: Option<Decimal>,
    /// Exact `btcValuation` wire field.
    #[serde(
        rename = "btcValuation",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub btc_valuation: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `queryUserWalletBalance`.
pub type QueryUserWalletBalanceResponse = Vec<QueryUserWalletBalanceResponseItem>;

/// Provider-native `ToggleBnbBurnOnSpotTradeAndMarginInterestResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterestResponse {
    /// Exact `spotBNBBurn` wire field.
    #[serde(
        rename = "spotBNBBurn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spot_bnb_burn: Option<bool>,
    /// Exact `interestBNBBurn` wire field.
    #[serde(
        rename = "interestBNBBurn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_bnb_burn: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `TradeFeeResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeFeeResponseItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
    /// Exact `makerCommission` wire field.
    #[serde(
        rename = "makerCommission",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_commission: Option<Decimal>,
    /// Exact `takerCommission` wire field.
    #[serde(
        rename = "takerCommission",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_commission: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `tradeFee`.
pub type TradeFeeResponse = Vec<TradeFeeResponseItem>;

/// Provider-native `UserAssetResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserAssetResponseItem {
    /// Exact `asset` wire field.
    #[serde(rename = "asset")]
    pub asset: crate::Asset,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `freeze` wire field.
    #[serde(
        rename = "freeze",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub freeze: Option<Decimal>,
    /// Exact `withdrawing` wire field.
    #[serde(
        rename = "withdrawing",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawing: Option<Decimal>,
    /// Exact `ipoable` wire field.
    #[serde(
        rename = "ipoable",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ipoable: Option<Decimal>,
    /// Exact `btcValuation` wire field.
    #[serde(
        rename = "btcValuation",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub btc_valuation: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `userAsset`.
pub type UserAssetResponse = Vec<UserAssetResponseItem>;

/// Provider-native `AllCoinsInformationResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllCoinsInformationResponseItem {
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `depositAllEnable` wire field.
    #[serde(
        rename = "depositAllEnable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_all_enable: Option<bool>,
    /// Exact `withdrawAllEnable` wire field.
    #[serde(
        rename = "withdrawAllEnable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_all_enable: Option<bool>,
    /// Exact `name` wire field.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Exact `free` wire field.
    #[serde(rename = "free", deserialize_with = "super::wire::decimal")]
    pub free: Decimal,
    /// Exact `locked` wire field.
    #[serde(rename = "locked", deserialize_with = "super::wire::decimal")]
    pub locked: Decimal,
    /// Exact `freeze` wire field.
    #[serde(
        rename = "freeze",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub freeze: Option<Decimal>,
    /// Exact `withdrawing` wire field.
    #[serde(
        rename = "withdrawing",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawing: Option<Decimal>,
    /// Exact `ipoing` wire field.
    #[serde(
        rename = "ipoing",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ipoing: Option<Decimal>,
    /// Exact `ipoable` wire field.
    #[serde(
        rename = "ipoable",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ipoable: Option<Decimal>,
    /// Exact `storage` wire field.
    #[serde(
        rename = "storage",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage: Option<Decimal>,
    /// Exact `isLegalMoney` wire field.
    #[serde(
        rename = "isLegalMoney",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_legal_money: Option<bool>,
    /// Exact `trading` wire field.
    #[serde(rename = "trading", default, skip_serializing_if = "Option::is_none")]
    pub trading: Option<bool>,
    /// Exact `networkList` wire field.
    #[serde(
        rename = "networkList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_list: Option<Vec<AllCoinsInformationResponseItemNetworkListItem>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `AllCoinsInformationResponseItemNetworkListItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllCoinsInformationResponseItemNetworkListItem {
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `withdrawIntegerMultiple` wire field.
    #[serde(
        rename = "withdrawIntegerMultiple",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_integer_multiple: Option<Decimal>,
    /// Exact `isDefault` wire field.
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Exact `depositEnable` wire field.
    #[serde(
        rename = "depositEnable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_enable: Option<bool>,
    /// Exact `withdrawEnable` wire field.
    #[serde(
        rename = "withdrawEnable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_enable: Option<bool>,
    /// Exact `depositDesc` wire field.
    #[serde(
        rename = "depositDesc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_desc: Option<String>,
    /// Exact `withdrawDesc` wire field.
    #[serde(
        rename = "withdrawDesc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_desc: Option<String>,
    /// Exact `specialTips` wire field.
    #[serde(
        rename = "specialTips",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_tips: Option<String>,
    /// Exact `specialWithdrawTips` wire field.
    #[serde(
        rename = "specialWithdrawTips",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_withdraw_tips: Option<String>,
    /// Exact `name` wire field.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Exact `resetAddressStatus` wire field.
    #[serde(
        rename = "resetAddressStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reset_address_status: Option<bool>,
    /// Exact `addressRegex` wire field.
    #[serde(
        rename = "addressRegex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_regex: Option<String>,
    /// Exact `memoRegex` wire field.
    #[serde(rename = "memoRegex", default, skip_serializing_if = "Option::is_none")]
    pub memo_regex: Option<String>,
    /// Exact `withdrawFee` wire field.
    #[serde(
        rename = "withdrawFee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_fee: Option<Decimal>,
    /// Exact `withdrawMin` wire field.
    #[serde(
        rename = "withdrawMin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_min: Option<Decimal>,
    /// Exact `withdrawMax` wire field.
    #[serde(
        rename = "withdrawMax",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_max: Option<Decimal>,
    /// Exact `withdrawInternalMin` wire field.
    #[serde(
        rename = "withdrawInternalMin",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_internal_min: Option<Decimal>,
    /// Exact `depositDust` wire field.
    #[serde(
        rename = "depositDust",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_dust: Option<Decimal>,
    /// Exact `minConfirm` wire field.
    #[serde(
        rename = "minConfirm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_confirm: Option<i64>,
    /// Exact `unLockConfirm` wire field.
    #[serde(
        rename = "unLockConfirm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub un_lock_confirm: Option<i64>,
    /// Exact `sameAddress` wire field.
    #[serde(
        rename = "sameAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub same_address: Option<bool>,
    /// Exact `withdrawTag` wire field.
    #[serde(
        rename = "withdrawTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_tag: Option<bool>,
    /// Exact `estimatedArrivalTime` wire field.
    #[serde(
        rename = "estimatedArrivalTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_arrival_time: Option<i64>,
    /// Exact `busy` wire field.
    #[serde(rename = "busy", default, skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    /// Exact `contractAddressUrl` wire field.
    #[serde(
        rename = "contractAddressUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_address_url: Option<String>,
    /// Exact `contractAddress` wire field.
    #[serde(
        rename = "contractAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_address: Option<String>,
    /// Exact `denomination` wire field.
    #[serde(
        rename = "denomination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub denomination: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `allCoinsInformation`.
pub type AllCoinsInformationResponse = Vec<AllCoinsInformationResponseItem>;

/// Provider-native `DepositAddressResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepositAddressResponse {
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `tag` wire field.
    #[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<crate::SensitiveString>,
    /// Exact `url` wire field.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DepositHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepositHistoryResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `addressTag` wire field.
    #[serde(
        rename = "addressTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_tag: Option<crate::SensitiveString>,
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<crate::SensitiveString>,
    /// Exact `insertTime` wire field.
    #[serde(
        rename = "insertTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_time: Option<i64>,
    /// Exact `completeTime` wire field.
    #[serde(
        rename = "completeTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complete_time: Option<i64>,
    /// Exact `transferType` wire field.
    #[serde(
        rename = "transferType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_type: Option<i64>,
    /// Exact `confirmTimes` wire field.
    #[serde(
        rename = "confirmTimes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confirm_times: Option<String>,
    /// Exact `unlockConfirm` wire field.
    #[serde(
        rename = "unlockConfirm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unlock_confirm: Option<i64>,
    /// Exact `walletType` wire field.
    #[serde(
        rename = "walletType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wallet_type: Option<i64>,
    /// Exact `travelRuleStatus` wire field.
    #[serde(
        rename = "travelRuleStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub travel_rule_status: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `depositHistory`.
pub type DepositHistoryResponse = Vec<DepositHistoryResponseItem>;

/// Provider-native `FetchDepositAddressListWithNetworkResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FetchDepositAddressListWithNetworkResponseItem {
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `tag` wire field.
    #[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<crate::SensitiveString>,
    /// Exact `isDefault` wire field.
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `fetchDepositAddressListWithNetwork`.
pub type FetchDepositAddressListWithNetworkResponse =
    Vec<FetchDepositAddressListWithNetworkResponseItem>;

/// Provider-native `FetchWithdrawAddressListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FetchWithdrawAddressListResponseItem {
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `addressTag` wire field.
    #[serde(
        rename = "addressTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_tag: Option<crate::SensitiveString>,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `name` wire field.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `origin` wire field.
    #[serde(rename = "origin", default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Exact `originType` wire field.
    #[serde(
        rename = "originType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_type: Option<String>,
    /// Exact `whiteStatus` wire field.
    #[serde(
        rename = "whiteStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub white_status: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `fetchWithdrawAddressList`.
pub type FetchWithdrawAddressListResponse = Vec<FetchWithdrawAddressListResponseItem>;

/// Provider-native `FetchWithdrawQuotaResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FetchWithdrawQuotaResponse {
    /// Exact `wdQuota` wire field.
    #[serde(
        rename = "wdQuota",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub wd_quota: Option<Decimal>,
    /// Exact `usedWdQuota` wire field.
    #[serde(
        rename = "usedWdQuota",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub used_wd_quota: Option<Decimal>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `OneClickArrivalDepositApplyResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OneClickArrivalDepositApplyResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<crate::SensitiveString>,
    /// Exact `data` wire field.
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// Exact `success` wire field.
    #[serde(rename = "success", default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `WithdrawResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WithdrawResponse {
    /// Exact `id` wire field.
    #[serde(rename = "id")]
    pub id: String,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `WithdrawHistoryResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WithdrawHistoryResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `transactionFee` wire field.
    #[serde(
        rename = "transactionFee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_fee: Option<Decimal>,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<crate::SensitiveString>,
    /// Exact `applyTime` wire field.
    #[serde(rename = "applyTime", default, skip_serializing_if = "Option::is_none")]
    pub apply_time: Option<String>,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `transferType` wire field.
    #[serde(
        rename = "transferType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_type: Option<i64>,
    /// Exact `withdrawOrderId` wire field.
    #[serde(
        rename = "withdrawOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_order_id: Option<super::WithdrawalId>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Exact `confirmNo` wire field.
    #[serde(rename = "confirmNo", default, skip_serializing_if = "Option::is_none")]
    pub confirm_no: Option<i64>,
    /// Exact `walletType` wire field.
    #[serde(
        rename = "walletType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wallet_type: Option<i64>,
    /// Exact `txKey` wire field.
    #[serde(rename = "txKey", default, skip_serializing_if = "Option::is_none")]
    pub tx_key: Option<crate::SensitiveString>,
    /// Exact `completeTime` wire field.
    #[serde(
        rename = "completeTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complete_time: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `withdrawHistory`.
pub type WithdrawHistoryResponse = Vec<WithdrawHistoryResponseItem>;

/// Provider-native `GetSymbolsDelistScheduleForSpotResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetSymbolsDelistScheduleForSpotResponseItem {
    /// Exact `delistTime` wire field.
    #[serde(
        rename = "delistTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delist_time: Option<i64>,
    /// Exact `symbols` wire field.
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `getSymbolsDelistScheduleForSpot`.
pub type GetSymbolsDelistScheduleForSpotResponse = Vec<GetSymbolsDelistScheduleForSpotResponseItem>;

/// Provider-native `SystemStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SystemStatusResponse {
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Exact `msg` wire field.
    #[serde(rename = "msg", default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `BrokerWithdrawResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BrokerWithdrawResponse {
    /// Exact `trId` wire field.
    #[serde(rename = "trId")]
    pub tr_id: i64,
    /// Exact `accepted` wire field.
    #[serde(rename = "accepted")]
    pub accepted: bool,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `CheckQuestionnaireRequirementsResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckQuestionnaireRequirementsResponse {
    /// Exact `questionnaireCountryCode` wire field.
    #[serde(
        rename = "questionnaireCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub questionnaire_country_code: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCountryListResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCountryListResponse {
    /// Exact `countries` wire field.
    #[serde(rename = "countries", default, skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<GetCountryListResponseCountriesItem>>,
    /// Exact `lastUpdated` wire field.
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetCountryListResponseCountriesItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetCountryListResponseCountriesItem {
    /// Exact `countryCode` wire field.
    #[serde(
        rename = "countryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_code: Option<String>,
    /// Exact `countryName` wire field.
    #[serde(
        rename = "countryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_name: Option<String>,
    /// Exact `blockType` wire field.
    #[serde(rename = "blockType", default, skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    /// Exact `depositAllowed` wire field.
    #[serde(
        rename = "depositAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_allowed: Option<bool>,
    /// Exact `withdrawalAllowed` wire field.
    #[serde(
        rename = "withdrawalAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_allowed: Option<bool>,
    /// Exact `hasRegionRestrictions` wire field.
    #[serde(
        rename = "hasRegionRestrictions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_region_restrictions: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DepositHistoryTravelRuleResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepositHistoryTravelRuleResponseItem {
    /// Exact `trId` wire field.
    #[serde(rename = "trId", default, skip_serializing_if = "Option::is_none")]
    pub tr_id: Option<i64>,
    /// Exact `tranId` wire field.
    #[serde(rename = "tranId", default, skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<i64>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `depositStatus` wire field.
    #[serde(
        rename = "depositStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_status: Option<i64>,
    /// Exact `travelRuleStatus` wire field.
    #[serde(
        rename = "travelRuleStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub travel_rule_status: Option<i64>,
    /// Exact `travelRuleStatusV2` wire field.
    #[serde(
        rename = "travelRuleStatusV2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub travel_rule_status_v2: Option<String>,
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `addressTag` wire field.
    #[serde(
        rename = "addressTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_tag: Option<crate::SensitiveString>,
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<crate::SensitiveString>,
    /// Exact `insertTime` wire field.
    #[serde(
        rename = "insertTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_time: Option<i64>,
    /// Exact `completeTime` wire field.
    #[serde(
        rename = "completeTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complete_time: Option<i64>,
    /// Exact `transferType` wire field.
    #[serde(
        rename = "transferType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_type: Option<i64>,
    /// Exact `confirmTimes` wire field.
    #[serde(
        rename = "confirmTimes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confirm_times: Option<String>,
    /// Exact `requireQuestionnaire` wire field.
    #[serde(
        rename = "requireQuestionnaire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_questionnaire: Option<bool>,
    /// Exact `questionnaire` wire field.
    #[serde(
        rename = "questionnaire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub questionnaire: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `depositHistoryTravelRule`.
pub type DepositHistoryTravelRuleResponse = Vec<DepositHistoryTravelRuleResponseItem>;

/// Provider-native `DepositHistoryV2ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepositHistoryV2ResponseItem {
    /// Exact `depositId` wire field.
    #[serde(rename = "depositId", default, skip_serializing_if = "Option::is_none")]
    pub deposit_id: Option<String>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `depositStatus` wire field.
    #[serde(
        rename = "depositStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_status: Option<i64>,
    /// Exact `travelRuleReqStatus` wire field.
    #[serde(
        rename = "travelRuleReqStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub travel_rule_req_status: Option<i64>,
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `addressTag` wire field.
    #[serde(
        rename = "addressTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_tag: Option<crate::SensitiveString>,
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<crate::SensitiveString>,
    /// Exact `transferType` wire field.
    #[serde(
        rename = "transferType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_type: Option<i64>,
    /// Exact `confirmTimes` wire field.
    #[serde(
        rename = "confirmTimes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confirm_times: Option<String>,
    /// Exact `requireQuestionnaire` wire field.
    #[serde(
        rename = "requireQuestionnaire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_questionnaire: Option<bool>,
    /// Exact `questionnaire` wire field.
    #[serde(
        rename = "questionnaire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub questionnaire: Option<DepositHistoryV2ResponseItemQuestionnaire>,
    /// Exact `insertTime` wire field.
    #[serde(
        rename = "insertTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `DepositHistoryV2ResponseItemQuestionnaire` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepositHistoryV2ResponseItemQuestionnaire {
    /// Exact `vaspName` wire field.
    #[serde(rename = "vaspName", default, skip_serializing_if = "Option::is_none")]
    pub vasp_name: Option<String>,
    /// Exact `depositOriginator` wire field.
    #[serde(
        rename = "depositOriginator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_originator: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `depositHistoryV2`.
pub type DepositHistoryV2Response = Vec<DepositHistoryV2ResponseItem>;

/// Provider-native `FetchAddressVerificationListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FetchAddressVerificationListResponseItem {
    /// Exact `status` wire field.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Exact `token` wire field.
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `walletAddress` wire field.
    #[serde(
        rename = "walletAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wallet_address: Option<crate::SensitiveString>,
    /// Exact `addressQuestionnaire` wire field.
    #[serde(
        rename = "addressQuestionnaire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub address_questionnaire: Option<FetchAddressVerificationListResponseItemAddressQuestionnaire>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `FetchAddressVerificationListResponseItemAddressQuestionnaire` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FetchAddressVerificationListResponseItemAddressQuestionnaire {
    /// Exact `sendTo` wire field.
    #[serde(rename = "sendTo", default, skip_serializing_if = "Option::is_none")]
    pub send_to: Option<i64>,
    /// Exact `satoshiToken` wire field.
    #[serde(
        rename = "satoshiToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub satoshi_token: Option<String>,
    /// Exact `isAddressOwner` wire field.
    #[serde(
        rename = "isAddressOwner",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_address_owner: Option<i64>,
    /// Exact `verifyMethod` wire field.
    #[serde(
        rename = "verifyMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verify_method: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `fetchAddressVerificationList`.
pub type FetchAddressVerificationListResponse = Vec<FetchAddressVerificationListResponseItem>;

/// Provider-native `GetRegionListResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetRegionListResponse {
    /// Exact `countryCode` wire field.
    #[serde(
        rename = "countryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_code: Option<String>,
    /// Exact `regions` wire field.
    #[serde(rename = "regions", default, skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<GetRegionListResponseRegionsItem>>,
    /// Exact `lastUpdated` wire field.
    #[serde(
        rename = "lastUpdated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_updated: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetRegionListResponseRegionsItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetRegionListResponseRegionsItem {
    /// Exact `regionName` wire field.
    #[serde(
        rename = "regionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub region_name: Option<String>,
    /// Exact `blockType` wire field.
    #[serde(rename = "blockType", default, skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    /// Exact `depositAllowed` wire field.
    #[serde(
        rename = "depositAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deposit_allowed: Option<bool>,
    /// Exact `withdrawalAllowed` wire field.
    #[serde(
        rename = "withdrawalAllowed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_allowed: Option<bool>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SubmitDepositQuestionnaireResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDepositQuestionnaireResponse {
    /// Exact `trId` wire field.
    #[serde(rename = "trId", default, skip_serializing_if = "Option::is_none")]
    pub tr_id: Option<i64>,
    /// Exact `accepted` wire field.
    #[serde(rename = "accepted", default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SubmitDepositQuestionnaireTravelRuleResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDepositQuestionnaireTravelRuleResponse {
    /// Exact `trId` wire field.
    #[serde(rename = "trId", default, skip_serializing_if = "Option::is_none")]
    pub tr_id: Option<i64>,
    /// Exact `accepted` wire field.
    #[serde(rename = "accepted", default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `SubmitDepositQuestionnaireV2Response` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitDepositQuestionnaireV2Response {
    /// Exact `trId` wire field.
    #[serde(rename = "trId", default, skip_serializing_if = "Option::is_none")]
    pub tr_id: Option<i64>,
    /// Exact `accepted` wire field.
    #[serde(rename = "accepted", default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `VaspListResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VaspListResponseItem {
    /// Exact `vaspName` wire field.
    #[serde(rename = "vaspName", default, skip_serializing_if = "Option::is_none")]
    pub vasp_name: Option<String>,
    /// Exact `vaspCode` wire field.
    #[serde(rename = "vaspCode", default, skip_serializing_if = "Option::is_none")]
    pub vasp_code: Option<String>,
    /// Exact `identifier` wire field.
    #[serde(
        rename = "identifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identifier: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `vaspList`.
pub type VaspListResponse = Vec<VaspListResponseItem>;

/// Provider-native `WithdrawHistoryV1ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WithdrawHistoryV1ResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Exact `trId` wire field.
    #[serde(rename = "trId", default, skip_serializing_if = "Option::is_none")]
    pub tr_id: Option<i64>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `transactionFee` wire field.
    #[serde(
        rename = "transactionFee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_fee: Option<Decimal>,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `withdrawalStatus` wire field.
    #[serde(
        rename = "withdrawalStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_status: Option<i64>,
    /// Exact `travelRuleStatus` wire field.
    #[serde(
        rename = "travelRuleStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub travel_rule_status: Option<i64>,
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<crate::SensitiveString>,
    /// Exact `applyTime` wire field.
    #[serde(rename = "applyTime", default, skip_serializing_if = "Option::is_none")]
    pub apply_time: Option<String>,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `transferType` wire field.
    #[serde(
        rename = "transferType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_type: Option<i64>,
    /// Exact `withdrawOrderId` wire field.
    #[serde(
        rename = "withdrawOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_order_id: Option<super::WithdrawalId>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Exact `confirmNo` wire field.
    #[serde(rename = "confirmNo", default, skip_serializing_if = "Option::is_none")]
    pub confirm_no: Option<i64>,
    /// Exact `walletType` wire field.
    #[serde(
        rename = "walletType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wallet_type: Option<i64>,
    /// Exact `txKey` wire field.
    #[serde(rename = "txKey", default, skip_serializing_if = "Option::is_none")]
    pub tx_key: Option<crate::SensitiveString>,
    /// Exact `questionnaire` wire field.
    #[serde(
        rename = "questionnaire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub questionnaire: Option<crate::SensitiveString>,
    /// Exact `completeTime` wire field.
    #[serde(
        rename = "completeTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complete_time: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `withdrawHistoryV1`.
pub type WithdrawHistoryV1Response = Vec<WithdrawHistoryV1ResponseItem>;

/// Provider-native `WithdrawHistoryV2ResponseItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WithdrawHistoryV2ResponseItem {
    /// Exact `id` wire field.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Exact `trId` wire field.
    #[serde(rename = "trId", default, skip_serializing_if = "Option::is_none")]
    pub tr_id: Option<i64>,
    /// Exact `amount` wire field.
    #[serde(rename = "amount", deserialize_with = "super::wire::decimal")]
    pub amount: Decimal,
    /// Exact `transactionFee` wire field.
    #[serde(
        rename = "transactionFee",
        default,
        deserialize_with = "super::wire::decimal_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_fee: Option<Decimal>,
    /// Exact `coin` wire field.
    #[serde(rename = "coin")]
    pub coin: crate::Asset,
    /// Exact `withdrawalStatus` wire field.
    #[serde(
        rename = "withdrawalStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_status: Option<i64>,
    /// Exact `travelRuleStatus` wire field.
    #[serde(
        rename = "travelRuleStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub travel_rule_status: Option<i64>,
    /// Exact `address` wire field.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::SensitiveString>,
    /// Exact `txId` wire field.
    #[serde(rename = "txId", default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<crate::SensitiveString>,
    /// Exact `applyTime` wire field.
    #[serde(rename = "applyTime", default, skip_serializing_if = "Option::is_none")]
    pub apply_time: Option<String>,
    /// Exact `network` wire field.
    #[serde(rename = "network", default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Exact `transferType` wire field.
    #[serde(
        rename = "transferType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_type: Option<i64>,
    /// Exact `withdrawOrderId` wire field.
    #[serde(
        rename = "withdrawOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_order_id: Option<super::WithdrawalId>,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Exact `confirmNo` wire field.
    #[serde(rename = "confirmNo", default, skip_serializing_if = "Option::is_none")]
    pub confirm_no: Option<i64>,
    /// Exact `walletType` wire field.
    #[serde(
        rename = "walletType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wallet_type: Option<i64>,
    /// Exact `txKey` wire field.
    #[serde(rename = "txKey", default, skip_serializing_if = "Option::is_none")]
    pub tx_key: Option<crate::SensitiveString>,
    /// Exact `questionnaire` wire field.
    #[serde(
        rename = "questionnaire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub questionnaire: Option<crate::SensitiveString>,
    /// Exact `completeTime` wire field.
    #[serde(
        rename = "completeTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complete_time: Option<String>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Exact response for `withdrawHistoryV2`.
pub type WithdrawHistoryV2Response = Vec<WithdrawHistoryV2ResponseItem>;

/// Provider-native `WithdrawTravelRuleResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WithdrawTravelRuleResponse {
    /// Exact `trId` wire field.
    #[serde(rename = "trId")]
    pub tr_id: i64,
    /// Exact `accepted` wire field.
    #[serde(rename = "accepted")]
    pub accepted: bool,
    /// Exact `info` wire field.
    #[serde(rename = "info", default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::SensitiveString>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
