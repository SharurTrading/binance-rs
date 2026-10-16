// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use crate::ClientOrderId;
use crate::Decimal;
use crate::Error;
use crate::Symbol;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`accountBalance`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#account-balance).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountBalance {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountBalance {
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
impl Request for AccountBalance {
    type Response = super::rest_models::AccountBalanceResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "accountBalance",
        path: "/papi/v1/balance",
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
        super::validation::validate("accountBalance", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountInformation`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#account-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountInformation {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountInformation {
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
impl Request for AccountInformation {
    type Response = super::rest_models::AccountInformationResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "accountInformation",
        path: "/papi/v1/account",
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
        super::validation::validate("accountInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`bnbTransfer`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#bnb-transfer).
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
        path: "/papi/v1/bnb-transfer",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 750,
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("bnbTransfer", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeAutoRepayFuturesStatus`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-auto-repay-futures-status).
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
        path: "/papi/v1/repay-futures-switch",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 750,
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

/// Validated request builder for [`getAutoRepayFuturesStatus`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-auto-repay-futures-status).
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
        path: "/papi/v1/repay-futures-switch",
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

/// Validated request builder for [`changeCmInitialLeverage`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-cm-initial-leverage).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeCmInitialLeverage {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    leverage: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeCmInitialLeverage {
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
}
impl Request for ChangeCmInitialLeverage {
    type Response = super::rest_models::ChangeCmInitialLeverageResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "changeCmInitialLeverage",
        path: "/papi/v1/cm/leverage",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["leverage", "symbol"], &[], &[("leverage", 1, 125)])?;
        super::validation::validate("changeCmInitialLeverage", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeCmPositionMode`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-cm-position-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeCmPositionMode {
    #[serde(rename = "dualSidePosition", skip_serializing_if = "Option::is_none")]
    dual_side_position: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeCmPositionMode {
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
}
impl Request for ChangeCmPositionMode {
    type Response = super::rest_models::ChangeCmPositionModeResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "changeCmPositionMode",
        path: "/papi/v1/cm/positionSide/dual",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["dualSidePosition"],
            &[("dualSidePosition", &["true", "false"])],
            &[],
        )?;
        super::validation::validate("changeCmPositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCmCurrentPositionMode`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-current-position-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCmCurrentPositionMode {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetCmCurrentPositionMode {
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
impl Request for GetCmCurrentPositionMode {
    type Response = super::rest_models::GetCmCurrentPositionModeResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getCmCurrentPositionMode",
        path: "/papi/v1/cm/positionSide/dual",
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
        super::validation::validate("getCmCurrentPositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeUmInitialLeverage`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-um-initial-leverage).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeUmInitialLeverage {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    leverage: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeUmInitialLeverage {
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
}
impl Request for ChangeUmInitialLeverage {
    type Response = super::rest_models::ChangeUmInitialLeverageResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "changeUmInitialLeverage",
        path: "/papi/v1/um/leverage",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["leverage", "symbol"],
            &[],
            &[("leverage", 1, 9_223_372_036_854_775_807)],
        )?;
        super::validation::validate("changeUmInitialLeverage", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeUmPositionMode`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-um-position-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeUmPositionMode {
    #[serde(rename = "dualSidePosition", skip_serializing_if = "Option::is_none")]
    dual_side_position: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ChangeUmPositionMode {
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
}
impl Request for ChangeUmPositionMode {
    type Response = super::rest_models::ChangeUmPositionModeResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "changeUmPositionMode",
        path: "/papi/v1/um/positionSide/dual",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["dualSidePosition"],
            &[("dualSidePosition", &["true", "false"])],
            &[],
        )?;
        super::validation::validate("changeUmPositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmCurrentPositionMode`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-current-position-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmCurrentPositionMode {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUmCurrentPositionMode {
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
impl Request for GetUmCurrentPositionMode {
    type Response = super::rest_models::GetUmCurrentPositionModeResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmCurrentPositionMode",
        path: "/papi/v1/um/positionSide/dual",
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
        super::validation::validate("getUmCurrentPositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cmNotionalAndLeverageBrackets`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#cm-notional-and-leverage-brackets).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CmNotionalAndLeverageBrackets {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CmNotionalAndLeverageBrackets {
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
impl Request for CmNotionalAndLeverageBrackets {
    type Response = super::rest_models::CmNotionalAndLeverageBracketsResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cmNotionalAndLeverageBrackets",
        path: "/papi/v1/cm/leverageBracket",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
        super::validation::validate("cmNotionalAndLeverageBrackets", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fundAutoCollection`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#fund-auto-collection).
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
        path: "/papi/v1/auto-collection",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 750,
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
        super::validation::validate("fundAutoCollection", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fundCollectionByAsset`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#fund-collection-by-asset).
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
        path: "/papi/v1/asset-collection",
        method: "POST",
        security: Security::Signed,
        mutation: true,
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

/// Validated request builder for [`getCmAccountDetail`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-account-detail).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCmAccountDetail {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetCmAccountDetail {
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
impl Request for GetCmAccountDetail {
    type Response = super::rest_models::GetCmAccountDetailResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getCmAccountDetail",
        path: "/papi/v1/cm/account",
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
        super::validation::validate("getCmAccountDetail", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCmIncomeHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-income-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCmIncomeHistory {
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
impl GetCmIncomeHistory {
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
}
impl Request for GetCmIncomeHistory {
    type Response = super::rest_models::GetCmIncomeHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getCmIncomeHistory",
        path: "/papi/v1/cm/income",
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
        validate_parameters(
            &p,
            &[],
            &[(
                "incomeType",
                &[
                    "TRANSFER",
                    "WELCOME_BONUS",
                    "FUNDING_FEE",
                    "REALIZED_PNL",
                    "COMMISSION",
                    "INSURANCE_CLEAR",
                    "DELIVERED_SETTELMENT",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("getCmIncomeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForUmFuturesOrderHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-order-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDownloadIdForUmFuturesOrderHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDownloadIdForUmFuturesOrderHistory {
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
}
impl Request for GetDownloadIdForUmFuturesOrderHistory {
    type Response = super::rest_models::GetDownloadIdForUmFuturesOrderHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getDownloadIdForUmFuturesOrderHistory",
        path: "/papi/v1/um/order/asyn",
        method: "GET",
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
        validate_parameters(&p, &["endTime", "startTime"], &[], &[])?;
        super::validation::validate("getDownloadIdForUmFuturesOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForUmFuturesTradeHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-trade-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDownloadIdForUmFuturesTradeHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDownloadIdForUmFuturesTradeHistory {
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
}
impl Request for GetDownloadIdForUmFuturesTradeHistory {
    type Response = super::rest_models::GetDownloadIdForUmFuturesTradeHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getDownloadIdForUmFuturesTradeHistory",
        path: "/papi/v1/um/trade/asyn",
        method: "GET",
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
        validate_parameters(&p, &["endTime", "startTime"], &[], &[])?;
        super::validation::validate("getDownloadIdForUmFuturesTradeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForUmFuturesTransactionHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-transaction-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDownloadIdForUmFuturesTransactionHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDownloadIdForUmFuturesTransactionHistory {
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
}
impl Request for GetDownloadIdForUmFuturesTransactionHistory {
    type Response = super::rest_models::GetDownloadIdForUmFuturesTransactionHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getDownloadIdForUmFuturesTransactionHistory",
        path: "/papi/v1/um/income/asyn",
        method: "GET",
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
        validate_parameters(&p, &["endTime", "startTime"], &[], &[])?;
        super::validation::validate("getDownloadIdForUmFuturesTransactionHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getMarginBorrowLoanInterestHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-margin-borrow-loan-interest-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetMarginBorrowLoanInterestHistory {
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
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    archived: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetMarginBorrowLoanInterestHistory {
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
    /// Set the provider `archived` parameter.
    #[must_use]
    pub fn archived(mut self, value: impl Into<String>) -> Self {
        self.archived = Some(value.into());
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
impl Request for GetMarginBorrowLoanInterestHistory {
    type Response = super::rest_models::GetMarginBorrowLoanInterestHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getMarginBorrowLoanInterestHistory",
        path: "/papi/v1/margin/marginInterestHistory",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
            &[("archived", &["true", "false"])],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
            ],
        )?;
        super::validation::validate("getMarginBorrowLoanInterestHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmAccountDetail`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-account-detail).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmAccountDetail {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUmAccountDetail {
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
impl Request for GetUmAccountDetail {
    type Response = super::rest_models::GetUmAccountDetailResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmAccountDetail",
        path: "/papi/v1/um/account",
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
        super::validation::validate("getUmAccountDetail", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmAccountDetailV2`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-account-detail-v2).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmAccountDetailV2 {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUmAccountDetailV2 {
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
impl Request for GetUmAccountDetailV2 {
    type Response = super::rest_models::GetUmAccountDetailV2Response;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmAccountDetailV2",
        path: "/papi/v2/um/account",
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
        super::validation::validate("getUmAccountDetailV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmFuturesOrderDownloadLinkById`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-order-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmFuturesOrderDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUmFuturesOrderDownloadLinkById {
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
}
impl Request for GetUmFuturesOrderDownloadLinkById {
    type Response = super::rest_models::GetUmFuturesOrderDownloadLinkByIdResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmFuturesOrderDownloadLinkById",
        path: "/papi/v1/um/order/asyn/id",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["downloadId"], &[], &[])?;
        super::validation::validate("getUmFuturesOrderDownloadLinkById", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmFuturesTradeDownloadLinkById`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-trade-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmFuturesTradeDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUmFuturesTradeDownloadLinkById {
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
}
impl Request for GetUmFuturesTradeDownloadLinkById {
    type Response = super::rest_models::GetUmFuturesTradeDownloadLinkByIdResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmFuturesTradeDownloadLinkById",
        path: "/papi/v1/um/trade/asyn/id",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["downloadId"], &[], &[])?;
        super::validation::validate("getUmFuturesTradeDownloadLinkById", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmFuturesTransactionDownloadLinkById`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-transaction-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmFuturesTransactionDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUmFuturesTransactionDownloadLinkById {
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
}
impl Request for GetUmFuturesTransactionDownloadLinkById {
    type Response = super::rest_models::GetUmFuturesTransactionDownloadLinkByIdResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmFuturesTransactionDownloadLinkById",
        path: "/papi/v1/um/income/asyn/id",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["downloadId"], &[], &[])?;
        super::validation::validate("getUmFuturesTransactionDownloadLinkById", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmIncomeHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-income-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmIncomeHistory {
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
impl GetUmIncomeHistory {
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
}
impl Request for GetUmIncomeHistory {
    type Response = super::rest_models::GetUmIncomeHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmIncomeHistory",
        path: "/papi/v1/um/income",
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
                    "DELIVERED_SETTELMENT",
                    "COIN_SWAP_DEPOSIT",
                    "COIN_SWAP_WITHDRAW",
                    "POSITION_LIMIT_INCREASE_FEE",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("getUmIncomeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUserCommissionRateForCm`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-user-commission-rate-for-cm).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUserCommissionRateForCm {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUserCommissionRateForCm {
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
impl Request for GetUserCommissionRateForCm {
    type Response = super::rest_models::GetUserCommissionRateForCmResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUserCommissionRateForCm",
        path: "/papi/v1/cm/commissionRate",
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
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("getUserCommissionRateForCm", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUserCommissionRateForUm`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-user-commission-rate-for-um).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUserCommissionRateForUm {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUserCommissionRateForUm {
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
impl Request for GetUserCommissionRateForUm {
    type Response = super::rest_models::GetUserCommissionRateForUmResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUserCommissionRateForUm",
        path: "/papi/v1/um/commissionRate",
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
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("getUserCommissionRateForUm", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginMaxBorrow`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#margin-max-borrow).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginMaxBorrow {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginMaxBorrow {
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
impl Request for MarginMaxBorrow {
    type Response = super::rest_models::MarginMaxBorrowResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "marginMaxBorrow",
        path: "/papi/v1/margin/maxBorrowable",
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
        validate_parameters(&p, &["asset"], &[], &[])?;
        super::validation::validate("marginMaxBorrow", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`portfolioMarginUmTradingQuantitativeRulesIndicators`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#portfolio-margin-um-trading-quantitative-rules-indicators).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicators {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PortfolioMarginUmTradingQuantitativeRulesIndicators {
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
impl Request for PortfolioMarginUmTradingQuantitativeRulesIndicators {
    type Response = super::rest_models::PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "portfolioMarginUmTradingQuantitativeRulesIndicators",
        path: "/papi/v1/um/apiTradingStatus",
        method: "GET",
        security: Security::Signed,
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
        super::validation::validate("portfolioMarginUmTradingQuantitativeRulesIndicators", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCmPositionInformation`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-cm-position-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCmPositionInformation {
    #[serde(rename = "marginAsset", skip_serializing_if = "Option::is_none")]
    margin_asset: Option<crate::Asset>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCmPositionInformation {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `marginAsset` parameter.
    #[must_use]
    pub fn margin_asset(mut self, value: crate::Asset) -> Self {
        self.margin_asset = Some(value);
        self
    }
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
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
impl Request for QueryCmPositionInformation {
    type Response = super::rest_models::QueryCmPositionInformationResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCmPositionInformation",
        path: "/papi/v1/cm/positionRisk",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
        super::validation::validate("queryCmPositionInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginLoanRecord`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-loan-record).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginLoanRecord {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    archived: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginLoanRecord {
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
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: i64) -> Self {
        self.tx_id = Some(value);
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
    /// Set the provider `archived` parameter.
    #[must_use]
    pub fn archived(mut self, value: impl Into<String>) -> Self {
        self.archived = Some(value.into());
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
impl Request for QueryMarginLoanRecord {
    type Response = super::rest_models::QueryMarginLoanRecordResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryMarginLoanRecord",
        path: "/papi/v1/margin/marginLoan",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
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
            &[("archived", &["true", "false"])],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
            ],
        )?;
        super::validation::validate("queryMarginLoanRecord", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginMaxWithdraw`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-max-withdraw).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginMaxWithdraw {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginMaxWithdraw {
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
impl Request for QueryMarginMaxWithdraw {
    type Response = super::rest_models::QueryMarginMaxWithdrawResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryMarginMaxWithdraw",
        path: "/papi/v1/margin/maxWithdraw",
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
        validate_parameters(
            &p,
            &["asset"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMarginMaxWithdraw", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginRepayRecord`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-repay-record).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginRepayRecord {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    archived: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginRepayRecord {
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
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: i64) -> Self {
        self.tx_id = Some(value);
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
    /// Set the provider `archived` parameter.
    #[must_use]
    pub fn archived(mut self, value: impl Into<String>) -> Self {
        self.archived = Some(value.into());
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
impl Request for QueryMarginRepayRecord {
    type Response = super::rest_models::QueryMarginRepayRecordResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryMarginRepayRecord",
        path: "/papi/v1/margin/repayLoan",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
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
            &[("archived", &["true", "false"])],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryMarginRepayRecord", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryPortfolioMarginNegativeBalanceInterestHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-portfolio-margin-negative-balance-interest-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryPortfolioMarginNegativeBalanceInterestHistory {
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
impl QueryPortfolioMarginNegativeBalanceInterestHistory {
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
impl Request for QueryPortfolioMarginNegativeBalanceInterestHistory {
    type Response = super::rest_models::QueryPortfolioMarginNegativeBalanceInterestHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryPortfolioMarginNegativeBalanceInterestHistory",
        path: "/papi/v1/portfolio/interest-history",
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
        super::validation::validate("queryPortfolioMarginNegativeBalanceInterestHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUmPositionInformation`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-um-position-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUmPositionInformation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUmPositionInformation {
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
impl Request for QueryUmPositionInformation {
    type Response = super::rest_models::QueryUmPositionInformationResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUmPositionInformation",
        path: "/papi/v1/um/positionRisk",
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
        super::validation::validate("queryUmPositionInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUserNegativeBalanceAutoExchangeRecord`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-user-negative-balance-auto-exchange-record).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUserNegativeBalanceAutoExchangeRecord {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUserNegativeBalanceAutoExchangeRecord {
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
}
impl Request for QueryUserNegativeBalanceAutoExchangeRecord {
    type Response = super::rest_models::QueryUserNegativeBalanceAutoExchangeRecordResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUserNegativeBalanceAutoExchangeRecord",
        path: "/papi/v1/portfolio/negative-balance-exchange-record",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 100,
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
            &["endTime", "startTime"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryUserNegativeBalanceAutoExchangeRecord", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUserRateLimit`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-user-rate-limit).
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
}
impl Request for QueryUserRateLimit {
    type Response = super::rest_models::QueryUserRateLimitResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUserRateLimit",
        path: "/papi/v1/rateLimit/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
        super::validation::validate("queryUserRateLimit", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`repayFuturesNegativeBalance`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#repay-futures-negative-balance).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RepayFuturesNegativeBalance {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl RepayFuturesNegativeBalance {
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
impl Request for RepayFuturesNegativeBalance {
    type Response = super::rest_models::RepayFuturesNegativeBalanceResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "repayFuturesNegativeBalance",
        path: "/papi/v1/repay-futures-negative-balance",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 750,
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
        super::validation::validate("repayFuturesNegativeBalance", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`umFuturesAccountConfiguration`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-futures-account-configuration).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UmFuturesAccountConfiguration {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UmFuturesAccountConfiguration {
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
impl Request for UmFuturesAccountConfiguration {
    type Response = super::rest_models::UmFuturesAccountConfigurationResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "umFuturesAccountConfiguration",
        path: "/papi/v1/um/accountConfig",
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
        super::validation::validate("umFuturesAccountConfiguration", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`umFuturesSymbolConfiguration`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-futures-symbol-configuration).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UmFuturesSymbolConfiguration {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UmFuturesSymbolConfiguration {
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
impl Request for UmFuturesSymbolConfiguration {
    type Response = super::rest_models::UmFuturesSymbolConfigurationResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "umFuturesSymbolConfiguration",
        path: "/papi/v1/um/symbolConfig",
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
        super::validation::validate("umFuturesSymbolConfiguration", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`umNotionalAndLeverageBrackets`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-notional-and-leverage-brackets).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UmNotionalAndLeverageBrackets {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UmNotionalAndLeverageBrackets {
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
impl Request for UmNotionalAndLeverageBrackets {
    type Response = super::rest_models::UmNotionalAndLeverageBracketsResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "umNotionalAndLeverageBrackets",
        path: "/papi/v1/um/leverageBracket",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
        super::validation::validate("umNotionalAndLeverageBrackets", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`testConnectivity`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/market-data#test-connectivity).
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
}
impl Request for TestConnectivity {
    type Response = super::rest_models::TestConnectivityResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "testConnectivity",
        path: "/papi/v1/ping",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 1,
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
        super::validation::validate("testConnectivity", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllCmOpenConditionalOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-cm-open-conditional-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllCmOpenConditionalOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllCmOpenConditionalOrders {
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
impl Request for CancelAllCmOpenConditionalOrders {
    type Response = super::rest_models::CancelAllCmOpenConditionalOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelAllCmOpenConditionalOrders",
        path: "/papi/v1/cm/conditional/allOpenOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelAllCmOpenConditionalOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllCmOpenOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-cm-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllCmOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllCmOpenOrders {
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
impl Request for CancelAllCmOpenOrders {
    type Response = super::rest_models::CancelAllCmOpenOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelAllCmOpenOrders",
        path: "/papi/v1/cm/allOpenOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelAllCmOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllUmOpenOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-um-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllUmOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllUmOpenOrders {
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
impl Request for CancelAllUmOpenOrders {
    type Response = super::rest_models::CancelAllUmOpenOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelAllUmOpenOrders",
        path: "/papi/v1/um/allOpenOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelAllUmOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelCmConditionalOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-cm-conditional-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelCmConditionalOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(
        rename = "newClientStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    new_client_strategy_id: Option<crate::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelCmConditionalOrder {
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
    /// Set the provider `strategyId` parameter.
    #[must_use]
    pub fn strategy_id(mut self, value: i64) -> Self {
        self.strategy_id = Some(value);
        self
    }
    /// Set the provider `newClientStrategyId` parameter.
    #[must_use]
    pub fn new_client_strategy_id(mut self, value: crate::ClientOrderId) -> Self {
        self.new_client_strategy_id = Some(value);
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
impl Request for CancelCmConditionalOrder {
    type Response = super::rest_models::CancelCmConditionalOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelCmConditionalOrder",
        path: "/papi/v1/cm/conditional/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelCmConditionalOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newCmConditionalOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-cm-conditional-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewCmConditionalOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    position_side: Option<String>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    working_type: Option<String>,
    #[serde(rename = "priceProtect", skip_serializing_if = "Option::is_none")]
    price_protect: Option<String>,
    #[serde(
        rename = "newClientStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    new_client_strategy_id: Option<crate::ClientOrderId>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "activationPrice", skip_serializing_if = "Option::is_none")]
    activation_price: Option<Decimal>,
    #[serde(rename = "callbackRate", skip_serializing_if = "Option::is_none")]
    callback_rate: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NewCmConditionalOrder {
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
    /// Set the provider `strategyType` parameter.
    #[must_use]
    pub fn strategy_type(mut self, value: impl Into<String>) -> Self {
        self.strategy_type = Some(value.into());
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
    /// Set the provider `reduceOnly` parameter.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
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
    /// Set the provider `newClientStrategyId` parameter.
    #[must_use]
    pub fn new_client_strategy_id(mut self, value: crate::ClientOrderId) -> Self {
        self.new_client_strategy_id = Some(value);
        self
    }
    /// Set the provider `stopPrice` parameter.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
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
impl Request for NewCmConditionalOrder {
    type Response = super::rest_models::NewCmConditionalOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "newCmConditionalOrder",
        path: "/papi/v1/cm/conditional/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["newClientStrategyId", "side", "strategyType", "symbol"],
            &[
                ("side", &["BUY", "SELL"]),
                ("positionSide", &["BOTH", "LONG", "SHORT"]),
                (
                    "strategyType",
                    &[
                        "STOP",
                        "STOP_MARKET",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_MARKET",
                        "TRAILING_STOP_MARKET",
                    ],
                ),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX"]),
                ("workingType", &["MARK_PRICE", "CONTRACT_PRICE"]),
                ("priceProtect", &["true", "false"]),
            ],
            &[],
        )?;
        super::validation::validate("newCmConditionalOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelCmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-cm-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelCmOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelCmOrder {
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
}
impl Request for CancelCmOrder {
    type Response = super::rest_models::CancelCmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelCmOrder",
        path: "/papi/v1/cm/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelCmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyCmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#modify-cm-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ModifyCmOrder {
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
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ModifyCmOrder {
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
impl Request for ModifyCmOrder {
    type Response = super::rest_models::ModifyCmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "modifyCmOrder",
        path: "/papi/v1/cm/order",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["quantity", "side", "symbol"],
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
            ],
            &[],
        )?;
        super::validation::validate("modifyCmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newCmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-cm-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewCmOrder {
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
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    price_match: Option<String>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NewCmOrder {
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
    /// Set the provider `quantity` parameter.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set the provider `reduceOnly` parameter.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
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
impl Request for NewCmOrder {
    type Response = super::rest_models::NewCmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "newCmOrder",
        path: "/papi/v1/cm/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["newClientOrderId", "side", "symbol", "type"],
            &[
                ("side", &["BUY", "SELL"]),
                ("positionSide", &["BOTH", "LONG", "SHORT"]),
                ("type", &["LIMIT", "MARKET"]),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX"]),
                ("reduceOnly", &["true", "false"]),
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
                ("newOrderRespType", &["ACK", "RESULT"]),
            ],
            &[],
        )?;
        super::validation::validate("newCmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCmOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCmOrder {
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
}
impl Request for QueryCmOrder {
    type Response = super::rest_models::QueryCmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCmOrder",
        path: "/papi/v1/cm/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryCmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelMarginAccountAllOpenOrdersOnASymbol`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-all-open-orders-on-asymbol).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelMarginAccountAllOpenOrdersOnASymbol {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelMarginAccountAllOpenOrdersOnASymbol {
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
impl Request for CancelMarginAccountAllOpenOrdersOnASymbol {
    type Response = super::rest_models::CancelMarginAccountAllOpenOrdersOnASymbolResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelMarginAccountAllOpenOrdersOnASymbol",
        path: "/papi/v1/margin/allOpenOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
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
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelMarginAccountAllOpenOrdersOnASymbol", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelMarginAccountOcoOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-oco-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelMarginAccountOcoOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    order_list_id: Option<i64>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelMarginAccountOcoOrders {
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
    /// Set the provider `orderListId` parameter.
    #[must_use]
    pub fn order_list_id(mut self, value: i64) -> Self {
        self.order_list_id = Some(value);
        self
    }
    /// Set the provider `listClientOrderId` parameter.
    #[must_use]
    pub fn list_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.list_client_order_id = Some(value);
        self
    }
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
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
impl Request for CancelMarginAccountOcoOrders {
    type Response = super::rest_models::CancelMarginAccountOcoOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelMarginAccountOcoOrders",
        path: "/papi/v1/margin/orderList",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 2,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelMarginAccountOcoOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsOco`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsOco {
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    order_list_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsOco {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `orderListId` parameter.
    #[must_use]
    pub fn order_list_id(mut self, value: i64) -> Self {
        self.order_list_id = Some(value);
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
}
impl Request for QueryMarginAccountsOco {
    type Response = super::rest_models::QueryMarginAccountsOcoResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryMarginAccountsOco",
        path: "/papi/v1/margin/orderList",
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
        super::validation::validate("queryMarginAccountsOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelMarginAccountOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelMarginAccountOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelMarginAccountOrder {
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
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
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
impl Request for CancelMarginAccountOrder {
    type Response = super::rest_models::CancelMarginAccountOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelMarginAccountOrder",
        path: "/papi/v1/margin/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 2,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelMarginAccountOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newMarginOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-margin-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewMarginOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "sideEffectType", skip_serializing_if = "Option::is_none")]
    side_effect_type: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "autoRepayAtCancel", skip_serializing_if = "Option::is_none")]
    auto_repay_at_cancel: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NewMarginOrder {
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
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `quantity` parameter.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set the provider `quoteOrderQty` parameter.
    #[must_use]
    pub fn quote_order_qty(mut self, value: Decimal) -> Self {
        self.quote_order_qty = Some(value);
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set the provider `stopPrice` parameter.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
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
    /// Set the provider `icebergQty` parameter.
    #[must_use]
    pub fn iceberg_qty(mut self, value: Decimal) -> Self {
        self.iceberg_qty = Some(value);
        self
    }
    /// Set the provider `sideEffectType` parameter.
    #[must_use]
    pub fn side_effect_type(mut self, value: impl Into<String>) -> Self {
        self.side_effect_type = Some(value.into());
        self
    }
    /// Set the provider `timeInForce` parameter.
    #[must_use]
    pub fn time_in_force(mut self, value: impl Into<String>) -> Self {
        self.time_in_force = Some(value.into());
        self
    }
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `autoRepayAtCancel` parameter.
    #[must_use]
    pub fn auto_repay_at_cancel(mut self, value: impl Into<String>) -> Self {
        self.auto_repay_at_cancel = Some(value.into());
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
impl Request for NewMarginOrder {
    type Response = super::rest_models::NewMarginOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "newMarginOrder",
        path: "/papi/v1/margin/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["newClientOrderId", "side", "symbol", "type"],
            &[
                ("side", &["BUY", "SELL"]),
                (
                    "type",
                    &[
                        "LIMIT",
                        "MARKET",
                        "STOP_LOSS",
                        "STOP_LOSS_LIMIT",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                        "LIMIT_MAKER",
                    ],
                ),
                ("newOrderRespType", &["ACK", "RESULT", "FULL"]),
                (
                    "sideEffectType",
                    &[
                        "NO_SIDE_EFFECT",
                        "MARGIN_BUY",
                        "AUTO_REPAY",
                        "AUTO_BORROW_REPAY",
                    ],
                ),
                ("timeInForce", &["GTC", "IOC", "FOK"]),
                (
                    "selfTradePreventionMode",
                    &["NONE", "EXPIRE_TAKER", "EXPIRE_BOTH", "EXPIRE_MAKER"],
                ),
                ("autoRepayAtCancel", &["True", "False"]),
            ],
            &[],
        )?;
        super::validation::validate("newMarginOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-account-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountOrder {
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
}
impl Request for QueryMarginAccountOrder {
    type Response = super::rest_models::QueryMarginAccountOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryMarginAccountOrder",
        path: "/papi/v1/margin/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryMarginAccountOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelUmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-um-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelUmOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelUmOrder {
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
}
impl Request for CancelUmOrder {
    type Response = super::rest_models::CancelUmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelUmOrder",
        path: "/papi/v1/um/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelUmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyUmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#modify-um-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ModifyUmOrder {
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
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ModifyUmOrder {
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
impl Request for ModifyUmOrder {
    type Response = super::rest_models::ModifyUmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "modifyUmOrder",
        path: "/papi/v1/um/order",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["quantity", "side", "symbol"],
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
            ],
            &[],
        )?;
        super::validation::validate("modifyUmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newUmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-um-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewUmOrder {
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
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NewUmOrder {
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
    /// Set the provider `quantity` parameter.
    #[must_use]
    pub fn quantity(mut self, value: Decimal) -> Self {
        self.quantity = Some(value);
        self
    }
    /// Set the provider `reduceOnly` parameter.
    #[must_use]
    pub fn reduce_only(mut self, value: impl Into<String>) -> Self {
        self.reduce_only = Some(value.into());
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
}
impl Request for NewUmOrder {
    type Response = super::rest_models::NewUmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "newUmOrder",
        path: "/papi/v1/um/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["newClientOrderId", "side", "symbol", "type"],
            &[
                ("side", &["BUY", "SELL"]),
                ("positionSide", &["BOTH", "LONG", "SHORT"]),
                ("type", &["LIMIT", "MARKET"]),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX", "GTD"]),
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
        super::validation::validate("newUmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUmOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUmOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUmOrder {
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
}
impl Request for QueryUmOrder {
    type Response = super::rest_models::QueryUmOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUmOrder",
        path: "/papi/v1/um/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryUmOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cmAccountTradeList`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cm-account-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CmAccountTradeList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
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
impl CmAccountTradeList {
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
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
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
}
impl Request for CmAccountTradeList {
    type Response = super::rest_models::CmAccountTradeListResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cmAccountTradeList",
        path: "/papi/v1/cm/userTrades",
        method: "GET",
        security: Security::Signed,
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
        validate_parameters(
            &p,
            &[],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("cmAccountTradeList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cmPositionAdlQuantileEstimation`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cm-position-adl-quantile-estimation).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CmPositionAdlQuantileEstimation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CmPositionAdlQuantileEstimation {
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
impl Request for CmPositionAdlQuantileEstimation {
    type Response = super::rest_models::CmPositionAdlQuantileEstimationResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cmPositionAdlQuantileEstimation",
        path: "/papi/v1/cm/adlQuantile",
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
        super::validation::validate("cmPositionAdlQuantileEstimation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getUmFuturesBnbBurnStatus`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#get-um-futures-bnb-burn-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUmFuturesBnbBurnStatus {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetUmFuturesBnbBurnStatus {
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
impl Request for GetUmFuturesBnbBurnStatus {
    type Response = super::rest_models::GetUmFuturesBnbBurnStatusResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getUmFuturesBnbBurnStatus",
        path: "/papi/v1/um/feeBurn",
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
        super::validation::validate("getUmFuturesBnbBurnStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`toggleBnbBurnOnUmFuturesTrade`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#toggle-bnb-burn-on-um-futures-trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ToggleBnbBurnOnUmFuturesTrade {
    #[serde(rename = "feeBurn", skip_serializing_if = "Option::is_none")]
    fee_burn: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ToggleBnbBurnOnUmFuturesTrade {
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
}
impl Request for ToggleBnbBurnOnUmFuturesTrade {
    type Response = super::rest_models::ToggleBnbBurnOnUmFuturesTradeResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "toggleBnbBurnOnUmFuturesTrade",
        path: "/papi/v1/um/feeBurn",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["feeBurn"], &[("feeBurn", &["true", "false"])], &[])?;
        super::validation::validate("toggleBnbBurnOnUmFuturesTrade", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountBorrow`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-borrow).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountBorrow {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountBorrow {
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
impl Request for MarginAccountBorrow {
    type Response = super::rest_models::MarginAccountBorrowResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "marginAccountBorrow",
        path: "/papi/v1/marginLoan",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 100,
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
            &["amount", "asset"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginAccountBorrow", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountNewOco`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-new-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountNewOco {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "limitClientOrderId", skip_serializing_if = "Option::is_none")]
    limit_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "limitIcebergQty", skip_serializing_if = "Option::is_none")]
    limit_iceberg_qty: Option<Decimal>,
    #[serde(rename = "stopClientOrderId", skip_serializing_if = "Option::is_none")]
    stop_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "stopLimitPrice", skip_serializing_if = "Option::is_none")]
    stop_limit_price: Option<Decimal>,
    #[serde(rename = "stopIcebergQty", skip_serializing_if = "Option::is_none")]
    stop_iceberg_qty: Option<Decimal>,
    #[serde(
        rename = "stopLimitTimeInForce",
        skip_serializing_if = "Option::is_none"
    )]
    stop_limit_time_in_force: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "sideEffectType", skip_serializing_if = "Option::is_none")]
    side_effect_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountNewOco {
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
    /// Set the provider `listClientOrderId` parameter.
    #[must_use]
    pub fn list_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.list_client_order_id = Some(value);
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
    /// Set the provider `limitClientOrderId` parameter.
    #[must_use]
    pub fn limit_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.limit_client_order_id = Some(value);
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
        self
    }
    /// Set the provider `limitIcebergQty` parameter.
    #[must_use]
    pub fn limit_iceberg_qty(mut self, value: Decimal) -> Self {
        self.limit_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `stopClientOrderId` parameter.
    #[must_use]
    pub fn stop_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.stop_client_order_id = Some(value);
        self
    }
    /// Set the provider `stopPrice` parameter.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }
    /// Set the provider `stopLimitPrice` parameter.
    #[must_use]
    pub fn stop_limit_price(mut self, value: Decimal) -> Self {
        self.stop_limit_price = Some(value);
        self
    }
    /// Set the provider `stopIcebergQty` parameter.
    #[must_use]
    pub fn stop_iceberg_qty(mut self, value: Decimal) -> Self {
        self.stop_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `stopLimitTimeInForce` parameter.
    #[must_use]
    pub fn stop_limit_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.stop_limit_time_in_force = Some(value.into());
        self
    }
    /// Set the provider `newOrderRespType` parameter.
    #[must_use]
    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }
    /// Set the provider `sideEffectType` parameter.
    #[must_use]
    pub fn side_effect_type(mut self, value: impl Into<String>) -> Self {
        self.side_effect_type = Some(value.into());
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
impl Request for MarginAccountNewOco {
    type Response = super::rest_models::MarginAccountNewOcoResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "marginAccountNewOco",
        path: "/papi/v1/margin/order/oco",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &[
                "limitClientOrderId",
                "listClientOrderId",
                "price",
                "quantity",
                "side",
                "stopClientOrderId",
                "stopPrice",
                "symbol",
            ],
            &[
                ("side", &["BUY", "SELL"]),
                ("stopLimitTimeInForce", &["GTC", "IOC", "FOK"]),
                ("newOrderRespType", &["ACK", "RESULT"]),
                (
                    "sideEffectType",
                    &["NO_SIDE_EFFECT", "MARGIN_BUY", "AUTO_REPAY"],
                ),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginAccountNewOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountRepay`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-repay).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountRepay {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountRepay {
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
impl Request for MarginAccountRepay {
    type Response = super::rest_models::MarginAccountRepayResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "marginAccountRepay",
        path: "/papi/v1/repayLoan",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 100,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["amount", "asset"], &[], &[])?;
        super::validation::validate("marginAccountRepay", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountRepayDebt`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-repay-debt).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountRepayDebt {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "specifyRepayAssets", skip_serializing_if = "Option::is_none")]
    specify_repay_assets: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountRepayDebt {
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
    /// Set the provider `specifyRepayAssets` parameter.
    #[must_use]
    pub fn specify_repay_assets(mut self, value: impl Into<String>) -> Self {
        self.specify_repay_assets = Some(value.into());
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
impl Request for MarginAccountRepayDebt {
    type Response = super::rest_models::MarginAccountRepayDebtResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "marginAccountRepayDebt",
        path: "/papi/v1/margin/repay-debt",
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
        validate_parameters(&p, &["asset"], &[], &[])?;
        super::validation::validate("marginAccountRepayDebt", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountTradeList`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountTradeList {
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
impl MarginAccountTradeList {
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
}
impl Request for MarginAccountTradeList {
    type Response = super::rest_models::MarginAccountTradeListResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "marginAccountTradeList",
        path: "/papi/v1/margin/myTrades",
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
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("marginAccountTradeList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllCmConditionalOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-cm-conditional-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllCmConditionalOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryAllCmConditionalOrders {
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
    /// Set the provider `strategyId` parameter.
    #[must_use]
    pub fn strategy_id(mut self, value: i64) -> Self {
        self.strategy_id = Some(value);
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
impl Request for QueryAllCmConditionalOrders {
    type Response = super::rest_models::QueryAllCmConditionalOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllCmConditionalOrders",
        path: "/papi/v1/cm/conditional/allOrders",
        method: "GET",
        security: Security::Signed,
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
        validate_parameters(
            &p,
            &[],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("queryAllCmConditionalOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllCmOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-cm-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllCmOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
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
impl QueryAllCmOrders {
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
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
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
}
impl Request for QueryAllCmOrders {
    type Response = super::rest_models::QueryAllCmOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllCmOrders",
        path: "/papi/v1/cm/allOrders",
        method: "GET",
        security: Security::Signed,
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
        validate_parameters(&p, &[], &[], &[("limit", -9_223_372_036_854_775_808, 100)])?;
        super::validation::validate("queryAllCmOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllCurrentCmOpenConditionalOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-cm-open-conditional-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllCurrentCmOpenConditionalOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryAllCurrentCmOpenConditionalOrders {
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
impl Request for QueryAllCurrentCmOpenConditionalOrders {
    type Response = super::rest_models::QueryAllCurrentCmOpenConditionalOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllCurrentCmOpenConditionalOrders",
        path: "/papi/v1/cm/conditional/openOrders",
        method: "GET",
        security: Security::Signed,
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
        super::validation::validate("queryAllCurrentCmOpenConditionalOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllCurrentCmOpenOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-cm-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllCurrentCmOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryAllCurrentCmOpenOrders {
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
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
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
impl Request for QueryAllCurrentCmOpenOrders {
    type Response = super::rest_models::QueryAllCurrentCmOpenOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllCurrentCmOpenOrders",
        path: "/papi/v1/cm/openOrders",
        method: "GET",
        security: Security::Signed,
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
        super::validation::validate("queryAllCurrentCmOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllCurrentUmOpenOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-um-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllCurrentUmOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryAllCurrentUmOpenOrders {
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
impl Request for QueryAllCurrentUmOpenOrders {
    type Response = super::rest_models::QueryAllCurrentUmOpenOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllCurrentUmOpenOrders",
        path: "/papi/v1/um/openOrders",
        method: "GET",
        security: Security::Signed,
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
        super::validation::validate("queryAllCurrentUmOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllMarginAccountOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-margin-account-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllMarginAccountOrders {
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
impl QueryAllMarginAccountOrders {
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
}
impl Request for QueryAllMarginAccountOrders {
    type Response = super::rest_models::QueryAllMarginAccountOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllMarginAccountOrders",
        path: "/papi/v1/margin/allOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 100,
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
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("queryAllMarginAccountOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllUmOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-um-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllUmOrders {
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
impl QueryAllUmOrders {
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
}
impl Request for QueryAllUmOrders {
    type Response = super::rest_models::QueryAllUmOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllUmOrders",
        path: "/papi/v1/um/allOrders",
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
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("queryAllUmOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCmConditionalOrderHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-conditional-order-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCmConditionalOrderHistory {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(
        rename = "newClientStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    new_client_strategy_id: Option<crate::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCmConditionalOrderHistory {
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
    /// Set the provider `strategyId` parameter.
    #[must_use]
    pub fn strategy_id(mut self, value: i64) -> Self {
        self.strategy_id = Some(value);
        self
    }
    /// Set the provider `newClientStrategyId` parameter.
    #[must_use]
    pub fn new_client_strategy_id(mut self, value: crate::ClientOrderId) -> Self {
        self.new_client_strategy_id = Some(value);
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
impl Request for QueryCmConditionalOrderHistory {
    type Response = super::rest_models::QueryCmConditionalOrderHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCmConditionalOrderHistory",
        path: "/papi/v1/cm/conditional/orderHistory",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryCmConditionalOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCmModifyOrderHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-modify-order-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCmModifyOrderHistory {
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
impl QueryCmModifyOrderHistory {
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
}
impl Request for QueryCmModifyOrderHistory {
    type Response = super::rest_models::QueryCmModifyOrderHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCmModifyOrderHistory",
        path: "/papi/v1/cm/orderAmendment",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("queryCmModifyOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentCmOpenConditionalOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-cm-open-conditional-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentCmOpenConditionalOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(
        rename = "newClientStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    new_client_strategy_id: Option<crate::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCurrentCmOpenConditionalOrder {
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
    /// Set the provider `strategyId` parameter.
    #[must_use]
    pub fn strategy_id(mut self, value: i64) -> Self {
        self.strategy_id = Some(value);
        self
    }
    /// Set the provider `newClientStrategyId` parameter.
    #[must_use]
    pub fn new_client_strategy_id(mut self, value: crate::ClientOrderId) -> Self {
        self.new_client_strategy_id = Some(value);
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
impl Request for QueryCurrentCmOpenConditionalOrder {
    type Response = super::rest_models::QueryCurrentCmOpenConditionalOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCurrentCmOpenConditionalOrder",
        path: "/papi/v1/cm/conditional/openOrder",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryCurrentCmOpenConditionalOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentCmOpenOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-cm-open-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentCmOpenOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCurrentCmOpenOrder {
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
}
impl Request for QueryCurrentCmOpenOrder {
    type Response = super::rest_models::QueryCurrentCmOpenOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCurrentCmOpenOrder",
        path: "/papi/v1/cm/openOrder",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryCurrentCmOpenOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentMarginOpenOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-margin-open-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentMarginOpenOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    #[serde(skip)]
    trading_symbol_count: Option<super::TradingSymbolCount>,
}
impl QueryCurrentMarginOpenOrder {
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
    /// Supply current venue trading-symbol count authority for all-symbol admission.
    /// This local authority and its expiry are never sent to Binance.
    #[must_use]
    pub fn trading_symbol_count(mut self, value: super::TradingSymbolCount) -> Self {
        self.trading_symbol_count = Some(value);
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
impl Request for QueryCurrentMarginOpenOrder {
    type Response = super::rest_models::QueryCurrentMarginOpenOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCurrentMarginOpenOrder",
        path: "/papi/v1/margin/openOrders",
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
        super::validation::validate("queryCurrentMarginOpenOrder", &p)
    }
    fn validate_authority(&self, now: u64) -> Result<(), Error> {
        self.cost()?.validate_authority(Self::OP.name, now)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::open_orders_cost(Self::OP, &parameters(self)?, self.trading_symbol_count)
    }
}

/// Validated request builder for [`queryCurrentUmOpenOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-um-open-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentUmOpenOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCurrentUmOpenOrder {
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
}
impl Request for QueryCurrentUmOpenOrder {
    type Response = super::rest_models::QueryCurrentUmOpenOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCurrentUmOpenOrder",
        path: "/papi/v1/um/openOrder",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryCurrentUmOpenOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsAllOco`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-all-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsAllOco {
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsAllOco {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
impl Request for QueryMarginAccountsAllOco {
    type Response = super::rest_models::QueryMarginAccountsAllOcoResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryMarginAccountsAllOco",
        path: "/papi/v1/margin/allOrderList",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 100,
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
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("queryMarginAccountsAllOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsOpenOco`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-open-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsOpenOco {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsOpenOco {
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
impl Request for QueryMarginAccountsOpenOco {
    type Response = super::rest_models::QueryMarginAccountsOpenOcoResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryMarginAccountsOpenOco",
        path: "/papi/v1/margin/openOrderList",
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
        super::validation::validate("queryMarginAccountsOpenOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUmModifyOrderHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-modify-order-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUmModifyOrderHistory {
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
impl QueryUmModifyOrderHistory {
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
}
impl Request for QueryUmModifyOrderHistory {
    type Response = super::rest_models::QueryUmModifyOrderHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUmModifyOrderHistory",
        path: "/papi/v1/um/orderAmendment",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("queryUmModifyOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUsersCmForceOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-cm-force-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUsersCmForceOrders {
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
impl QueryUsersCmForceOrders {
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
}
impl Request for QueryUsersCmForceOrders {
    type Response = super::rest_models::QueryUsersCmForceOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUsersCmForceOrders",
        path: "/papi/v1/cm/forceOrders",
        method: "GET",
        security: Security::Signed,
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
        validate_parameters(
            &p,
            &[],
            &[("autoCloseType", &["LIQUIDATION", "ADL"])],
            &[("limit", -9_223_372_036_854_775_808, 100)],
        )?;
        super::validation::validate("queryUsersCmForceOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUsersMarginForceOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-margin-force-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUsersMarginForceOrders {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryUsersMarginForceOrders {
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
impl Request for QueryUsersMarginForceOrders {
    type Response = super::rest_models::QueryUsersMarginForceOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUsersMarginForceOrders",
        path: "/papi/v1/margin/forceOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
        super::validation::validate("queryUsersMarginForceOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUsersUmForceOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-um-force-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUsersUmForceOrders {
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
impl QueryUsersUmForceOrders {
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
}
impl Request for QueryUsersUmForceOrders {
    type Response = super::rest_models::QueryUsersUmForceOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUsersUmForceOrders",
        path: "/papi/v1/um/forceOrders",
        method: "GET",
        security: Security::Signed,
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
        validate_parameters(
            &p,
            &[],
            &[("autoCloseType", &["LIQUIDATION", "ADL"])],
            &[("limit", -9_223_372_036_854_775_808, 100)],
        )?;
        super::validation::validate("queryUsersUmForceOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`umAccountTradeList`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#um-account-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UmAccountTradeList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
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
impl UmAccountTradeList {
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
}
impl Request for UmAccountTradeList {
    type Response = super::rest_models::UmAccountTradeListResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "umAccountTradeList",
        path: "/papi/v1/um/userTrades",
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
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("umAccountTradeList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`umPositionAdlQuantileEstimation`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#um-position-adl-quantile-estimation).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UmPositionAdlQuantileEstimation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UmPositionAdlQuantileEstimation {
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
impl Request for UmPositionAdlQuantileEstimation {
    type Response = super::rest_models::UmPositionAdlQuantileEstimationResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "umPositionAdlQuantileEstimation",
        path: "/papi/v1/um/adlQuantile",
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
        super::validation::validate("umPositionAdlQuantileEstimation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newUmAlgoOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-um-algo-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewUmAlgoOrder {
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
impl NewUmAlgoOrder {
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
}
impl Request for NewUmAlgoOrder {
    type Response = super::rest_models::NewUmAlgoOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "newUmAlgoOrder",
        path: "/papi/v1/um/algo/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["algoType", "clientAlgoId", "side", "symbol", "type"],
            &[
                ("algoType", &["CONDITIONAL"]),
                ("side", &["BUY", "SELL"]),
                ("positionSide", &["BOTH", "LONG", "SHORT"]),
                (
                    "type",
                    &[
                        "STOP",
                        "TAKE_PROFIT",
                        "STOP_MARKET",
                        "TAKE_PROFIT_MARKET",
                        "TRAILING_STOP_MARKET",
                    ],
                ),
                ("timeInForce", &["IOC", "GTC", "FOK", "GTX", "GTD"]),
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
                    &["NONE", "EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH"],
                ),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("newUmAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelUmAlgoOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-um-algo-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelUmAlgoOrder {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "clientAlgoId", skip_serializing_if = "Option::is_none")]
    client_algo_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelUmAlgoOrder {
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
}
impl Request for CancelUmAlgoOrder {
    type Response = super::rest_models::CancelUmAlgoOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelUmAlgoOrder",
        path: "/papi/v1/um/algo/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
        super::validation::validate("cancelUmAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllUmAlgoOpenOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-um-algo-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllUmAlgoOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllUmAlgoOpenOrders {
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
impl Request for CancelAllUmAlgoOpenOrders {
    type Response = super::rest_models::CancelAllUmAlgoOpenOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "cancelAllUmAlgoOpenOrders",
        path: "/papi/v1/um/algo/allOpenOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
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
            &["symbol"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelAllUmAlgoOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentUmOpenAlgoOrder`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-um-open-algo-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentUmOpenAlgoOrder {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "clientAlgoId", skip_serializing_if = "Option::is_none")]
    client_algo_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCurrentUmOpenAlgoOrder {
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
}
impl Request for QueryCurrentUmOpenAlgoOrder {
    type Response = super::rest_models::QueryCurrentUmOpenAlgoOrderResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryCurrentUmOpenAlgoOrder",
        path: "/papi/v1/um/algo/algoOrder",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
        super::validation::validate("queryCurrentUmOpenAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryAllCurrentUmOpenAlgoOrders`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-um-open-algo-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryAllCurrentUmOpenAlgoOrders {
    #[serde(rename = "algoType", skip_serializing_if = "Option::is_none")]
    algo_type: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryAllCurrentUmOpenAlgoOrders {
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
}
impl Request for QueryAllCurrentUmOpenAlgoOrders {
    type Response = super::rest_models::QueryAllCurrentUmOpenAlgoOrdersResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryAllCurrentUmOpenAlgoOrders",
        path: "/papi/v1/um/algo/openAlgoOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
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
        super::validation::validate("queryAllCurrentUmOpenAlgoOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryUmAlgoOrderHistory`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-algo-order-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryUmAlgoOrderHistory {
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
impl QueryUmAlgoOrderHistory {
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
}
impl Request for QueryUmAlgoOrderHistory {
    type Response = super::rest_models::QueryUmAlgoOrderHistoryResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "queryUmAlgoOrderHistory",
        path: "/papi/v1/um/algo/allAlgoOrders",
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
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryUmAlgoOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresTradfiPerpsContract`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#futures-tradfi-perps-contract).
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
}
impl Request for FuturesTradfiPerpsContract {
    type Response = super::rest_models::FuturesTradfiPerpsContractResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "futuresTradfiPerpsContract",
        path: "/papi/v1/um/stock/contract",
        method: "POST",
        security: Security::Signed,
        mutation: true,
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

/// Validated request builder for [`closeUserDataStream`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#close-user-data-stream).
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
}
impl Request for CloseUserDataStream {
    type Response = super::rest_models::CloseUserDataStreamResponse;
    const EMPTY_RESPONSE: bool = true;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "closeUserDataStream",
        path: "/papi/v1/listenKey",
        method: "DELETE",
        security: Security::Key,
        mutation: true,
        weight: 1,
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
        super::validation::validate("closeUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`keepaliveUserDataStream`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#keepalive-user-data-stream).
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
}
impl Request for KeepaliveUserDataStream {
    type Response = super::rest_models::KeepaliveUserDataStreamResponse;
    const EMPTY_RESPONSE: bool = true;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "keepaliveUserDataStream",
        path: "/papi/v1/listenKey",
        method: "PUT",
        security: Security::Key,
        mutation: true,
        weight: 1,
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
        super::validation::validate("keepaliveUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`startUserDataStream`](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#start-user-data-stream).
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
}
impl Request for StartUserDataStream {
    type Response = super::rest_models::StartUserDataStreamResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "startUserDataStream",
        path: "/papi/v1/listenKey",
        method: "POST",
        security: Security::Key,
        mutation: true,
        weight: 1,
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
        super::validation::validate("startUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [accountBalance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#account-balance).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_balance(
        &self,
        request: &AccountBalance,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountBalanceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [accountInformation](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#account-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_information(
        &self,
        request: &AccountInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountInformationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [bnbTransfer](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#bnb-transfer).
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

    /// [changeAutoRepayFuturesStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-auto-repay-futures-status).
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

    /// [getAutoRepayFuturesStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-auto-repay-futures-status).
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

    /// [changeCmInitialLeverage](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-cm-initial-leverage).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_cm_initial_leverage(
        &self,
        request: &ChangeCmInitialLeverage,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeCmInitialLeverageResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changeCmPositionMode](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-cm-position-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_cm_position_mode(
        &self,
        request: &ChangeCmPositionMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeCmPositionModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getCmCurrentPositionMode](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-current-position-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_cm_current_position_mode(
        &self,
        request: &GetCmCurrentPositionMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetCmCurrentPositionModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changeUmInitialLeverage](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-um-initial-leverage).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_um_initial_leverage(
        &self,
        request: &ChangeUmInitialLeverage,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeUmInitialLeverageResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [changeUmPositionMode](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#change-um-position-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn change_um_position_mode(
        &self,
        request: &ChangeUmPositionMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ChangeUmPositionModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getUmCurrentPositionMode](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-current-position-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_current_position_mode(
        &self,
        request: &GetUmCurrentPositionMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUmCurrentPositionModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cmNotionalAndLeverageBrackets](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#cm-notional-and-leverage-brackets).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cm_notional_and_leverage_brackets(
        &self,
        request: &CmNotionalAndLeverageBrackets,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CmNotionalAndLeverageBracketsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [fundAutoCollection](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#fund-auto-collection).
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

    /// [fundCollectionByAsset](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#fund-collection-by-asset).
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

    /// [getCmAccountDetail](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-account-detail).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_cm_account_detail(
        &self,
        request: &GetCmAccountDetail,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetCmAccountDetailResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getCmIncomeHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-cm-income-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_cm_income_history(
        &self,
        request: &GetCmIncomeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetCmIncomeHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getDownloadIdForUmFuturesOrderHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-order-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_download_id_for_um_futures_order_history(
        &self,
        request: &GetDownloadIdForUmFuturesOrderHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetDownloadIdForUmFuturesOrderHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getDownloadIdForUmFuturesTradeHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-trade-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_download_id_for_um_futures_trade_history(
        &self,
        request: &GetDownloadIdForUmFuturesTradeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetDownloadIdForUmFuturesTradeHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getDownloadIdForUmFuturesTransactionHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-download-id-for-um-futures-transaction-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_download_id_for_um_futures_transaction_history(
        &self,
        request: &GetDownloadIdForUmFuturesTransactionHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetDownloadIdForUmFuturesTransactionHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getMarginBorrowLoanInterestHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-margin-borrow-loan-interest-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_margin_borrow_loan_interest_history(
        &self,
        request: &GetMarginBorrowLoanInterestHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetMarginBorrowLoanInterestHistoryResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getUmAccountDetail](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-account-detail).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_account_detail(
        &self,
        request: &GetUmAccountDetail,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUmAccountDetailResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getUmAccountDetailV2](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-account-detail-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_account_detail_v2(
        &self,
        request: &GetUmAccountDetailV2,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUmAccountDetailV2Response>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getUmFuturesOrderDownloadLinkById](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-order-download-link-by-id).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_futures_order_download_link_by_id(
        &self,
        request: &GetUmFuturesOrderDownloadLinkById,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUmFuturesOrderDownloadLinkByIdResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getUmFuturesTradeDownloadLinkById](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-trade-download-link-by-id).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_futures_trade_download_link_by_id(
        &self,
        request: &GetUmFuturesTradeDownloadLinkById,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUmFuturesTradeDownloadLinkByIdResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getUmFuturesTransactionDownloadLinkById](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-futures-transaction-download-link-by-id).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_futures_transaction_download_link_by_id(
        &self,
        request: &GetUmFuturesTransactionDownloadLinkById,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetUmFuturesTransactionDownloadLinkByIdResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getUmIncomeHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-um-income-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_income_history(
        &self,
        request: &GetUmIncomeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUmIncomeHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getUserCommissionRateForCm](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-user-commission-rate-for-cm).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_user_commission_rate_for_cm(
        &self,
        request: &GetUserCommissionRateForCm,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUserCommissionRateForCmResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getUserCommissionRateForUm](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#get-user-commission-rate-for-um).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_user_commission_rate_for_um(
        &self,
        request: &GetUserCommissionRateForUm,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUserCommissionRateForUmResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [marginMaxBorrow](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#margin-max-borrow).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_max_borrow(
        &self,
        request: &MarginMaxBorrow,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginMaxBorrowResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [portfolioMarginUmTradingQuantitativeRulesIndicators](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#portfolio-margin-um-trading-quantitative-rules-indicators).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn portfolio_margin_um_trading_quantitative_rules_indicators(
        &self,
        request: &PortfolioMarginUmTradingQuantitativeRulesIndicators,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<
            super::rest_models::PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponse,
        >,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryCmPositionInformation](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-cm-position-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_cm_position_information(
        &self,
        request: &QueryCmPositionInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCmPositionInformationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginLoanRecord](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-loan-record).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_loan_record(
        &self,
        request: &QueryMarginLoanRecord,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginLoanRecordResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginMaxWithdraw](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-max-withdraw).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_max_withdraw(
        &self,
        request: &QueryMarginMaxWithdraw,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginMaxWithdrawResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginRepayRecord](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-margin-repay-record).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_repay_record(
        &self,
        request: &QueryMarginRepayRecord,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginRepayRecordResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryPortfolioMarginNegativeBalanceInterestHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-portfolio-margin-negative-balance-interest-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_portfolio_margin_negative_balance_interest_history(
        &self,
        request: &QueryPortfolioMarginNegativeBalanceInterestHistory,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<
            super::rest_models::QueryPortfolioMarginNegativeBalanceInterestHistoryResponse,
        >,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryUmPositionInformation](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-um-position-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_um_position_information(
        &self,
        request: &QueryUmPositionInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUmPositionInformationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryUserNegativeBalanceAutoExchangeRecord](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-user-negative-balance-auto-exchange-record).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_user_negative_balance_auto_exchange_record(
        &self,
        request: &QueryUserNegativeBalanceAutoExchangeRecord,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryUserNegativeBalanceAutoExchangeRecordResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryUserRateLimit](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#query-user-rate-limit).
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

    /// [repayFuturesNegativeBalance](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#repay-futures-negative-balance).
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

    /// [umFuturesAccountConfiguration](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-futures-account-configuration).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn um_futures_account_configuration(
        &self,
        request: &UmFuturesAccountConfiguration,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UmFuturesAccountConfigurationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [umFuturesSymbolConfiguration](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-futures-symbol-configuration).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn um_futures_symbol_configuration(
        &self,
        request: &UmFuturesSymbolConfiguration,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UmFuturesSymbolConfigurationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [umNotionalAndLeverageBrackets](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/account#um-notional-and-leverage-brackets).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn um_notional_and_leverage_brackets(
        &self,
        request: &UmNotionalAndLeverageBrackets,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UmNotionalAndLeverageBracketsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [testConnectivity](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/market-data#test-connectivity).
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

    /// [cancelAllCmOpenConditionalOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-cm-open-conditional-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_cm_open_conditional_orders(
        &self,
        request: &CancelAllCmOpenConditionalOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAllCmOpenConditionalOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [cancelAllCmOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-cm-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_cm_open_orders(
        &self,
        request: &CancelAllCmOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAllCmOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelAllUmOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-um-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_um_open_orders(
        &self,
        request: &CancelAllUmOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAllUmOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelCmConditionalOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-cm-conditional-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_cm_conditional_order(
        &self,
        request: &CancelCmConditionalOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelCmConditionalOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newCmConditionalOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-cm-conditional-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_cm_conditional_order(
        &self,
        request: &NewCmConditionalOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewCmConditionalOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelCmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-cm-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_cm_order(
        &self,
        request: &CancelCmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelCmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [modifyCmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#modify-cm-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn modify_cm_order(
        &self,
        request: &ModifyCmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ModifyCmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newCmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-cm-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_cm_order(
        &self,
        request: &NewCmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewCmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_cm_order(
        &self,
        request: &QueryCmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelMarginAccountAllOpenOrdersOnASymbol](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-all-open-orders-on-asymbol).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_margin_account_all_open_orders_on_a_symbol(
        &self,
        request: &CancelMarginAccountAllOpenOrdersOnASymbol,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::CancelMarginAccountAllOpenOrdersOnASymbolResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [cancelMarginAccountOcoOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-oco-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_margin_account_oco_orders(
        &self,
        request: &CancelMarginAccountOcoOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelMarginAccountOcoOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsOco](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-oco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_oco(
        &self,
        request: &QueryMarginAccountsOco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountsOcoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelMarginAccountOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-margin-account-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_margin_account_order(
        &self,
        request: &CancelMarginAccountOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelMarginAccountOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newMarginOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-margin-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_margin_order(
        &self,
        request: &NewMarginOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewMarginOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-account-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_account_order(
        &self,
        request: &QueryMarginAccountOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelUmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-um-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_um_order(
        &self,
        request: &CancelUmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelUmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [modifyUmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#modify-um-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn modify_um_order(
        &self,
        request: &ModifyUmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ModifyUmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newUmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-um-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_um_order(
        &self,
        request: &NewUmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewUmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryUmOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_um_order(
        &self,
        request: &QueryUmOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUmOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cmAccountTradeList](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cm-account-trade-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cm_account_trade_list(
        &self,
        request: &CmAccountTradeList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CmAccountTradeListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cmPositionAdlQuantileEstimation](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cm-position-adl-quantile-estimation).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cm_position_adl_quantile_estimation(
        &self,
        request: &CmPositionAdlQuantileEstimation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CmPositionAdlQuantileEstimationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getUmFuturesBnbBurnStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#get-um-futures-bnb-burn-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_um_futures_bnb_burn_status(
        &self,
        request: &GetUmFuturesBnbBurnStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetUmFuturesBnbBurnStatusResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [toggleBnbBurnOnUmFuturesTrade](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#toggle-bnb-burn-on-um-futures-trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn toggle_bnb_burn_on_um_futures_trade(
        &self,
        request: &ToggleBnbBurnOnUmFuturesTrade,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ToggleBnbBurnOnUmFuturesTradeResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountBorrow](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-borrow).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_borrow(
        &self,
        request: &MarginAccountBorrow,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountBorrowResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountNewOco](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-new-oco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_new_oco(
        &self,
        request: &MarginAccountNewOco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountNewOcoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountRepay](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-repay).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_repay(
        &self,
        request: &MarginAccountRepay,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountRepayResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountRepayDebt](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-repay-debt).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_repay_debt(
        &self,
        request: &MarginAccountRepayDebt,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountRepayDebtResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountTradeList](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#margin-account-trade-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_trade_list(
        &self,
        request: &MarginAccountTradeList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountTradeListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllCmConditionalOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-cm-conditional-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_cm_conditional_orders(
        &self,
        request: &QueryAllCmConditionalOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllCmConditionalOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllCmOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-cm-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_cm_orders(
        &self,
        request: &QueryAllCmOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllCmOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllCurrentCmOpenConditionalOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-cm-open-conditional-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_current_cm_open_conditional_orders(
        &self,
        request: &QueryAllCurrentCmOpenConditionalOrders,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryAllCurrentCmOpenConditionalOrdersResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllCurrentCmOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-cm-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_current_cm_open_orders(
        &self,
        request: &QueryAllCurrentCmOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllCurrentCmOpenOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllCurrentUmOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-um-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_current_um_open_orders(
        &self,
        request: &QueryAllCurrentUmOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllCurrentUmOpenOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllMarginAccountOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-margin-account-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_margin_account_orders(
        &self,
        request: &QueryAllMarginAccountOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllMarginAccountOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllUmOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-um-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_um_orders(
        &self,
        request: &QueryAllUmOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllUmOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCmConditionalOrderHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-conditional-order-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_cm_conditional_order_history(
        &self,
        request: &QueryCmConditionalOrderHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCmConditionalOrderHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryCmModifyOrderHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-cm-modify-order-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_cm_modify_order_history(
        &self,
        request: &QueryCmModifyOrderHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCmModifyOrderHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentCmOpenConditionalOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-cm-open-conditional-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_cm_open_conditional_order(
        &self,
        request: &QueryCurrentCmOpenConditionalOrder,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryCurrentCmOpenConditionalOrderResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentCmOpenOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-cm-open-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_cm_open_order(
        &self,
        request: &QueryCurrentCmOpenOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCurrentCmOpenOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentMarginOpenOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-margin-open-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_margin_open_order(
        &self,
        request: &QueryCurrentMarginOpenOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCurrentMarginOpenOrderResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentUmOpenOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-um-open-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_um_open_order(
        &self,
        request: &QueryCurrentUmOpenOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCurrentUmOpenOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsAllOco](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-all-oco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_all_oco(
        &self,
        request: &QueryMarginAccountsAllOco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountsAllOcoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsOpenOco](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-margin-accounts-open-oco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_open_oco(
        &self,
        request: &QueryMarginAccountsOpenOco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountsOpenOcoResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryUmModifyOrderHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-modify-order-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_um_modify_order_history(
        &self,
        request: &QueryUmModifyOrderHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUmModifyOrderHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryUsersCmForceOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-cm-force-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_users_cm_force_orders(
        &self,
        request: &QueryUsersCmForceOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUsersCmForceOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryUsersMarginForceOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-margin-force-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_users_margin_force_orders(
        &self,
        request: &QueryUsersMarginForceOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUsersMarginForceOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryUsersUmForceOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-users-um-force-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_users_um_force_orders(
        &self,
        request: &QueryUsersUmForceOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUsersUmForceOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [umAccountTradeList](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#um-account-trade-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn um_account_trade_list(
        &self,
        request: &UmAccountTradeList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UmAccountTradeListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [umPositionAdlQuantileEstimation](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#um-position-adl-quantile-estimation).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn um_position_adl_quantile_estimation(
        &self,
        request: &UmPositionAdlQuantileEstimation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UmPositionAdlQuantileEstimationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [newUmAlgoOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#new-um-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_um_algo_order(
        &self,
        request: &NewUmAlgoOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewUmAlgoOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelUmAlgoOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-um-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_um_algo_order(
        &self,
        request: &CancelUmAlgoOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelUmAlgoOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelAllUmAlgoOpenOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#cancel-all-um-algo-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_um_algo_open_orders(
        &self,
        request: &CancelAllUmAlgoOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAllUmAlgoOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentUmOpenAlgoOrder](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-current-um-open-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_um_open_algo_order(
        &self,
        request: &QueryCurrentUmOpenAlgoOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCurrentUmOpenAlgoOrderResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryAllCurrentUmOpenAlgoOrders](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-all-current-um-open-algo-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_all_current_um_open_algo_orders(
        &self,
        request: &QueryAllCurrentUmOpenAlgoOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryAllCurrentUmOpenAlgoOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryUmAlgoOrderHistory](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#query-um-algo-order-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_um_algo_order_history(
        &self,
        request: &QueryUmAlgoOrderHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryUmAlgoOrderHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [futuresTradfiPerpsContract](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/trade#futures-tradfi-perps-contract).
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

    /// [closeUserDataStream](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#close-user-data-stream).
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

    /// [keepaliveUserDataStream](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#keepalive-user-data-stream).
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

    /// [startUserDataStream](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api/user-data-streams#start-user-data-stream).
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
