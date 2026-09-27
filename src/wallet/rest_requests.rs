// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use crate::Decimal;
use crate::Error;
use crate::Symbol;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`accountApiTradingStatus`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-api-trading-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountApiTradingStatus {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountApiTradingStatus {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for AccountApiTradingStatus {
    type Response = super::rest_models::AccountApiTradingStatusResponse;
    const OP: Operation = Operation {
        name: "accountApiTradingStatus",
        path: "/sapi/v1/account/apiTradingStatus",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("accountApiTradingStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountInfo`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-info).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountInfo {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountInfo {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for AccountInfo {
    type Response = super::rest_models::AccountInfoResponse;
    const OP: Operation = Operation {
        name: "accountInfo",
        path: "/sapi/v1/account/info",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("accountInfo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountStatus`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountStatus {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountStatus {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for AccountStatus {
    type Response = super::rest_models::AccountStatusResponse;
    const OP: Operation = Operation {
        name: "accountStatus",
        path: "/sapi/v1/account/status",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("accountStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`dailyAccountSnapshot`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#daily-account-snapshot).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DailyAccountSnapshot {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl DailyAccountSnapshot {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DailyAccountSnapshot {
    type Response = super::rest_models::DailyAccountSnapshotResponse;
    const OP: Operation = Operation {
        name: "dailyAccountSnapshot",
        path: "/sapi/v1/accountSnapshot",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 2400,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["type"],
            &[("type", &["SPOT", "MARGIN", "FUTURES"])],
            &[
                ("limit", 7, 30),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("dailyAccountSnapshot", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`disableFastWithdrawSwitch`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#disable-fast-withdraw-switch).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DisableFastWithdrawSwitch {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl DisableFastWithdrawSwitch {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DisableFastWithdrawSwitch {
    type Response = super::rest_models::DisableFastWithdrawSwitchResponse;
    const OP: Operation = Operation {
        name: "disableFastWithdrawSwitch",
        path: "/sapi/v1/account/disableFastWithdrawSwitch",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("disableFastWithdrawSwitch", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`enableFastWithdrawSwitch`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#enable-fast-withdraw-switch).
#[derive(Clone, Debug, Default, Serialize)]
pub struct EnableFastWithdrawSwitch {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl EnableFastWithdrawSwitch {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for EnableFastWithdrawSwitch {
    type Response = super::rest_models::EnableFastWithdrawSwitchResponse;
    const OP: Operation = Operation {
        name: "enableFastWithdrawSwitch",
        path: "/sapi/v1/account/enableFastWithdrawSwitch",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("enableFastWithdrawSwitch", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getApiKeyPermission`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#get-api-key-permission).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetApiKeyPermission {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetApiKeyPermission {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetApiKeyPermission {
    type Response = super::rest_models::GetApiKeyPermissionResponse;
    const OP: Operation = Operation {
        name: "getApiKeyPermission",
        path: "/sapi/v1/account/apiRestrictions",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getApiKeyPermission", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`assetDetail`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-detail).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AssetDetail {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AssetDetail {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for AssetDetail {
    type Response = super::rest_models::AssetDetailResponse;
    const OP: Operation = Operation {
        name: "assetDetail",
        path: "/sapi/v1/asset/assetDetail",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("assetDetail", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`assetDividendRecord`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-dividend-record).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AssetDividendRecord {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AssetDividendRecord {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for AssetDividendRecord {
    type Response = super::rest_models::AssetDividendRecordResponse;
    const OP: Operation = Operation {
        name: "assetDividendRecord",
        path: "/sapi/v1/asset/assetDividend",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[
                ("limit", -9_223_372_036_854_775_808, 500),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("assetDividendRecord", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`dustConvert`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convert).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DustConvert {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    account_type: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<crate::RequestId>,
    #[serde(rename = "targetAsset", skip_serializing_if = "Option::is_none")]
    target_asset: Option<crate::Asset>,
    #[serde(rename = "thirdPartyClientId", skip_serializing_if = "Option::is_none")]
    third_party_client_id: Option<String>,
    #[serde(
        rename = "dustQuotaAssetToTargetAssetPrice",
        skip_serializing_if = "Option::is_none"
    )]
    dust_quota_asset_to_target_asset_price: Option<Decimal>,
}
impl DustConvert {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `accountType` parameter.
    #[must_use]
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }
    /// Set the provider `clientId` parameter.
    #[must_use]
    pub fn client_id(mut self, value: crate::RequestId) -> Self {
        self.client_id = Some(value);
        self
    }
    /// Set the provider `targetAsset` parameter.
    #[must_use]
    pub fn target_asset(mut self, value: crate::Asset) -> Self {
        self.target_asset = Some(value);
        self
    }
    /// Set the provider `thirdPartyClientId` parameter.
    #[must_use]
    pub fn third_party_client_id(mut self, value: impl Into<String>) -> Self {
        self.third_party_client_id = Some(value.into());
        self
    }
    /// Set the provider `dustQuotaAssetToTargetAssetPrice` parameter.
    #[must_use]
    pub fn dust_quota_asset_to_target_asset_price(mut self, value: Decimal) -> Self {
        self.dust_quota_asset_to_target_asset_price = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DustConvert {
    type Response = super::rest_models::DustConvertResponse;
    const OP: Operation = Operation {
        name: "dustConvert",
        path: "/sapi/v1/asset/dust-convert/convert",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["asset"], &[], &[])?;
        super::validation::validate("dustConvert", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`dustConvertibleAssets`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convertible-assets).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DustConvertibleAssets {
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    account_type: Option<String>,
    #[serde(rename = "targetAsset", skip_serializing_if = "Option::is_none")]
    target_asset: Option<crate::Asset>,
    #[serde(
        rename = "dustQuotaAssetToTargetAssetPrice",
        skip_serializing_if = "Option::is_none"
    )]
    dust_quota_asset_to_target_asset_price: Option<Decimal>,
}
impl DustConvertibleAssets {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `accountType` parameter.
    #[must_use]
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }
    /// Set the provider `targetAsset` parameter.
    #[must_use]
    pub fn target_asset(mut self, value: crate::Asset) -> Self {
        self.target_asset = Some(value);
        self
    }
    /// Set the provider `dustQuotaAssetToTargetAssetPrice` parameter.
    #[must_use]
    pub fn dust_quota_asset_to_target_asset_price(mut self, value: Decimal) -> Self {
        self.dust_quota_asset_to_target_asset_price = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DustConvertibleAssets {
    type Response = super::rest_models::DustConvertibleAssetsResponse;
    const OP: Operation = Operation {
        name: "dustConvertibleAssets",
        path: "/sapi/v1/asset/dust-convert/query-convertible-assets",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["targetAsset"], &[], &[])?;
        super::validation::validate("dustConvertibleAssets", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`dustlog`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dustlog).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Dustlog {
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    account_type: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl Dustlog {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `accountType` parameter.
    #[must_use]
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for Dustlog {
    type Response = super::rest_models::DustlogResponse;
    const OP: Operation = Operation {
        name: "dustlog",
        path: "/sapi/v1/asset/dribblet",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[("accountType", &["SPOT", "MARGIN"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("dustlog", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`dustTransfer`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-transfer).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DustTransfer {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<super::DustAssets>,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    account_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl DustTransfer {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: super::DustAssets) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `accountType` parameter.
    #[must_use]
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DustTransfer {
    type Response = super::rest_models::DustTransferResponse;
    const OP: Operation = Operation {
        name: "dustTransfer",
        path: "/sapi/v1/asset/dust",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["asset"],
            &[("accountType", &["SPOT", "MARGIN"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("dustTransfer", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fundingWallet`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#funding-wallet).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FundingWallet {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "needBtcValuation", skip_serializing_if = "Option::is_none")]
    need_btc_valuation: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FundingWallet {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `needBtcValuation` parameter.
    #[must_use]
    pub fn need_btc_valuation(mut self, value: bool) -> Self {
        self.need_btc_valuation = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for FundingWallet {
    type Response = super::rest_models::FundingWalletResponse;
    const OP: Operation = Operation {
        name: "fundingWallet",
        path: "/sapi/v1/asset/get-funding-asset",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("fundingWallet", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getSpotAssetTags`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-spot-asset-tags).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSpotAssetTags {
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    tag: Option<crate::SensitiveString>,
}
impl GetSpotAssetTags {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `tag` parameter.
    #[must_use]
    pub fn tag(mut self, value: crate::SensitiveString) -> Self {
        self.tag = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetSpotAssetTags {
    type Response = super::rest_models::GetSpotAssetTagsResponse;
    const OP: Operation = Operation {
        name: "getSpotAssetTags",
        path: "/sapi/v1/spot/asset/tags",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 100,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getSpotAssetTags", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getAssetsThatCanBeConvertedIntoBnb`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-assets-that-can-be-converted-into-bnb).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAssetsThatCanBeConvertedIntoBnb {
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    account_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetAssetsThatCanBeConvertedIntoBnb {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `accountType` parameter.
    #[must_use]
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetAssetsThatCanBeConvertedIntoBnb {
    type Response = super::rest_models::GetAssetsThatCanBeConvertedIntoBnbResponse;
    const OP: Operation = Operation {
        name: "getAssetsThatCanBeConvertedIntoBnb",
        path: "/sapi/v1/asset/dust-btc",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[("accountType", &["SPOT", "MARGIN"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getAssetsThatCanBeConvertedIntoBnb", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCloudMiningPaymentAndRefundHistory`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCloudMiningPaymentAndRefundHistory {
    #[serde(rename = "tranId", skip_serializing_if = "Option::is_none")]
    tran_id: Option<i64>,
    #[serde(rename = "clientTranId", skip_serializing_if = "Option::is_none")]
    client_tran_id: Option<String>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
}
impl GetCloudMiningPaymentAndRefundHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `tranId` parameter.
    #[must_use]
    pub fn tran_id(mut self, value: i64) -> Self {
        self.tran_id = Some(value);
        self
    }
    /// Set the provider `clientTranId` parameter.
    #[must_use]
    pub fn client_tran_id(mut self, value: impl Into<String>) -> Self {
        self.client_tran_id = Some(value.into());
        self
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `current` parameter.
    #[must_use]
    pub fn current(mut self, value: i64) -> Self {
        self.current = Some(value);
        self
    }
    /// Set the provider `size` parameter.
    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetCloudMiningPaymentAndRefundHistory {
    type Response = super::rest_models::GetCloudMiningPaymentAndRefundHistoryResponse;
    const OP: Operation = Operation {
        name: "getCloudMiningPaymentAndRefundHistory",
        path: "/sapi/v1/asset/ledger-transfer/cloud-mining/queryByPage",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 600,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["endTime", "startTime"],
            &[],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
            ],
        )?;
        super::validation::validate("getCloudMiningPaymentAndRefundHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getOpenSymbolList`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-open-symbol-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetOpenSymbolList {}
impl GetOpenSymbolList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetOpenSymbolList {
    type Response = super::rest_models::GetOpenSymbolListResponse;
    const OP: Operation = Operation {
        name: "getOpenSymbolList",
        path: "/sapi/v1/spot/open-symbol-list",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 100,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getOpenSymbolList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUserDelegationHistory`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-delegation-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUserDelegationHistory {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    email: Option<crate::SensitiveString>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUserDelegationHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `email` parameter.
    #[must_use]
    pub fn email(mut self, value: crate::SensitiveString) -> Self {
        self.email = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `current` parameter.
    #[must_use]
    pub fn current(mut self, value: i64) -> Self {
        self.current = Some(value);
        self
    }
    /// Set the provider `size` parameter.
    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for QueryUserDelegationHistory {
    type Response = super::rest_models::QueryUserDelegationHistoryResponse;
    const OP: Operation = Operation {
        name: "queryUserDelegationHistory",
        path: "/sapi/v1/asset/custody/transfer-history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 60,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["email", "endTime", "startTime"],
            &[("type", &["DELEGATE", "UNDELEGATE"])],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryUserDelegationHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUserUniversalTransferHistory`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-universal-transfer-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUserUniversalTransferHistory {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "fromSymbol", skip_serializing_if = "Option::is_none")]
    from_symbol: Option<Symbol>,
    #[serde(rename = "toSymbol", skip_serializing_if = "Option::is_none")]
    to_symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUserUniversalTransferHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `current` parameter.
    #[must_use]
    pub fn current(mut self, value: i64) -> Self {
        self.current = Some(value);
        self
    }
    /// Set the provider `size` parameter.
    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
    /// Set the provider `fromSymbol` parameter.
    #[must_use]
    pub fn from_symbol(mut self, value: Symbol) -> Self {
        self.from_symbol = Some(value);
        self
    }
    /// Set the provider `toSymbol` parameter.
    #[must_use]
    pub fn to_symbol(mut self, value: Symbol) -> Self {
        self.to_symbol = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for QueryUserUniversalTransferHistory {
    type Response = super::rest_models::QueryUserUniversalTransferHistoryResponse;
    const OP: Operation = Operation {
        name: "queryUserUniversalTransferHistory",
        path: "/sapi/v1/asset/transfer",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["type"],
            &[],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryUserUniversalTransferHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userUniversalTransfer`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-universal-transfer).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserUniversalTransfer {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "fromSymbol", skip_serializing_if = "Option::is_none")]
    from_symbol: Option<Symbol>,
    #[serde(rename = "toSymbol", skip_serializing_if = "Option::is_none")]
    to_symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UserUniversalTransfer {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `fromSymbol` parameter.
    #[must_use]
    pub fn from_symbol(mut self, value: Symbol) -> Self {
        self.from_symbol = Some(value);
        self
    }
    /// Set the provider `toSymbol` parameter.
    #[must_use]
    pub fn to_symbol(mut self, value: Symbol) -> Self {
        self.to_symbol = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for UserUniversalTransfer {
    type Response = super::rest_models::UserUniversalTransferResponse;
    const OP: Operation = Operation {
        name: "userUniversalTransfer",
        path: "/sapi/v1/asset/transfer",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 300,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["amount", "asset", "type"],
            &[(
                "type",
                &[
                    "MAIN_UMFUTURE",
                    "MAIN_CMFUTURE",
                    "MAIN_MARGIN",
                    "UMFUTURE_MAIN",
                    "UMFUTURE_MARGIN",
                    "CMFUTURE_MAIN",
                    "CMFUTURE_MARGIN",
                    "MARGIN_MAIN",
                    "MARGIN_UMFUTURE",
                    "MARGIN_CMFUTURE",
                    "ISOLATEDMARGIN_MARGIN",
                    "MARGIN_ISOLATEDMARGIN",
                    "ISOLATEDMARGIN_ISOLATEDMARGIN",
                    "MAIN_FUNDING",
                    "FUNDING_MAIN",
                    "FUNDING_UMFUTURE",
                    "UMFUTURE_FUNDING",
                    "MARGIN_FUNDING",
                    "FUNDING_MARGIN",
                    "FUNDING_CMFUTURE",
                    "CMFUTURE_FUNDING",
                    "MAIN_OPTION",
                    "OPTION_MAIN",
                    "UMFUTURE_OPTION",
                    "OPTION_UMFUTURE",
                    "MARGIN_OPTION",
                    "OPTION_MARGIN",
                    "FUNDING_OPTION",
                    "OPTION_FUNDING",
                    "MAIN_PORTFOLIO_MARGIN",
                    "PORTFOLIO_MARGIN_MAIN",
                ],
            )],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("userUniversalTransfer", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUserWalletBalance`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-wallet-balance).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUserWalletBalance {
    #[serde(rename = "quoteAsset", skip_serializing_if = "Option::is_none")]
    quote_asset: Option<crate::Asset>,
    #[serde(rename = "needBalanceDetail", skip_serializing_if = "Option::is_none")]
    need_balance_detail: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUserWalletBalance {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `quoteAsset` parameter.
    #[must_use]
    pub fn quote_asset(mut self, value: crate::Asset) -> Self {
        self.quote_asset = Some(value);
        self
    }
    /// Set the provider `needBalanceDetail` parameter.
    #[must_use]
    pub fn need_balance_detail(mut self, value: bool) -> Self {
        self.need_balance_detail = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for QueryUserWalletBalance {
    type Response = super::rest_models::QueryUserWalletBalanceResponse;
    const OP: Operation = Operation {
        name: "queryUserWalletBalance",
        path: "/sapi/v1/asset/wallet/balance",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 60,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryUserWalletBalance", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`toggleBnbBurnOnSpotTradeAndMarginInterest`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#toggle-bnb-burn-on-spot-trade-and-margin-interest).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterest {
    #[serde(rename = "spotBNBBurn", skip_serializing_if = "Option::is_none")]
    spot_bnb_burn: Option<String>,
    #[serde(rename = "interestBNBBurn", skip_serializing_if = "Option::is_none")]
    interest_bnb_burn: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ToggleBnbBurnOnSpotTradeAndMarginInterest {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `spotBNBBurn` parameter.
    #[must_use]
    pub fn spot_bnb_burn(mut self, value: impl Into<String>) -> Self {
        self.spot_bnb_burn = Some(value.into());
        self
    }
    /// Set the provider `interestBNBBurn` parameter.
    #[must_use]
    pub fn interest_bnb_burn(mut self, value: impl Into<String>) -> Self {
        self.interest_bnb_burn = Some(value.into());
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for ToggleBnbBurnOnSpotTradeAndMarginInterest {
    type Response = super::rest_models::ToggleBnbBurnOnSpotTradeAndMarginInterestResponse;
    const OP: Operation = Operation {
        name: "toggleBnbBurnOnSpotTradeAndMarginInterest",
        path: "/sapi/v1/bnbBurn",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("toggleBnbBurnOnSpotTradeAndMarginInterest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tradeFee`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#trade-fee).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TradeFee {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl TradeFee {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for TradeFee {
    type Response = super::rest_models::TradeFeeResponse;
    const OP: Operation = Operation {
        name: "tradeFee",
        path: "/sapi/v1/asset/tradeFee",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("tradeFee", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userAsset`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-asset).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserAsset {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "needBtcValuation", skip_serializing_if = "Option::is_none")]
    need_btc_valuation: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UserAsset {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: crate::Asset) -> Self {
        self.asset = Some(value);
        self
    }
    /// Set the provider `needBtcValuation` parameter.
    #[must_use]
    pub fn need_btc_valuation(mut self, value: bool) -> Self {
        self.need_btc_valuation = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for UserAsset {
    type Response = super::rest_models::UserAssetResponse;
    const OP: Operation = Operation {
        name: "userAsset",
        path: "/sapi/v3/asset/getUserAsset",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("userAsset", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`allCoinsInformation`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#all-coins-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AllCoinsInformation {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AllCoinsInformation {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for AllCoinsInformation {
    type Response = super::rest_models::AllCoinsInformationResponse;
    const OP: Operation = Operation {
        name: "allCoinsInformation",
        path: "/sapi/v1/capital/config/getall",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("allCoinsInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`depositAddress`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-address).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DepositAddress {
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl DepositAddress {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DepositAddress {
    type Response = super::rest_models::DepositAddressResponse;
    const OP: Operation = Operation {
        name: "depositAddress",
        path: "/sapi/v1/capital/deposit/address",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["coin"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("depositAddress", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`depositHistory`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DepositHistory {
    #[serde(rename = "includeSource", skip_serializing_if = "Option::is_none")]
    include_source: Option<bool>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<crate::SensitiveString>,
}
impl DepositHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `includeSource` parameter.
    #[must_use]
    pub fn include_source(mut self, value: bool) -> Self {
        self.include_source = Some(value);
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `status` parameter.
    #[must_use]
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `offset` parameter.
    #[must_use]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: crate::SensitiveString) -> Self {
        self.tx_id = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DepositHistory {
    type Response = super::rest_models::DepositHistoryResponse;
    const OP: Operation = Operation {
        name: "depositHistory",
        path: "/sapi/v1/capital/deposit/hisrec",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[("status", &["0", "1", "2", "6", "7", "8"])],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("depositHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fetchDepositAddressListWithNetwork`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-deposit-address-list-with-network).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FetchDepositAddressListWithNetwork {
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
}
impl FetchDepositAddressListWithNetwork {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for FetchDepositAddressListWithNetwork {
    type Response = super::rest_models::FetchDepositAddressListWithNetworkResponse;
    const OP: Operation = Operation {
        name: "fetchDepositAddressListWithNetwork",
        path: "/sapi/v1/capital/deposit/address/list",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["coin"], &[], &[])?;
        super::validation::validate("fetchDepositAddressListWithNetwork", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fetchWithdrawAddressList`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-address-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FetchWithdrawAddressList {}
impl FetchWithdrawAddressList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for FetchWithdrawAddressList {
    type Response = super::rest_models::FetchWithdrawAddressListResponse;
    const OP: Operation = Operation {
        name: "fetchWithdrawAddressList",
        path: "/sapi/v1/capital/withdraw/address/list",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("fetchWithdrawAddressList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fetchWithdrawQuota`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-quota).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FetchWithdrawQuota {}
impl FetchWithdrawQuota {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for FetchWithdrawQuota {
    type Response = super::rest_models::FetchWithdrawQuotaResponse;
    const OP: Operation = Operation {
        name: "fetchWithdrawQuota",
        path: "/sapi/v1/capital/withdraw/quota",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("fetchWithdrawQuota", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`oneClickArrivalDepositApply`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#one-click-arrival-deposit-apply).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OneClickArrivalDepositApply {
    #[serde(rename = "depositId", skip_serializing_if = "Option::is_none")]
    deposit: Option<i64>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    transaction: Option<crate::SensitiveString>,
    #[serde(rename = "subAccountId", skip_serializing_if = "Option::is_none")]
    sub_account: Option<String>,
    #[serde(rename = "subUserId", skip_serializing_if = "Option::is_none")]
    sub_user: Option<i64>,
}
impl OneClickArrivalDepositApply {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `depositId` parameter.
    #[must_use]
    pub fn deposit_id(mut self, value: i64) -> Self {
        self.deposit = Some(value);
        self
    }
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: crate::SensitiveString) -> Self {
        self.transaction = Some(value);
        self
    }
    /// Set the provider `subAccountId` parameter.
    #[must_use]
    pub fn sub_account_id(mut self, value: impl Into<String>) -> Self {
        self.sub_account = Some(value.into());
        self
    }
    /// Set the provider `subUserId` parameter.
    #[must_use]
    pub fn sub_user_id(mut self, value: i64) -> Self {
        self.sub_user = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for OneClickArrivalDepositApply {
    type Response = super::rest_models::OneClickArrivalDepositApplyResponse;
    const OP: Operation = Operation {
        name: "oneClickArrivalDepositApply",
        path: "/sapi/v1/capital/deposit/credit-apply",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("oneClickArrivalDepositApply", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`withdraw`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Withdraw {
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "withdrawOrderId", skip_serializing_if = "Option::is_none")]
    caller_id: Option<super::WithdrawalId>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    address: Option<crate::SensitiveString>,
    #[serde(rename = "addressTag", skip_serializing_if = "Option::is_none")]
    address_tag: Option<crate::SensitiveString>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "transactionFeeFlag", skip_serializing_if = "Option::is_none")]
    transaction_fee_flag: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "walletType", skip_serializing_if = "Option::is_none")]
    wallet_type: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl Withdraw {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `withdrawOrderId` parameter.
    #[must_use]
    pub fn withdraw_order_id(mut self, value: super::WithdrawalId) -> Self {
        self.caller_id = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `address` parameter.
    #[must_use]
    pub fn address(mut self, value: crate::SensitiveString) -> Self {
        self.address = Some(value);
        self
    }
    /// Set the provider `addressTag` parameter.
    #[must_use]
    pub fn address_tag(mut self, value: crate::SensitiveString) -> Self {
        self.address_tag = Some(value);
        self
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `transactionFeeFlag` parameter.
    #[must_use]
    pub fn transaction_fee_flag(mut self, value: bool) -> Self {
        self.transaction_fee_flag = Some(value);
        self
    }
    /// Set the provider `name` parameter.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Set the provider `walletType` parameter.
    #[must_use]
    pub fn wallet_type(mut self, value: i64) -> Self {
        self.wallet_type = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for Withdraw {
    type Response = super::rest_models::WithdrawResponse;
    const OP: Operation = Operation {
        name: "withdraw",
        path: "/sapi/v1/capital/withdraw/apply",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 900,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["address", "amount", "coin"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("withdraw", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`withdrawHistory`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct WithdrawHistory {
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "withdrawOrderId", skip_serializing_if = "Option::is_none")]
    withdraw_order_id: Option<super::WithdrawalId>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    status: Option<i64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "idList", skip_serializing_if = "Option::is_none")]
    id_list: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl WithdrawHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `withdrawOrderId` parameter.
    #[must_use]
    pub fn withdraw_order_id(mut self, value: super::WithdrawalId) -> Self {
        self.withdraw_order_id = Some(value);
        self
    }
    /// Set the provider `status` parameter.
    #[must_use]
    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }
    /// Set the provider `offset` parameter.
    #[must_use]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `idList` parameter.
    #[must_use]
    pub fn id_list(mut self, value: impl Into<String>) -> Self {
        self.id_list = Some(value.into());
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for WithdrawHistory {
    type Response = super::rest_models::WithdrawHistoryResponse;
    const OP: Operation = Operation {
        name: "withdrawHistory",
        path: "/sapi/v1/capital/withdraw/history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 18000,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("withdrawHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getSymbolsDelistScheduleForSpot`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#get-symbols-delist-schedule-for-spot).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSymbolsDelistScheduleForSpot {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetSymbolsDelistScheduleForSpot {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetSymbolsDelistScheduleForSpot {
    type Response = super::rest_models::GetSymbolsDelistScheduleForSpotResponse;
    const OP: Operation = Operation {
        name: "getSymbolsDelistScheduleForSpot",
        path: "/sapi/v1/spot/delist-schedule",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 100,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getSymbolsDelistScheduleForSpot", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`systemStatus`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#system-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SystemStatus {}
impl SystemStatus {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for SystemStatus {
    type Response = super::rest_models::SystemStatusResponse;
    const OP: Operation = Operation {
        name: "systemStatus",
        path: "/sapi/v1/system/status",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("systemStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`brokerWithdraw`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#broker-withdraw).
#[derive(Clone, Debug, Default, Serialize)]
pub struct BrokerWithdraw {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    address: Option<crate::SensitiveString>,
    #[serde(rename = "addressTag", skip_serializing_if = "Option::is_none")]
    address_tag: Option<crate::SensitiveString>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "addressName", skip_serializing_if = "Option::is_none")]
    address_name: Option<crate::SensitiveString>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "withdrawOrderId", skip_serializing_if = "Option::is_none")]
    withdraw_order_id: Option<super::WithdrawalId>,
    #[serde(rename = "transactionFeeFlag", skip_serializing_if = "Option::is_none")]
    transaction_fee_flag: Option<bool>,
    #[serde(rename = "walletType", skip_serializing_if = "Option::is_none")]
    wallet_type: Option<i64>,
    #[serde(rename = "questionnaire", skip_serializing_if = "Option::is_none")]
    questionnaire: Option<crate::SensitiveString>,
    #[serde(rename = "originatorPii", skip_serializing_if = "Option::is_none")]
    originator_pii: Option<crate::SensitiveString>,
}
impl BrokerWithdraw {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `address` parameter.
    #[must_use]
    pub fn address(mut self, value: crate::SensitiveString) -> Self {
        self.address = Some(value);
        self
    }
    /// Set the provider `addressTag` parameter.
    #[must_use]
    pub fn address_tag(mut self, value: crate::SensitiveString) -> Self {
        self.address_tag = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `addressName` parameter.
    #[must_use]
    pub fn address_name(mut self, value: crate::SensitiveString) -> Self {
        self.address_name = Some(value);
        self
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `withdrawOrderId` parameter.
    #[must_use]
    pub fn withdraw_order_id(mut self, value: super::WithdrawalId) -> Self {
        self.withdraw_order_id = Some(value);
        self
    }
    /// Set the provider `transactionFeeFlag` parameter.
    #[must_use]
    pub fn transaction_fee_flag(mut self, value: bool) -> Self {
        self.transaction_fee_flag = Some(value);
        self
    }
    /// Set the provider `walletType` parameter.
    #[must_use]
    pub fn wallet_type(mut self, value: i64) -> Self {
        self.wallet_type = Some(value);
        self
    }
    /// Set the provider `questionnaire` parameter.
    #[must_use]
    pub fn questionnaire(mut self, value: crate::SensitiveString) -> Self {
        self.questionnaire = Some(value);
        self
    }
    /// Set the provider `originatorPii` parameter.
    #[must_use]
    pub fn originator_pii(mut self, value: crate::SensitiveString) -> Self {
        self.originator_pii = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for BrokerWithdraw {
    type Response = super::rest_models::BrokerWithdrawResponse;
    const OP: Operation = Operation {
        name: "brokerWithdraw",
        path: "/sapi/v1/localentity/broker/withdraw/apply",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 600,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[
                "address",
                "amount",
                "coin",
                "originatorPii",
                "questionnaire",
                "withdrawOrderId",
            ],
            &[],
            &[],
        )?;
        super::validation::validate("brokerWithdraw", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`checkQuestionnaireRequirements`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#check-questionnaire-requirements).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CheckQuestionnaireRequirements {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CheckQuestionnaireRequirements {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for CheckQuestionnaireRequirements {
    type Response = super::rest_models::CheckQuestionnaireRequirementsResponse;
    const OP: Operation = Operation {
        name: "checkQuestionnaireRequirements",
        path: "/sapi/v1/localentity/questionnaire-requirements",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("checkQuestionnaireRequirements", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCountryList`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-country-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCountryList {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetCountryList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetCountryList {
    type Response = super::rest_models::GetCountryListResponse;
    const OP: Operation = Operation {
        name: "getCountryList",
        path: "/sapi/v1/localentity/country/list",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getCountryList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`depositHistoryTravelRule`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DepositHistoryTravelRule {
    #[serde(rename = "trId", skip_serializing_if = "Option::is_none")]
    tr_id: Option<String>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<crate::SensitiveString>,
    #[serde(rename = "tranId", skip_serializing_if = "Option::is_none")]
    tran_id: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "travelRuleStatus", skip_serializing_if = "Option::is_none")]
    travel_rule_status: Option<i64>,
    #[serde(
        rename = "pendingQuestionnaire",
        skip_serializing_if = "Option::is_none"
    )]
    pending_questionnaire: Option<bool>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl DepositHistoryTravelRule {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `trId` parameter.
    #[must_use]
    pub fn tr_id(mut self, value: impl Into<String>) -> Self {
        self.tr_id = Some(value.into());
        self
    }
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: crate::SensitiveString) -> Self {
        self.tx_id = Some(value);
        self
    }
    /// Set the provider `tranId` parameter.
    #[must_use]
    pub fn tran_id(mut self, value: impl Into<String>) -> Self {
        self.tran_id = Some(value.into());
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `travelRuleStatus` parameter.
    #[must_use]
    pub fn travel_rule_status(mut self, value: i64) -> Self {
        self.travel_rule_status = Some(value);
        self
    }
    /// Set the provider `pendingQuestionnaire` parameter.
    #[must_use]
    pub fn pending_questionnaire(mut self, value: bool) -> Self {
        self.pending_questionnaire = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `offset` parameter.
    #[must_use]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DepositHistoryTravelRule {
    type Response = super::rest_models::DepositHistoryTravelRuleResponse;
    const OP: Operation = Operation {
        name: "depositHistoryTravelRule",
        path: "/sapi/v1/localentity/deposit/history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("depositHistoryTravelRule", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`depositHistoryV2`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DepositHistoryV2 {
    #[serde(rename = "depositId", skip_serializing_if = "Option::is_none")]
    deposit_id: Option<i64>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<crate::SensitiveString>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(
        rename = "retrieveQuestionnaire",
        skip_serializing_if = "Option::is_none"
    )]
    retrieve_questionnaire: Option<bool>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl DepositHistoryV2 {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `depositId` parameter.
    #[must_use]
    pub fn deposit_id(mut self, value: i64) -> Self {
        self.deposit_id = Some(value);
        self
    }
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: crate::SensitiveString) -> Self {
        self.tx_id = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `retrieveQuestionnaire` parameter.
    #[must_use]
    pub fn retrieve_questionnaire(mut self, value: bool) -> Self {
        self.retrieve_questionnaire = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `offset` parameter.
    #[must_use]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for DepositHistoryV2 {
    type Response = super::rest_models::DepositHistoryV2Response;
    const OP: Operation = Operation {
        name: "depositHistoryV2",
        path: "/sapi/v2/localentity/deposit/history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("depositHistoryV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fetchAddressVerificationList`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#fetch-address-verification-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FetchAddressVerificationList {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FetchAddressVerificationList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for FetchAddressVerificationList {
    type Response = super::rest_models::FetchAddressVerificationListResponse;
    const OP: Operation = Operation {
        name: "fetchAddressVerificationList",
        path: "/sapi/v1/addressVerify/list",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("fetchAddressVerificationList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getRegionList`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-region-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetRegionList {
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    country_code: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetRegionList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `countryCode` parameter.
    #[must_use]
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for GetRegionList {
    type Response = super::rest_models::GetRegionListResponse;
    const OP: Operation = Operation {
        name: "getRegionList",
        path: "/sapi/v1/localentity/region/list",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["countryCode"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getRegionList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`submitDepositQuestionnaire`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SubmitDepositQuestionnaire {
    #[serde(rename = "subAccountId", skip_serializing_if = "Option::is_none")]
    sub_account_id: Option<String>,
    #[serde(rename = "depositId", skip_serializing_if = "Option::is_none")]
    deposit_id: Option<i64>,
    #[serde(rename = "questionnaire", skip_serializing_if = "Option::is_none")]
    questionnaire: Option<crate::SensitiveString>,
    #[serde(rename = "beneficiaryPii", skip_serializing_if = "Option::is_none")]
    beneficiary_pii: Option<crate::SensitiveString>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    address: Option<crate::SensitiveString>,
    #[serde(rename = "addressTag", skip_serializing_if = "Option::is_none")]
    address_tag: Option<crate::SensitiveString>,
}
impl SubmitDepositQuestionnaire {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `subAccountId` parameter.
    #[must_use]
    pub fn sub_account_id(mut self, value: impl Into<String>) -> Self {
        self.sub_account_id = Some(value.into());
        self
    }
    /// Set the provider `depositId` parameter.
    #[must_use]
    pub fn deposit_id(mut self, value: i64) -> Self {
        self.deposit_id = Some(value);
        self
    }
    /// Set the provider `questionnaire` parameter.
    #[must_use]
    pub fn questionnaire(mut self, value: crate::SensitiveString) -> Self {
        self.questionnaire = Some(value);
        self
    }
    /// Set the provider `beneficiaryPii` parameter.
    #[must_use]
    pub fn beneficiary_pii(mut self, value: crate::SensitiveString) -> Self {
        self.beneficiary_pii = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `address` parameter.
    #[must_use]
    pub fn address(mut self, value: crate::SensitiveString) -> Self {
        self.address = Some(value);
        self
    }
    /// Set the provider `addressTag` parameter.
    #[must_use]
    pub fn address_tag(mut self, value: crate::SensitiveString) -> Self {
        self.address_tag = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for SubmitDepositQuestionnaire {
    type Response = super::rest_models::SubmitDepositQuestionnaireResponse;
    const OP: Operation = Operation {
        name: "submitDepositQuestionnaire",
        path: "/sapi/v1/localentity/broker/deposit/provide-info",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 600,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[
                "beneficiaryPii",
                "depositId",
                "questionnaire",
                "subAccountId",
            ],
            &[],
            &[],
        )?;
        super::validation::validate("submitDepositQuestionnaire", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`submitDepositQuestionnaireTravelRule`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-travel-rule).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SubmitDepositQuestionnaireTravelRule {
    #[serde(rename = "tranId", skip_serializing_if = "Option::is_none")]
    tran_id: Option<i64>,
    #[serde(rename = "questionnaire", skip_serializing_if = "Option::is_none")]
    questionnaire: Option<crate::SensitiveString>,
}
impl SubmitDepositQuestionnaireTravelRule {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `tranId` parameter.
    #[must_use]
    pub fn tran_id(mut self, value: i64) -> Self {
        self.tran_id = Some(value);
        self
    }
    /// Set the provider `questionnaire` parameter.
    #[must_use]
    pub fn questionnaire(mut self, value: crate::SensitiveString) -> Self {
        self.questionnaire = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for SubmitDepositQuestionnaireTravelRule {
    type Response = super::rest_models::SubmitDepositQuestionnaireTravelRuleResponse;
    const OP: Operation = Operation {
        name: "submitDepositQuestionnaireTravelRule",
        path: "/sapi/v1/localentity/deposit/provide-info",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 600,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["questionnaire", "tranId"], &[], &[])?;
        super::validation::validate("submitDepositQuestionnaireTravelRule", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`submitDepositQuestionnaireV2`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SubmitDepositQuestionnaireV2 {
    #[serde(rename = "depositId", skip_serializing_if = "Option::is_none")]
    deposit_id: Option<i64>,
    #[serde(rename = "questionnaire", skip_serializing_if = "Option::is_none")]
    questionnaire: Option<crate::SensitiveString>,
}
impl SubmitDepositQuestionnaireV2 {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `depositId` parameter.
    #[must_use]
    pub fn deposit_id(mut self, value: i64) -> Self {
        self.deposit_id = Some(value);
        self
    }
    /// Set the provider `questionnaire` parameter.
    #[must_use]
    pub fn questionnaire(mut self, value: crate::SensitiveString) -> Self {
        self.questionnaire = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for SubmitDepositQuestionnaireV2 {
    type Response = super::rest_models::SubmitDepositQuestionnaireV2Response;
    const OP: Operation = Operation {
        name: "submitDepositQuestionnaireV2",
        path: "/sapi/v2/localentity/deposit/provide-info",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 600,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["depositId", "questionnaire"], &[], &[])?;
        super::validation::validate("submitDepositQuestionnaireV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`vaspList`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#vasp-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct VaspList {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl VaspList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for VaspList {
    type Response = super::rest_models::VaspListResponse;
    const OP: Operation = Operation {
        name: "vaspList",
        path: "/sapi/v1/localentity/vasp",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("vaspList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`withdrawHistoryV1`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v1).
#[derive(Clone, Debug, Default, Serialize)]
pub struct WithdrawHistoryV1 {
    #[serde(rename = "trId", skip_serializing_if = "Option::is_none")]
    tr_id: Option<String>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<crate::SensitiveString>,
    #[serde(rename = "withdrawOrderId", skip_serializing_if = "Option::is_none")]
    withdraw_order_id: Option<super::WithdrawalId>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "travelRuleStatus", skip_serializing_if = "Option::is_none")]
    travel_rule_status: Option<i64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl WithdrawHistoryV1 {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `trId` parameter.
    #[must_use]
    pub fn tr_id(mut self, value: impl Into<String>) -> Self {
        self.tr_id = Some(value.into());
        self
    }
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: crate::SensitiveString) -> Self {
        self.tx_id = Some(value);
        self
    }
    /// Set the provider `withdrawOrderId` parameter.
    #[must_use]
    pub fn withdraw_order_id(mut self, value: super::WithdrawalId) -> Self {
        self.withdraw_order_id = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `travelRuleStatus` parameter.
    #[must_use]
    pub fn travel_rule_status(mut self, value: i64) -> Self {
        self.travel_rule_status = Some(value);
        self
    }
    /// Set the provider `offset` parameter.
    #[must_use]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for WithdrawHistoryV1 {
    type Response = super::rest_models::WithdrawHistoryV1Response;
    const OP: Operation = Operation {
        name: "withdrawHistoryV1",
        path: "/sapi/v1/localentity/withdraw/history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("withdrawHistoryV1", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`withdrawHistoryV2`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct WithdrawHistoryV2 {
    #[serde(rename = "trId", skip_serializing_if = "Option::is_none")]
    tr_id: Option<String>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<crate::SensitiveString>,
    #[serde(rename = "withdrawOrderId", skip_serializing_if = "Option::is_none")]
    withdraw_order_id: Option<super::WithdrawalId>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "travelRuleStatus", skip_serializing_if = "Option::is_none")]
    travel_rule_status: Option<i64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl WithdrawHistoryV2 {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `trId` parameter.
    #[must_use]
    pub fn tr_id(mut self, value: impl Into<String>) -> Self {
        self.tr_id = Some(value.into());
        self
    }
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: crate::SensitiveString) -> Self {
        self.tx_id = Some(value);
        self
    }
    /// Set the provider `withdrawOrderId` parameter.
    #[must_use]
    pub fn withdraw_order_id(mut self, value: super::WithdrawalId) -> Self {
        self.withdraw_order_id = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `travelRuleStatus` parameter.
    #[must_use]
    pub fn travel_rule_status(mut self, value: i64) -> Self {
        self.travel_rule_status = Some(value);
        self
    }
    /// Set the provider `offset` parameter.
    #[must_use]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for WithdrawHistoryV2 {
    type Response = super::rest_models::WithdrawHistoryV2Response;
    const OP: Operation = Operation {
        name: "withdrawHistoryV2",
        path: "/sapi/v2/localentity/withdraw/history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("withdrawHistoryV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`withdrawTravelRule`](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-travel-rule).
#[derive(Clone, Debug, Default, Serialize)]
pub struct WithdrawTravelRule {
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "withdrawOrderId", skip_serializing_if = "Option::is_none")]
    withdraw_order_id: Option<super::WithdrawalId>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    network: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    address: Option<crate::SensitiveString>,
    #[serde(rename = "addressTag", skip_serializing_if = "Option::is_none")]
    address_tag: Option<crate::SensitiveString>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "transactionFeeFlag", skip_serializing_if = "Option::is_none")]
    transaction_fee_flag: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "walletType", skip_serializing_if = "Option::is_none")]
    wallet_type: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    #[serde(rename = "questionnaire", skip_serializing_if = "Option::is_none")]
    questionnaire: Option<crate::SensitiveString>,
}
impl WithdrawTravelRule {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
        self
    }
    /// Set the provider `withdrawOrderId` parameter.
    #[must_use]
    pub fn withdraw_order_id(mut self, value: super::WithdrawalId) -> Self {
        self.withdraw_order_id = Some(value);
        self
    }
    /// Set the provider `network` parameter.
    #[must_use]
    pub fn network(mut self, value: impl Into<String>) -> Self {
        self.network = Some(value.into());
        self
    }
    /// Set the provider `address` parameter.
    #[must_use]
    pub fn address(mut self, value: crate::SensitiveString) -> Self {
        self.address = Some(value);
        self
    }
    /// Set the provider `addressTag` parameter.
    #[must_use]
    pub fn address_tag(mut self, value: crate::SensitiveString) -> Self {
        self.address_tag = Some(value);
        self
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `transactionFeeFlag` parameter.
    #[must_use]
    pub fn transaction_fee_flag(mut self, value: bool) -> Self {
        self.transaction_fee_flag = Some(value);
        self
    }
    /// Set the provider `name` parameter.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
    /// Set the provider `walletType` parameter.
    #[must_use]
    pub fn wallet_type(mut self, value: i64) -> Self {
        self.wallet_type = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Set the provider `questionnaire` parameter.
    #[must_use]
    pub fn questionnaire(mut self, value: crate::SensitiveString) -> Self {
        self.questionnaire = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for WithdrawTravelRule {
    type Response = super::rest_models::WithdrawTravelRuleResponse;
    const OP: Operation = Operation {
        name: "withdrawTravelRule",
        path: "/sapi/v1/localentity/withdraw/apply",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 600,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["address", "amount", "coin", "questionnaire"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("withdrawTravelRule", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [accountApiTradingStatus](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-api-trading-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_api_trading_status(
        &self,
        request: &AccountApiTradingStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountApiTradingStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [accountInfo](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-info).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_info(
        &self,
        request: &AccountInfo,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountInfoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [accountStatus](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_status(
        &self,
        request: &AccountStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [dailyAccountSnapshot](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#daily-account-snapshot).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn daily_account_snapshot(
        &self,
        request: &DailyAccountSnapshot,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DailyAccountSnapshotResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [disableFastWithdrawSwitch](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#disable-fast-withdraw-switch).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn disable_fast_withdraw_switch(
        &self,
        request: &DisableFastWithdrawSwitch,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DisableFastWithdrawSwitchResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [enableFastWithdrawSwitch](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#enable-fast-withdraw-switch).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn enable_fast_withdraw_switch(
        &self,
        request: &EnableFastWithdrawSwitch,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::EnableFastWithdrawSwitchResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getApiKeyPermission](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#get-api-key-permission).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_api_key_permission(
        &self,
        request: &GetApiKeyPermission,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetApiKeyPermissionResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [assetDetail](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-detail).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn asset_detail(
        &self,
        request: &AssetDetail,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AssetDetailResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [assetDividendRecord](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-dividend-record).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn asset_dividend_record(
        &self,
        request: &AssetDividendRecord,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AssetDividendRecordResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [dustConvert](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convert).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn dust_convert(
        &self,
        request: &DustConvert,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::DustConversion>, Error> {
        let target_asset = request
            .target_asset
            .clone()
            .ok_or(Error::Validation("asset provenance required"))?;
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::DustConversion {
                target_asset,
                receipt: response.data,
            },
            meta: response.meta,
        })
    }

    /// [dustConvertibleAssets](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convertible-assets).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn dust_convertible_assets(
        &self,
        request: &DustConvertibleAssets,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ConvertibleDust>, Error> {
        let target_asset = request
            .target_asset
            .clone()
            .ok_or(Error::Validation("asset provenance required"))?;
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::ConvertibleDust {
                target_asset,
                assets: response.data,
            },
            meta: response.meta,
        })
    }

    /// [dustlog](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dustlog).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn dustlog(
        &self,
        request: &Dustlog,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DustlogResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [dustTransfer](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-transfer).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn dust_transfer(
        &self,
        request: &DustTransfer,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DustTransferResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [fundingWallet](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#funding-wallet).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn funding_wallet(
        &self,
        request: &FundingWallet,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FundingWalletResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getSpotAssetTags](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-spot-asset-tags).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_spot_asset_tags(
        &self,
        request: &GetSpotAssetTags,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetSpotAssetTagsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getAssetsThatCanBeConvertedIntoBnb](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-assets-that-can-be-converted-into-bnb).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_assets_that_can_be_converted_into_bnb(
        &self,
        request: &GetAssetsThatCanBeConvertedIntoBnb,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetAssetsThatCanBeConvertedIntoBnbResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getCloudMiningPaymentAndRefundHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_cloud_mining_payment_and_refund_history(
        &self,
        request: &GetCloudMiningPaymentAndRefundHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetCloudMiningPaymentAndRefundHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getOpenSymbolList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-open-symbol-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_open_symbol_list(
        &self,
        request: &GetOpenSymbolList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetOpenSymbolListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryUserDelegationHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-delegation-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_user_delegation_history(
        &self,
        request: &QueryUserDelegationHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUserDelegationHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryUserUniversalTransferHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-universal-transfer-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_user_universal_transfer_history(
        &self,
        request: &QueryUserUniversalTransferHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUserUniversalTransferHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [userUniversalTransfer](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-universal-transfer).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_universal_transfer(
        &self,
        request: &UserUniversalTransfer,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UserUniversalTransferResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryUserWalletBalance](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-wallet-balance).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_user_wallet_balance(
        &self,
        request: &QueryUserWalletBalance,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::QuotedWalletBalance>, Error> {
        let quote_asset = request
            .quote_asset
            .clone()
            .ok_or(Error::Validation("asset provenance required"))?;
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::QuotedWalletBalance {
                quote_asset,
                wallets: response.data,
            },
            meta: response.meta,
        })
    }

    /// [toggleBnbBurnOnSpotTradeAndMarginInterest](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#toggle-bnb-burn-on-spot-trade-and-margin-interest).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn toggle_bnb_burn_on_spot_trade_and_margin_interest(
        &self,
        request: &ToggleBnbBurnOnSpotTradeAndMarginInterest,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::ToggleBnbBurnOnSpotTradeAndMarginInterestResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [tradeFee](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#trade-fee).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn trade_fee(
        &self,
        request: &TradeFee,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TradeFeeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [userAsset](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-asset).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_asset(
        &self,
        request: &UserAsset,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UserAssetResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [allCoinsInformation](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#all-coins-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn all_coins_information(
        &self,
        request: &AllCoinsInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AllCoinsInformationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [depositAddress](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-address).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn deposit_address(
        &self,
        request: &DepositAddress,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DepositAddressResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [depositHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn deposit_history(
        &self,
        request: &DepositHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DepositHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [fetchDepositAddressListWithNetwork](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-deposit-address-list-with-network).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn fetch_deposit_address_list_with_network(
        &self,
        request: &FetchDepositAddressListWithNetwork,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::FetchDepositAddressListWithNetworkResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [fetchWithdrawAddressList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-address-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn fetch_withdraw_address_list(
        &self,
        request: &FetchWithdrawAddressList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FetchWithdrawAddressListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [fetchWithdrawQuota](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-quota).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn fetch_withdraw_quota(
        &self,
        request: &FetchWithdrawQuota,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FetchWithdrawQuotaResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [oneClickArrivalDepositApply](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#one-click-arrival-deposit-apply).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn one_click_arrival_deposit_apply(
        &self,
        request: &OneClickArrivalDepositApply,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OneClickArrivalDepositApplyResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [withdraw](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn withdraw(
        &self,
        request: &Withdraw,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::WithdrawResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [withdrawHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn withdraw_history(
        &self,
        request: &WithdrawHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::WithdrawHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getSymbolsDelistScheduleForSpot](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#get-symbols-delist-schedule-for-spot).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_symbols_delist_schedule_for_spot(
        &self,
        request: &GetSymbolsDelistScheduleForSpot,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetSymbolsDelistScheduleForSpotResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [systemStatus](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#system-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn system_status(
        &self,
        request: &SystemStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SystemStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [brokerWithdraw](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#broker-withdraw).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn broker_withdraw(
        &self,
        request: &BrokerWithdraw,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::BrokerWithdrawResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [checkQuestionnaireRequirements](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#check-questionnaire-requirements).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn check_questionnaire_requirements(
        &self,
        request: &CheckQuestionnaireRequirements,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CheckQuestionnaireRequirementsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getCountryList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-country-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_country_list(
        &self,
        request: &GetCountryList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetCountryListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [depositHistoryTravelRule](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn deposit_history_travel_rule(
        &self,
        request: &DepositHistoryTravelRule,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DepositHistoryTravelRuleResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [depositHistoryV2](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn deposit_history_v2(
        &self,
        request: &DepositHistoryV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DepositHistoryV2Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [fetchAddressVerificationList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#fetch-address-verification-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn fetch_address_verification_list(
        &self,
        request: &FetchAddressVerificationList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FetchAddressVerificationListResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getRegionList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-region-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_region_list(
        &self,
        request: &GetRegionList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetRegionListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [submitDepositQuestionnaire](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn submit_deposit_questionnaire(
        &self,
        request: &SubmitDepositQuestionnaire,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SubmitDepositQuestionnaireResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [submitDepositQuestionnaireTravelRule](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-travel-rule).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn submit_deposit_questionnaire_travel_rule(
        &self,
        request: &SubmitDepositQuestionnaireTravelRule,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::SubmitDepositQuestionnaireTravelRuleResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [submitDepositQuestionnaireV2](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn submit_deposit_questionnaire_v2(
        &self,
        request: &SubmitDepositQuestionnaireV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SubmitDepositQuestionnaireV2Response>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [vaspList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#vasp-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn vasp_list(
        &self,
        request: &VaspList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::VaspListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [withdrawHistoryV1](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v1).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn withdraw_history_v1(
        &self,
        request: &WithdrawHistoryV1,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::WithdrawHistoryV1Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [withdrawHistoryV2](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn withdraw_history_v2(
        &self,
        request: &WithdrawHistoryV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::WithdrawHistoryV2Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [withdrawTravelRule](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-travel-rule).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn withdraw_travel_rule(
        &self,
        request: &WithdrawTravelRule,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::WithdrawTravelRuleResponse>, Error> {
        self.inner.execute(request, deadline).await
    }
}
