// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use crate::Decimal;
use crate::Error;
use crate::Symbol;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`adjustCrossMarginMaxLeverage`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#adjust-cross-margin-max-leverage).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AdjustCrossMarginMaxLeverage {
    #[serde(rename = "maxLeverage", skip_serializing_if = "Option::is_none")]
    max_leverage: Option<i64>,
}
impl AdjustCrossMarginMaxLeverage {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `maxLeverage` parameter.
    #[must_use]
    pub fn max_leverage(mut self, value: i64) -> Self {
        self.max_leverage = Some(value);
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
impl Request for AdjustCrossMarginMaxLeverage {
    type Response = super::rest_models::AdjustCrossMarginMaxLeverageResponse;
    const OP: Operation = Operation {
        name: "adjustCrossMarginMaxLeverage",
        path: "/sapi/v1/margin/max-leverage",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 3000,
        requests_per_second: None,
        requests_per_minute: Some(1),
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["maxLeverage"], &[], &[])?;
        super::validation::validate("adjustCrossMarginMaxLeverage", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`disableIsolatedMarginAccount`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#disable-isolated-margin-account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DisableIsolatedMarginAccount {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl DisableIsolatedMarginAccount {
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
impl Request for DisableIsolatedMarginAccount {
    type Response = super::rest_models::DisableIsolatedMarginAccountResponse;
    const OP: Operation = Operation {
        name: "disableIsolatedMarginAccount",
        path: "/sapi/v1/margin/isolated/account",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 300,
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
        super::validation::validate("disableIsolatedMarginAccount", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`enableIsolatedMarginAccount`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#enable-isolated-margin-account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct EnableIsolatedMarginAccount {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl EnableIsolatedMarginAccount {
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
impl Request for EnableIsolatedMarginAccount {
    type Response = super::rest_models::EnableIsolatedMarginAccountResponse;
    const OP: Operation = Operation {
        name: "enableIsolatedMarginAccount",
        path: "/sapi/v1/margin/isolated/account",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 300,
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
        super::validation::validate("enableIsolatedMarginAccount", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryIsolatedMarginAccountInfo`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-isolated-margin-account-info).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryIsolatedMarginAccountInfo {
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryIsolatedMarginAccountInfo {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: impl Into<String>) -> Self {
        self.symbols = Some(value.into());
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
impl Request for QueryIsolatedMarginAccountInfo {
    type Response = super::rest_models::QueryIsolatedMarginAccountInfoResponse;
    const OP: Operation = Operation {
        name: "queryIsolatedMarginAccountInfo",
        path: "/sapi/v1/margin/isolated/account",
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryIsolatedMarginAccountInfo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getBnbBurnStatus`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#get-bnb-burn-status).
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
}
impl Request for GetBnbBurnStatus {
    type Response = super::rest_models::GetBnbBurnStatusResponse;
    const OP: Operation = Operation {
        name: "getBnbBurnStatus",
        path: "/sapi/v1/bnbBurn",
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
        super::validation::validate("getBnbBurnStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getSummaryOfMarginAccount`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#get-summary-of-margin-account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSummaryOfMarginAccount {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetSummaryOfMarginAccount {
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
impl Request for GetSummaryOfMarginAccount {
    type Response = super::rest_models::GetSummaryOfMarginAccountResponse;
    const OP: Operation = Operation {
        name: "getSummaryOfMarginAccount",
        path: "/sapi/v1/margin/tradeCoeff",
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getSummaryOfMarginAccount", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCrossIsolatedMarginCapitalFlow`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-isolated-margin-capital-flow).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCrossIsolatedMarginCapitalFlow {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<crate::margin::RecordId>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCrossIsolatedMarginCapitalFlow {
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
    /// Set the provider `fromId` parameter.
    #[must_use]
    pub fn from_id(mut self, value: crate::margin::RecordId) -> Self {
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
impl Request for QueryCrossIsolatedMarginCapitalFlow {
    type Response = super::rest_models::QueryCrossIsolatedMarginCapitalFlowResponse;
    const OP: Operation = Operation {
        name: "queryCrossIsolatedMarginCapitalFlow",
        path: "/sapi/v1/margin/capital-flow",
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
            &[(
                "type",
                &[
                    "TRANSFER",
                    "BORROW",
                    "REPAY",
                    "BUY_INCOME",
                    "BUY_EXPENSE",
                    "SELL_INCOME",
                    "SELL_EXPENSE",
                    "TRADING_COMMISSION",
                    "BUY_LIQUIDATION",
                    "SELL_LIQUIDATION",
                    "REPAY_LIQUIDATION",
                    "OTHER_LIQUIDATION",
                    "LIQUIDATION_FEE",
                    "SMALL_BALANCE_CONVERT",
                    "COMMISSION_RETURN",
                    "SMALL_CONVERT",
                ],
            )],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryCrossIsolatedMarginCapitalFlow", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCrossMarginAccountDetails`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-margin-account-details).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCrossMarginAccountDetails {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCrossMarginAccountDetails {
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
impl Request for QueryCrossMarginAccountDetails {
    type Response = super::rest_models::QueryCrossMarginAccountDetailsResponse;
    const OP: Operation = Operation {
        name: "queryCrossMarginAccountDetails",
        path: "/sapi/v1/margin/account",
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryCrossMarginAccountDetails", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCrossMarginFeeData`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-margin-fee-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCrossMarginFeeData {
    #[serde(rename = "vipLevel", skip_serializing_if = "Option::is_none")]
    vip_level: Option<i64>,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    coin: Option<crate::Asset>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCrossMarginFeeData {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `vipLevel` parameter.
    #[must_use]
    pub fn vip_level(mut self, value: i64) -> Self {
        self.vip_level = Some(value);
        self
    }
    /// Set the provider `coin` parameter.
    #[must_use]
    pub fn coin(mut self, value: crate::Asset) -> Self {
        self.coin = Some(value);
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
impl Request for QueryCrossMarginFeeData {
    type Response = super::rest_models::QueryCrossMarginFeeDataResponse;
    const OP: Operation = Operation {
        name: "queryCrossMarginFeeData",
        path: "/sapi/v1/margin/crossMarginData",
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryCrossMarginFeeData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryEnabledIsolatedMarginAccountLimit`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-enabled-isolated-margin-account-limit).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryEnabledIsolatedMarginAccountLimit {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryEnabledIsolatedMarginAccountLimit {
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
impl Request for QueryEnabledIsolatedMarginAccountLimit {
    type Response = super::rest_models::QueryEnabledIsolatedMarginAccountLimitResponse;
    const OP: Operation = Operation {
        name: "queryEnabledIsolatedMarginAccountLimit",
        path: "/sapi/v1/margin/isolated/accountLimit",
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
        super::validation::validate("queryEnabledIsolatedMarginAccountLimit", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryIsolatedMarginFeeData`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-isolated-margin-fee-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryIsolatedMarginFeeData {
    #[serde(rename = "vipLevel", skip_serializing_if = "Option::is_none")]
    vip_level: Option<i64>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryIsolatedMarginFeeData {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `vipLevel` parameter.
    #[must_use]
    pub fn vip_level(mut self, value: i64) -> Self {
        self.vip_level = Some(value);
        self
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
impl Request for QueryIsolatedMarginFeeData {
    type Response = super::rest_models::QueryIsolatedMarginFeeDataResponse;
    const OP: Operation = Operation {
        name: "queryIsolatedMarginFeeData",
        path: "/sapi/v1/margin/isolatedMarginData",
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryIsolatedMarginFeeData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFutureHourlyInterestRate`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#get-future-hourly-interest-rate).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFutureHourlyInterestRate {
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    assets: Option<String>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
}
impl GetFutureHourlyInterestRate {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `assets` parameter.
    #[must_use]
    pub fn assets(mut self, value: impl Into<String>) -> Self {
        self.assets = Some(value.into());
        self
    }
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
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
impl Request for GetFutureHourlyInterestRate {
    type Response = super::rest_models::GetFutureHourlyInterestRateResponse;
    const OP: Operation = Operation {
        name: "getFutureHourlyInterestRate",
        path: "/sapi/v1/margin/next-hourly-interest-rate",
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
            &["assets", "isIsolated"],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[],
        )?;
        super::validation::validate("getFutureHourlyInterestRate", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getInterestHistory`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#get-interest-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetInterestHistory {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "isolatedSymbol", skip_serializing_if = "Option::is_none")]
    isolated_symbol: Option<crate::Symbol>,
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
impl GetInterestHistory {
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
    /// Set the provider `isolatedSymbol` parameter.
    #[must_use]
    pub fn isolated_symbol(mut self, value: crate::Symbol) -> Self {
        self.isolated_symbol = Some(value);
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
impl Request for GetInterestHistory {
    type Response = super::rest_models::GetInterestHistoryResponse;
    const OP: Operation = Operation {
        name: "getInterestHistory",
        path: "/sapi/v1/margin/interestHistory",
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
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("getInterestHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountBorrowRepay`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#margin-account-borrow-repay).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountBorrowRepay {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountBorrowRepay {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
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
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
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
impl Request for MarginAccountBorrowRepay {
    type Response = super::rest_models::MarginAccountBorrowRepayResponse;
    const OP: Operation = Operation {
        name: "marginAccountBorrowRepay",
        path: "/sapi/v1/margin/borrow-repay",
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
            &["amount", "asset", "isIsolated", "type"],
            &[
                ("isIsolated", &["TRUE", "FALSE"]),
                ("type", &["BORROW", "REPAY"]),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginAccountBorrowRepay", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryBorrowRepayRecordsInMarginAccount`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-borrow-repay-records-in-margin-account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryBorrowRepayRecordsInMarginAccount {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "isolatedSymbol", skip_serializing_if = "Option::is_none")]
    isolated_symbol: Option<crate::Symbol>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<crate::margin::TransactionId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryBorrowRepayRecordsInMarginAccount {
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
    /// Set the provider `isolatedSymbol` parameter.
    #[must_use]
    pub fn isolated_symbol(mut self, value: crate::Symbol) -> Self {
        self.isolated_symbol = Some(value);
        self
    }
    /// Set the provider `txId` parameter.
    #[must_use]
    pub fn tx_id(mut self, value: crate::margin::TransactionId) -> Self {
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
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
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
impl Request for QueryBorrowRepayRecordsInMarginAccount {
    type Response = super::rest_models::QueryBorrowRepayRecordsInMarginAccountResponse;
    const OP: Operation = Operation {
        name: "queryBorrowRepayRecordsInMarginAccount",
        path: "/sapi/v1/margin/borrow-repay",
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
            &["type"],
            &[("type", &["BORROW", "REPAY"])],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryBorrowRepayRecordsInMarginAccount", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginInterestRateHistory`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-margin-interest-rate-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginInterestRateHistory {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "vipLevel", skip_serializing_if = "Option::is_none")]
    vip_level: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginInterestRateHistory {
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
    /// Set the provider `vipLevel` parameter.
    #[must_use]
    pub fn vip_level(mut self, value: i64) -> Self {
        self.vip_level = Some(value);
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
impl Request for QueryMarginInterestRateHistory {
    type Response = super::rest_models::QueryMarginInterestRateHistoryResponse;
    const OP: Operation = Operation {
        name: "queryMarginInterestRateHistory",
        path: "/sapi/v1/margin/interestRateHistory",
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
            &["asset"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMarginInterestRateHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMaxBorrow`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-max-borrow).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMaxBorrow {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "isolatedSymbol", skip_serializing_if = "Option::is_none")]
    isolated_symbol: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMaxBorrow {
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
    /// Set the provider `isolatedSymbol` parameter.
    #[must_use]
    pub fn isolated_symbol(mut self, value: crate::Symbol) -> Self {
        self.isolated_symbol = Some(value);
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
impl Request for QueryMaxBorrow {
    type Response = super::rest_models::QueryMaxBorrowResponse;
    const OP: Operation = Operation {
        name: "queryMaxBorrow",
        path: "/sapi/v1/margin/maxBorrowable",
        method: "GET",
        security: Security::Signed,
        mutation: false,
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
            &["asset"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMaxBorrow", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`crossMarginCollateralRatio`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#cross-margin-collateral-ratio).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CrossMarginCollateralRatio {}
impl CrossMarginCollateralRatio {
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
impl Request for CrossMarginCollateralRatio {
    type Response = super::rest_models::CrossMarginCollateralRatioResponse;
    const OP: Operation = Operation {
        name: "crossMarginCollateralRatio",
        path: "/sapi/v1/margin/crossMarginCollateralRatio",
        method: "GET",
        security: Security::Key,
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("crossMarginCollateralRatio", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getAllCrossMarginPairs`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-cross-margin-pairs).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAllCrossMarginPairs {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl GetAllCrossMarginPairs {
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
}
impl Request for GetAllCrossMarginPairs {
    type Response = super::rest_models::GetAllCrossMarginPairsResponse;
    const OP: Operation = Operation {
        name: "getAllCrossMarginPairs",
        path: "/sapi/v1/margin/allPairs",
        method: "GET",
        security: Security::Key,
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
        super::validation::validate("getAllCrossMarginPairs", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getAllIsolatedMarginSymbol`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-isolated-margin-symbol).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAllIsolatedMarginSymbol {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetAllIsolatedMarginSymbol {
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
impl Request for GetAllIsolatedMarginSymbol {
    type Response = super::rest_models::GetAllIsolatedMarginSymbolResponse;
    const OP: Operation = Operation {
        name: "getAllIsolatedMarginSymbol",
        path: "/sapi/v1/margin/isolated/allPairs",
        method: "GET",
        security: Security::Key,
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getAllIsolatedMarginSymbol", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getAllMarginAssets`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-margin-assets).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAllMarginAssets {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
}
impl GetAllMarginAssets {
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
impl Request for GetAllMarginAssets {
    type Response = super::rest_models::GetAllMarginAssetsResponse;
    const OP: Operation = Operation {
        name: "getAllMarginAssets",
        path: "/sapi/v1/margin/allAssets",
        method: "GET",
        security: Security::Key,
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
        super::validation::validate("getAllMarginAssets", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDelistSchedule`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-delist-schedule).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDelistSchedule {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetDelistSchedule {
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
impl Request for GetDelistSchedule {
    type Response = super::rest_models::GetDelistScheduleResponse;
    const OP: Operation = Operation {
        name: "getDelistSchedule",
        path: "/sapi/v1/margin/delist-schedule",
        method: "GET",
        security: Security::Key,
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getDelistSchedule", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getLimitPricePairs`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-limit-price-pairs).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetLimitPricePairs {}
impl GetLimitPricePairs {
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
impl Request for GetLimitPricePairs {
    type Response = super::rest_models::GetLimitPricePairsResponse;
    const OP: Operation = Operation {
        name: "getLimitPricePairs",
        path: "/sapi/v1/margin/limit-price-pairs",
        method: "GET",
        security: Security::Key,
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
        super::validation::validate("getLimitPricePairs", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getListSchedule`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-list-schedule).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetListSchedule {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetListSchedule {
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
impl Request for GetListSchedule {
    type Response = super::rest_models::GetListScheduleResponse;
    const OP: Operation = Operation {
        name: "getListSchedule",
        path: "/sapi/v1/margin/list-schedule",
        method: "GET",
        security: Security::Key,
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getListSchedule", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getMarginAssetRiskBasedLiquidationRatio`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-margin-asset-risk-based-liquidation-ratio).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetMarginAssetRiskBasedLiquidationRatio {}
impl GetMarginAssetRiskBasedLiquidationRatio {
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
impl Request for GetMarginAssetRiskBasedLiquidationRatio {
    type Response = super::rest_models::GetMarginAssetRiskBasedLiquidationRatioResponse;
    const OP: Operation = Operation {
        name: "getMarginAssetRiskBasedLiquidationRatio",
        path: "/sapi/v1/margin/risk-based-liquidation-ratio",
        method: "GET",
        security: Security::Key,
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
        super::validation::validate("getMarginAssetRiskBasedLiquidationRatio", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getMarginRestrictedAssets`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-margin-restricted-assets).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetMarginRestrictedAssets {}
impl GetMarginRestrictedAssets {
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
impl Request for GetMarginRestrictedAssets {
    type Response = super::rest_models::GetMarginRestrictedAssetsResponse;
    const OP: Operation = Operation {
        name: "getMarginRestrictedAssets",
        path: "/sapi/v1/margin/restricted-asset",
        method: "GET",
        security: Security::Key,
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
        super::validation::validate("getMarginRestrictedAssets", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryIsolatedMarginTierData`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-isolated-margin-tier-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryIsolatedMarginTierData {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    tier: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryIsolatedMarginTierData {
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
    /// Set the provider `tier` parameter.
    #[must_use]
    pub fn tier(mut self, value: i64) -> Self {
        self.tier = Some(value);
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
impl Request for QueryIsolatedMarginTierData {
    type Response = super::rest_models::QueryIsolatedMarginTierDataResponse;
    const OP: Operation = Operation {
        name: "queryIsolatedMarginTierData",
        path: "/sapi/v1/margin/isolatedMarginTier",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryIsolatedMarginTierData", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAvailableInventory`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-margin-available-inventory).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAvailableInventory {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
}
impl QueryMarginAvailableInventory {
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
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for QueryMarginAvailableInventory {
    type Response = super::rest_models::QueryMarginAvailableInventoryResponse;
    const OP: Operation = Operation {
        name: "queryMarginAvailableInventory",
        path: "/sapi/v1/margin/available-inventory",
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
        validate_parameters(&p, &["type"], &[("type", &["MARGIN", "ISOLATED"])], &[])?;
        super::validation::validate("queryMarginAvailableInventory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginPriceindex`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-margin-priceindex).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginPriceindex {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl QueryMarginPriceindex {
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
}
impl Request for QueryMarginPriceindex {
    type Response = super::rest_models::QueryMarginPriceindexResponse;
    const OP: Operation = Operation {
        name: "queryMarginPriceindex",
        path: "/sapi/v1/margin/priceIndex",
        method: "GET",
        security: Security::Key,
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
        super::validation::validate("queryMarginPriceindex", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`closeUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#close-user-data-stream).
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
    const OP: Operation = Operation {
        name: "closeUserDataStream",
        path: "/sapi/v1/margin/listen-key",
        method: "DELETE",
        security: Security::Key,
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("closeUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`keepaliveUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#keepalive-user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct KeepaliveUserDataStream {
    #[serde(rename = "listenKey", skip_serializing_if = "Option::is_none")]
    listen_key: Option<crate::SensitiveString>,
}
impl KeepaliveUserDataStream {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `listenKey` parameter.
    #[must_use]
    pub fn listen_key(mut self, value: crate::SensitiveString) -> Self {
        self.listen_key = Some(value);
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
impl Request for KeepaliveUserDataStream {
    type Response = super::rest_models::KeepaliveUserDataStreamResponse;
    const EMPTY_RESPONSE: bool = true;
    const OP: Operation = Operation {
        name: "keepaliveUserDataStream",
        path: "/sapi/v1/margin/listen-key",
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
        validate_parameters(&p, &["listenKey"], &[], &[])?;
        super::validation::validate("keepaliveUserDataStream", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`startUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#start-user-data-stream).
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
    const OP: Operation = Operation {
        name: "startUserDataStream",
        path: "/sapi/v1/margin/listen-key",
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

/// Validated request builder for [`createSpecialKey`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#create-special-key).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateSpecialKey {
    #[serde(rename = "apiName", skip_serializing_if = "Option::is_none")]
    api_name: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    public_key: Option<crate::SensitiveString>,
    #[serde(rename = "permissionMode", skip_serializing_if = "Option::is_none")]
    permission_mode: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CreateSpecialKey {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `apiName` parameter.
    #[must_use]
    pub fn api_name(mut self, value: impl Into<String>) -> Self {
        self.api_name = Some(value.into());
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `ip` parameter.
    #[must_use]
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }
    /// Set the provider `publicKey` parameter.
    #[must_use]
    pub fn public_key(mut self, value: crate::SensitiveString) -> Self {
        self.public_key = Some(value);
        self
    }
    /// Set the provider `permissionMode` parameter.
    #[must_use]
    pub fn permission_mode(mut self, value: impl Into<String>) -> Self {
        self.permission_mode = Some(value.into());
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
impl Request for CreateSpecialKey {
    type Response = super::rest_models::CreateSpecialKeyResponse;
    const OP: Operation = Operation {
        name: "createSpecialKey",
        path: "/sapi/v1/margin/apiKey",
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
            &["apiName"],
            &[("permissionMode", &["TRADE", "READ"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("createSpecialKey", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`deleteSpecialKey`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#delete-special-key).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteSpecialKey {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    api_key: Option<crate::SensitiveString>,
    #[serde(rename = "apiName", skip_serializing_if = "Option::is_none")]
    api_name: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl DeleteSpecialKey {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `apiKey` parameter.
    #[must_use]
    pub fn api_key(mut self, value: crate::SensitiveString) -> Self {
        self.api_key = Some(value);
        self
    }
    /// Set the provider `apiName` parameter.
    #[must_use]
    pub fn api_name(mut self, value: impl Into<String>) -> Self {
        self.api_name = Some(value.into());
        self
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
impl Request for DeleteSpecialKey {
    type Response = super::rest_models::DeleteSpecialKeyResponse;
    const EMPTY_RESPONSE: bool = true;
    const OP: Operation = Operation {
        name: "deleteSpecialKey",
        path: "/sapi/v1/margin/apiKey",
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
        super::validation::validate("deleteSpecialKey", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`querySpecialKey`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-special-key).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QuerySpecialKey {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    api_key: Option<crate::SensitiveString>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QuerySpecialKey {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `apiKey` parameter.
    #[must_use]
    pub fn api_key(mut self, value: crate::SensitiveString) -> Self {
        self.api_key = Some(value);
        self
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
impl Request for QuerySpecialKey {
    type Response = super::rest_models::QuerySpecialKeyResponse;
    const OP: Operation = Operation {
        name: "querySpecialKey",
        path: "/sapi/v1/margin/apiKey",
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
            &["apiKey"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("querySpecialKey", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`editIpForSpecialKey`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#edit-ip-for-special-key).
#[derive(Clone, Debug, Default, Serialize)]
pub struct EditIpForSpecialKey {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    api_key: Option<crate::SensitiveString>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl EditIpForSpecialKey {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `apiKey` parameter.
    #[must_use]
    pub fn api_key(mut self, value: crate::SensitiveString) -> Self {
        self.api_key = Some(value);
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `ip` parameter.
    #[must_use]
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
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
impl Request for EditIpForSpecialKey {
    type Response = super::rest_models::EditIpForSpecialKeyResponse;
    const EMPTY_RESPONSE: bool = true;
    const OP: Operation = Operation {
        name: "editIpForSpecialKey",
        path: "/sapi/v1/margin/apiKey/ip",
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
            &["apiKey", "ip"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("editIpForSpecialKey", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`exitSpecialKeyMode`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#exit-special-key-mode).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ExitSpecialKeyMode {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ExitSpecialKeyMode {
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
impl Request for ExitSpecialKeyMode {
    type Response = super::rest_models::ExitSpecialKeyModeResponse;
    const OP: Operation = Operation {
        name: "exitSpecialKeyMode",
        path: "/sapi/v1/margin/exit-special-key-mode",
        method: "POST",
        security: Security::Signed,
        mutation: true,
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("exitSpecialKeyMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getForceLiquidationRecord`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-force-liquidation-record).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetForceLiquidationRecord {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "isolatedSymbol", skip_serializing_if = "Option::is_none")]
    isolated_symbol: Option<crate::Symbol>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetForceLiquidationRecord {
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
    /// Set the provider `isolatedSymbol` parameter.
    #[must_use]
    pub fn isolated_symbol(mut self, value: crate::Symbol) -> Self {
        self.isolated_symbol = Some(value);
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
impl Request for GetForceLiquidationRecord {
    type Response = super::rest_models::GetForceLiquidationRecordResponse;
    const OP: Operation = Operation {
        name: "getForceLiquidationRecord",
        path: "/sapi/v1/margin/forceLiquidationRec",
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
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("getForceLiquidationRecord", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getSmallLiabilityExchangeCoinList`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-small-liability-exchange-coin-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSmallLiabilityExchangeCoinList {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetSmallLiabilityExchangeCoinList {
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
impl Request for GetSmallLiabilityExchangeCoinList {
    type Response = super::rest_models::GetSmallLiabilityExchangeCoinListResponse;
    const OP: Operation = Operation {
        name: "getSmallLiabilityExchangeCoinList",
        path: "/sapi/v1/margin/exchange-small-liability",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getSmallLiabilityExchangeCoinList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`smallLiabilityExchange`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#small-liability-exchange).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SmallLiabilityExchange {
    #[serde(rename = "assetNames", skip_serializing_if = "Option::is_none")]
    asset_names: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl SmallLiabilityExchange {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `assetNames` parameter.
    #[must_use]
    pub fn asset_names(mut self, value: impl Into<String>) -> Self {
        self.asset_names = Some(value.into());
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
impl Request for SmallLiabilityExchange {
    type Response = super::rest_models::SmallLiabilityExchangeResponse;
    const EMPTY_RESPONSE: bool = true;
    const OP: Operation = Operation {
        name: "smallLiabilityExchange",
        path: "/sapi/v1/margin/exchange-small-liability",
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
        validate_parameters(
            &p,
            &["assetNames"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("smallLiabilityExchange", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getSmallLiabilityExchangeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-small-liability-exchange-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSmallLiabilityExchangeHistory {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    current: Option<i64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetSmallLiabilityExchangeHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
impl Request for GetSmallLiabilityExchangeHistory {
    type Response = super::rest_models::GetSmallLiabilityExchangeHistoryResponse;
    const OP: Operation = Operation {
        name: "getSmallLiabilityExchangeHistory",
        path: "/sapi/v1/margin/exchange-small-liability-history",
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
            &["current", "size"],
            &[],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("getSmallLiabilityExchangeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountCancelAllOpenOrdersOnASymbol`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-all-open-orders-on-asymbol).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountCancelAllOpenOrdersOnASymbol {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountCancelAllOpenOrdersOnASymbol {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
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
impl Request for MarginAccountCancelAllOpenOrdersOnASymbol {
    type Response = super::rest_models::MarginAccountCancelAllOpenOrdersOnASymbolResponse;
    const OP: Operation = Operation {
        name: "marginAccountCancelAllOpenOrdersOnASymbol",
        path: "/sapi/v1/margin/openOrders",
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
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginAccountCancelAllOpenOrdersOnASymbol", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsOpenOrders {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
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
impl Request for QueryMarginAccountsOpenOrders {
    type Response = super::rest_models::QueryMarginAccountsOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "queryMarginAccountsOpenOrders",
        path: "/sapi/v1/margin/openOrders",
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
            &[],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMarginAccountsOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountCancelOco`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountCancelOco {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    order_list_id: Option<crate::margin::OrderListId>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountCancelOco {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `orderListId` parameter.
    #[must_use]
    pub fn order_list_id(mut self, value: crate::margin::OrderListId) -> Self {
        self.order_list_id = Some(value);
        self
    }
    /// Set the provider `listClientOrderId` parameter.
    #[must_use]
    pub fn list_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.list_client_order_id = Some(value);
        self
    }
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
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
impl Request for MarginAccountCancelOco {
    type Response = super::rest_models::MarginAccountCancelOcoResponse;
    const OP: Operation = Operation {
        name: "marginAccountCancelOco",
        path: "/sapi/v1/margin/orderList",
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
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginAccountCancelOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsOco`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsOco {
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    order_list_id: Option<crate::margin::OrderListId>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsOco {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `orderListId` parameter.
    #[must_use]
    pub fn order_list_id(mut self, value: crate::margin::OrderListId) -> Self {
        self.order_list_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
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
    const OP: Operation = Operation {
        name: "queryMarginAccountsOco",
        path: "/sapi/v1/margin/orderList",
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
            &[],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMarginAccountsOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountCancelOrder`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountCancelOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::margin::OrderId>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountCancelOrder {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::margin::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.orig_client_order_id = Some(value);
        self
    }
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
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
impl Request for MarginAccountCancelOrder {
    type Response = super::rest_models::MarginAccountCancelOrderResponse;
    const OP: Operation = Operation {
        name: "marginAccountCancelOrder",
        path: "/sapi/v1/margin/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
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
            &["symbol"],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginAccountCancelOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountNewOrder`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountNewOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
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
    new_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "sideEffectType", skip_serializing_if = "Option::is_none")]
    side_effect_type: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<i64>,
    #[serde(rename = "autoRepayAtCancel", skip_serializing_if = "Option::is_none")]
    auto_repay_at_cancel: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginAccountNewOrder {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
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
    pub fn new_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
        self
    }
    /// Set the provider `icebergQty` parameter.
    #[must_use]
    pub fn iceberg_qty(mut self, value: Decimal) -> Self {
        self.iceberg_qty = Some(value);
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
    /// Set the provider `trailingDelta` parameter.
    #[must_use]
    pub fn trailing_delta(mut self, value: i64) -> Self {
        self.trailing_delta = Some(value);
        self
    }
    /// Set the provider `autoRepayAtCancel` parameter.
    #[must_use]
    pub fn auto_repay_at_cancel(mut self, value: bool) -> Self {
        self.auto_repay_at_cancel = Some(value);
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
impl Request for MarginAccountNewOrder {
    type Response = super::rest_models::MarginAccountNewOrderResponse;
    const OP: Operation = Operation {
        name: "marginAccountNewOrder",
        path: "/sapi/v1/margin/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 6,
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
                ("isIsolated", &["TRUE", "FALSE"]),
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
                    &["EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH", "NONE"],
                ),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginAccountNewOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsOrder`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::margin::OrderId>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsOrder {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::margin::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `origClientOrderId` parameter.
    #[must_use]
    pub fn orig_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
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
impl Request for QueryMarginAccountsOrder {
    type Response = super::rest_models::QueryMarginAccountsOrderResponse;
    const OP: Operation = Operation {
        name: "queryMarginAccountsOrder",
        path: "/sapi/v1/margin/order",
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
            &["symbol"],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMarginAccountsOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountNewOco`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountNewOco {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "limitClientOrderId", skip_serializing_if = "Option::is_none")]
    limit_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "limitIcebergQty", skip_serializing_if = "Option::is_none")]
    limit_iceberg_qty: Option<Decimal>,
    #[serde(rename = "stopClientOrderId", skip_serializing_if = "Option::is_none")]
    stop_client_order_id: Option<crate::margin::ClientOrderId>,
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
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "autoRepayAtCancel", skip_serializing_if = "Option::is_none")]
    auto_repay_at_cancel: Option<bool>,
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `listClientOrderId` parameter.
    #[must_use]
    pub fn list_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
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
    pub fn limit_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
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
    pub fn stop_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
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
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `autoRepayAtCancel` parameter.
    #[must_use]
    pub fn auto_repay_at_cancel(mut self, value: bool) -> Self {
        self.auto_repay_at_cancel = Some(value);
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
    const OP: Operation = Operation {
        name: "marginAccountNewOco",
        path: "/sapi/v1/margin/order/oco",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 6,
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
                ("isIsolated", &["TRUE", "FALSE"]),
                ("side", &["BUY", "SELL"]),
                ("stopLimitTimeInForce", &["GTC", "FOK", "IOC"]),
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
                (
                    "selfTradePreventionMode",
                    &["EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH", "NONE"],
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

/// Validated request builder for [`marginAccountNewOto`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-oto).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountNewOto {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "sideEffectType", skip_serializing_if = "Option::is_none")]
    side_effect_type: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "autoRepayAtCancel", skip_serializing_if = "Option::is_none")]
    auto_repay_at_cancel: Option<bool>,
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    working_type: Option<String>,
    #[serde(rename = "workingSide", skip_serializing_if = "Option::is_none")]
    working_side: Option<String>,
    #[serde(
        rename = "workingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    working_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "workingPrice", skip_serializing_if = "Option::is_none")]
    working_price: Option<Decimal>,
    #[serde(rename = "workingQuantity", skip_serializing_if = "Option::is_none")]
    working_quantity: Option<Decimal>,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    working_iceberg_qty: Option<Decimal>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    working_time_in_force: Option<String>,
    #[serde(rename = "pendingType", skip_serializing_if = "Option::is_none")]
    pending_type: Option<String>,
    #[serde(rename = "pendingSide", skip_serializing_if = "Option::is_none")]
    pending_side: Option<String>,
    #[serde(
        rename = "pendingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "pendingPrice", skip_serializing_if = "Option::is_none")]
    pending_price: Option<Decimal>,
    #[serde(rename = "pendingStopPrice", skip_serializing_if = "Option::is_none")]
    pending_stop_price: Option<Decimal>,
    #[serde(
        rename = "pendingTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pending_trailing_delta: Option<Decimal>,
    #[serde(rename = "pendingQuantity", skip_serializing_if = "Option::is_none")]
    pending_quantity: Option<Decimal>,
    #[serde(rename = "pendingIcebergQty", skip_serializing_if = "Option::is_none")]
    pending_iceberg_qty: Option<Decimal>,
    #[serde(rename = "pendingTimeInForce", skip_serializing_if = "Option::is_none")]
    pending_time_in_force: Option<String>,
}
impl MarginAccountNewOto {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `listClientOrderId` parameter.
    #[must_use]
    pub fn list_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.list_client_order_id = Some(value);
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
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `autoRepayAtCancel` parameter.
    #[must_use]
    pub fn auto_repay_at_cancel(mut self, value: bool) -> Self {
        self.auto_repay_at_cancel = Some(value);
        self
    }
    /// Set the provider `workingType` parameter.
    #[must_use]
    pub fn working_type(mut self, value: impl Into<String>) -> Self {
        self.working_type = Some(value.into());
        self
    }
    /// Set the provider `workingSide` parameter.
    #[must_use]
    pub fn working_side(mut self, value: impl Into<String>) -> Self {
        self.working_side = Some(value.into());
        self
    }
    /// Set the provider `workingClientOrderId` parameter.
    #[must_use]
    pub fn working_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.working_client_order_id = Some(value);
        self
    }
    /// Set the provider `workingPrice` parameter.
    #[must_use]
    pub fn working_price(mut self, value: Decimal) -> Self {
        self.working_price = Some(value);
        self
    }
    /// Set the provider `workingQuantity` parameter.
    #[must_use]
    pub fn working_quantity(mut self, value: Decimal) -> Self {
        self.working_quantity = Some(value);
        self
    }
    /// Set the provider `workingIcebergQty` parameter.
    #[must_use]
    pub fn working_iceberg_qty(mut self, value: Decimal) -> Self {
        self.working_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `workingTimeInForce` parameter.
    #[must_use]
    pub fn working_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.working_time_in_force = Some(value.into());
        self
    }
    /// Set the provider `pendingType` parameter.
    #[must_use]
    pub fn pending_type(mut self, value: impl Into<String>) -> Self {
        self.pending_type = Some(value.into());
        self
    }
    /// Set the provider `pendingSide` parameter.
    #[must_use]
    pub fn pending_side(mut self, value: impl Into<String>) -> Self {
        self.pending_side = Some(value.into());
        self
    }
    /// Set the provider `pendingClientOrderId` parameter.
    #[must_use]
    pub fn pending_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.pending_client_order_id = Some(value);
        self
    }
    /// Set the provider `pendingPrice` parameter.
    #[must_use]
    pub fn pending_price(mut self, value: Decimal) -> Self {
        self.pending_price = Some(value);
        self
    }
    /// Set the provider `pendingStopPrice` parameter.
    #[must_use]
    pub fn pending_stop_price(mut self, value: Decimal) -> Self {
        self.pending_stop_price = Some(value);
        self
    }
    /// Set the provider `pendingTrailingDelta` parameter.
    #[must_use]
    pub fn pending_trailing_delta(mut self, value: Decimal) -> Self {
        self.pending_trailing_delta = Some(value);
        self
    }
    /// Set the provider `pendingQuantity` parameter.
    #[must_use]
    pub fn pending_quantity(mut self, value: Decimal) -> Self {
        self.pending_quantity = Some(value);
        self
    }
    /// Set the provider `pendingIcebergQty` parameter.
    #[must_use]
    pub fn pending_iceberg_qty(mut self, value: Decimal) -> Self {
        self.pending_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `pendingTimeInForce` parameter.
    #[must_use]
    pub fn pending_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.pending_time_in_force = Some(value.into());
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
impl Request for MarginAccountNewOto {
    type Response = super::rest_models::MarginAccountNewOtoResponse;
    const OP: Operation = Operation {
        name: "marginAccountNewOto",
        path: "/sapi/v1/margin/order/oto",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 6,
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
                "listClientOrderId",
                "pendingClientOrderId",
                "pendingQuantity",
                "pendingSide",
                "pendingType",
                "symbol",
                "workingClientOrderId",
                "workingPrice",
                "workingQuantity",
                "workingSide",
                "workingType",
            ],
            &[
                ("isIsolated", &["TRUE", "FALSE"]),
                ("newOrderRespType", &["ACK", "RESULT", "FULL"]),
                (
                    "sideEffectType",
                    &["NO_SIDE_EFFECT", "MARGIN_BUY", "AUTO_BORROW_REPAY"],
                ),
                (
                    "selfTradePreventionMode",
                    &["EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH", "NONE"],
                ),
                ("workingType", &["LIMIT", "LIMIT_MAKER"]),
                ("workingSide", &["BUY", "SELL"]),
                ("workingTimeInForce", &["GTC", "IOC", "FOK"]),
                (
                    "pendingType",
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
                ("pendingSide", &["BUY", "SELL"]),
                ("pendingTimeInForce", &["GTC", "IOC", "FOK"]),
            ],
            &[],
        )?;
        super::validation::validate("marginAccountNewOto", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginAccountNewOtoco`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-otoco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginAccountNewOtoco {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "sideEffectType", skip_serializing_if = "Option::is_none")]
    side_effect_type: Option<String>,
    #[serde(rename = "autoRepayAtCancel", skip_serializing_if = "Option::is_none")]
    auto_repay_at_cancel: Option<bool>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    working_type: Option<String>,
    #[serde(rename = "workingSide", skip_serializing_if = "Option::is_none")]
    working_side: Option<String>,
    #[serde(
        rename = "workingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    working_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "workingPrice", skip_serializing_if = "Option::is_none")]
    working_price: Option<Decimal>,
    #[serde(rename = "workingQuantity", skip_serializing_if = "Option::is_none")]
    working_quantity: Option<Decimal>,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    working_iceberg_qty: Option<Decimal>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    working_time_in_force: Option<String>,
    #[serde(rename = "pendingSide", skip_serializing_if = "Option::is_none")]
    pending_side: Option<String>,
    #[serde(rename = "pendingQuantity", skip_serializing_if = "Option::is_none")]
    pending_quantity: Option<Decimal>,
    #[serde(rename = "pendingAboveType", skip_serializing_if = "Option::is_none")]
    pending_above_type: Option<String>,
    #[serde(
        rename = "pendingAboveClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "pendingAbovePrice", skip_serializing_if = "Option::is_none")]
    pending_above_price: Option<Decimal>,
    #[serde(
        rename = "pendingAboveStopPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_stop_price: Option<Decimal>,
    #[serde(
        rename = "pendingAboveTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_trailing_delta: Option<Decimal>,
    #[serde(
        rename = "pendingAboveIcebergQty",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_iceberg_qty: Option<Decimal>,
    #[serde(
        rename = "pendingAboveTimeInForce",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_time_in_force: Option<String>,
    #[serde(rename = "pendingBelowType", skip_serializing_if = "Option::is_none")]
    pending_below_type: Option<String>,
    #[serde(
        rename = "pendingBelowClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_client_order_id: Option<crate::margin::ClientOrderId>,
    #[serde(rename = "pendingBelowPrice", skip_serializing_if = "Option::is_none")]
    pending_below_price: Option<Decimal>,
    #[serde(
        rename = "pendingBelowStopPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_stop_price: Option<Decimal>,
    #[serde(
        rename = "pendingBelowTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_trailing_delta: Option<Decimal>,
    #[serde(
        rename = "pendingBelowIcebergQty",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_iceberg_qty: Option<Decimal>,
    #[serde(
        rename = "pendingBelowTimeInForce",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_time_in_force: Option<String>,
}
impl MarginAccountNewOtoco {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `sideEffectType` parameter.
    #[must_use]
    pub fn side_effect_type(mut self, value: impl Into<String>) -> Self {
        self.side_effect_type = Some(value.into());
        self
    }
    /// Set the provider `autoRepayAtCancel` parameter.
    #[must_use]
    pub fn auto_repay_at_cancel(mut self, value: bool) -> Self {
        self.auto_repay_at_cancel = Some(value);
        self
    }
    /// Set the provider `listClientOrderId` parameter.
    #[must_use]
    pub fn list_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.list_client_order_id = Some(value);
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
    /// Set the provider `workingType` parameter.
    #[must_use]
    pub fn working_type(mut self, value: impl Into<String>) -> Self {
        self.working_type = Some(value.into());
        self
    }
    /// Set the provider `workingSide` parameter.
    #[must_use]
    pub fn working_side(mut self, value: impl Into<String>) -> Self {
        self.working_side = Some(value.into());
        self
    }
    /// Set the provider `workingClientOrderId` parameter.
    #[must_use]
    pub fn working_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.working_client_order_id = Some(value);
        self
    }
    /// Set the provider `workingPrice` parameter.
    #[must_use]
    pub fn working_price(mut self, value: Decimal) -> Self {
        self.working_price = Some(value);
        self
    }
    /// Set the provider `workingQuantity` parameter.
    #[must_use]
    pub fn working_quantity(mut self, value: Decimal) -> Self {
        self.working_quantity = Some(value);
        self
    }
    /// Set the provider `workingIcebergQty` parameter.
    #[must_use]
    pub fn working_iceberg_qty(mut self, value: Decimal) -> Self {
        self.working_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `workingTimeInForce` parameter.
    #[must_use]
    pub fn working_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.working_time_in_force = Some(value.into());
        self
    }
    /// Set the provider `pendingSide` parameter.
    #[must_use]
    pub fn pending_side(mut self, value: impl Into<String>) -> Self {
        self.pending_side = Some(value.into());
        self
    }
    /// Set the provider `pendingQuantity` parameter.
    #[must_use]
    pub fn pending_quantity(mut self, value: Decimal) -> Self {
        self.pending_quantity = Some(value);
        self
    }
    /// Set the provider `pendingAboveType` parameter.
    #[must_use]
    pub fn pending_above_type(mut self, value: impl Into<String>) -> Self {
        self.pending_above_type = Some(value.into());
        self
    }
    /// Set the provider `pendingAboveClientOrderId` parameter.
    #[must_use]
    pub fn pending_above_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.pending_above_client_order_id = Some(value);
        self
    }
    /// Set the provider `pendingAbovePrice` parameter.
    #[must_use]
    pub fn pending_above_price(mut self, value: Decimal) -> Self {
        self.pending_above_price = Some(value);
        self
    }
    /// Set the provider `pendingAboveStopPrice` parameter.
    #[must_use]
    pub fn pending_above_stop_price(mut self, value: Decimal) -> Self {
        self.pending_above_stop_price = Some(value);
        self
    }
    /// Set the provider `pendingAboveTrailingDelta` parameter.
    #[must_use]
    pub fn pending_above_trailing_delta(mut self, value: Decimal) -> Self {
        self.pending_above_trailing_delta = Some(value);
        self
    }
    /// Set the provider `pendingAboveIcebergQty` parameter.
    #[must_use]
    pub fn pending_above_iceberg_qty(mut self, value: Decimal) -> Self {
        self.pending_above_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `pendingAboveTimeInForce` parameter.
    #[must_use]
    pub fn pending_above_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.pending_above_time_in_force = Some(value.into());
        self
    }
    /// Set the provider `pendingBelowType` parameter.
    #[must_use]
    pub fn pending_below_type(mut self, value: impl Into<String>) -> Self {
        self.pending_below_type = Some(value.into());
        self
    }
    /// Set the provider `pendingBelowClientOrderId` parameter.
    #[must_use]
    pub fn pending_below_client_order_id(mut self, value: crate::margin::ClientOrderId) -> Self {
        self.pending_below_client_order_id = Some(value);
        self
    }
    /// Set the provider `pendingBelowPrice` parameter.
    #[must_use]
    pub fn pending_below_price(mut self, value: Decimal) -> Self {
        self.pending_below_price = Some(value);
        self
    }
    /// Set the provider `pendingBelowStopPrice` parameter.
    #[must_use]
    pub fn pending_below_stop_price(mut self, value: Decimal) -> Self {
        self.pending_below_stop_price = Some(value);
        self
    }
    /// Set the provider `pendingBelowTrailingDelta` parameter.
    #[must_use]
    pub fn pending_below_trailing_delta(mut self, value: Decimal) -> Self {
        self.pending_below_trailing_delta = Some(value);
        self
    }
    /// Set the provider `pendingBelowIcebergQty` parameter.
    #[must_use]
    pub fn pending_below_iceberg_qty(mut self, value: Decimal) -> Self {
        self.pending_below_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `pendingBelowTimeInForce` parameter.
    #[must_use]
    pub fn pending_below_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.pending_below_time_in_force = Some(value.into());
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
impl Request for MarginAccountNewOtoco {
    type Response = super::rest_models::MarginAccountNewOtocoResponse;
    const OP: Operation = Operation {
        name: "marginAccountNewOtoco",
        path: "/sapi/v1/margin/order/otoco",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 6,
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
                "listClientOrderId",
                "pendingAboveClientOrderId",
                "pendingAboveType",
                "pendingBelowClientOrderId",
                "pendingQuantity",
                "pendingSide",
                "symbol",
                "workingClientOrderId",
                "workingPrice",
                "workingQuantity",
                "workingSide",
                "workingType",
            ],
            &[
                ("isIsolated", &["TRUE", "FALSE"]),
                (
                    "sideEffectType",
                    &["NO_SIDE_EFFECT", "MARGIN_BUY", "AUTO_BORROW_REPAY"],
                ),
                ("newOrderRespType", &["ACK", "RESULT", "FULL"]),
                (
                    "selfTradePreventionMode",
                    &["EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH", "NONE"],
                ),
                ("workingType", &["LIMIT", "LIMIT_MAKER"]),
                ("workingSide", &["BUY", "SELL"]),
                ("workingTimeInForce", &["GTC", "IOC", "FOK"]),
                ("pendingSide", &["BUY", "SELL"]),
                (
                    "pendingAboveType",
                    &["LIMIT_MAKER", "STOP_LOSS", "STOP_LOSS_LIMIT"],
                ),
                ("pendingAboveTimeInForce", &["GTC", "IOC", "FOK"]),
                (
                    "pendingBelowType",
                    &["LIMIT_MAKER", "STOP_LOSS", "STOP_LOSS_LIMIT"],
                ),
                ("pendingBelowTimeInForce", &["GTC", "IOC", "FOK"]),
            ],
            &[],
        )?;
        super::validation::validate("marginAccountNewOtoco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`marginManualLiquidation`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-manual-liquidation).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MarginManualLiquidation {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl MarginManualLiquidation {
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
impl Request for MarginManualLiquidation {
    type Response = super::rest_models::MarginManualLiquidationResponse;
    const OP: Operation = Operation {
        name: "marginManualLiquidation",
        path: "/sapi/v1/margin/manual-liquidation",
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
        validate_parameters(
            &p,
            &["type"],
            &[("type", &["MARGIN", "ISOLATED"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("marginManualLiquidation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentMarginOrderCountUsage`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-current-margin-order-count-usage).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentMarginOrderCountUsage {
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCurrentMarginOrderCountUsage {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
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
impl Request for QueryCurrentMarginOrderCountUsage {
    type Response = super::rest_models::QueryCurrentMarginOrderCountUsageResponse;
    const OP: Operation = Operation {
        name: "queryCurrentMarginOrderCountUsage",
        path: "/sapi/v1/margin/rateLimit/order",
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
        validate_parameters(
            &p,
            &[],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryCurrentMarginOrderCountUsage", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsAllOco`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-all-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsAllOco {
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<crate::margin::OrderListId>,
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `fromId` parameter.
    #[must_use]
    pub fn from_id(mut self, value: crate::margin::OrderListId) -> Self {
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
    const OP: Operation = Operation {
        name: "queryMarginAccountsAllOco",
        path: "/sapi/v1/margin/allOrderList",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 200,
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
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryMarginAccountsAllOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsAllOrders`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-all-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsAllOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::margin::OrderId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsAllOrders {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::margin::OrderId) -> Self {
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
impl Request for QueryMarginAccountsAllOrders {
    type Response = super::rest_models::QueryMarginAccountsAllOrdersResponse;
    const OP: Operation = Operation {
        name: "queryMarginAccountsAllOrders",
        path: "/sapi/v1/margin/allOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 200,
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
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[
                ("limit", -9_223_372_036_854_775_808, 500),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryMarginAccountsAllOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsOpenOco`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsOpenOco {
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsOpenOco {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
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
impl Request for QueryMarginAccountsOpenOco {
    type Response = super::rest_models::QueryMarginAccountsOpenOcoResponse;
    const OP: Operation = Operation {
        name: "queryMarginAccountsOpenOco",
        path: "/sapi/v1/margin/openOrderList",
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
            &[],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMarginAccountsOpenOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsOpenOtootocoOrderLists`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-otootoco-order-lists).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsOpenOtootocoOrderLists {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsOpenOtootocoOrderLists {
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
impl Request for QueryMarginAccountsOpenOtootocoOrderLists {
    type Response = super::rest_models::QueryMarginAccountsOpenOtootocoOrderListsResponse;
    const OP: Operation = Operation {
        name: "queryMarginAccountsOpenOtootocoOrderLists",
        path: "/sapi/v1/margin/oto/openOrderList",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 10,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::future_open_order_lists,
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
        super::validation::validate("queryMarginAccountsOpenOtootocoOrderLists", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMarginAccountsTradeList`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMarginAccountsTradeList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::margin::OrderId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<crate::margin::TradeId>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMarginAccountsTradeList {
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
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::margin::OrderId) -> Self {
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
    pub fn from_id(mut self, value: crate::margin::TradeId) -> Self {
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
impl Request for QueryMarginAccountsTradeList {
    type Response = super::rest_models::QueryMarginAccountsTradeListResponse;
    const OP: Operation = Operation {
        name: "queryMarginAccountsTradeList",
        path: "/sapi/v1/margin/myTrades",
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
            &["symbol"],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("queryMarginAccountsTradeList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryPreventedMatches`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-prevented-matches).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryPreventedMatches {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "preventedMatchId", skip_serializing_if = "Option::is_none")]
    prevented_match_id: Option<crate::margin::PreventedMatchId>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::margin::OrderId>,
    #[serde(
        rename = "fromPreventedMatchId",
        skip_serializing_if = "Option::is_none"
    )]
    from_prevented_match_id: Option<crate::margin::PreventedMatchId>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryPreventedMatches {
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
    /// Set the provider `preventedMatchId` parameter.
    #[must_use]
    pub fn prevented_match_id(mut self, value: crate::margin::PreventedMatchId) -> Self {
        self.prevented_match_id = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::margin::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `fromPreventedMatchId` parameter.
    #[must_use]
    pub fn from_prevented_match_id(mut self, value: crate::margin::PreventedMatchId) -> Self {
        self.from_prevented_match_id = Some(value);
        self
    }
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: impl Into<String>) -> Self {
        self.is_isolated = Some(value.into());
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
impl Request for QueryPreventedMatches {
    type Response = super::rest_models::QueryPreventedMatchesResponse;
    const OP: Operation = Operation {
        name: "queryPreventedMatches",
        path: "/sapi/v1/margin/myPreventedMatches",
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
            &["symbol"],
            &[("isIsolated", &["TRUE", "FALSE"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryPreventedMatches", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`querySpecialKeyList`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-special-key-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QuerySpecialKeyList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QuerySpecialKeyList {
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
impl Request for QuerySpecialKeyList {
    type Response = super::rest_models::QuerySpecialKeyListResponse;
    const OP: Operation = Operation {
        name: "querySpecialKeyList",
        path: "/sapi/v1/margin/api-key-list",
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
        super::validation::validate("querySpecialKeyList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryLiquidationLoan`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-liquidation-loan).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryLiquidationLoan {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryLiquidationLoan {
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
impl Request for QueryLiquidationLoan {
    type Response = super::rest_models::QueryLiquidationLoanResponse;
    const OP: Operation = Operation {
        name: "queryLiquidationLoan",
        path: "/sapi/v1/margin/liquidation-loan",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryLiquidationLoan", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`liquidationLoanRepay`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#liquidation-loan-repay).
#[derive(Clone, Debug, Default, Serialize)]
pub struct LiquidationLoanRepay {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl LiquidationLoanRepay {
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
impl Request for LiquidationLoanRepay {
    type Response = super::rest_models::LiquidationLoanRepayResponse;
    const OP: Operation = Operation {
        name: "liquidationLoanRepay",
        path: "/sapi/v1/margin/liquidation-loan/repay",
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
        super::validation::validate("liquidationLoanRepay", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryLiquidationLoanRepayHistory`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-liquidation-loan-repay-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryLiquidationLoanRepayHistory {
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
impl QueryLiquidationLoanRepayHistory {
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
impl Request for QueryLiquidationLoanRepayHistory {
    type Response = super::rest_models::QueryLiquidationLoanRepayHistoryResponse;
    const OP: Operation = Operation {
        name: "queryLiquidationLoanRepayHistory",
        path: "/sapi/v1/margin/liquidation-loan/repay-history",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryLiquidationLoanRepayHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCrossMarginTransferHistory`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/transfer#get-cross-margin-transfer-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCrossMarginTransferHistory {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
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
    #[serde(rename = "isolatedSymbol", skip_serializing_if = "Option::is_none")]
    isolated_symbol: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetCrossMarginTransferHistory {
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
    /// Set the provider `isolatedSymbol` parameter.
    #[must_use]
    pub fn isolated_symbol(mut self, value: crate::Symbol) -> Self {
        self.isolated_symbol = Some(value);
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
impl Request for GetCrossMarginTransferHistory {
    type Response = super::rest_models::GetCrossMarginTransferHistoryResponse;
    const OP: Operation = Operation {
        name: "getCrossMarginTransferHistory",
        path: "/sapi/v1/margin/transfer",
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
            &[("type", &["ROLL_IN", "ROLL_OUT"])],
            &[
                ("current", 1, 9_223_372_036_854_775_807),
                ("size", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("getCrossMarginTransferHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryMaxTransferOutAmount`](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/transfer#query-max-transfer-out-amount).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryMaxTransferOutAmount {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    asset: Option<crate::Asset>,
    #[serde(rename = "isolatedSymbol", skip_serializing_if = "Option::is_none")]
    isolated_symbol: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryMaxTransferOutAmount {
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
    /// Set the provider `isolatedSymbol` parameter.
    #[must_use]
    pub fn isolated_symbol(mut self, value: crate::Symbol) -> Self {
        self.isolated_symbol = Some(value);
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
impl Request for QueryMaxTransferOutAmount {
    type Response = super::rest_models::QueryMaxTransferOutAmountResponse;
    const OP: Operation = Operation {
        name: "queryMaxTransferOutAmount",
        path: "/sapi/v1/margin/maxTransferable",
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
        validate_parameters(
            &p,
            &["asset"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryMaxTransferOutAmount", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`createUserListenToken`](https://developers.binance.com/en/docs/products/margin-trading/listen-token-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateUserListenToken {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::Symbol>,
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    is_isolated: Option<bool>,
    #[serde(rename = "validity", skip_serializing_if = "Option::is_none")]
    validity: Option<i64>,
}
impl CreateUserListenToken {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `isIsolated` parameter.
    #[must_use]
    pub fn is_isolated(mut self, value: bool) -> Self {
        self.is_isolated = Some(value);
        self
    }
    /// Set the provider `validity` parameter.
    #[must_use]
    pub fn validity(mut self, value: i64) -> Self {
        self.validity = Some(value);
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
impl Request for CreateUserListenToken {
    type Response = super::rest_models::CreateUserListenTokenResponse;
    const OP: Operation = Operation {
        name: "createUserListenToken",
        path: "/sapi/v1/userListenToken",
        method: "POST",
        security: Security::Unresolved,
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
        validate_parameters(&p, &[], &[], &[("validity", 1, 86_400_000)])?;
        super::validation::validate("createUserListenToken", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [adjustCrossMarginMaxLeverage](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#adjust-cross-margin-max-leverage).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn adjust_cross_margin_max_leverage(
        &self,
        request: &AdjustCrossMarginMaxLeverage,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AdjustCrossMarginMaxLeverageResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [disableIsolatedMarginAccount](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#disable-isolated-margin-account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn disable_isolated_margin_account(
        &self,
        request: &DisableIsolatedMarginAccount,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DisableIsolatedMarginAccountResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [enableIsolatedMarginAccount](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#enable-isolated-margin-account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn enable_isolated_margin_account(
        &self,
        request: &EnableIsolatedMarginAccount,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::EnableIsolatedMarginAccountResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryIsolatedMarginAccountInfo](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-isolated-margin-account-info).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_isolated_margin_account_info(
        &self,
        request: &QueryIsolatedMarginAccountInfo,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryIsolatedMarginAccountInfoResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getBnbBurnStatus](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#get-bnb-burn-status).
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

    /// [getSummaryOfMarginAccount](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#get-summary-of-margin-account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_summary_of_margin_account(
        &self,
        request: &GetSummaryOfMarginAccount,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetSummaryOfMarginAccountResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCrossIsolatedMarginCapitalFlow](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-isolated-margin-capital-flow).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_cross_isolated_margin_capital_flow(
        &self,
        request: &QueryCrossIsolatedMarginCapitalFlow,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryCrossIsolatedMarginCapitalFlowResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryCrossMarginAccountDetails](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-margin-account-details).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_cross_margin_account_details(
        &self,
        request: &QueryCrossMarginAccountDetails,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCrossMarginAccountDetailsResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryCrossMarginFeeData](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-cross-margin-fee-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_cross_margin_fee_data(
        &self,
        request: &QueryCrossMarginFeeData,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCrossMarginFeeDataResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryEnabledIsolatedMarginAccountLimit](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-enabled-isolated-margin-account-limit).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_enabled_isolated_margin_account_limit(
        &self,
        request: &QueryEnabledIsolatedMarginAccountLimit,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryEnabledIsolatedMarginAccountLimitResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryIsolatedMarginFeeData](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/account#query-isolated-margin-fee-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_isolated_margin_fee_data(
        &self,
        request: &QueryIsolatedMarginFeeData,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryIsolatedMarginFeeDataResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getFutureHourlyInterestRate](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#get-future-hourly-interest-rate).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_future_hourly_interest_rate(
        &self,
        request: &GetFutureHourlyInterestRate,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetFutureHourlyInterestRateResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getInterestHistory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#get-interest-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_interest_history(
        &self,
        request: &GetInterestHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetInterestHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountBorrowRepay](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#margin-account-borrow-repay).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_borrow_repay(
        &self,
        request: &MarginAccountBorrowRepay,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountBorrowRepayResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryBorrowRepayRecordsInMarginAccount](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-borrow-repay-records-in-margin-account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_borrow_repay_records_in_margin_account(
        &self,
        request: &QueryBorrowRepayRecordsInMarginAccount,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryBorrowRepayRecordsInMarginAccountResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginInterestRateHistory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-margin-interest-rate-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_interest_rate_history(
        &self,
        request: &QueryMarginInterestRateHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginInterestRateHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMaxBorrow](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/borrow-repay#query-max-borrow).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_max_borrow(
        &self,
        request: &QueryMaxBorrow,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::BorrowCapacity>, Error> {
        let asset = request
            .asset
            .clone()
            .ok_or(Error::Validation("asset provenance required"))?;
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::BorrowCapacity {
                asset,
                isolated_symbol: request.isolated_symbol.clone(),
                capacity: response.data,
            },
            meta: response.meta,
        })
    }

    /// [crossMarginCollateralRatio](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#cross-margin-collateral-ratio).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cross_margin_collateral_ratio(
        &self,
        request: &CrossMarginCollateralRatio,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CrossMarginCollateralRatioResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getAllCrossMarginPairs](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-cross-margin-pairs).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_all_cross_margin_pairs(
        &self,
        request: &GetAllCrossMarginPairs,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetAllCrossMarginPairsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getAllIsolatedMarginSymbol](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-isolated-margin-symbol).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_all_isolated_margin_symbol(
        &self,
        request: &GetAllIsolatedMarginSymbol,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetAllIsolatedMarginSymbolResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getAllMarginAssets](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-all-margin-assets).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_all_margin_assets(
        &self,
        request: &GetAllMarginAssets,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetAllMarginAssetsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getDelistSchedule](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-delist-schedule).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_delist_schedule(
        &self,
        request: &GetDelistSchedule,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetDelistScheduleResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getLimitPricePairs](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-limit-price-pairs).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_limit_price_pairs(
        &self,
        request: &GetLimitPricePairs,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetLimitPricePairsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getListSchedule](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-list-schedule).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_list_schedule(
        &self,
        request: &GetListSchedule,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetListScheduleResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getMarginAssetRiskBasedLiquidationRatio](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-margin-asset-risk-based-liquidation-ratio).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_margin_asset_risk_based_liquidation_ratio(
        &self,
        request: &GetMarginAssetRiskBasedLiquidationRatio,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetMarginAssetRiskBasedLiquidationRatioResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [getMarginRestrictedAssets](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#get-margin-restricted-assets).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_margin_restricted_assets(
        &self,
        request: &GetMarginRestrictedAssets,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetMarginRestrictedAssetsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryIsolatedMarginTierData](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-isolated-margin-tier-data).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_isolated_margin_tier_data(
        &self,
        request: &QueryIsolatedMarginTierData,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryIsolatedMarginTierDataResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAvailableInventory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-margin-available-inventory).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_available_inventory(
        &self,
        request: &QueryMarginAvailableInventory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAvailableInventoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginPriceindex](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/market-data#query-margin-priceindex).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_priceindex(
        &self,
        request: &QueryMarginPriceindex,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginPriceindexResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [closeUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#close-user-data-stream).
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

    /// [keepaliveUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#keepalive-user-data-stream).
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

    /// [startUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/user-data-stream#start-user-data-stream).
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

    /// [createSpecialKey](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#create-special-key).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn create_special_key(
        &self,
        request: &CreateSpecialKey,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CreateSpecialKeyResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [deleteSpecialKey](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#delete-special-key).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn delete_special_key(
        &self,
        request: &DeleteSpecialKey,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DeleteSpecialKeyResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [querySpecialKey](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-special-key).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_special_key(
        &self,
        request: &QuerySpecialKey,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QuerySpecialKeyResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [editIpForSpecialKey](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#edit-ip-for-special-key).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn edit_ip_for_special_key(
        &self,
        request: &EditIpForSpecialKey,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::EditIpForSpecialKeyResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [exitSpecialKeyMode](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#exit-special-key-mode).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn exit_special_key_mode(
        &self,
        request: &ExitSpecialKeyMode,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ExitSpecialKeyModeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getForceLiquidationRecord](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-force-liquidation-record).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_force_liquidation_record(
        &self,
        request: &GetForceLiquidationRecord,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetForceLiquidationRecordResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getSmallLiabilityExchangeCoinList](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-small-liability-exchange-coin-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_small_liability_exchange_coin_list(
        &self,
        request: &GetSmallLiabilityExchangeCoinList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetSmallLiabilityExchangeCoinListResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [smallLiabilityExchange](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#small-liability-exchange).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn small_liability_exchange(
        &self,
        request: &SmallLiabilityExchange,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SmallLiabilityExchangeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getSmallLiabilityExchangeHistory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#get-small-liability-exchange-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_small_liability_exchange_history(
        &self,
        request: &GetSmallLiabilityExchangeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetSmallLiabilityExchangeHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountCancelAllOpenOrdersOnASymbol](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-all-open-orders-on-asymbol).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_cancel_all_open_orders_on_a_symbol(
        &self,
        request: &MarginAccountCancelAllOpenOrdersOnASymbol,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::MarginAccountCancelAllOpenOrdersOnASymbolResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_open_orders(
        &self,
        request: &QueryMarginAccountsOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountsOpenOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountCancelOco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-oco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_cancel_oco(
        &self,
        request: &MarginAccountCancelOco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountCancelOcoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsOco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-oco).
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

    /// [marginAccountCancelOrder](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-cancel-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_cancel_order(
        &self,
        request: &MarginAccountCancelOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountCancelOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountNewOrder](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_new_order(
        &self,
        request: &MarginAccountNewOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountNewOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsOrder](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_order(
        &self,
        request: &QueryMarginAccountsOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountsOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountNewOco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-oco).
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

    /// [marginAccountNewOto](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-oto).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_new_oto(
        &self,
        request: &MarginAccountNewOto,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountNewOtoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginAccountNewOtoco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-otoco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_account_new_otoco(
        &self,
        request: &MarginAccountNewOtoco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginAccountNewOtocoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [marginManualLiquidation](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-manual-liquidation).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn margin_manual_liquidation(
        &self,
        request: &MarginManualLiquidation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MarginManualLiquidationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentMarginOrderCountUsage](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-current-margin-order-count-usage).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_margin_order_count_usage(
        &self,
        request: &QueryCurrentMarginOrderCountUsage,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCurrentMarginOrderCountUsageResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsAllOco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-all-oco).
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

    /// [queryMarginAccountsAllOrders](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-all-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_all_orders(
        &self,
        request: &QueryMarginAccountsAllOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountsAllOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsOpenOco](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-oco).
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

    /// [queryMarginAccountsOpenOtootocoOrderLists](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-open-otootoco-order-lists).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_open_otootoco_order_lists(
        &self,
        request: &QueryMarginAccountsOpenOtootocoOrderLists,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryMarginAccountsOpenOtootocoOrderListsResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [queryMarginAccountsTradeList](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-margin-accounts-trade-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_margin_accounts_trade_list(
        &self,
        request: &QueryMarginAccountsTradeList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryMarginAccountsTradeListResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryPreventedMatches](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-prevented-matches).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_prevented_matches(
        &self,
        request: &QueryPreventedMatches,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryPreventedMatchesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [querySpecialKeyList](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-special-key-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_special_key_list(
        &self,
        request: &QuerySpecialKeyList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QuerySpecialKeyListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryLiquidationLoan](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-liquidation-loan).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_liquidation_loan(
        &self,
        request: &QueryLiquidationLoan,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryLiquidationLoanResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [liquidationLoanRepay](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#liquidation-loan-repay).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn liquidation_loan_repay(
        &self,
        request: &LiquidationLoanRepay,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::LiquidationLoanRepayResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryLiquidationLoanRepayHistory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#query-liquidation-loan-repay-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_liquidation_loan_repay_history(
        &self,
        request: &QueryLiquidationLoanRepayHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryLiquidationLoanRepayHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getCrossMarginTransferHistory](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/transfer#get-cross-margin-transfer-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_cross_margin_transfer_history(
        &self,
        request: &GetCrossMarginTransferHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetCrossMarginTransferHistoryResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryMaxTransferOutAmount](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/transfer#query-max-transfer-out-amount).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_max_transfer_out_amount(
        &self,
        request: &QueryMaxTransferOutAmount,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::TransferCapacity>, Error> {
        let asset = request
            .asset
            .clone()
            .ok_or(Error::Validation("asset provenance required"))?;
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::TransferCapacity {
                asset,
                isolated_symbol: request.isolated_symbol.clone(),
                available: response.data,
            },
            meta: response.meta,
        })
    }

    /// [createUserListenToken](https://developers.binance.com/en/docs/products/margin-trading/listen-token-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn create_user_listen_token(
        &self,
        request: &CreateUserListenToken,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ListenToken>, Error> {
        request.validate()?;
        let scope = if request.is_isolated == Some(true) {
            super::AccountScope::Isolated(
                request
                    .symbol
                    .clone()
                    .ok_or(Error::Validation("isolated token symbol required"))?,
            )
        } else {
            super::AccountScope::Cross
        };
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::ListenToken {
                scope,
                receipt: response.data,
            },
            meta: response.meta,
        })
    }
}
