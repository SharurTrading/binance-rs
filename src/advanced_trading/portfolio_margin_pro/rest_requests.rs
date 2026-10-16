// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use crate::Decimal;
use crate::Error;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`bnbTransfer`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#bnb-transfer).
#[derive(Clone, Debug, Default, Serialize)]
pub struct BnbTransfer {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "transferSide", skip_serializing_if = "Option::is_none")]
    transfer_side: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl BnbTransfer {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `transferSide` parameter.
    #[must_use]
    pub fn transfer_side(mut self, value: impl Into<String>) -> Self {
        self.transfer_side = Some(value.into());
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
impl Request for BnbTransfer {
    type Response = super::rest_models::BnbTransferResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "bnbTransfer",
        path: "/sapi/v1/portfolio/bnb-transfer",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["amount", "transferSide"],
            &[("transferSide", &["TO_UM", "FROM_UM"])],
            &[],
        )?;
        super::validation::validate("bnbTransfer", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeAutoRepayFuturesStatus`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#change-auto-repay-futures-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeAutoRepayFuturesStatus {
    #[serde(rename = "autoRepay", skip_serializing_if = "Option::is_none")]
    auto_repay: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeAutoRepayFuturesStatus {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `autoRepay` parameter.
    #[must_use]
    pub fn auto_repay(mut self, value: impl Into<String>) -> Self {
        self.auto_repay = Some(value.into());
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
impl Request for ChangeAutoRepayFuturesStatus {
    type Response = super::rest_models::ChangeAutoRepayFuturesStatusResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "changeAutoRepayFuturesStatus",
        path: "/sapi/v1/portfolio/repay-futures-switch",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["autoRepay"],
            &[("autoRepay", &["true", "false"])],
            &[],
        )?;
        super::validation::validate("changeAutoRepayFuturesStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getAutoRepayFuturesStatus`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-auto-repay-futures-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAutoRepayFuturesStatus {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetAutoRepayFuturesStatus {
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
impl Request for GetAutoRepayFuturesStatus {
    type Response = super::rest_models::GetAutoRepayFuturesStatusResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getAutoRepayFuturesStatus",
        path: "/sapi/v1/portfolio/repay-futures-switch",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 30,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getAutoRepayFuturesStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fundAutoCollection`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#fund-auto-collection).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FundAutoCollection {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FundAutoCollection {
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
impl Request for FundAutoCollection {
    type Response = super::rest_models::FundAutoCollectionResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "fundAutoCollection",
        path: "/sapi/v1/portfolio/auto-collection",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("fundAutoCollection", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fundCollectionByAsset`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#fund-collection-by-asset).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FundCollectionByAsset {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "futureType", skip_serializing_if = "Option::is_none")]
    future_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FundCollectionByAsset {
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
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `futureType` parameter.
    #[must_use]
    pub fn future_type(mut self, value: impl Into<String>) -> Self {
        self.future_type = Some(value.into());
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
impl Request for FundCollectionByAsset {
    type Response = super::rest_models::FundCollectionByAssetResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "fundCollectionByAsset",
        path: "/sapi/v1/portfolio/asset-collection",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 60,
        requests_per_second: None,
        requests_per_minute: None,
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
            &[("futureType", &["UM", "CM", "OPTION"])],
            &[],
        )?;
        super::validation::validate("fundCollectionByAsset", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDeltaModeStatus`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-delta-mode-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDeltaModeStatus {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDeltaModeStatus {
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
impl Request for GetDeltaModeStatus {
    type Response = super::rest_models::GetDeltaModeStatusResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getDeltaModeStatus",
        path: "/sapi/v1/portfolio/delta-mode",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getDeltaModeStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`switchDeltaMode`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#switch-delta-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SwitchDeltaMode {
    #[serde(rename = "deltaEnabled", skip_serializing_if = "Option::is_none")]
    delta_enabled: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl SwitchDeltaMode {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `deltaEnabled` parameter.
    #[must_use]
    pub fn delta_enabled(mut self, value: impl Into<String>) -> Self {
        self.delta_enabled = Some(value.into());
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
impl Request for SwitchDeltaMode {
    type Response = super::rest_models::SwitchDeltaModeResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "switchDeltaMode",
        path: "/sapi/v1/portfolio/delta-mode",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["deltaEnabled"],
            &[("deltaEnabled", &["true", "false"])],
            &[],
        )?;
        super::validation::validate("switchDeltaMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getPortfolioMarginProAccountBalance`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-account-balance).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetPortfolioMarginProAccountBalance {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetPortfolioMarginProAccountBalance {
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
impl Request for GetPortfolioMarginProAccountBalance {
    type Response = super::rest_models::GetPortfolioMarginProAccountBalanceResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getPortfolioMarginProAccountBalance",
        path: "/sapi/v1/portfolio/balance",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 20,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getPortfolioMarginProAccountBalance", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getPortfolioMarginProAccountInfo`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-account-info).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetPortfolioMarginProAccountInfo {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetPortfolioMarginProAccountInfo {
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
impl Request for GetPortfolioMarginProAccountInfo {
    type Response = super::rest_models::GetPortfolioMarginProAccountInfoResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getPortfolioMarginProAccountInfo",
        path: "/sapi/v1/portfolio/account",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getPortfolioMarginProAccountInfo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getPortfolioMarginProSpanAccountInfo`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-span-account-info).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetPortfolioMarginProSpanAccountInfo {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetPortfolioMarginProSpanAccountInfo {
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
impl Request for GetPortfolioMarginProSpanAccountInfo {
    type Response = super::rest_models::GetPortfolioMarginProSpanAccountInfoResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getPortfolioMarginProSpanAccountInfo",
        path: "/sapi/v2/portfolio/account",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getPortfolioMarginProSpanAccountInfo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getTransferableEarnAssetBalanceForPortfolioMargin`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-transferable-earn-asset-balance-for-portfolio-margin).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetTransferableEarnAssetBalanceForPortfolioMargin {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "transferType", skip_serializing_if = "Option::is_none")]
    transfer_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetTransferableEarnAssetBalanceForPortfolioMargin {
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
    /// Set the provider `transferType` parameter.
    #[must_use]
    pub fn transfer_type(mut self, value: impl Into<String>) -> Self {
        self.transfer_type = Some(value.into());
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
impl Request for GetTransferableEarnAssetBalanceForPortfolioMargin {
    type Response = super::rest_models::GetTransferableEarnAssetBalanceForPortfolioMarginResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getTransferableEarnAssetBalanceForPortfolioMargin",
        path: "/sapi/v1/portfolio/earn-asset-balance",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["asset", "transferType"],
            &[("transferType", &["EARN_TO_FUTURE", "FUTURE_TO_EARN"])],
            &[],
        )?;
        super::validation::validate("getTransferableEarnAssetBalanceForPortfolioMargin", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`portfolioMarginProBankruptcyLoanRepay`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#portfolio-margin-pro-bankruptcy-loan-repay).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PortfolioMarginProBankruptcyLoanRepay {
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PortfolioMarginProBankruptcyLoanRepay {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `from` parameter.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
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
impl Request for PortfolioMarginProBankruptcyLoanRepay {
    type Response = super::rest_models::PortfolioMarginProBankruptcyLoanRepayResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "portfolioMarginProBankruptcyLoanRepay",
        path: "/sapi/v1/portfolio/repay",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 3000,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[("from", &["SPOT", "MARGIN"])], &[])?;
        super::validation::validate("portfolioMarginProBankruptcyLoanRepay", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryPortfolioMarginProBankruptcyLoanAmount`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-bankruptcy-loan-amount).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryPortfolioMarginProBankruptcyLoanAmount {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryPortfolioMarginProBankruptcyLoanAmount {
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
impl Request for QueryPortfolioMarginProBankruptcyLoanAmount {
    type Response = super::rest_models::QueryPortfolioMarginProBankruptcyLoanAmountResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryPortfolioMarginProBankruptcyLoanAmount",
        path: "/sapi/v1/portfolio/pmLoan",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("queryPortfolioMarginProBankruptcyLoanAmount", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryPortfolioMarginProBankruptcyLoanRepayHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-bankruptcy-loan-repay-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryPortfolioMarginProBankruptcyLoanRepayHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryPortfolioMarginProBankruptcyLoanRepayHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
    /// Set the provider `size` parameter.
    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }
    /// Set the provider `current` parameter.
    #[must_use]
    pub fn current(mut self, value: i64) -> Self {
        self.current = Some(value);
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
impl Request for QueryPortfolioMarginProBankruptcyLoanRepayHistory {
    type Response = super::rest_models::QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryPortfolioMarginProBankruptcyLoanRepayHistory",
        path: "/sapi/v1/portfolio/pmloan-history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 500,
        requests_per_second: None,
        requests_per_minute: None,
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
                ("size", -9_223_372_036_854_775_808, 100),
                ("current", 1, 9_223_372_036_854_775_807),
            ],
        )?;
        super::validation::validate("queryPortfolioMarginProBankruptcyLoanRepayHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryPortfolioMarginProNegativeBalanceInterestHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-negative-balance-interest-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryPortfolioMarginProNegativeBalanceInterestHistory {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryPortfolioMarginProNegativeBalanceInterestHistory {
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
impl Request for QueryPortfolioMarginProNegativeBalanceInterestHistory {
    type Response =
        super::rest_models::QueryPortfolioMarginProNegativeBalanceInterestHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryPortfolioMarginProNegativeBalanceInterestHistory",
        path: "/sapi/v1/portfolio/interest-history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 50,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[("size", -9_223_372_036_854_775_808, 100)])?;
        super::validation::validate("queryPortfolioMarginProNegativeBalanceInterestHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`repayFuturesNegativeBalance`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#repay-futures-negative-balance).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RepayFuturesNegativeBalance {
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl RepayFuturesNegativeBalance {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `from` parameter.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
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
impl Request for RepayFuturesNegativeBalance {
    type Response = super::rest_models::RepayFuturesNegativeBalanceResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "repayFuturesNegativeBalance",
        path: "/sapi/v1/portfolio/repay-futures-negative-balance",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[("from", &["SPOT", "MARGIN"])], &[])?;
        super::validation::validate("repayFuturesNegativeBalance", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`transferLdusdtRwusdForPortfolioMargin`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#transfer-ldusdt-rwusd-for-portfolio-margin).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TransferLdusdtRwusdForPortfolioMargin {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "transferType", skip_serializing_if = "Option::is_none")]
    transfer_type: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl TransferLdusdtRwusdForPortfolioMargin {
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
    /// Set the provider `transferType` parameter.
    #[must_use]
    pub fn transfer_type(mut self, value: impl Into<String>) -> Self {
        self.transfer_type = Some(value.into());
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
impl Request for TransferLdusdtRwusdForPortfolioMargin {
    type Response = super::rest_models::TransferLdusdtRwusdForPortfolioMarginResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "transferLdusdtRwusdForPortfolioMargin",
        path: "/sapi/v1/portfolio/earn-asset-transfer",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["amount", "asset", "transferType"],
            &[
                ("asset", &["LDUSDT", "RWUSD"]),
                ("transferType", &["EARN_TO_FUTURE", "FUTURE_TO_EARN"]),
            ],
            &[],
        )?;
        super::validation::validate("transferLdusdtRwusdForPortfolioMargin", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getMarginCallLevel`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-margin-call-level).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetMarginCallLevel {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetMarginCallLevel {
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
impl Request for GetMarginCallLevel {
    type Response = super::rest_models::GetMarginCallLevelResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getMarginCallLevel",
        path: "/sapi/v1/portfolio/margin-call-level",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
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
        super::validation::validate("getMarginCallLevel", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`setMarginCallLevel`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#set-margin-call-level).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetMarginCallLevel {
    #[serde(rename = "marginCallLevel", skip_serializing_if = "Option::is_none")]
    margin_call_level: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl SetMarginCallLevel {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `marginCallLevel` parameter.
    #[must_use]
    pub fn margin_call_level(mut self, value: Decimal) -> Self {
        self.margin_call_level = Some(value);
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
impl Request for SetMarginCallLevel {
    type Response = super::rest_models::SetMarginCallLevelResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "setMarginCallLevel",
        path: "/sapi/v1/portfolio/margin-call-level",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["marginCallLevel"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("setMarginCallLevel", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`deleteMarginCallLevel`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#delete-margin-call-level).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteMarginCallLevel {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl DeleteMarginCallLevel {
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
impl Request for DeleteMarginCallLevel {
    type Response = super::rest_models::DeleteMarginCallLevelResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "deleteMarginCallLevel",
        path: "/sapi/v1/portfolio/margin-call-level",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1500,
        requests_per_second: None,
        requests_per_minute: None,
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
        super::validation::validate("deleteMarginCallLevel", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getPortfolioMarginAssetLeverage`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#get-portfolio-margin-asset-leverage).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetPortfolioMarginAssetLeverage {}
impl GetPortfolioMarginAssetLeverage {
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
impl Request for GetPortfolioMarginAssetLeverage {
    type Response = super::rest_models::GetPortfolioMarginAssetLeverageResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getPortfolioMarginAssetLeverage",
        path: "/sapi/v1/portfolio/margin-asset-leverage",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 50,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getPortfolioMarginAssetLeverage", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`portfolioMarginCollateralRate`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#portfolio-margin-collateral-rate).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PortfolioMarginCollateralRate {}
impl PortfolioMarginCollateralRate {
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
impl Request for PortfolioMarginCollateralRate {
    type Response = super::rest_models::PortfolioMarginCollateralRateResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "portfolioMarginCollateralRate",
        path: "/sapi/v1/portfolio/collateralRate",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 50,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("portfolioMarginCollateralRate", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`portfolioMarginProTieredCollateralRate`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#portfolio-margin-pro-tiered-collateral-rate).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PortfolioMarginProTieredCollateralRate {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PortfolioMarginProTieredCollateralRate {
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
impl Request for PortfolioMarginProTieredCollateralRate {
    type Response = super::rest_models::PortfolioMarginProTieredCollateralRateResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "portfolioMarginProTieredCollateralRate",
        path: "/sapi/v2/portfolio/collateralRate",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 50,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("portfolioMarginProTieredCollateralRate", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryPortfolioMarginAssetIndexPrice`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#query-portfolio-margin-asset-index-price).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryPortfolioMarginAssetIndexPrice {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
}
impl QueryPortfolioMarginAssetIndexPrice {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for QueryPortfolioMarginAssetIndexPrice {
    type Response = super::rest_models::QueryPortfolioMarginAssetIndexPriceResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryPortfolioMarginAssetIndexPrice",
        path: "/sapi/v1/portfolio/asset-index-price",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("queryPortfolioMarginAssetIndexPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [bnbTransfer](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#bnb-transfer).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn bnb_transfer(
        &self,
        request: &BnbTransfer,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::BnbTransferResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changeAutoRepayFuturesStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#change-auto-repay-futures-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_auto_repay_futures_status(
        &self,
        request: &ChangeAutoRepayFuturesStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeAutoRepayFuturesStatusResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getAutoRepayFuturesStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-auto-repay-futures-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_auto_repay_futures_status(
        &self,
        request: &GetAutoRepayFuturesStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetAutoRepayFuturesStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [fundAutoCollection](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#fund-auto-collection).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn fund_auto_collection(
        &self,
        request: &FundAutoCollection,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FundAutoCollectionResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [fundCollectionByAsset](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#fund-collection-by-asset).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn fund_collection_by_asset(
        &self,
        request: &FundCollectionByAsset,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FundCollectionByAssetResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getDeltaModeStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-delta-mode-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_delta_mode_status(
        &self,
        request: &GetDeltaModeStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetDeltaModeStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [switchDeltaMode](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#switch-delta-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn switch_delta_mode(
        &self,
        request: &SwitchDeltaMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SwitchDeltaModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getPortfolioMarginProAccountBalance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-account-balance).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_portfolio_margin_pro_account_balance(
        &self,
        request: &GetPortfolioMarginProAccountBalance,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetPortfolioMarginProAccountBalanceResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getPortfolioMarginProAccountInfo](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-account-info).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_portfolio_margin_pro_account_info(
        &self,
        request: &GetPortfolioMarginProAccountInfo,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetPortfolioMarginProAccountInfoResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getPortfolioMarginProSpanAccountInfo](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-portfolio-margin-pro-span-account-info).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_portfolio_margin_pro_span_account_info(
        &self,
        request: &GetPortfolioMarginProSpanAccountInfo,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetPortfolioMarginProSpanAccountInfoResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getTransferableEarnAssetBalanceForPortfolioMargin](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-transferable-earn-asset-balance-for-portfolio-margin).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_transferable_earn_asset_balance_for_portfolio_margin(
        &self,
        request: &GetTransferableEarnAssetBalanceForPortfolioMargin,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<
            super::rest_models::GetTransferableEarnAssetBalanceForPortfolioMarginResponse,
        >,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [portfolioMarginProBankruptcyLoanRepay](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#portfolio-margin-pro-bankruptcy-loan-repay).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn portfolio_margin_pro_bankruptcy_loan_repay(
        &self,
        request: &PortfolioMarginProBankruptcyLoanRepay,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::PortfolioMarginProBankruptcyLoanRepayResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryPortfolioMarginProBankruptcyLoanAmount](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-bankruptcy-loan-amount).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_portfolio_margin_pro_bankruptcy_loan_amount(
        &self,
        request: &QueryPortfolioMarginProBankruptcyLoanAmount,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryPortfolioMarginProBankruptcyLoanAmountResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryPortfolioMarginProBankruptcyLoanRepayHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-bankruptcy-loan-repay-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_portfolio_margin_pro_bankruptcy_loan_repay_history(
        &self,
        request: &QueryPortfolioMarginProBankruptcyLoanRepayHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<
            super::rest_models::QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponse,
        >,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryPortfolioMarginProNegativeBalanceInterestHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#query-portfolio-margin-pro-negative-balance-interest-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_portfolio_margin_pro_negative_balance_interest_history(
        &self,
        request: &QueryPortfolioMarginProNegativeBalanceInterestHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<
            super::rest_models::QueryPortfolioMarginProNegativeBalanceInterestHistoryResponse,
        >,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [repayFuturesNegativeBalance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#repay-futures-negative-balance).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn repay_futures_negative_balance(
        &self,
        request: &RepayFuturesNegativeBalance,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::RepayFuturesNegativeBalanceResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [transferLdusdtRwusdForPortfolioMargin](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#transfer-ldusdt-rwusd-for-portfolio-margin).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn transfer_ldusdt_rwusd_for_portfolio_margin(
        &self,
        request: &TransferLdusdtRwusdForPortfolioMargin,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::TransferLdusdtRwusdForPortfolioMarginResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getMarginCallLevel](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#get-margin-call-level).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_margin_call_level(
        &self,
        request: &GetMarginCallLevel,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetMarginCallLevelResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [setMarginCallLevel](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#set-margin-call-level).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn set_margin_call_level(
        &self,
        request: &SetMarginCallLevel,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SetMarginCallLevelResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [deleteMarginCallLevel](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/account#delete-margin-call-level).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn delete_margin_call_level(
        &self,
        request: &DeleteMarginCallLevel,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DeleteMarginCallLevelResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getPortfolioMarginAssetLeverage](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#get-portfolio-margin-asset-leverage).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_portfolio_margin_asset_leverage(
        &self,
        request: &GetPortfolioMarginAssetLeverage,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetPortfolioMarginAssetLeverageResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [portfolioMarginCollateralRate](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#portfolio-margin-collateral-rate).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn portfolio_margin_collateral_rate(
        &self,
        request: &PortfolioMarginCollateralRate,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PortfolioMarginCollateralRateResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [portfolioMarginProTieredCollateralRate](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#portfolio-margin-pro-tiered-collateral-rate).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn portfolio_margin_pro_tiered_collateral_rate(
        &self,
        request: &PortfolioMarginProTieredCollateralRate,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::PortfolioMarginProTieredCollateralRateResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryPortfolioMarginAssetIndexPrice](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api/market-data#query-portfolio-margin-asset-index-price).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_portfolio_margin_asset_index_price(
        &self,
        request: &QueryPortfolioMarginAssetIndexPrice,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryPortfolioMarginAssetIndexPriceResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }
}
