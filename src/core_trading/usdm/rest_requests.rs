// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use super::rest_models::{
    ModifyMultipleOrdersBatchOrdersInputItem, PlaceMultipleOrdersBatchOrdersInputItem,
};
use crate::ClientOrderId;
use crate::Decimal;
use crate::Error;
use crate::Symbol;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`accountInformationV2`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#account-information-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountInformationV2 {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountInformationV2 {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AccountInformationV2 {
    type Response = super::rest_models::AccountInformationV2Response;
    const OP: Operation = Operation {
        name: "accountInformationV2",
        path: "/fapi/v2/account",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("accountInformationV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountInformationV3`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#account-information-v3).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountInformationV3 {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountInformationV3 {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AccountInformationV3 {
    type Response = super::rest_models::AccountInformationV3Response;
    const OP: Operation = Operation {
        name: "accountInformationV3",
        path: "/fapi/v3/account",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("accountInformationV3", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresAccountBalanceV2`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-balance-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FuturesAccountBalanceV2 {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FuturesAccountBalanceV2 {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for FuturesAccountBalanceV2 {
    type Response = super::rest_models::FuturesAccountBalanceV2Response;
    const OP: Operation = Operation {
        name: "futuresAccountBalanceV2",
        path: "/fapi/v2/balance",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("futuresAccountBalanceV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresAccountBalanceV3`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-balance-v3).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FuturesAccountBalanceV3 {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FuturesAccountBalanceV3 {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for FuturesAccountBalanceV3 {
    type Response = super::rest_models::FuturesAccountBalanceV3Response;
    const OP: Operation = Operation {
        name: "futuresAccountBalanceV3",
        path: "/fapi/v3/balance",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("futuresAccountBalanceV3", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresAccountConfiguration`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-configuration).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FuturesAccountConfiguration {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FuturesAccountConfiguration {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for FuturesAccountConfiguration {
    type Response = super::rest_models::FuturesAccountConfigurationResponse;
    const OP: Operation = Operation {
        name: "futuresAccountConfiguration",
        path: "/fapi/v1/accountConfig",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("futuresAccountConfiguration", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresTradingQuantitativeRulesIndicators`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-trading-quantitative-rules-indicators).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FuturesTradingQuantitativeRulesIndicators {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FuturesTradingQuantitativeRulesIndicators {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for FuturesTradingQuantitativeRulesIndicators {
    type Response = super::rest_models::FuturesTradingQuantitativeRulesIndicatorsResponse;
    const OP: Operation = Operation {
        name: "futuresTradingQuantitativeRulesIndicators",
        path: "/fapi/v1/apiTradingStatus",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        requests_per_second: None,
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
        super::validation::validate("futuresTradingQuantitativeRulesIndicators", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getBnbBurnStatus`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-bnb-burn-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetBnbBurnStatus {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetBnbBurnStatus {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetBnbBurnStatus {
    type Response = super::rest_models::GetBnbBurnStatusResponse;
    const OP: Operation = Operation {
        name: "getBnbBurnStatus",
        path: "/fapi/v1/feeBurn",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 30,
        requests_per_second: None,
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
        super::validation::validate("getBnbBurnStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`toggleBnbBurnOnFuturesTrade`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#toggle-bnb-burn-on-futures-trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ToggleBnbBurnOnFuturesTrade {
    #[serde(rename = "feeBurn", skip_serializing_if = "Option::is_none")]
    fee_burn: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ToggleBnbBurnOnFuturesTrade {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `feeBurn` parameter.
    #[must_use]
    pub fn fee_burn(mut self, value: impl Into<String>) -> Self {
        self.fee_burn = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ToggleBnbBurnOnFuturesTrade {
    type Response = super::rest_models::ToggleBnbBurnOnFuturesTradeResponse;
    const OP: Operation = Operation {
        name: "toggleBnbBurnOnFuturesTrade",
        path: "/fapi/v1/feeBurn",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["feeBurn"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("toggleBnbBurnOnFuturesTrade", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCurrentMultiAssetsMode`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-current-multi-assets-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCurrentMultiAssetsMode {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetCurrentMultiAssetsMode {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetCurrentMultiAssetsMode {
    type Response = super::rest_models::GetCurrentMultiAssetsModeResponse;
    const OP: Operation = Operation {
        name: "getCurrentMultiAssetsMode",
        path: "/fapi/v1/multiAssetsMargin",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 30,
        requests_per_second: None,
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
        super::validation::validate("getCurrentMultiAssetsMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeMultiAssetsMode`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-multi-assets-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeMultiAssetsMode {
    #[serde(rename = "multiAssetsMargin", skip_serializing_if = "Option::is_none")]
    multi_assets_margin: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeMultiAssetsMode {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `multiAssetsMargin` parameter.
    #[must_use]
    pub fn multi_assets_margin(mut self, value: impl Into<String>) -> Self {
        self.multi_assets_margin = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ChangeMultiAssetsMode {
    type Response = super::rest_models::ChangeMultiAssetsModeResponse;
    const OP: Operation = Operation {
        name: "changeMultiAssetsMode",
        path: "/fapi/v1/multiAssetsMargin",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["multiAssetsMargin"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("changeMultiAssetsMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCurrentPositionMode`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-current-position-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCurrentPositionMode {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetCurrentPositionMode {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetCurrentPositionMode {
    type Response = super::rest_models::GetCurrentPositionModeResponse;
    const OP: Operation = Operation {
        name: "getCurrentPositionMode",
        path: "/fapi/v1/positionSide/dual",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 30,
        requests_per_second: None,
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
        super::validation::validate("getCurrentPositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changePositionMode`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-position-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangePositionMode {
    #[serde(rename = "dualSidePosition", skip_serializing_if = "Option::is_none")]
    dual_side_position: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangePositionMode {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `dualSidePosition` parameter.
    #[must_use]
    pub fn dual_side_position(mut self, value: impl Into<String>) -> Self {
        self.dual_side_position = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ChangePositionMode {
    type Response = super::rest_models::ChangePositionModeResponse;
    const OP: Operation = Operation {
        name: "changePositionMode",
        path: "/fapi/v1/positionSide/dual",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["dualSidePosition"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("changePositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForFuturesOrderHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-order-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDownloadIdForFuturesOrderHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDownloadIdForFuturesOrderHistory {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetDownloadIdForFuturesOrderHistory {
    type Response = super::rest_models::GetDownloadIdForFuturesOrderHistoryResponse;
    const OP: Operation = Operation {
        name: "getDownloadIdForFuturesOrderHistory",
        path: "/fapi/v1/order/asyn",
        method: "GET",
        security: Security::Signed,
        mutation: true,
        weight: 1000,
        requests_per_second: None,
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getDownloadIdForFuturesOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForFuturesTradeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-trade-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDownloadIdForFuturesTradeHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDownloadIdForFuturesTradeHistory {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetDownloadIdForFuturesTradeHistory {
    type Response = super::rest_models::GetDownloadIdForFuturesTradeHistoryResponse;
    const OP: Operation = Operation {
        name: "getDownloadIdForFuturesTradeHistory",
        path: "/fapi/v1/trade/asyn",
        method: "GET",
        security: Security::Signed,
        mutation: true,
        weight: 1000,
        requests_per_second: None,
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getDownloadIdForFuturesTradeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForFuturesTransactionHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-transaction-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDownloadIdForFuturesTransactionHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDownloadIdForFuturesTransactionHistory {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetDownloadIdForFuturesTransactionHistory {
    type Response = super::rest_models::GetDownloadIdForFuturesTransactionHistoryResponse;
    const OP: Operation = Operation {
        name: "getDownloadIdForFuturesTransactionHistory",
        path: "/fapi/v1/income/asyn",
        method: "GET",
        security: Security::Signed,
        mutation: true,
        weight: 1000,
        requests_per_second: None,
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getDownloadIdForFuturesTransactionHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFuturesOrderHistoryDownloadLinkById`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-order-history-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesOrderHistoryDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetFuturesOrderHistoryDownloadLinkById {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `downloadId` parameter.
    #[must_use]
    pub fn download_id(mut self, value: impl Into<String>) -> Self {
        self.download_id = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetFuturesOrderHistoryDownloadLinkById {
    type Response = super::rest_models::GetFuturesOrderHistoryDownloadLinkByIdResponse;
    const OP: Operation = Operation {
        name: "getFuturesOrderHistoryDownloadLinkById",
        path: "/fapi/v1/order/asyn/id",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["downloadId"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getFuturesOrderHistoryDownloadLinkById", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFuturesTradeDownloadLinkById`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-trade-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesTradeDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetFuturesTradeDownloadLinkById {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `downloadId` parameter.
    #[must_use]
    pub fn download_id(mut self, value: impl Into<String>) -> Self {
        self.download_id = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetFuturesTradeDownloadLinkById {
    type Response = super::rest_models::GetFuturesTradeDownloadLinkByIdResponse;
    const OP: Operation = Operation {
        name: "getFuturesTradeDownloadLinkById",
        path: "/fapi/v1/trade/asyn/id",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["downloadId"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getFuturesTradeDownloadLinkById", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFuturesTransactionHistoryDownloadLinkById`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-transaction-history-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesTransactionHistoryDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetFuturesTransactionHistoryDownloadLinkById {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `downloadId` parameter.
    #[must_use]
    pub fn download_id(mut self, value: impl Into<String>) -> Self {
        self.download_id = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetFuturesTransactionHistoryDownloadLinkById {
    type Response = super::rest_models::GetFuturesTransactionHistoryDownloadLinkByIdResponse;
    const OP: Operation = Operation {
        name: "getFuturesTransactionHistoryDownloadLinkById",
        path: "/fapi/v1/income/asyn/id",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["downloadId"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getFuturesTransactionHistoryDownloadLinkById", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getIncomeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-income-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetIncomeHistory {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "incomeType", skip_serializing_if = "Option::is_none")]
    income_type: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetIncomeHistory {
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
    /// Set the provider `incomeType` parameter.
    #[must_use]
    pub fn income_type(mut self, value: impl Into<String>) -> Self {
        self.income_type = Some(value.into());
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
    /// Set the provider `page` parameter.
    #[must_use]
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetIncomeHistory {
    type Response = super::rest_models::GetIncomeHistoryResponse;
    const OP: Operation = Operation {
        name: "getIncomeHistory",
        path: "/fapi/v1/income",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 30,
        requests_per_second: None,
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
            &[(
                "incomeType",
                &[
                    "TRANSFER",
                    "WELCOME_BONUS",
                    "REALIZED_PNL",
                    "FUNDING_FEE",
                    "COMMISSION",
                    "INSURANCE_CLEAR",
                    "REFERRAL_KICKBACK",
                    "COMMISSION_REBATE",
                    "API_REBATE",
                    "CONTEST_REWARD",
                    "CROSS_COLLATERAL_TRANSFER",
                    "OPTIONS_PREMIUM_FEE",
                    "OPTIONS_SETTLE_PROFIT",
                    "INTERNAL_TRANSFER",
                    "AUTO_EXCHANGE",
                    "DELIVERED_SETTELMENT",
                    "COIN_SWAP_DEPOSIT",
                    "COIN_SWAP_WITHDRAW",
                    "POSITION_LIMIT_INCREASE_FEE",
                    "STRATEGY_UMFUTURES_TRANSFER",
                    "FEE_RETURN",
                    "BFUSD_REWARD",
                    "SPECIAL_FUNDING_FEE",
                ],
            )],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("getIncomeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`notionalAndLeverageBrackets`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#notional-and-leverage-brackets).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NotionalAndLeverageBrackets {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NotionalAndLeverageBrackets {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for NotionalAndLeverageBrackets {
    type Response = super::rest_models::NotionalAndLeverageBracketsResponse;
    const OP: Operation = Operation {
        name: "notionalAndLeverageBrackets",
        path: "/fapi/v1/leverageBracket",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
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
        super::validation::validate("notionalAndLeverageBrackets", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUserRateLimit`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#query-user-rate-limit).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUserRateLimit {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUserRateLimit {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QueryUserRateLimit {
    type Response = super::rest_models::QueryUserRateLimitResponse;
    const OP: Operation = Operation {
        name: "queryUserRateLimit",
        path: "/fapi/v1/rateLimit/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
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
        super::validation::validate("queryUserRateLimit", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolConfiguration`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#symbol-configuration).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SymbolConfiguration {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl SymbolConfiguration {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for SymbolConfiguration {
    type Response = super::rest_models::SymbolConfigurationResponse;
    const OP: Operation = Operation {
        name: "symbolConfiguration",
        path: "/fapi/v1/symbolConfig",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("symbolConfiguration", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userCommissionRate`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#user-commission-rate).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserCommissionRate {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UserCommissionRate {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for UserCommissionRate {
    type Response = super::rest_models::UserCommissionRateResponse;
    const OP: Operation = Operation {
        name: "userCommissionRate",
        path: "/fapi/v1/commissionRate",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 20,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("userCommissionRate", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`acceptTheOfferedQuote`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#accept-the-offered-quote).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AcceptTheOfferedQuote {
    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    quote_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AcceptTheOfferedQuote {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `quoteId` parameter.
    #[must_use]
    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AcceptTheOfferedQuote {
    type Response = super::rest_models::AcceptTheOfferedQuoteResponse;
    const OP: Operation = Operation {
        name: "acceptTheOfferedQuote",
        path: "/fapi/v1/convert/acceptQuote",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 200,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["quoteId"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("acceptTheOfferedQuote", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`listAllConvertPairs`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#list-all-convert-pairs).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ListAllConvertPairs {
    #[serde(rename = "fromAsset", skip_serializing_if = "Option::is_none")]
    from_asset: Option<String>,
    #[serde(rename = "toAsset", skip_serializing_if = "Option::is_none")]
    to_asset: Option<String>,
}
impl ListAllConvertPairs {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `fromAsset` parameter.
    #[must_use]
    pub fn from_asset(mut self, value: impl Into<String>) -> Self {
        self.from_asset = Some(value.into());
        self
    }
    /// Set the provider `toAsset` parameter.
    #[must_use]
    pub fn to_asset(mut self, value: impl Into<String>) -> Self {
        self.to_asset = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ListAllConvertPairs {
    type Response = super::rest_models::ListAllConvertPairsResponse;
    const OP: Operation = Operation {
        name: "listAllConvertPairs",
        path: "/fapi/v1/convert/exchangeInfo",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 20,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("listAllConvertPairs", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderStatus`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#order-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderStatus {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    quote_id: Option<String>,
}
impl OrderStatus {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }
    /// Set the provider `quoteId` parameter.
    #[must_use]
    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for OrderStatus {
    type Response = super::rest_models::OrderStatusResponse;
    const OP: Operation = Operation {
        name: "orderStatus",
        path: "/fapi/v1/convert/orderStatus",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 50,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("orderStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`sendQuoteRequest`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#send-quote-request).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SendQuoteRequest {
    #[serde(rename = "fromAsset", skip_serializing_if = "Option::is_none")]
    from_asset: Option<String>,
    #[serde(rename = "toAsset", skip_serializing_if = "Option::is_none")]
    to_asset: Option<String>,
    #[serde(rename = "fromAmount", skip_serializing_if = "Option::is_none")]
    from_amount: Option<Decimal>,
    #[serde(rename = "toAmount", skip_serializing_if = "Option::is_none")]
    to_amount: Option<Decimal>,
    #[serde(rename = "validTime", skip_serializing_if = "Option::is_none")]
    valid_time: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl SendQuoteRequest {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `fromAsset` parameter.
    #[must_use]
    pub fn from_asset(mut self, value: impl Into<String>) -> Self {
        self.from_asset = Some(value.into());
        self
    }
    /// Set the provider `toAsset` parameter.
    #[must_use]
    pub fn to_asset(mut self, value: impl Into<String>) -> Self {
        self.to_asset = Some(value.into());
        self
    }
    /// Set the provider `fromAmount` parameter.
    #[must_use]
    pub fn from_amount(mut self, value: Decimal) -> Self {
        self.from_amount = Some(value);
        self
    }
    /// Set the provider `toAmount` parameter.
    #[must_use]
    pub fn to_amount(mut self, value: Decimal) -> Self {
        self.to_amount = Some(value);
        self
    }
    /// Set the provider `validTime` parameter.
    #[must_use]
    pub fn valid_time(mut self, value: impl Into<String>) -> Self {
        self.valid_time = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for SendQuoteRequest {
    type Response = super::rest_models::SendQuoteRequestResponse;
    const OP: Operation = Operation {
        name: "sendQuoteRequest",
        path: "/fapi/v1/convert/getQuote",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["fromAsset", "toAsset"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("sendQuoteRequest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`adlRisk`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#adl-risk).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AdlRisk {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl AdlRisk {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AdlRisk {
    type Response = super::rest_models::AdlRiskResponse;
    const OP: Operation = Operation {
        name: "adlRisk",
        path: "/fapi/v1/symbolAdlRisk",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("adlRisk", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`basis`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#basis).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Basis {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    contract_type: Option<String>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
}
impl Basis {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
        self
    }
    /// Set the provider `contractType` parameter.
    #[must_use]
    pub fn contract_type(mut self, value: impl Into<String>) -> Self {
        self.contract_type = Some(value.into());
        self
    }
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for Basis {
    type Response = super::rest_models::BasisResponse;
    const OP: Operation = Operation {
        name: "basis",
        path: "/futures/data/basis",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["contractType", "pair", "period"],
            &[
                (
                    "contractType",
                    &["PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER"],
                ),
                (
                    "period",
                    &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
                ),
            ],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("basis", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`checkServerTime`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#check-server-time).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CheckServerTime {}
impl CheckServerTime {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CheckServerTime {
    type Response = super::rest_models::CheckServerTimeResponse;
    const OP: Operation = Operation {
        name: "checkServerTime",
        path: "/fapi/v1/time",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("checkServerTime", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`compositeIndexSymbolInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#composite-index-symbol-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CompositeIndexSymbolInformation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl CompositeIndexSymbolInformation {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CompositeIndexSymbolInformation {
    type Response = super::rest_models::CompositeIndexSymbolInformationResponse;
    const OP: Operation = Operation {
        name: "compositeIndexSymbolInformation",
        path: "/fapi/v1/indexInfo",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("compositeIndexSymbolInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`compressedAggregateTradesList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#compressed-aggregate-trades-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CompressedAggregateTradesList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl CompressedAggregateTradesList {
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
    /// Set the provider `fromId` parameter.
    #[must_use]
    pub fn from_id(mut self, value: i64) -> Self {
        self.from_id = Some(value);
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CompressedAggregateTradesList {
    type Response = super::rest_models::CompressedAggregateTradesListResponse;
    const OP: Operation = Operation {
        name: "compressedAggregateTradesList",
        path: "/fapi/v1/aggTrades",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 20,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("compressedAggregateTradesList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`continuousContractKlineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#continuous-contract-kline-candlestick-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ContinuousContractKlineCandlestickData {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    contract_type: Option<String>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl ContinuousContractKlineCandlestickData {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
        self
    }
    /// Set the provider `contractType` parameter.
    #[must_use]
    pub fn contract_type(mut self, value: impl Into<String>) -> Self {
        self.contract_type = Some(value.into());
        self
    }
    /// Set the provider `interval` parameter.
    #[must_use]
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ContinuousContractKlineCandlestickData {
    type Response = super::rest_models::ContinuousContractKlineCandlestickDataResponse;
    const OP: Operation = Operation {
        name: "continuousContractKlineCandlestickData",
        path: "/fapi/v1/continuousKlines",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["contractType", "interval", "pair"],
            &[
                (
                    "contractType",
                    &[
                        "PERPETUAL",
                        "CURRENT_QUARTER",
                        "NEXT_QUARTER",
                        "TRADIFI_PERPETUAL",
                    ],
                ),
                (
                    "interval",
                    &[
                        "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d",
                        "3d", "1w", "1M",
                    ],
                ),
            ],
            &[("limit", -9_223_372_036_854_775_808, 1_500)],
        )?;
        super::validation::validate("continuousContractKlineCandlestickData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`exchangeInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#exchange-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ExchangeInformation {}
impl ExchangeInformation {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ExchangeInformation {
    type Response = super::rest_models::ExchangeInformationResponse;
    const OP: Operation = Operation {
        name: "exchangeInformation",
        path: "/fapi/v1/exchangeInfo",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("exchangeInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFundingRateHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#get-funding-rate-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFundingRateHistory {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl GetFundingRateHistory {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetFundingRateHistory {
    type Response = super::rest_models::GetFundingRateHistoryResponse;
    const OP: Operation = Operation {
        name: "getFundingRateHistory",
        path: "/fapi/v1/fundingRate",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
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
        super::validation::validate("getFundingRateHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFundingRateInfo`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#get-funding-rate-info).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFundingRateInfo {}
impl GetFundingRateInfo {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetFundingRateInfo {
    type Response = super::rest_models::GetFundingRateInfoResponse;
    const OP: Operation = Operation {
        name: "getFundingRateInfo",
        path: "/fapi/v1/fundingInfo",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getFundingRateInfo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`indexPriceKlineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#index-price-kline-candlestick-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct IndexPriceKlineCandlestickData {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl IndexPriceKlineCandlestickData {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
        self
    }
    /// Set the provider `interval` parameter.
    #[must_use]
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for IndexPriceKlineCandlestickData {
    type Response = super::rest_models::IndexPriceKlineCandlestickDataResponse;
    const OP: Operation = Operation {
        name: "indexPriceKlineCandlestickData",
        path: "/fapi/v1/indexPriceKlines",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["interval", "pair"],
            &[(
                "interval",
                &[
                    "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d",
                    "3d", "1w", "1M",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_500)],
        )?;
        super::validation::validate("indexPriceKlineCandlestickData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`klineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#kline-candlestick-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct KlineCandlestickData {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl KlineCandlestickData {
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
    /// Set the provider `interval` parameter.
    #[must_use]
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for KlineCandlestickData {
    type Response = super::rest_models::KlineCandlestickDataResponse;
    const OP: Operation = Operation {
        name: "klineCandlestickData",
        path: "/fapi/v1/klines",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["interval", "symbol"],
            &[(
                "interval",
                &[
                    "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d",
                    "3d", "1w", "1M",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_500)],
        )?;
        super::validation::validate("klineCandlestickData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`longShortRatio`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#long-short-ratio).
#[derive(Clone, Debug, Default, Serialize)]
pub struct LongShortRatio {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
}
impl LongShortRatio {
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
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for LongShortRatio {
    type Response = super::rest_models::LongShortRatioResponse;
    const OP: Operation = Operation {
        name: "longShortRatio",
        path: "/futures/data/globalLongShortAccountRatio",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["period", "symbol"],
            &[(
                "period",
                &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
            )],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("longShortRatio", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`markPrice`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#mark-price).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarkPrice {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl MarkPrice {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for MarkPrice {
    type Response = super::rest_models::MarkPriceResponse;
    const OP: Operation = Operation {
        name: "markPrice",
        path: "/fapi/v1/premiumIndex",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("markPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`markPriceKlineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#mark-price-kline-candlestick-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarkPriceKlineCandlestickData {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl MarkPriceKlineCandlestickData {
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
    /// Set the provider `interval` parameter.
    #[must_use]
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for MarkPriceKlineCandlestickData {
    type Response = super::rest_models::MarkPriceKlineCandlestickDataResponse;
    const OP: Operation = Operation {
        name: "markPriceKlineCandlestickData",
        path: "/fapi/v1/markPriceKlines",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["interval", "symbol"],
            &[(
                "interval",
                &[
                    "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d",
                    "3d", "1w", "1M",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_500)],
        )?;
        super::validation::validate("markPriceKlineCandlestickData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`assetIndex`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#asset-index).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AssetIndex {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl AssetIndex {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AssetIndex {
    type Response = super::rest_models::AssetIndexResponse;
    const OP: Operation = Operation {
        name: "assetIndex",
        path: "/fapi/v1/assetIndex",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("assetIndex", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`oldTradesLookup`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#old-trades-lookup).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OldTradesLookup {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
}
impl OldTradesLookup {
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
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `fromId` parameter.
    #[must_use]
    pub fn from_id(mut self, value: i64) -> Self {
        self.from_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for OldTradesLookup {
    type Response = super::rest_models::OldTradesLookupResponse;
    const OP: Operation = Operation {
        name: "oldTradesLookup",
        path: "/fapi/v1/historicalTrades",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 200,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("oldTradesLookup", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openInterest`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#open-interest).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenInterest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl OpenInterest {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for OpenInterest {
    type Response = super::rest_models::OpenInterestResponse;
    const OP: Operation = Operation {
        name: "openInterest",
        path: "/fapi/v1/openInterest",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("openInterest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openInterestStatistics`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#open-interest-statistics).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenInterestStatistics {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
}
impl OpenInterestStatistics {
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
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for OpenInterestStatistics {
    type Response = super::rest_models::OpenInterestStatisticsResponse;
    const OP: Operation = Operation {
        name: "openInterestStatistics",
        path: "/futures/data/openInterestHist",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["period", "symbol"],
            &[(
                "period",
                &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
            )],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("openInterestStatistics", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderBook`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#order-book).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderBook {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl OrderBook {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for OrderBook {
    type Response = super::rest_models::OrderBookResponse;
    const OP: Operation = Operation {
        name: "orderBook",
        path: "/fapi/v1/depth",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("orderBook", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`premiumIndexKlineData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#premium-index-kline-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PremiumIndexKlineData {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl PremiumIndexKlineData {
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
    /// Set the provider `interval` parameter.
    #[must_use]
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for PremiumIndexKlineData {
    type Response = super::rest_models::PremiumIndexKlineDataResponse;
    const OP: Operation = Operation {
        name: "premiumIndexKlineData",
        path: "/fapi/v1/premiumIndexKlines",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["interval", "symbol"],
            &[(
                "interval",
                &[
                    "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d",
                    "3d", "1w", "1M",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_500)],
        )?;
        super::validation::validate("premiumIndexKlineData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`quarterlyContractSettlementPrice`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#quarterly-contract-settlement-price).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QuarterlyContractSettlementPrice {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
}
impl QuarterlyContractSettlementPrice {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QuarterlyContractSettlementPrice {
    type Response = super::rest_models::QuarterlyContractSettlementPriceResponse;
    const OP: Operation = Operation {
        name: "quarterlyContractSettlementPrice",
        path: "/futures/data/delivery-price",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["pair"], &[], &[])?;
        super::validation::validate("quarterlyContractSettlementPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryIndexPriceConstituents`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#query-index-price-constituents).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryIndexPriceConstituents {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl QueryIndexPriceConstituents {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QueryIndexPriceConstituents {
    type Response = super::rest_models::QueryIndexPriceConstituentsResponse;
    const OP: Operation = Operation {
        name: "queryIndexPriceConstituents",
        path: "/fapi/v1/constituents",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 2,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryIndexPriceConstituents", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryInsuranceFundBalanceSnapshot`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#query-insurance-fund-balance-snapshot).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryInsuranceFundBalanceSnapshot {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl QueryInsuranceFundBalanceSnapshot {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QueryInsuranceFundBalanceSnapshot {
    type Response = super::rest_models::QueryInsuranceFundBalanceSnapshotResponse;
    const OP: Operation = Operation {
        name: "queryInsuranceFundBalanceSnapshot",
        path: "/fapi/v1/insuranceBalance",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("queryInsuranceFundBalanceSnapshot", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`recentTradesList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#recent-trades-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RecentTradesList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl RecentTradesList {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for RecentTradesList {
    type Response = super::rest_models::RecentTradesListResponse;
    const OP: Operation = Operation {
        name: "recentTradesList",
        path: "/fapi/v1/trades",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 5,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("recentTradesList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`rpiOrderBook`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#rpi-order-book).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RpiOrderBook {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl RpiOrderBook {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for RpiOrderBook {
    type Response = super::rest_models::RpiOrderBookResponse;
    const OP: Operation = Operation {
        name: "rpiOrderBook",
        path: "/fapi/v1/rpiDepth",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("rpiOrderBook", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolOrderBookTicker`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-order-book-ticker).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SymbolOrderBookTicker {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl SymbolOrderBookTicker {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for SymbolOrderBookTicker {
    type Response = super::rest_models::SymbolOrderBookTickerResponse;
    const OP: Operation = Operation {
        name: "symbolOrderBookTicker",
        path: "/fapi/v1/ticker/bookTicker",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("symbolOrderBookTicker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolPriceTicker`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-price-ticker).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SymbolPriceTicker {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl SymbolPriceTicker {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for SymbolPriceTicker {
    type Response = super::rest_models::SymbolPriceTickerResponse;
    const OP: Operation = Operation {
        name: "symbolPriceTicker",
        path: "/fapi/v1/ticker/price",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("symbolPriceTicker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolPriceTickerV2`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-price-ticker-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SymbolPriceTickerV2 {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl SymbolPriceTickerV2 {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for SymbolPriceTickerV2 {
    type Response = super::rest_models::SymbolPriceTickerV2Response;
    const OP: Operation = Operation {
        name: "symbolPriceTickerV2",
        path: "/fapi/v2/ticker/price",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("symbolPriceTickerV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`takerBuySellVolume`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#taker-buy-sell-volume).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TakerBuySellVolume {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
}
impl TakerBuySellVolume {
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
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for TakerBuySellVolume {
    type Response = super::rest_models::TakerBuySellVolumeResponse;
    const OP: Operation = Operation {
        name: "takerBuySellVolume",
        path: "/futures/data/takerlongshortRatio",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["period", "symbol"],
            &[(
                "period",
                &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
            )],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("takerBuySellVolume", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`testConnectivity`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#test-connectivity).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TestConnectivity {}
impl TestConnectivity {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for TestConnectivity {
    type Response = super::rest_models::TestConnectivityResponse;
    const OP: Operation = Operation {
        name: "testConnectivity",
        path: "/fapi/v1/ping",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("testConnectivity", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`ticker24hrPriceChangeStatistics`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#ticker24hr-price-change-statistics).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Ticker24hrPriceChangeStatistics {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl Ticker24hrPriceChangeStatistics {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for Ticker24hrPriceChangeStatistics {
    type Response = super::rest_models::Ticker24hrPriceChangeStatisticsResponse;
    const OP: Operation = Operation {
        name: "ticker24hrPriceChangeStatistics",
        path: "/fapi/v1/ticker/24hr",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("ticker24hrPriceChangeStatistics", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`topTraderLongShortRatioAccounts`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-accounts).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TopTraderLongShortRatioAccounts {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
}
impl TopTraderLongShortRatioAccounts {
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
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for TopTraderLongShortRatioAccounts {
    type Response = super::rest_models::TopTraderLongShortRatioAccountsResponse;
    const OP: Operation = Operation {
        name: "topTraderLongShortRatioAccounts",
        path: "/futures/data/topLongShortAccountRatio",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["period", "symbol"],
            &[(
                "period",
                &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
            )],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("topTraderLongShortRatioAccounts", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`topTraderLongShortRatioPositions`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-positions).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TopTraderLongShortRatioPositions {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
}
impl TopTraderLongShortRatioPositions {
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
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for TopTraderLongShortRatioPositions {
    type Response = super::rest_models::TopTraderLongShortRatioPositionsResponse;
    const OP: Operation = Operation {
        name: "topTraderLongShortRatioPositions",
        path: "/futures/data/topLongShortPositionRatio",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["period", "symbol"],
            &[(
                "period",
                &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
            )],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("topTraderLongShortRatioPositions", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tradingSchedule`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#trading-schedule).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TradingSchedule {}
impl TradingSchedule {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for TradingSchedule {
    type Response = super::rest_models::TradingScheduleResponse;
    const OP: Operation = Operation {
        name: "tradingSchedule",
        path: "/fapi/v1/tradingSchedule",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 5,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("tradingSchedule", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`classicPortfolioMarginAccountInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/portfolio-margin-endpoints#classic-portfolio-margin-account-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ClassicPortfolioMarginAccountInformation {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ClassicPortfolioMarginAccountInformation {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `asset` parameter.
    #[must_use]
    pub fn asset(mut self, value: impl Into<String>) -> Self {
        self.asset = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ClassicPortfolioMarginAccountInformation {
    type Response = super::rest_models::ClassicPortfolioMarginAccountInformationResponse;
    const OP: Operation = Operation {
        name: "classicPortfolioMarginAccountInformation",
        path: "/fapi/v1/pmAccountInfo",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("classicPortfolioMarginAccountInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountTradeList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#account-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountTradeList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountTradeList {
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
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
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
    /// Set the provider `fromId` parameter.
    #[must_use]
    pub fn from_id(mut self, value: i64) -> Self {
        self.from_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AccountTradeList {
    type Response = super::rest_models::AccountTradeListResponse;
    const OP: Operation = Operation {
        name: "accountTradeList",
        path: "/fapi/v1/userTrades",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("accountTradeList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`allOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#all-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AllOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AllOrders {
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
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AllOrders {
    type Response = super::rest_models::AllOrdersResponse;
    const OP: Operation = Operation {
        name: "allOrders",
        path: "/fapi/v1/allOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("allOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`autoCancelAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#auto-cancel-all-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AutoCancelAllOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "countdownTime", skip_serializing_if = "Option::is_none")]
    countdown_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AutoCancelAllOpenOrders {
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
    /// Set the provider `countdownTime` parameter.
    #[must_use]
    pub fn countdown_time(mut self, value: i64) -> Self {
        self.countdown_time = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for AutoCancelAllOpenOrders {
    type Response = super::rest_models::AutoCancelAllOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "autoCancelAllOpenOrders",
        path: "/fapi/v1/countdownCancelAll",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 10,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["countdownTime", "symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("autoCancelAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAlgoOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-algo-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAlgoOrder {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "clientAlgoId", skip_serializing_if = "Option::is_none")]
    client_algo_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAlgoOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `algoId` parameter.
    #[must_use]
    pub fn algo_id(mut self, value: i64) -> Self {
        self.algo_id = Some(value);
        self
    }
    /// Set the provider `clientAlgoId` parameter.
    #[must_use]
    pub fn client_algo_id(mut self, value: ClientOrderId) -> Self {
        self.client_algo_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CancelAlgoOrder {
    type Response = super::rest_models::CancelAlgoOrderResponse;
    const OP: Operation = Operation {
        name: "cancelAlgoOrder",
        path: "/fapi/v1/algoOrder",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
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
        super::validation::validate("cancelAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newAlgoOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-algo-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewAlgoOrder {
    #[serde(rename = "algoType", skip_serializing_if = "Option::is_none")]
    algo_type: Option<String>,
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
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "triggerPrice", skip_serializing_if = "Option::is_none")]
    trigger_price: Option<Decimal>,
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    working_type: Option<String>,
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    price_match: Option<String>,
    #[serde(rename = "closePosition", skip_serializing_if = "Option::is_none")]
    close_position: Option<String>,
    #[serde(rename = "priceProtect", skip_serializing_if = "Option::is_none")]
    price_protect: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<String>,
    #[serde(rename = "activatePrice", skip_serializing_if = "Option::is_none")]
    activate_price: Option<Decimal>,
    #[serde(rename = "callbackRate", skip_serializing_if = "Option::is_none")]
    callback_rate: Option<Decimal>,
    #[serde(rename = "clientAlgoId", skip_serializing_if = "Option::is_none")]
    client_algo_id: Option<ClientOrderId>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "goodTillDate", skip_serializing_if = "Option::is_none")]
    good_till_date: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NewAlgoOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `algoType` parameter.
    #[must_use]
    pub fn algo_type(mut self, value: impl Into<String>) -> Self {
        self.algo_type = Some(value.into());
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `side` parameter.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set the provider `positionSide` parameter.
    #[must_use]
    pub fn position_side(mut self, value: impl Into<String>) -> Self {
        self.position_side = Some(value.into());
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `timeInForce` parameter.
    #[must_use]
    pub fn time_in_force(mut self, value: impl Into<String>) -> Self {
        self.time_in_force = Some(value.into());
        self
    }
    /// Set the provider `quantity` parameter.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set the provider `triggerPrice` parameter.
    #[must_use]
    pub fn trigger_price(mut self, value: Decimal) -> Self {
        self.trigger_price = Some(value);
        self
    }
    /// Set the provider `workingType` parameter.
    #[must_use]
    pub fn working_type(mut self, value: impl Into<String>) -> Self {
        self.working_type = Some(value.into());
        self
    }
    /// Set the provider `priceMatch` parameter.
    #[must_use]
    pub fn price_match(mut self, value: impl Into<String>) -> Self {
        self.price_match = Some(value.into());
        self
    }
    /// Set the provider `closePosition` parameter.
    #[must_use]
    pub fn close_position(mut self, value: impl Into<String>) -> Self {
        self.close_position = Some(value.into());
        self
    }
    /// Set the provider `priceProtect` parameter.
    #[must_use]
    pub fn price_protect(mut self, value: impl Into<String>) -> Self {
        self.price_protect = Some(value.into());
        self
    }
    /// Set the provider `reduceOnly` parameter.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
        self
    }
    /// Set the provider `activatePrice` parameter.
    #[must_use]
    pub fn activate_price(mut self, value: Decimal) -> Self {
        self.activate_price = Some(value);
        self
    }
    /// Set the provider `callbackRate` parameter.
    #[must_use]
    pub fn callback_rate(mut self, value: Decimal) -> Self {
        self.callback_rate = Some(value);
        self
    }
    /// Set the provider `clientAlgoId` parameter.
    #[must_use]
    pub fn client_algo_id(mut self, value: ClientOrderId) -> Self {
        self.client_algo_id = Some(value);
        self
    }
    /// Set the provider `newOrderRespType` parameter.
    #[must_use]
    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `goodTillDate` parameter.
    #[must_use]
    pub fn good_till_date(mut self, value: i64) -> Self {
        self.good_till_date = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for NewAlgoOrder {
    type Response = super::rest_models::NewAlgoOrderResponse;
    const OP: Operation = Operation {
        name: "newAlgoOrder",
        path: "/fapi/v1/algoOrder",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["algoType", "clientAlgoId", "side", "symbol", "type"],
            &[
                ("algoType", &["CONDITIONAL"]),
                ("side", &["BUY", "SELL"]),
                (
                    "type",
                    &[
                        "LIMIT",
                        "MARKET",
                        "STOP",
                        "STOP_MARKET",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_MARKET",
                        "TRAILING_STOP_MARKET",
                    ],
                ),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX", "GTD", "RPI"]),
                ("workingType", &["MARK_PRICE", "CONTRACT_PRICE"]),
                (
                    "priceMatch",
                    &[
                        "OPPONENT",
                        "OPPONENT_5",
                        "OPPONENT_10",
                        "OPPONENT_20",
                        "QUEUE",
                        "QUEUE_5",
                        "QUEUE_10",
                        "QUEUE_20",
                    ],
                ),
                ("closePosition", &["true", "false"]),
                ("priceProtect", &["true", "false"]),
                ("reduceOnly", &["true", "false"]),
                ("newOrderRespType", &["ACK", "RESULT"]),
                (
                    "selfTradePreventionMode",
                    &["NONE", "EXPIRE_TAKER", "EXPIRE_BOTH", "EXPIRE_MAKER"],
                ),
            ],
            &[],
        )?;
        super::validation::validate("newAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAlgoOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-algo-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAlgoOrder {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "clientAlgoId", skip_serializing_if = "Option::is_none")]
    client_algo_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryAlgoOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `algoId` parameter.
    #[must_use]
    pub fn algo_id(mut self, value: i64) -> Self {
        self.algo_id = Some(value);
        self
    }
    /// Set the provider `clientAlgoId` parameter.
    #[must_use]
    pub fn client_algo_id(mut self, value: ClientOrderId) -> Self {
        self.client_algo_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QueryAlgoOrder {
    type Response = super::rest_models::QueryAlgoOrderResponse;
    const OP: Operation = Operation {
        name: "queryAlgoOrder",
        path: "/fapi/v1/algoOrder",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("queryAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllAlgoOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-all-algo-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllAlgoOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllAlgoOpenOrders {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CancelAllAlgoOpenOrders {
    type Response = super::rest_models::CancelAllAlgoOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "cancelAllAlgoOpenOrders",
        path: "/fapi/v1/algoOpenOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelAllAlgoOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-all-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllOpenOrders {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CancelAllOpenOrders {
    type Response = super::rest_models::CancelAllOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "cancelAllOpenOrders",
        path: "/fapi/v1/allOpenOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelMultipleOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-multiple-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelMultipleOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderIdList", skip_serializing_if = "Option::is_none")]
    order_id_list: Option<Vec<i64>>,
    #[serde(
        rename = "origClientOrderIdList",
        skip_serializing_if = "Option::is_none"
    )]
    orig_client_order_id_list: Option<Vec<String>>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelMultipleOrders {
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
    /// Set the provider `orderIdList` parameter.
    #[must_use]
    pub fn order_id_list(mut self, value: Vec<i64>) -> Self {
        self.order_id_list = Some(value);
        self
    }
    /// Set the provider `origClientOrderIdList` parameter.
    #[must_use]
    pub fn orig_client_order_id_list(mut self, value: Vec<String>) -> Self {
        self.orig_client_order_id_list = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CancelMultipleOrders {
    type Response = super::rest_models::CancelMultipleOrdersResponse;
    const OP: Operation = Operation {
        name: "cancelMultipleOrders",
        path: "/fapi/v1/batchOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelMultipleOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyMultipleOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-multiple-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ModifyMultipleOrders {
    #[serde(rename = "batchOrders", skip_serializing_if = "Option::is_none")]
    batch_orders: Option<Vec<ModifyMultipleOrdersBatchOrdersInputItem>>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ModifyMultipleOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `batchOrders` parameter.
    #[must_use]
    pub fn batch_orders(mut self, value: Vec<ModifyMultipleOrdersBatchOrdersInputItem>) -> Self {
        self.batch_orders = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ModifyMultipleOrders {
    type Response = super::rest_models::ModifyMultipleOrdersResponse;
    const OP: Operation = Operation {
        name: "modifyMultipleOrders",
        path: "/fapi/v1/batchOrders",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["batchOrders"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("modifyMultipleOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`placeMultipleOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#place-multiple-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaceMultipleOrders {
    #[serde(rename = "batchOrders", skip_serializing_if = "Option::is_none")]
    batch_orders: Option<Vec<PlaceMultipleOrdersBatchOrdersInputItem>>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PlaceMultipleOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `batchOrders` parameter.
    #[must_use]
    pub fn batch_orders(mut self, value: Vec<PlaceMultipleOrdersBatchOrdersInputItem>) -> Self {
        self.batch_orders = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for PlaceMultipleOrders {
    type Response = super::rest_models::PlaceMultipleOrdersResponse;
    const OP: Operation = Operation {
        name: "placeMultipleOrders",
        path: "/fapi/v1/batchOrders",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["batchOrders"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("placeMultipleOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelOrder {
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
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.orig_client_order_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CancelOrder {
    type Response = super::rest_models::CancelOrderResponse;
    const OP: Operation = Operation {
        name: "cancelOrder",
        path: "/fapi/v1/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ModifyOrder {
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
    #[serde(rename = "modifyId", skip_serializing_if = "Option::is_none")]
    modify_id: Option<i64>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ModifyOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.orig_client_order_id = Some(value);
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `side` parameter.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set the provider `quantity` parameter.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set the provider `priceMatch` parameter.
    #[must_use]
    pub fn price_match(mut self, value: impl Into<String>) -> Self {
        self.price_match = Some(value.into());
        self
    }
    /// Set the provider `modifyId` parameter.
    #[must_use]
    pub fn modify_id(mut self, value: i64) -> Self {
        self.modify_id = Some(value);
        self
    }
    /// Set the provider `reduceOnly` parameter.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ModifyOrder {
    type Response = super::rest_models::ModifyOrderResponse;
    const OP: Operation = Operation {
        name: "modifyOrder",
        path: "/fapi/v1/order",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["price", "quantity", "side", "symbol"],
            &[
                ("side", &["BUY", "SELL"]),
                (
                    "priceMatch",
                    &[
                        "OPPONENT",
                        "OPPONENT_5",
                        "OPPONENT_10",
                        "OPPONENT_20",
                        "QUEUE",
                        "QUEUE_5",
                        "QUEUE_10",
                        "QUEUE_20",
                    ],
                ),
                ("reduceOnly", &["true", "false"]),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("modifyOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewOrder {
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
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
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
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NewOrder {
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
    /// Set the provider `side` parameter.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set the provider `positionSide` parameter.
    #[must_use]
    pub fn position_side(mut self, value: impl Into<String>) -> Self {
        self.position_side = Some(value.into());
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `timeInForce` parameter.
    #[must_use]
    pub fn time_in_force(mut self, value: impl Into<String>) -> Self {
        self.time_in_force = Some(value.into());
        self
    }
    /// Set the provider `reduceOnly` parameter.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
        self
    }
    /// Set the provider `quantity` parameter.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
        self
    }
    /// Set the provider `newOrderRespType` parameter.
    #[must_use]
    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }
    /// Set the provider `priceMatch` parameter.
    #[must_use]
    pub fn price_match(mut self, value: impl Into<String>) -> Self {
        self.price_match = Some(value.into());
        self
    }
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `goodTillDate` parameter.
    #[must_use]
    pub fn good_till_date(mut self, value: i64) -> Self {
        self.good_till_date = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for NewOrder {
    type Response = super::rest_models::NewOrderResponse;
    const OP: Operation = Operation {
        name: "newOrder",
        path: "/fapi/v1/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["newClientOrderId", "side", "symbol", "type"],
            &[
                ("side", &["BUY", "SELL"]),
                (
                    "type",
                    &[
                        "LIMIT",
                        "MARKET",
                        "STOP",
                        "STOP_MARKET",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_MARKET",
                        "TRAILING_STOP_MARKET",
                    ],
                ),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX", "GTD", "RPI"]),
                ("reduceOnly", &["true", "false"]),
                ("newOrderRespType", &["ACK", "RESULT"]),
                (
                    "priceMatch",
                    &[
                        "OPPONENT",
                        "OPPONENT_5",
                        "OPPONENT_10",
                        "OPPONENT_20",
                        "QUEUE",
                        "QUEUE_5",
                        "QUEUE_10",
                        "QUEUE_20",
                    ],
                ),
                (
                    "selfTradePreventionMode",
                    &["EXPIRE_TAKER", "EXPIRE_BOTH", "EXPIRE_MAKER"],
                ),
            ],
            &[],
        )?;
        super::validation::validate("newOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryOrder {
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
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.orig_client_order_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QueryOrder {
    type Response = super::rest_models::QueryOrderResponse;
    const OP: Operation = Operation {
        name: "queryOrder",
        path: "/fapi/v1/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeInitialLeverage`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-initial-leverage).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeInitialLeverage {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    leverage: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeInitialLeverage {
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
    /// Set the provider `leverage` parameter.
    #[must_use]
    pub fn leverage(mut self, value: i64) -> Self {
        self.leverage = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ChangeInitialLeverage {
    type Response = super::rest_models::ChangeInitialLeverageResponse;
    const OP: Operation = Operation {
        name: "changeInitialLeverage",
        path: "/fapi/v1/leverage",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["leverage", "symbol"],
            &[],
            &[
                ("leverage", 1, 125),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("changeInitialLeverage", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeMarginType`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-margin-type).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeMarginType {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "marginType", skip_serializing_if = "Option::is_none")]
    margin_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeMarginType {
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
    /// Set the provider `marginType` parameter.
    #[must_use]
    pub fn margin_type(mut self, value: impl Into<String>) -> Self {
        self.margin_type = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ChangeMarginType {
    type Response = super::rest_models::ChangeMarginTypeResponse;
    const OP: Operation = Operation {
        name: "changeMarginType",
        path: "/fapi/v1/marginType",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["marginType", "symbol"],
            &[("marginType", &["ISOLATED", "CROSSED"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("changeMarginType", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`currentAllAlgoOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-algo-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CurrentAllAlgoOpenOrders {
    #[serde(rename = "algoType", skip_serializing_if = "Option::is_none")]
    algo_type: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CurrentAllAlgoOpenOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `algoType` parameter.
    #[must_use]
    pub fn algo_type(mut self, value: impl Into<String>) -> Self {
        self.algo_type = Some(value.into());
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `algoId` parameter.
    #[must_use]
    pub fn algo_id(mut self, value: i64) -> Self {
        self.algo_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CurrentAllAlgoOpenOrders {
    type Response = super::rest_models::CurrentAllAlgoOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "currentAllAlgoOpenOrders",
        path: "/fapi/v1/openAlgoOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        requests_per_second: None,
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
        super::validation::validate("currentAllAlgoOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`currentAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CurrentAllOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CurrentAllOpenOrders {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CurrentAllOpenOrders {
    type Response = super::rest_models::CurrentAllOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "currentAllOpenOrders",
        path: "/fapi/v1/openOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        requests_per_second: None,
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
        super::validation::validate("currentAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresTradfiPerpsContract`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#futures-tradfi-perps-contract).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FuturesTradfiPerpsContract {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FuturesTradfiPerpsContract {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for FuturesTradfiPerpsContract {
    type Response = super::rest_models::FuturesTradfiPerpsContractResponse;
    const OP: Operation = Operation {
        name: "futuresTradfiPerpsContract",
        path: "/fapi/v1/stock/contract",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 50,
        requests_per_second: None,
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
        super::validation::validate("futuresTradfiPerpsContract", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getOrderModifyHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#get-order-modify-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetOrderModifyHistory {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetOrderModifyHistory {
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
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.orig_client_order_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetOrderModifyHistory {
    type Response = super::rest_models::GetOrderModifyHistoryResponse;
    const OP: Operation = Operation {
        name: "getOrderModifyHistory",
        path: "/fapi/v1/orderAmendment",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[
                ("limit", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("getOrderModifyHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getPositionMarginChangeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#get-position-margin-change-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetPositionMarginChangeHistory {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
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
impl GetPositionMarginChangeHistory {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for GetPositionMarginChangeHistory {
    type Response = super::rest_models::GetPositionMarginChangeHistoryResponse;
    const OP: Operation = Operation {
        name: "getPositionMarginChangeHistory",
        path: "/fapi/v1/positionMargin/history",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getPositionMarginChangeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyIsolatedPositionMargin`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-isolated-position-margin).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ModifyIsolatedPositionMargin {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    position_side: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ModifyIsolatedPositionMargin {
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
    /// Set the provider `positionSide` parameter.
    #[must_use]
    pub fn position_side(mut self, value: impl Into<String>) -> Self {
        self.position_side = Some(value.into());
        self
    }
    /// Set the provider `amount` parameter.
    #[must_use]
    pub fn amount(mut self, value: Decimal) -> Self {
        self.amount = Some(value);
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: i64) -> Self {
        self.type_value = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for ModifyIsolatedPositionMargin {
    type Response = super::rest_models::ModifyIsolatedPositionMarginResponse;
    const OP: Operation = Operation {
        name: "modifyIsolatedPositionMargin",
        path: "/fapi/v1/positionMargin",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["amount", "symbol", "type"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("modifyIsolatedPositionMargin", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`positionAdlQuantileEstimation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-adl-quantile-estimation).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PositionAdlQuantileEstimation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PositionAdlQuantileEstimation {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for PositionAdlQuantileEstimation {
    type Response = super::rest_models::PositionAdlQuantileEstimationResponse;
    const OP: Operation = Operation {
        name: "positionAdlQuantileEstimation",
        path: "/fapi/v1/adlQuantile",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("positionAdlQuantileEstimation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`positionInformationV2`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-information-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PositionInformationV2 {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PositionInformationV2 {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for PositionInformationV2 {
    type Response = super::rest_models::PositionInformationV2Response;
    const OP: Operation = Operation {
        name: "positionInformationV2",
        path: "/fapi/v2/positionRisk",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("positionInformationV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`positionInformationV3`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-information-v3).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PositionInformationV3 {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PositionInformationV3 {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for PositionInformationV3 {
    type Response = super::rest_models::PositionInformationV3Response;
    const OP: Operation = Operation {
        name: "positionInformationV3",
        path: "/fapi/v3/positionRisk",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
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
        super::validation::validate("positionInformationV3", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllAlgoOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-all-algo-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllAlgoOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryAllAlgoOrders {
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
    /// Set the provider `algoId` parameter.
    #[must_use]
    pub fn algo_id(mut self, value: i64) -> Self {
        self.algo_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QueryAllAlgoOrders {
    type Response = super::rest_models::QueryAllAlgoOrdersResponse;
    const OP: Operation = Operation {
        name: "queryAllAlgoOrders",
        path: "/fapi/v1/allAlgoOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("queryAllAlgoOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentOpenOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-current-open-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentOpenOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCurrentOpenOrder {
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
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.orig_client_order_id = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for QueryCurrentOpenOrder {
    type Response = super::rest_models::QueryCurrentOpenOrderResponse;
    const OP: Operation = Operation {
        name: "queryCurrentOpenOrder",
        path: "/fapi/v1/openOrder",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryCurrentOpenOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`testOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#test-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TestOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    position_side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "closePosition", skip_serializing_if = "Option::is_none")]
    close_position: Option<String>,
    #[serde(rename = "activationPrice", skip_serializing_if = "Option::is_none")]
    activation_price: Option<Decimal>,
    #[serde(rename = "callbackRate", skip_serializing_if = "Option::is_none")]
    callback_rate: Option<Decimal>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    working_type: Option<String>,
    #[serde(rename = "priceProtect", skip_serializing_if = "Option::is_none")]
    price_protect: Option<String>,
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
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl TestOrder {
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
    /// Set the provider `side` parameter.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set the provider `positionSide` parameter.
    #[must_use]
    pub fn position_side(mut self, value: impl Into<String>) -> Self {
        self.position_side = Some(value.into());
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `reduceOnly` parameter.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
        self
    }
    /// Set the provider `quantity` parameter.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
        self
    }
    /// Set the provider `stopPrice` parameter.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }
    /// Set the provider `closePosition` parameter.
    #[must_use]
    pub fn close_position(mut self, value: impl Into<String>) -> Self {
        self.close_position = Some(value.into());
        self
    }
    /// Set the provider `activationPrice` parameter.
    #[must_use]
    pub fn activation_price(mut self, value: Decimal) -> Self {
        self.activation_price = Some(value);
        self
    }
    /// Set the provider `callbackRate` parameter.
    #[must_use]
    pub fn callback_rate(mut self, value: Decimal) -> Self {
        self.callback_rate = Some(value);
        self
    }
    /// Set the provider `timeInForce` parameter.
    #[must_use]
    pub fn time_in_force(mut self, value: impl Into<String>) -> Self {
        self.time_in_force = Some(value.into());
        self
    }
    /// Set the provider `workingType` parameter.
    #[must_use]
    pub fn working_type(mut self, value: impl Into<String>) -> Self {
        self.working_type = Some(value.into());
        self
    }
    /// Set the provider `priceProtect` parameter.
    #[must_use]
    pub fn price_protect(mut self, value: impl Into<String>) -> Self {
        self.price_protect = Some(value.into());
        self
    }
    /// Set the provider `newOrderRespType` parameter.
    #[must_use]
    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }
    /// Set the provider `priceMatch` parameter.
    #[must_use]
    pub fn price_match(mut self, value: impl Into<String>) -> Self {
        self.price_match = Some(value.into());
        self
    }
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `goodTillDate` parameter.
    #[must_use]
    pub fn good_till_date(mut self, value: i64) -> Self {
        self.good_till_date = Some(value);
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for TestOrder {
    type Response = super::rest_models::TestOrderResponse;
    const OP: Operation = Operation {
        name: "testOrder",
        path: "/fapi/v1/order/test",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["newClientOrderId", "side", "symbol", "type"],
            &[
                ("side", &["BUY", "SELL"]),
                ("positionSide", &["BOTH", "LONG", "SHORT"]),
                (
                    "type",
                    &[
                        "LIMIT",
                        "MARKET",
                        "STOP",
                        "STOP_MARKET",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_MARKET",
                        "TRAILING_STOP_MARKET",
                    ],
                ),
                ("reduceOnly", &["true", "false"]),
                ("closePosition", &["true", "false"]),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX", "GTD", "RPI"]),
                ("workingType", &["MARK_PRICE", "CONTRACT_PRICE"]),
                ("priceProtect", &["true", "false"]),
                ("newOrderRespType", &["ACK", "RESULT"]),
                (
                    "priceMatch",
                    &[
                        "OPPONENT",
                        "OPPONENT_5",
                        "OPPONENT_10",
                        "OPPONENT_20",
                        "QUEUE",
                        "QUEUE_5",
                        "QUEUE_10",
                        "QUEUE_20",
                    ],
                ),
                (
                    "selfTradePreventionMode",
                    &["NONE", "EXPIRE_TAKER", "EXPIRE_BOTH", "EXPIRE_MAKER"],
                ),
            ],
            &[],
        )?;
        super::validation::validate("testOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`usersForceOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#users-force-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UsersForceOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "autoCloseType", skip_serializing_if = "Option::is_none")]
    auto_close_type: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UsersForceOrders {
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
    /// Set the provider `autoCloseType` parameter.
    #[must_use]
    pub fn auto_close_type(mut self, value: impl Into<String>) -> Self {
        self.auto_close_type = Some(value.into());
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for UsersForceOrders {
    type Response = super::rest_models::UsersForceOrdersResponse;
    const OP: Operation = Operation {
        name: "usersForceOrders",
        path: "/fapi/v1/forceOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        requests_per_second: None,
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
            &[("autoCloseType", &["LIQUIDATION", "ADL"])],
            &[("limit", -9_223_372_036_854_775_808, 100)],
        )?;
        super::validation::validate("usersForceOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`closeUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#close-user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CloseUserDataStream {}
impl CloseUserDataStream {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for CloseUserDataStream {
    type Response = super::rest_models::CloseUserDataStreamResponse;
    const OP: Operation = Operation {
        name: "closeUserDataStream",
        path: "/fapi/v1/listenKey",
        method: "DELETE",
        security: Security::Key,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("closeUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`keepaliveUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#keepalive-user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct KeepaliveUserDataStream {}
impl KeepaliveUserDataStream {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for KeepaliveUserDataStream {
    type Response = super::rest_models::KeepaliveUserDataStreamResponse;
    const OP: Operation = Operation {
        name: "keepaliveUserDataStream",
        path: "/fapi/v1/listenKey",
        method: "PUT",
        security: Security::Key,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("keepaliveUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`startUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#start-user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct StartUserDataStream {}
impl StartUserDataStream {
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
    /// The weight admission charges this request against its pool's minute weight
    /// window, as the venue documents it for these parameters; read before sending.
    ///
    /// # Errors
    /// Refuses a request dispatch would refuse before admission.
    pub fn weight(&self) -> Result<u64, Error> {
        self.validate()?;
        Ok(self.cost()?.request_weight())
    }
}
impl Request for StartUserDataStream {
    type Response = super::rest_models::StartUserDataStreamResponse;
    const OP: Operation = Operation {
        name: "startUserDataStream",
        path: "/fapi/v1/listenKey",
        method: "POST",
        security: Security::Key,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("startUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [accountInformationV2](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#account-information-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_information_v2(
        &self,
        request: &AccountInformationV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountInformationV2Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [accountInformationV3](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#account-information-v3).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_information_v3(
        &self,
        request: &AccountInformationV3,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountInformationV3Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [futuresAccountBalanceV2](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-balance-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_account_balance_v2(
        &self,
        request: &FuturesAccountBalanceV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FuturesAccountBalanceV2Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [futuresAccountBalanceV3](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-balance-v3).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_account_balance_v3(
        &self,
        request: &FuturesAccountBalanceV3,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FuturesAccountBalanceV3Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [futuresAccountConfiguration](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-account-configuration).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_account_configuration(
        &self,
        request: &FuturesAccountConfiguration,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FuturesAccountConfigurationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [futuresTradingQuantitativeRulesIndicators](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#futures-trading-quantitative-rules-indicators).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_trading_quantitative_rules_indicators(
        &self,
        request: &FuturesTradingQuantitativeRulesIndicators,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::FuturesTradingQuantitativeRulesIndicatorsResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getBnbBurnStatus](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-bnb-burn-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_bnb_burn_status(
        &self,
        request: &GetBnbBurnStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetBnbBurnStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [toggleBnbBurnOnFuturesTrade](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#toggle-bnb-burn-on-futures-trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn toggle_bnb_burn_on_futures_trade(
        &self,
        request: &ToggleBnbBurnOnFuturesTrade,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ToggleBnbBurnOnFuturesTradeResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getCurrentMultiAssetsMode](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-current-multi-assets-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_current_multi_assets_mode(
        &self,
        request: &GetCurrentMultiAssetsMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetCurrentMultiAssetsModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changeMultiAssetsMode](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-multi-assets-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_multi_assets_mode(
        &self,
        request: &ChangeMultiAssetsMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeMultiAssetsModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getCurrentPositionMode](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-current-position-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_current_position_mode(
        &self,
        request: &GetCurrentPositionMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetCurrentPositionModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changePositionMode](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-position-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_position_mode(
        &self,
        request: &ChangePositionMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangePositionModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getDownloadIdForFuturesOrderHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-order-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_download_id_for_futures_order_history(
        &self,
        request: &GetDownloadIdForFuturesOrderHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetDownloadIdForFuturesOrderHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getDownloadIdForFuturesTradeHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-trade-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_download_id_for_futures_trade_history(
        &self,
        request: &GetDownloadIdForFuturesTradeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetDownloadIdForFuturesTradeHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getDownloadIdForFuturesTransactionHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-download-id-for-futures-transaction-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_download_id_for_futures_transaction_history(
        &self,
        request: &GetDownloadIdForFuturesTransactionHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetDownloadIdForFuturesTransactionHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getFuturesOrderHistoryDownloadLinkById](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-order-history-download-link-by-id).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_futures_order_history_download_link_by_id(
        &self,
        request: &GetFuturesOrderHistoryDownloadLinkById,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetFuturesOrderHistoryDownloadLinkByIdResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getFuturesTradeDownloadLinkById](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-trade-download-link-by-id).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_futures_trade_download_link_by_id(
        &self,
        request: &GetFuturesTradeDownloadLinkById,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetFuturesTradeDownloadLinkByIdResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getFuturesTransactionHistoryDownloadLinkById](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-futures-transaction-history-download-link-by-id).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_futures_transaction_history_download_link_by_id(
        &self,
        request: &GetFuturesTransactionHistoryDownloadLinkById,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetFuturesTransactionHistoryDownloadLinkByIdResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getIncomeHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#get-income-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_income_history(
        &self,
        request: &GetIncomeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetIncomeHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [notionalAndLeverageBrackets](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#notional-and-leverage-brackets).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn notional_and_leverage_brackets(
        &self,
        request: &NotionalAndLeverageBrackets,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NotionalAndLeverageBracketsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryUserRateLimit](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#query-user-rate-limit).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_user_rate_limit(
        &self,
        request: &QueryUserRateLimit,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUserRateLimitResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [symbolConfiguration](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#symbol-configuration).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn symbol_configuration(
        &self,
        request: &SymbolConfiguration,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SymbolConfigurationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [userCommissionRate](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/account#user-commission-rate).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_commission_rate(
        &self,
        request: &UserCommissionRate,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UserCommissionRateResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [acceptTheOfferedQuote](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#accept-the-offered-quote).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn accept_the_offered_quote(
        &self,
        request: &AcceptTheOfferedQuote,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AcceptTheOfferedQuoteResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [listAllConvertPairs](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#list-all-convert-pairs).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn list_all_convert_pairs(
        &self,
        request: &ListAllConvertPairs,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ListAllConvertPairsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderStatus](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#order-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_status(
        &self,
        request: &OrderStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [sendQuoteRequest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/convert#send-quote-request).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn send_quote_request(
        &self,
        request: &SendQuoteRequest,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SendQuoteRequestResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [adlRisk](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#adl-risk).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn adl_risk(
        &self,
        request: &AdlRisk,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AdlRiskResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [basis](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#basis).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn basis(
        &self,
        request: &Basis,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::BasisResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [checkServerTime](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#check-server-time).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn check_server_time(
        &self,
        request: &CheckServerTime,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CheckServerTimeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [compositeIndexSymbolInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#composite-index-symbol-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn composite_index_symbol_information(
        &self,
        request: &CompositeIndexSymbolInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CompositeIndexSymbolInformationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [compressedAggregateTradesList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#compressed-aggregate-trades-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn compressed_aggregate_trades_list(
        &self,
        request: &CompressedAggregateTradesList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CompressedAggregateTradesListResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [continuousContractKlineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#continuous-contract-kline-candlestick-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn continuous_contract_kline_candlestick_data(
        &self,
        request: &ContinuousContractKlineCandlestickData,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::ContinuousContractKlineCandlestickDataResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [exchangeInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#exchange-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn exchange_information(
        &self,
        request: &ExchangeInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ExchangeInformationResponse>, Error> {
        let response = self.inner.execute(request, deadline).await?;
        super::rate::adopt_stated_limits(&self.inner, &response.data)?;
        Ok(response)
    }

    /// [getFundingRateHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#get-funding-rate-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_funding_rate_history(
        &self,
        request: &GetFundingRateHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetFundingRateHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getFundingRateInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#get-funding-rate-info).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_funding_rate_info(
        &self,
        request: &GetFundingRateInfo,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetFundingRateInfoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [indexPriceKlineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#index-price-kline-candlestick-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn index_price_kline_candlestick_data(
        &self,
        request: &IndexPriceKlineCandlestickData,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::IndexPriceKlineCandlestickDataResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [klineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#kline-candlestick-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn kline_candlestick_data(
        &self,
        request: &KlineCandlestickData,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::KlineCandlestickDataResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [longShortRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#long-short-ratio).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn long_short_ratio(
        &self,
        request: &LongShortRatio,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::LongShortRatioResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [markPrice](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#mark-price).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn mark_price(
        &self,
        request: &MarkPrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarkPriceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [markPriceKlineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#mark-price-kline-candlestick-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn mark_price_kline_candlestick_data(
        &self,
        request: &MarkPriceKlineCandlestickData,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarkPriceKlineCandlestickDataResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [assetIndex](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#asset-index).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn asset_index(
        &self,
        request: &AssetIndex,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AssetIndexResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [oldTradesLookup](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#old-trades-lookup).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn old_trades_lookup(
        &self,
        request: &OldTradesLookup,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OldTradesLookupResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [openInterest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#open-interest).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn open_interest(
        &self,
        request: &OpenInterest,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OpenInterestResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [openInterestStatistics](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#open-interest-statistics).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn open_interest_statistics(
        &self,
        request: &OpenInterestStatistics,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OpenInterestStatisticsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderBook](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#order-book).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_book(
        &self,
        request: &OrderBook,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderBookResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [premiumIndexKlineData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#premium-index-kline-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn premium_index_kline_data(
        &self,
        request: &PremiumIndexKlineData,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PremiumIndexKlineDataResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [quarterlyContractSettlementPrice](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#quarterly-contract-settlement-price).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn quarterly_contract_settlement_price(
        &self,
        request: &QuarterlyContractSettlementPrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QuarterlyContractSettlementPriceResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryIndexPriceConstituents](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#query-index-price-constituents).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_index_price_constituents(
        &self,
        request: &QueryIndexPriceConstituents,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryIndexPriceConstituentsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryInsuranceFundBalanceSnapshot](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#query-insurance-fund-balance-snapshot).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_insurance_fund_balance_snapshot(
        &self,
        request: &QueryInsuranceFundBalanceSnapshot,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryInsuranceFundBalanceSnapshotResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [recentTradesList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#recent-trades-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn recent_trades_list(
        &self,
        request: &RecentTradesList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::RecentTradesListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [rpiOrderBook](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#rpi-order-book).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn rpi_order_book(
        &self,
        request: &RpiOrderBook,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::RpiOrderBookResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [symbolOrderBookTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-order-book-ticker).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn symbol_order_book_ticker(
        &self,
        request: &SymbolOrderBookTicker,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SymbolOrderBookTickerResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [symbolPriceTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-price-ticker).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn symbol_price_ticker(
        &self,
        request: &SymbolPriceTicker,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SymbolPriceTickerResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [symbolPriceTickerV2](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#symbol-price-ticker-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn symbol_price_ticker_v2(
        &self,
        request: &SymbolPriceTickerV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SymbolPriceTickerV2Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [takerBuySellVolume](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#taker-buy-sell-volume).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn taker_buy_sell_volume(
        &self,
        request: &TakerBuySellVolume,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TakerBuySellVolumeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [testConnectivity](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#test-connectivity).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn test_connectivity(
        &self,
        request: &TestConnectivity,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TestConnectivityResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [ticker24hrPriceChangeStatistics](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#ticker24hr-price-change-statistics).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker24hr_price_change_statistics(
        &self,
        request: &Ticker24hrPriceChangeStatistics,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::Ticker24hrPriceChangeStatisticsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [topTraderLongShortRatioAccounts](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-accounts).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn top_trader_long_short_ratio_accounts(
        &self,
        request: &TopTraderLongShortRatioAccounts,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TopTraderLongShortRatioAccountsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [topTraderLongShortRatioPositions](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-positions).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn top_trader_long_short_ratio_positions(
        &self,
        request: &TopTraderLongShortRatioPositions,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TopTraderLongShortRatioPositionsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [tradingSchedule](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/market-data#trading-schedule).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn trading_schedule(
        &self,
        request: &TradingSchedule,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TradingScheduleResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [classicPortfolioMarginAccountInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/portfolio-margin-endpoints#classic-portfolio-margin-account-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn classic_portfolio_margin_account_information(
        &self,
        request: &ClassicPortfolioMarginAccountInformation,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::ClassicPortfolioMarginAccountInformationResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [accountTradeList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#account-trade-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_trade_list(
        &self,
        request: &AccountTradeList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountTradeListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [allOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#all-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn all_orders(
        &self,
        request: &AllOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AllOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [autoCancelAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#auto-cancel-all-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn auto_cancel_all_open_orders(
        &self,
        request: &AutoCancelAllOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AutoCancelAllOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelAlgoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_algo_order(
        &self,
        request: &CancelAlgoOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAlgoOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newAlgoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_algo_order(
        &self,
        request: &NewAlgoOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewAlgoOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryAlgoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_algo_order(
        &self,
        request: &QueryAlgoOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAlgoOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelAllAlgoOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-all-algo-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_algo_open_orders(
        &self,
        request: &CancelAllAlgoOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAllAlgoOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-all-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_open_orders(
        &self,
        request: &CancelAllOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAllOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelMultipleOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-multiple-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_multiple_orders(
        &self,
        request: &CancelMultipleOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelMultipleOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [modifyMultipleOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-multiple-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn modify_multiple_orders(
        &self,
        request: &ModifyMultipleOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ModifyMultipleOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [placeMultipleOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#place-multiple-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn place_multiple_orders(
        &self,
        request: &PlaceMultipleOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PlaceMultipleOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#cancel-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_order(
        &self,
        request: &CancelOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [modifyOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn modify_order(
        &self,
        request: &ModifyOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ModifyOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_order(
        &self,
        request: &NewOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_order(
        &self,
        request: &QueryOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changeInitialLeverage](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-initial-leverage).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_initial_leverage(
        &self,
        request: &ChangeInitialLeverage,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeInitialLeverageResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changeMarginType](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#change-margin-type).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_margin_type(
        &self,
        request: &ChangeMarginType,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeMarginTypeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [currentAllAlgoOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-algo-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn current_all_algo_open_orders(
        &self,
        request: &CurrentAllAlgoOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CurrentAllAlgoOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [currentAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn current_all_open_orders(
        &self,
        request: &CurrentAllOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CurrentAllOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [futuresTradfiPerpsContract](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#futures-tradfi-perps-contract).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_tradfi_perps_contract(
        &self,
        request: &FuturesTradfiPerpsContract,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FuturesTradfiPerpsContractResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getOrderModifyHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#get-order-modify-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_order_modify_history(
        &self,
        request: &GetOrderModifyHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetOrderModifyHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getPositionMarginChangeHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#get-position-margin-change-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_position_margin_change_history(
        &self,
        request: &GetPositionMarginChangeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetPositionMarginChangeHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [modifyIsolatedPositionMargin](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#modify-isolated-position-margin).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn modify_isolated_position_margin(
        &self,
        request: &ModifyIsolatedPositionMargin,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ModifyIsolatedPositionMarginResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [positionAdlQuantileEstimation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-adl-quantile-estimation).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn position_adl_quantile_estimation(
        &self,
        request: &PositionAdlQuantileEstimation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PositionAdlQuantileEstimationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [positionInformationV2](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-information-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn position_information_v2(
        &self,
        request: &PositionInformationV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PositionInformationV2Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [positionInformationV3](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-information-v3).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn position_information_v3(
        &self,
        request: &PositionInformationV3,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PositionInformationV3Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllAlgoOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-all-algo-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_algo_orders(
        &self,
        request: &QueryAllAlgoOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllAlgoOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentOpenOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#query-current-open-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_open_order(
        &self,
        request: &QueryCurrentOpenOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCurrentOpenOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [testOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#test-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn test_order(
        &self,
        request: &TestOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TestOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [usersForceOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#users-force-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn users_force_orders(
        &self,
        request: &UsersForceOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UsersForceOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [closeUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#close-user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn close_user_data_stream(
        &self,
        request: &CloseUserDataStream,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CloseUserDataStreamResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [keepaliveUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#keepalive-user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn keepalive_user_data_stream(
        &self,
        request: &KeepaliveUserDataStream,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::KeepaliveUserDataStreamResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [startUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/user-data-streams#start-user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn start_user_data_stream(
        &self,
        request: &StartUserDataStream,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::StartUserDataStreamResponse>, Error> {
        self.inner.execute(request, deadline).await
    }
}
