// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated ws request builders.

use super::ClientOrderId;
use crate::Decimal;
use crate::Error;
use crate::Symbol;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`accountCommission`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountCommission {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl AccountCommission {
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
impl Request for AccountCommission {
    type Response = super::ws_models::AccountCommissionResponse;
    const OP: Operation = Operation {
        name: "accountCommission",
        path: "/account.commission",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 20,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("accountCommission", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountRateLimitsOrders`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountRateLimitsOrders {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl AccountRateLimitsOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for AccountRateLimitsOrders {
    type Response = super::ws_models::AccountRateLimitsOrdersResponse;
    const OP: Operation = Operation {
        name: "accountRateLimitsOrders",
        path: "/account.rateLimits.orders",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 40,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("accountRateLimitsOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountStatus`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountStatus {
    #[serde(rename = "omitZeroBalances", skip_serializing_if = "Option::is_none")]
    omit_zero_balances: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl AccountStatus {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `omitZeroBalances` parameter.
    #[must_use]
    pub fn omit_zero_balances(mut self, value: bool) -> Self {
        self.omit_zero_balances = Some(value);
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
    type Response = super::ws_models::AccountStatusResponse;
    const OP: Operation = Operation {
        name: "accountStatus",
        path: "/account.status",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 20,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("accountStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`allOrders`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
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
    recv_window: Option<Decimal>,
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
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for AllOrders {
    type Response = super::ws_models::AllOrdersResponse;
    const OP: Operation = Operation {
        name: "allOrders",
        path: "/allOrders",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 20,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("allOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`myTrades`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MyTrades {
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
    recv_window: Option<Decimal>,
}
impl MyTrades {
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
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for MyTrades {
    type Response = super::ws_models::MyTradesResponse;
    const OP: Operation = Operation {
        name: "myTrades",
        path: "/myTrades",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("myTrades", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openOrdersStatus`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenOrdersStatus {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OpenOrdersStatus {
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
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for OpenOrdersStatus {
    type Response = super::ws_models::OpenOrdersStatusResponse;
    const OP: Operation = Operation {
        name: "openOrdersStatus",
        path: "/openOrders.status",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("openOrdersStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderStatus`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderStatus {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderStatus {
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
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for OrderStatus {
    type Response = super::ws_models::OrderStatusResponse;
    const OP: Operation = Operation {
        name: "orderStatus",
        path: "/order.status",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 4,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("orderStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`exchangeInfo`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ExchangeInfo {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<String>>,
    #[serde(rename = "showPermissionSets", skip_serializing_if = "Option::is_none")]
    show_permission_sets: Option<bool>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl ExchangeInfo {
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
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: Vec<String>) -> Self {
        self.symbols = Some(value);
        self
    }
    /// Set the provider `permissions` parameter.
    #[must_use]
    pub fn permissions(mut self, value: Vec<String>) -> Self {
        self.permissions = Some(value);
        self
    }
    /// Set the provider `showPermissionSets` parameter.
    #[must_use]
    pub fn show_permission_sets(mut self, value: bool) -> Self {
        self.show_permission_sets = Some(value);
        self
    }
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for ExchangeInfo {
    type Response = super::ws_models::ExchangeInfoResponse;
    const OP: Operation = Operation {
        name: "exchangeInfo",
        path: "/exchangeInfo",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 20,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("exchangeInfo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`executionRules`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ExecutionRules {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl ExecutionRules {
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
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: Vec<String>) -> Self {
        self.symbols = Some(value);
        self
    }
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for ExecutionRules {
    type Response = super::ws_models::ExecutionRulesResponse;
    const OP: Operation = Operation {
        name: "executionRules",
        path: "/executionRules",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("executionRules", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`ping`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Ping {}
impl Ping {
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
impl Request for Ping {
    type Response = super::ws_models::PingResponse;
    const OP: Operation = Operation {
        name: "ping",
        path: "/ping",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("ping", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`time`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Time {}
impl Time {
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
impl Request for Time {
    type Response = super::ws_models::TimeResponse;
    const OP: Operation = Operation {
        name: "time",
        path: "/time",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("time", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`avgPrice`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AvgPrice {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl AvgPrice {
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
impl Request for AvgPrice {
    type Response = super::ws_models::AvgPriceResponse;
    const OP: Operation = Operation {
        name: "avgPrice",
        path: "/avgPrice",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("avgPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`depth`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Depth {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl Depth {
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
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for Depth {
    type Response = super::ws_models::DepthResponse;
    const OP: Operation = Operation {
        name: "depth",
        path: "/depth",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[("limit", -9_223_372_036_854_775_808, 5_000)],
        )?;
        super::validation::validate("depth", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`klines`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Klines {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl Klines {
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
    /// Set the provider `timeZone` parameter.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
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
impl Request for Klines {
    type Response = super::ws_models::KlinesResponse;
    const OP: Operation = Operation {
        name: "klines",
        path: "/klines",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["interval", "symbol"],
            &[(
                "interval",
                &[
                    "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h",
                    "1d", "3d", "1w", "1M",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("klines", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`ticker`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Ticker {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "windowSize", skip_serializing_if = "Option::is_none")]
    window_size: Option<String>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl Ticker {
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
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: Vec<String>) -> Self {
        self.symbols = Some(value);
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `windowSize` parameter.
    #[must_use]
    pub fn window_size(mut self, value: impl Into<String>) -> Self {
        self.window_size = Some(value.into());
        self
    }
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for Ticker {
    type Response = super::ws_models::TickerResponse;
    const OP: Operation = Operation {
        name: "ticker",
        path: "/ticker",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[
                ("type", &["FULL", "MINI"]),
                (
                    "windowSize",
                    &[
                        "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "10m", "11m", "12m",
                        "13m", "14m", "15m", "16m", "17m", "18m", "19m", "20m", "21m", "22m",
                        "23m", "24m", "25m", "26m", "27m", "28m", "29m", "30m", "31m", "32m",
                        "33m", "34m", "35m", "36m", "37m", "38m", "39m", "40m", "41m", "42m",
                        "43m", "44m", "45m", "46m", "47m", "48m", "49m", "50m", "51m", "52m",
                        "53m", "54m", "55m", "56m", "57m", "58m", "59m", "1h", "2h", "3h", "4h",
                        "5h", "6h", "7h", "8h", "9h", "10h", "11h", "12h", "13h", "14h", "15h",
                        "16h", "17h", "18h", "19h", "20h", "21h", "22h", "23h", "1d", "2d", "3d",
                        "4d", "5d", "6d", "7d",
                    ],
                ),
                ("symbolStatus", &["TRADING", "HALT", "BREAK"]),
            ],
            &[],
        )?;
        super::validation::validate("ticker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`ticker24hr`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Ticker24hr {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl Ticker24hr {
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
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: Vec<String>) -> Self {
        self.symbols = Some(value);
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for Ticker24hr {
    type Response = super::ws_models::Ticker24hrResponse;
    const OP: Operation = Operation {
        name: "ticker24hr",
        path: "/ticker.24hr",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[
                ("type", &["FULL", "MINI"]),
                ("symbolStatus", &["TRADING", "HALT", "BREAK"]),
            ],
            &[],
        )?;
        super::validation::validate("ticker24hr", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tickerBook`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TickerBook {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl TickerBook {
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
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: Vec<String>) -> Self {
        self.symbols = Some(value);
        self
    }
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for TickerBook {
    type Response = super::ws_models::TickerBookResponse;
    const OP: Operation = Operation {
        name: "tickerBook",
        path: "/ticker.book",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("tickerBook", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tickerPrice`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TickerPrice {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl TickerPrice {
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
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: Vec<String>) -> Self {
        self.symbols = Some(value);
        self
    }
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for TickerPrice {
    type Response = super::ws_models::TickerPriceResponse;
    const OP: Operation = Operation {
        name: "tickerPrice",
        path: "/ticker.price",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("tickerPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tickerTradingDay`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TickerTradingDay {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl TickerTradingDay {
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
    /// Set the provider `symbols` parameter.
    #[must_use]
    pub fn symbols(mut self, value: Vec<String>) -> Self {
        self.symbols = Some(value);
        self
    }
    /// Set the provider `timeZone` parameter.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }
    /// Set the provider `type` parameter.
    #[must_use]
    pub fn type_value(mut self, value: impl Into<String>) -> Self {
        self.type_value = Some(value.into());
        self
    }
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for TickerTradingDay {
    type Response = super::ws_models::TickerTradingDayResponse;
    const OP: Operation = Operation {
        name: "tickerTradingDay",
        path: "/ticker.tradingDay",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &[],
            &[
                ("type", &["FULL", "MINI"]),
                ("symbolStatus", &["TRADING", "HALT", "BREAK"]),
            ],
            &[],
        )?;
        super::validation::validate("tickerTradingDay", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tradesAggregate`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TradesAggregate {
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
impl TradesAggregate {
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
}
impl Request for TradesAggregate {
    type Response = super::ws_models::TradesAggregateResponse;
    const OP: Operation = Operation {
        name: "tradesAggregate",
        path: "/trades.aggregate",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 4,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("tradesAggregate", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tradesHistorical`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TradesHistorical {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl TradesHistorical {
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
impl Request for TradesHistorical {
    type Response = super::ws_models::TradesHistoricalResponse;
    const OP: Operation = Operation {
        name: "tradesHistorical",
        path: "/trades.historical",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 25,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("tradesHistorical", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`blockTradesHistorical`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct BlockTradesHistorical {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl BlockTradesHistorical {
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
impl Request for BlockTradesHistorical {
    type Response = super::ws_models::BlockTradesHistoricalResponse;
    const OP: Operation = Operation {
        name: "blockTradesHistorical",
        path: "/blockTrades.historical",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 25,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["fromId", "symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("blockTradesHistorical", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tradesRecent`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TradesRecent {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl TradesRecent {
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
}
impl Request for TradesRecent {
    type Response = super::ws_models::TradesRecentResponse;
    const OP: Operation = Operation {
        name: "tradesRecent",
        path: "/trades.recent",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 25,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("tradesRecent", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`uiKlines`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UiKlines {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    interval: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl UiKlines {
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
    /// Set the provider `timeZone` parameter.
    #[must_use]
    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
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
impl Request for UiKlines {
    type Response = super::ws_models::UiKlinesResponse;
    const OP: Operation = Operation {
        name: "uiKlines",
        path: "/uiKlines",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["interval", "symbol"],
            &[(
                "interval",
                &[
                    "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h",
                    "1d", "3d", "1w", "1M",
                ],
            )],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("uiKlines", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`referencePrice`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ReferencePrice {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
}
impl ReferencePrice {
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
impl Request for ReferencePrice {
    type Response = super::ws_models::ReferencePriceResponse;
    const OP: Operation = Operation {
        name: "referencePrice",
        path: "/referencePrice",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("referencePrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`referencePriceCalculation`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ReferencePriceCalculation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl ReferencePriceCalculation {
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
    /// Set the provider `symbolStatus` parameter.
    #[must_use]
    pub fn symbol_status(mut self, value: impl Into<String>) -> Self {
        self.symbol_status = Some(value.into());
        self
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
impl Request for ReferencePriceCalculation {
    type Response = super::ws_models::ReferencePriceCalculationResponse;
    const OP: Operation = Operation {
        name: "referencePriceCalculation",
        path: "/referencePrice.calculation",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("referencePriceCalculation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openOrdersCancelAll`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenOrdersCancelAll {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OpenOrdersCancelAll {
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
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for OpenOrdersCancelAll {
    type Response = super::ws_models::OpenOrdersCancelAllResponse;
    const OP: Operation = Operation {
        name: "openOrdersCancelAll",
        path: "/openOrders.cancelAll",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: Some(0),
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("openOrdersCancelAll", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderCancel`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderCancel {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    cancel_restrictions: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderCancel {
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
    /// Set the provider `cancelRestrictions` parameter.
    #[must_use]
    pub fn cancel_restrictions(mut self, value: impl Into<String>) -> Self {
        self.cancel_restrictions = Some(value.into());
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for OrderCancel {
    type Response = super::ws_models::OrderCancelResponse;
    const OP: Operation = Operation {
        name: "orderCancel",
        path: "/order.cancel",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: Some(0),
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[("cancelRestrictions", &["ONLY_NEW", "ONLY_PARTIALLY_FILLED"])],
            &[],
        )?;
        super::validation::validate("orderCancel", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderPlace`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderPlace {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<i64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "pegPriceType", skip_serializing_if = "Option::is_none")]
    peg_price_type: Option<String>,
    #[serde(rename = "pegOffsetValue", skip_serializing_if = "Option::is_none")]
    peg_offset_value: Option<i64>,
    #[serde(rename = "pegOffsetType", skip_serializing_if = "Option::is_none")]
    peg_offset_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderPlace {
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
    /// Set the provider `timeInForce` parameter.
    #[must_use]
    pub fn time_in_force(mut self, value: impl Into<String>) -> Self {
        self.time_in_force = Some(value.into());
        self
    }
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
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
    /// Set the provider `stopPrice` parameter.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }
    /// Set the provider `trailingDelta` parameter.
    #[must_use]
    pub fn trailing_delta(mut self, value: i64) -> Self {
        self.trailing_delta = Some(value);
        self
    }
    /// Set the provider `icebergQty` parameter.
    #[must_use]
    pub fn iceberg_qty(mut self, value: Decimal) -> Self {
        self.iceberg_qty = Some(value);
        self
    }
    /// Set the provider `strategyId` parameter.
    #[must_use]
    pub fn strategy_id(mut self, value: i64) -> Self {
        self.strategy_id = Some(value);
        self
    }
    /// Set the provider `strategyType` parameter.
    #[must_use]
    pub fn strategy_type(mut self, value: i64) -> Self {
        self.strategy_type = Some(value);
        self
    }
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `pegPriceType` parameter.
    #[must_use]
    pub fn peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pegOffsetValue` parameter.
    #[must_use]
    pub fn peg_offset_value(mut self, value: i64) -> Self {
        self.peg_offset_value = Some(value);
        self
    }
    /// Set the provider `pegOffsetType` parameter.
    #[must_use]
    pub fn peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for OrderPlace {
    type Response = super::ws_models::OrderPlaceResponse;
    const OP: Operation = Operation {
        name: "orderPlace",
        path: "/order.place",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: Some(0),
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
                        "MARKET",
                        "LIMIT",
                        "STOP_LOSS",
                        "STOP_LOSS_LIMIT",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                        "LIMIT_MAKER",
                    ],
                ),
                ("timeInForce", &["GTC", "IOC", "FOK"]),
                ("newOrderRespType", &["ACK", "RESULT", "FULL"]),
                (
                    "selfTradePreventionMode",
                    &[
                        "NONE",
                        "EXPIRE_TAKER",
                        "EXPIRE_MAKER",
                        "EXPIRE_BOTH",
                        "DECREMENT",
                        "TRANSFER",
                    ],
                ),
                ("pegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pegOffsetType", &["PRICE_LEVEL"]),
            ],
            &[
                ("strategyType", 1_000_000, 9_223_372_036_854_775_807),
                ("pegOffsetValue", -9_223_372_036_854_775_808, 100),
            ],
        )?;
        super::validation::validate("orderPlace", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderTest`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderTest {
    #[serde(
        rename = "computeCommissionRates",
        skip_serializing_if = "Option::is_none"
    )]
    compute_commission_rates: Option<bool>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<i64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "pegPriceType", skip_serializing_if = "Option::is_none")]
    peg_price_type: Option<String>,
    #[serde(rename = "pegOffsetValue", skip_serializing_if = "Option::is_none")]
    peg_offset_value: Option<i64>,
    #[serde(rename = "pegOffsetType", skip_serializing_if = "Option::is_none")]
    peg_offset_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderTest {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `computeCommissionRates` parameter.
    #[must_use]
    pub fn compute_commission_rates(mut self, value: bool) -> Self {
        self.compute_commission_rates = Some(value);
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
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
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
    /// Set the provider `stopPrice` parameter.
    #[must_use]
    pub fn stop_price(mut self, value: Decimal) -> Self {
        self.stop_price = Some(value);
        self
    }
    /// Set the provider `trailingDelta` parameter.
    #[must_use]
    pub fn trailing_delta(mut self, value: i64) -> Self {
        self.trailing_delta = Some(value);
        self
    }
    /// Set the provider `icebergQty` parameter.
    #[must_use]
    pub fn iceberg_qty(mut self, value: Decimal) -> Self {
        self.iceberg_qty = Some(value);
        self
    }
    /// Set the provider `strategyId` parameter.
    #[must_use]
    pub fn strategy_id(mut self, value: i64) -> Self {
        self.strategy_id = Some(value);
        self
    }
    /// Set the provider `strategyType` parameter.
    #[must_use]
    pub fn strategy_type(mut self, value: i64) -> Self {
        self.strategy_type = Some(value);
        self
    }
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
        self
    }
    /// Set the provider `pegPriceType` parameter.
    #[must_use]
    pub fn peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pegOffsetValue` parameter.
    #[must_use]
    pub fn peg_offset_value(mut self, value: i64) -> Self {
        self.peg_offset_value = Some(value);
        self
    }
    /// Set the provider `pegOffsetType` parameter.
    #[must_use]
    pub fn peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for OrderTest {
    type Response = super::ws_models::OrderTestResponse;
    const OP: Operation = Operation {
        name: "orderTest",
        path: "/order.test",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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
                        "MARKET",
                        "LIMIT",
                        "STOP_LOSS",
                        "STOP_LOSS_LIMIT",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                        "LIMIT_MAKER",
                    ],
                ),
                ("timeInForce", &["GTC", "IOC", "FOK"]),
                ("newOrderRespType", &["ACK", "RESULT", "FULL"]),
                (
                    "selfTradePreventionMode",
                    &[
                        "NONE",
                        "EXPIRE_TAKER",
                        "EXPIRE_MAKER",
                        "EXPIRE_BOTH",
                        "DECREMENT",
                        "TRANSFER",
                    ],
                ),
                ("pegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pegOffsetType", &["PRICE_LEVEL"]),
            ],
            &[
                ("strategyType", 1_000_000, 9_223_372_036_854_775_807),
                ("pegOffsetValue", -9_223_372_036_854_775_808, 100),
            ],
        )?;
        super::validation::validate("orderTest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`sessionSubscriptions`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SessionSubscriptions {}
impl SessionSubscriptions {
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
impl Request for SessionSubscriptions {
    type Response = super::ws_models::SessionSubscriptionsResponse;
    const OP: Operation = Operation {
        name: "sessionSubscriptions",
        path: "/session.subscriptions",
        method: "POST",
        security: Security::Public,
        mutation: false,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("sessionSubscriptions", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userDataStreamSubscribe`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserDataStreamSubscribe {}
impl UserDataStreamSubscribe {
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
impl Request for UserDataStreamSubscribe {
    type Response = super::ws_models::UserDataStreamSubscribeResponse;
    const OP: Operation = Operation {
        name: "userDataStreamSubscribe",
        path: "/userDataStream.subscribe",
        method: "POST",
        security: Security::Public,
        mutation: true,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("userDataStreamSubscribe", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userDataStreamSubscribeSignature`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserDataStreamSubscribeSignature {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl UserDataStreamSubscribeSignature {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: Decimal) -> Self {
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
impl Request for UserDataStreamSubscribeSignature {
    type Response = super::ws_models::UserDataStreamSubscribeSignatureResponse;
    const OP: Operation = Operation {
        name: "userDataStreamSubscribeSignature",
        path: "/userDataStream.subscribe.signature",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("userDataStreamSubscribeSignature", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userDataStreamUnsubscribe`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserDataStreamUnsubscribe {
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    subscription_id: Option<i64>,
}
impl UserDataStreamUnsubscribe {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `subscriptionId` parameter.
    #[must_use]
    pub fn subscription_id(mut self, value: i64) -> Self {
        self.subscription_id = Some(value);
        self
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
impl Request for UserDataStreamUnsubscribe {
    type Response = super::ws_models::UserDataStreamUnsubscribeResponse;
    const OP: Operation = Operation {
        name: "userDataStreamUnsubscribe",
        path: "/userDataStream.unsubscribe",
        method: "POST",
        security: Security::Public,
        mutation: true,
        weight: 2,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("userDataStreamUnsubscribe", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::WsClient {
    /// [accountCommission](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_commission(
        &self,
        request: &AccountCommission,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::AccountCommissionResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [accountRateLimitsOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_rate_limits_orders(
        &self,
        request: &AccountRateLimitsOrders,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::AccountRateLimitsOrdersResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [accountStatus](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_status(
        &self,
        request: &AccountStatus,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::AccountStatusResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [allOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn all_orders(
        &self,
        request: &AllOrders,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::AllOrdersResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [myTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn my_trades(
        &self,
        request: &MyTrades,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::MyTradesResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [openOrdersStatus](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn open_orders_status(
        &self,
        request: &OpenOrdersStatus,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::OpenOrdersStatusResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [orderStatus](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_status(
        &self,
        request: &OrderStatus,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::OrderStatusResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn exchange_info(
        &self,
        request: &ExchangeInfo,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::ExchangeInfoResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [executionRules](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn execution_rules(
        &self,
        request: &ExecutionRules,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::ExecutionRulesResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [ping](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ping(
        &self,
        request: &Ping,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::PingResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [time](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn time(
        &self,
        request: &Time,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TimeResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [avgPrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn avg_price(
        &self,
        request: &AvgPrice,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::AvgPriceResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [depth](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn depth(
        &self,
        request: &Depth,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::DepthResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [klines](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn klines(
        &self,
        request: &Klines,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::KlinesResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [ticker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker(
        &self,
        request: &Ticker,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TickerResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [ticker24hr](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker24hr(
        &self,
        request: &Ticker24hr,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::Ticker24hrResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [tickerBook](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker_book(
        &self,
        request: &TickerBook,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TickerBookResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [tickerPrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker_price(
        &self,
        request: &TickerPrice,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TickerPriceResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [tickerTradingDay](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker_trading_day(
        &self,
        request: &TickerTradingDay,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TickerTradingDayResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [tradesAggregate](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn trades_aggregate(
        &self,
        request: &TradesAggregate,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TradesAggregateResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [tradesHistorical](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn trades_historical(
        &self,
        request: &TradesHistorical,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TradesHistoricalResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [blockTradesHistorical](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn block_trades_historical(
        &self,
        request: &BlockTradesHistorical,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::BlockTradesHistoricalResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [tradesRecent](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn trades_recent(
        &self,
        request: &TradesRecent,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::TradesRecentResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [uiKlines](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ui_klines(
        &self,
        request: &UiKlines,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::UiKlinesResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [referencePrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn reference_price(
        &self,
        request: &ReferencePrice,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::ReferencePriceResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [referencePriceCalculation](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn reference_price_calculation(
        &self,
        request: &ReferencePriceCalculation,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::ReferencePriceCalculationResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [openOrdersCancelAll](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn open_orders_cancel_all(
        &self,
        request: &OpenOrdersCancelAll,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::OpenOrdersCancelAllResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [orderCancel](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_cancel(
        &self,
        request: &OrderCancel,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::OrderCancelResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [orderPlace](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_place(
        &self,
        request: &OrderPlace,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::OrderPlaceResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [orderTest](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_test(
        &self,
        request: &OrderTest,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::OrderTestResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [sessionSubscriptions](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn session_subscriptions(
        &self,
        request: &SessionSubscriptions,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::SessionSubscriptionsResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [userDataStreamSubscribe](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_data_stream_subscribe(
        &self,
        request: &UserDataStreamSubscribe,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::UserDataStreamSubscribeResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [userDataStreamSubscribeSignature](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_data_stream_subscribe_signature(
        &self,
        request: &UserDataStreamSubscribeSignature,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::UserDataStreamSubscribeSignatureResponse>, Error>
    {
        self.execute(request, id, deadline).await
    }

    /// [userDataStreamUnsubscribe](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_data_stream_unsubscribe(
        &self,
        request: &UserDataStreamUnsubscribe,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::UserDataStreamUnsubscribeResponse>, Error> {
        self.execute(request, id, deadline).await
    }
}
