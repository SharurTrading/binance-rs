// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use super::ClientOrderId;
use crate::Decimal;
use crate::Error;
use crate::Symbol;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`accountCommission`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
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
impl Request for AccountCommission {
    type Response = super::rest_models::AccountCommissionResponse;
    const OP: Operation = Operation {
        name: "accountCommission",
        path: "/api/v3/account/commission",
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
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("accountCommission", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`allOrders`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
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
        path: "/api/v3/allOrders",
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
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("allOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getAccount`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAccount {
    #[serde(rename = "omitZeroBalances", skip_serializing_if = "Option::is_none")]
    omit_zero_balances: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl GetAccount {
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
impl Request for GetAccount {
    type Response = super::rest_models::GetAccountResponse;
    const OP: Operation = Operation {
        name: "getAccount",
        path: "/api/v3/account",
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getAccount", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl GetOpenOrders {
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
impl Request for GetOpenOrders {
    type Response = super::rest_models::GetOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "getOpenOrders",
        path: "/api/v3/openOrders",
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`deleteOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl DeleteOpenOrders {
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
impl Request for DeleteOpenOrders {
    type Response = super::rest_models::DeleteOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "deleteOpenOrders",
        path: "/api/v3/openOrders",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: Some(0),
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("deleteOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getOrder`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl GetOrder {
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
impl Request for GetOrder {
    type Response = super::rest_models::GetOrderResponse;
    const OP: Operation = Operation {
        name: "getOrder",
        path: "/api/v3/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 4,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("getOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`deleteOrder`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteOrder {
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
impl DeleteOrder {
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
impl Request for DeleteOrder {
    type Response = super::rest_models::DeleteOrderResponse;
    const OP: Operation = Operation {
        name: "deleteOrder",
        path: "/api/v3/order",
        method: "DELETE",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: Some(0),
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["symbol"],
            &[("cancelRestrictions", &["ONLY_NEW", "ONLY_PARTIALLY_FILLED"])],
            &[],
        )?;
        super::validation::validate("deleteOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newOrder`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<i64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
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
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
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
        path: "/api/v3/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: Some(0),
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
        super::validation::validate("newOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`myTrades`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
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
impl Request for MyTrades {
    type Response = super::rest_models::MyTradesResponse;
    const OP: Operation = Operation {
        name: "myTrades",
        path: "/api/v3/myTrades",
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

/// Validated request builder for [`rateLimitOrder`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RateLimitOrder {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl RateLimitOrder {
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
impl Request for RateLimitOrder {
    type Response = super::rest_models::RateLimitOrderResponse;
    const OP: Operation = Operation {
        name: "rateLimitOrder",
        path: "/api/v3/rateLimit/order",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 40,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("rateLimitOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`exchangeInfo`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
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
impl Request for ExchangeInfo {
    type Response = super::rest_models::ExchangeInfoResponse;
    const OP: Operation = Operation {
        name: "exchangeInfo",
        path: "/api/v3/exchangeInfo",
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

/// Validated request builder for [`executionRules`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
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
impl Request for ExecutionRules {
    type Response = super::rest_models::ExecutionRulesResponse;
    const OP: Operation = Operation {
        name: "executionRules",
        path: "/api/v3/executionRules",
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
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("executionRules", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`ping`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
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
impl Request for Ping {
    type Response = super::rest_models::PingResponse;
    const OP: Operation = Operation {
        name: "ping",
        path: "/api/v3/ping",
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
        super::validation::validate("ping", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`time`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
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
impl Request for Time {
    type Response = super::rest_models::TimeResponse;
    const OP: Operation = Operation {
        name: "time",
        path: "/api/v3/time",
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
        super::validation::validate("time", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`aggTrades`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AggTrades {
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
impl AggTrades {
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
impl Request for AggTrades {
    type Response = super::rest_models::AggTradesResponse;
    const OP: Operation = Operation {
        name: "aggTrades",
        path: "/api/v3/aggTrades",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 4,
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
        super::validation::validate("aggTrades", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`avgPrice`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
impl Request for AvgPrice {
    type Response = super::rest_models::AvgPriceResponse;
    const OP: Operation = Operation {
        name: "avgPrice",
        path: "/api/v3/avgPrice",
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
        super::validation::validate("avgPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`depth`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
impl Request for Depth {
    type Response = super::rest_models::DepthResponse;
    const OP: Operation = Operation {
        name: "depth",
        path: "/api/v3/depth",
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
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[("limit", -9_223_372_036_854_775_808, 5_000)],
        )?;
        super::validation::validate("depth", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getTrades`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetTrades {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl GetTrades {
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
impl Request for GetTrades {
    type Response = super::rest_models::GetTradesResponse;
    const OP: Operation = Operation {
        name: "getTrades",
        path: "/api/v3/trades",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 25,
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
        super::validation::validate("getTrades", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`historicalTrades`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct HistoricalTrades {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
}
impl HistoricalTrades {
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
impl Request for HistoricalTrades {
    type Response = super::rest_models::HistoricalTradesResponse;
    const OP: Operation = Operation {
        name: "historicalTrades",
        path: "/api/v3/historicalTrades",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 25,
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
        super::validation::validate("historicalTrades", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`historicalBlockTrades`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct HistoricalBlockTrades {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl HistoricalBlockTrades {
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
impl Request for HistoricalBlockTrades {
    type Response = super::rest_models::HistoricalBlockTradesResponse;
    const OP: Operation = Operation {
        name: "historicalBlockTrades",
        path: "/api/v3/historicalBlockTrades",
        method: "GET",
        security: Security::Key,
        mutation: false,
        weight: 25,
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
            &["fromId", "symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("historicalBlockTrades", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`klines`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
    time_zone: Option<Decimal>,
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
    pub fn time_zone(mut self, value: Decimal) -> Self {
        self.time_zone = Some(value);
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
impl Request for Klines {
    type Response = super::rest_models::KlinesResponse;
    const OP: Operation = Operation {
        name: "klines",
        path: "/api/v3/klines",
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

/// Validated request builder for [`ticker`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Ticker {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "windowSize", skip_serializing_if = "Option::is_none")]
    window_size: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
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
    /// Set the provider `windowSize` parameter.
    #[must_use]
    pub fn window_size(mut self, value: impl Into<String>) -> Self {
        self.window_size = Some(value.into());
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
impl Request for Ticker {
    type Response = super::rest_models::TickerResponse;
    const OP: Operation = Operation {
        name: "ticker",
        path: "/api/v3/ticker",
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
            &[
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
                ("type", &["FULL", "MINI"]),
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

/// Validated request builder for [`ticker24hr`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
impl Request for Ticker24hr {
    type Response = super::rest_models::Ticker24hrResponse;
    const OP: Operation = Operation {
        name: "ticker24hr",
        path: "/api/v3/ticker/24hr",
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

/// Validated request builder for [`tickerBookTicker`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TickerBookTicker {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    symbol_status: Option<String>,
}
impl TickerBookTicker {
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
impl Request for TickerBookTicker {
    type Response = super::rest_models::TickerBookTickerResponse;
    const OP: Operation = Operation {
        name: "tickerBookTicker",
        path: "/api/v3/ticker/bookTicker",
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
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("tickerBookTicker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tickerPrice`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
impl Request for TickerPrice {
    type Response = super::rest_models::TickerPriceResponse;
    const OP: Operation = Operation {
        name: "tickerPrice",
        path: "/api/v3/ticker/price",
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
            &[("symbolStatus", &["TRADING", "HALT", "BREAK"])],
            &[],
        )?;
        super::validation::validate("tickerPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tickerTradingDay`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TickerTradingDay {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    time_zone: Option<Decimal>,
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
    pub fn time_zone(mut self, value: Decimal) -> Self {
        self.time_zone = Some(value);
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
impl Request for TickerTradingDay {
    type Response = super::rest_models::TickerTradingDayResponse;
    const OP: Operation = Operation {
        name: "tickerTradingDay",
        path: "/api/v3/ticker/tradingDay",
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

/// Validated request builder for [`uiKlines`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
    time_zone: Option<Decimal>,
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
    pub fn time_zone(mut self, value: Decimal) -> Self {
        self.time_zone = Some(value);
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
impl Request for UiKlines {
    type Response = super::rest_models::UiKlinesResponse;
    const OP: Operation = Operation {
        name: "uiKlines",
        path: "/api/v3/uiKlines",
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

/// Validated request builder for [`referencePrice`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
impl Request for ReferencePrice {
    type Response = super::rest_models::ReferencePriceResponse;
    const OP: Operation = Operation {
        name: "referencePrice",
        path: "/api/v3/referencePrice",
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
        super::validation::validate("referencePrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`referencePriceCalculation`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
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
impl Request for ReferencePriceCalculation {
    type Response = super::rest_models::ReferencePriceCalculationResponse;
    const OP: Operation = Operation {
        name: "referencePriceCalculation",
        path: "/api/v3/referencePrice/calculation",
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

/// Validated request builder for [`orderTest`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
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
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<i64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
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
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
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
impl Request for OrderTest {
    type Response = super::rest_models::OrderTestResponse;
    const OP: Operation = Operation {
        name: "orderTest",
        path: "/api/v3/order/test",
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
            &[],
        )?;
        super::validation::validate("orderTest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`allOrderList`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#all-order-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AllOrderList {
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl AllOrderList {
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
impl Request for AllOrderList {
    type Response = super::rest_models::AllOrderListResponse;
    const OP: Operation = Operation {
        name: "allOrderList",
        path: "/api/v3/allOrderList",
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
            &[],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("allOrderList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getOrderList`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#get-order-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetOrderList {
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    order_list_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl GetOrderList {
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
impl Request for GetOrderList {
    type Response = super::rest_models::GetOrderListResponse;
    const OP: Operation = Operation {
        name: "getOrderList",
        path: "/api/v3/orderList",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 4,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("getOrderList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`deleteOrderList`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#delete-order-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteOrderList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    order_list_id: Option<i64>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl DeleteOrderList {
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
impl Request for DeleteOrderList {
    type Response = super::rest_models::DeleteOrderListResponse;
    const OP: Operation = Operation {
        name: "deleteOrderList",
        path: "/api/v3/orderList",
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
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("deleteOrderList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`myAllocations`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-allocations).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MyAllocations {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "fromAllocationId", skip_serializing_if = "Option::is_none")]
    from_allocation_id: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl MyAllocations {
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
    /// Set the provider `fromAllocationId` parameter.
    #[must_use]
    pub fn from_allocation_id(mut self, value: i64) -> Self {
        self.from_allocation_id = Some(value);
        self
    }
    /// Set the provider `limit` parameter.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
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
impl Request for MyAllocations {
    type Response = super::rest_models::MyAllocationsResponse;
    const OP: Operation = Operation {
        name: "myAllocations",
        path: "/api/v3/myAllocations",
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
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("myAllocations", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`myFilters`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-filters).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MyFilters {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl MyFilters {
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
impl Request for MyFilters {
    type Response = super::rest_models::MyFiltersResponse;
    const OP: Operation = Operation {
        name: "myFilters",
        path: "/api/v3/myFilters",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 40,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("myFilters", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`myPreventedMatches`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-prevented-matches).
#[derive(Clone, Debug, Default, Serialize)]
pub struct MyPreventedMatches {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "preventedMatchId", skip_serializing_if = "Option::is_none")]
    prevented_match_id: Option<i64>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(
        rename = "fromPreventedMatchId",
        skip_serializing_if = "Option::is_none"
    )]
    from_prevented_match_id: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl MyPreventedMatches {
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
    pub fn prevented_match_id(mut self, value: i64) -> Self {
        self.prevented_match_id = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `fromPreventedMatchId` parameter.
    #[must_use]
    pub fn from_prevented_match_id(mut self, value: i64) -> Self {
        self.from_prevented_match_id = Some(value);
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
impl Request for MyPreventedMatches {
    type Response = super::rest_models::MyPreventedMatchesResponse;
    const OP: Operation = Operation {
        name: "myPreventedMatches",
        path: "/api/v3/myPreventedMatches",
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
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("myPreventedMatches", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openOrderList`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#open-order-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenOrderList {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OpenOrderList {
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
impl Request for OpenOrderList {
    type Response = super::rest_models::OpenOrderListResponse;
    const OP: Operation = Operation {
        name: "openOrderList",
        path: "/api/v3/openOrderList",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 6,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("openOrderList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderAmendments`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#order-amendments).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderAmendments {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "fromExecutionId", skip_serializing_if = "Option::is_none")]
    from_execution_id: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderAmendments {
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
    /// Set the provider `fromExecutionId` parameter.
    #[must_use]
    pub fn from_execution_id(mut self, value: i64) -> Self {
        self.from_execution_id = Some(value);
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
impl Request for OrderAmendments {
    type Response = super::rest_models::OrderAmendmentsResponse;
    const OP: Operation = Operation {
        name: "orderAmendments",
        path: "/api/v3/order/amendments",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 4,
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
            &["orderId", "symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("orderAmendments", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderAmendKeepPriority`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-amend-keep-priority).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderAmendKeepPriority {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "newQty", skip_serializing_if = "Option::is_none")]
    new_qty: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderAmendKeepPriority {
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
    /// Set the provider `newQty` parameter.
    #[must_use]
    pub fn new_qty(mut self, value: Decimal) -> Self {
        self.new_qty = Some(value);
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
impl Request for OrderAmendKeepPriority {
    type Response = super::rest_models::OrderAmendKeepPriorityResponse;
    const OP: Operation = Operation {
        name: "orderAmendKeepPriority",
        path: "/api/v3/order/amend/keepPriority",
        method: "PUT",
        security: Security::Signed,
        mutation: true,
        weight: 4,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["newQty", "symbol"], &[], &[])?;
        super::validation::validate("orderAmendKeepPriority", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderCancelReplace`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-cancel-replace).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderCancelReplace {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "cancelReplaceMode", skip_serializing_if = "Option::is_none")]
    cancel_replace_mode: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    quote_order_qty: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(
        rename = "cancelNewClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    cancel_new_client_order_id: Option<ClientOrderId>,
    #[serde(
        rename = "cancelOrigClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    cancel_orig_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "cancelOrderId", skip_serializing_if = "Option::is_none")]
    cancel_order_id: Option<i64>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<i64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    cancel_restrictions: Option<String>,
    #[serde(
        rename = "orderRateLimitExceededMode",
        skip_serializing_if = "Option::is_none"
    )]
    order_rate_limit_exceeded_mode: Option<String>,
    #[serde(rename = "pegPriceType", skip_serializing_if = "Option::is_none")]
    peg_price_type: Option<String>,
    #[serde(rename = "pegOffsetValue", skip_serializing_if = "Option::is_none")]
    peg_offset_value: Option<i64>,
    #[serde(rename = "pegOffsetType", skip_serializing_if = "Option::is_none")]
    peg_offset_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderCancelReplace {
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
    /// Set the provider `cancelReplaceMode` parameter.
    #[must_use]
    pub fn cancel_replace_mode(mut self, value: impl Into<String>) -> Self {
        self.cancel_replace_mode = Some(value.into());
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
    /// Set the provider `cancelNewClientOrderId` parameter.
    #[must_use]
    pub fn cancel_new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.cancel_new_client_order_id = Some(value);
        self
    }
    /// Set the provider `cancelOrigClientOrderId` parameter.
    #[must_use]
    pub fn cancel_orig_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.cancel_orig_client_order_id = Some(value);
        self
    }
    /// Set the provider `cancelOrderId` parameter.
    #[must_use]
    pub fn cancel_order_id(mut self, value: i64) -> Self {
        self.cancel_order_id = Some(value);
        self
    }
    /// Set the provider `newClientOrderId` parameter.
    #[must_use]
    pub fn new_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.new_client_order_id = Some(value);
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
    /// Set the provider `cancelRestrictions` parameter.
    #[must_use]
    pub fn cancel_restrictions(mut self, value: impl Into<String>) -> Self {
        self.cancel_restrictions = Some(value.into());
        self
    }
    /// Set the provider `orderRateLimitExceededMode` parameter.
    #[must_use]
    pub fn order_rate_limit_exceeded_mode(mut self, value: impl Into<String>) -> Self {
        self.order_rate_limit_exceeded_mode = Some(value.into());
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
impl Request for OrderCancelReplace {
    type Response = super::rest_models::OrderCancelReplaceResponse;
    const OP: Operation = Operation {
        name: "orderCancelReplace",
        path: "/api/v3/order/cancelReplace",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        requests_per_second: None,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: Some(super::validation::partial),
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["cancelReplaceMode", "side", "symbol", "type"],
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
                ("cancelReplaceMode", &["STOP_ON_FAILURE", "ALLOW_FAILURE"]),
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
                ("cancelRestrictions", &["ONLY_NEW", "ONLY_PARTIALLY_FILLED"]),
                ("orderRateLimitExceededMode", &["DO_NOTHING", "CANCEL_ONLY"]),
                ("pegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pegOffsetType", &["PRICE_LEVEL"]),
            ],
            &[],
        )?;
        super::validation::validate("orderCancelReplace", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderListOco`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderListOco {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "aboveType", skip_serializing_if = "Option::is_none")]
    above_type: Option<String>,
    #[serde(rename = "aboveClientOrderId", skip_serializing_if = "Option::is_none")]
    above_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "aboveIcebergQty", skip_serializing_if = "Option::is_none")]
    above_iceberg_qty: Option<i64>,
    #[serde(rename = "abovePrice", skip_serializing_if = "Option::is_none")]
    above_price: Option<Decimal>,
    #[serde(rename = "aboveStopPrice", skip_serializing_if = "Option::is_none")]
    above_stop_price: Option<Decimal>,
    #[serde(rename = "aboveTrailingDelta", skip_serializing_if = "Option::is_none")]
    above_trailing_delta: Option<i64>,
    #[serde(rename = "aboveTimeInForce", skip_serializing_if = "Option::is_none")]
    above_time_in_force: Option<String>,
    #[serde(rename = "aboveStrategyId", skip_serializing_if = "Option::is_none")]
    above_strategy_id: Option<i64>,
    #[serde(rename = "aboveStrategyType", skip_serializing_if = "Option::is_none")]
    above_strategy_type: Option<i64>,
    #[serde(rename = "abovePegPriceType", skip_serializing_if = "Option::is_none")]
    above_peg_price_type: Option<String>,
    #[serde(rename = "abovePegOffsetType", skip_serializing_if = "Option::is_none")]
    above_peg_offset_type: Option<String>,
    #[serde(
        rename = "abovePegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    above_peg_offset_value: Option<i64>,
    #[serde(rename = "belowType", skip_serializing_if = "Option::is_none")]
    below_type: Option<String>,
    #[serde(rename = "belowClientOrderId", skip_serializing_if = "Option::is_none")]
    below_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "belowIcebergQty", skip_serializing_if = "Option::is_none")]
    below_iceberg_qty: Option<i64>,
    #[serde(rename = "belowPrice", skip_serializing_if = "Option::is_none")]
    below_price: Option<Decimal>,
    #[serde(rename = "belowStopPrice", skip_serializing_if = "Option::is_none")]
    below_stop_price: Option<Decimal>,
    #[serde(rename = "belowTrailingDelta", skip_serializing_if = "Option::is_none")]
    below_trailing_delta: Option<i64>,
    #[serde(rename = "belowTimeInForce", skip_serializing_if = "Option::is_none")]
    below_time_in_force: Option<String>,
    #[serde(rename = "belowStrategyId", skip_serializing_if = "Option::is_none")]
    below_strategy_id: Option<i64>,
    #[serde(rename = "belowStrategyType", skip_serializing_if = "Option::is_none")]
    below_strategy_type: Option<i64>,
    #[serde(rename = "belowPegPriceType", skip_serializing_if = "Option::is_none")]
    below_peg_price_type: Option<String>,
    #[serde(rename = "belowPegOffsetType", skip_serializing_if = "Option::is_none")]
    below_peg_offset_type: Option<String>,
    #[serde(
        rename = "belowPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    below_peg_offset_value: Option<i64>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderListOco {
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
    /// Set the provider `aboveType` parameter.
    #[must_use]
    pub fn above_type(mut self, value: impl Into<String>) -> Self {
        self.above_type = Some(value.into());
        self
    }
    /// Set the provider `aboveClientOrderId` parameter.
    #[must_use]
    pub fn above_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.above_client_order_id = Some(value);
        self
    }
    /// Set the provider `aboveIcebergQty` parameter.
    #[must_use]
    pub fn above_iceberg_qty(mut self, value: i64) -> Self {
        self.above_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `abovePrice` parameter.
    #[must_use]
    pub fn above_price(mut self, value: Decimal) -> Self {
        self.above_price = Some(value);
        self
    }
    /// Set the provider `aboveStopPrice` parameter.
    #[must_use]
    pub fn above_stop_price(mut self, value: Decimal) -> Self {
        self.above_stop_price = Some(value);
        self
    }
    /// Set the provider `aboveTrailingDelta` parameter.
    #[must_use]
    pub fn above_trailing_delta(mut self, value: i64) -> Self {
        self.above_trailing_delta = Some(value);
        self
    }
    /// Set the provider `aboveTimeInForce` parameter.
    #[must_use]
    pub fn above_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.above_time_in_force = Some(value.into());
        self
    }
    /// Set the provider `aboveStrategyId` parameter.
    #[must_use]
    pub fn above_strategy_id(mut self, value: i64) -> Self {
        self.above_strategy_id = Some(value);
        self
    }
    /// Set the provider `aboveStrategyType` parameter.
    #[must_use]
    pub fn above_strategy_type(mut self, value: i64) -> Self {
        self.above_strategy_type = Some(value);
        self
    }
    /// Set the provider `abovePegPriceType` parameter.
    #[must_use]
    pub fn above_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.above_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `abovePegOffsetType` parameter.
    #[must_use]
    pub fn above_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.above_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `abovePegOffsetValue` parameter.
    #[must_use]
    pub fn above_peg_offset_value(mut self, value: i64) -> Self {
        self.above_peg_offset_value = Some(value);
        self
    }
    /// Set the provider `belowType` parameter.
    #[must_use]
    pub fn below_type(mut self, value: impl Into<String>) -> Self {
        self.below_type = Some(value.into());
        self
    }
    /// Set the provider `belowClientOrderId` parameter.
    #[must_use]
    pub fn below_client_order_id(mut self, value: ClientOrderId) -> Self {
        self.below_client_order_id = Some(value);
        self
    }
    /// Set the provider `belowIcebergQty` parameter.
    #[must_use]
    pub fn below_iceberg_qty(mut self, value: i64) -> Self {
        self.below_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `belowPrice` parameter.
    #[must_use]
    pub fn below_price(mut self, value: Decimal) -> Self {
        self.below_price = Some(value);
        self
    }
    /// Set the provider `belowStopPrice` parameter.
    #[must_use]
    pub fn below_stop_price(mut self, value: Decimal) -> Self {
        self.below_stop_price = Some(value);
        self
    }
    /// Set the provider `belowTrailingDelta` parameter.
    #[must_use]
    pub fn below_trailing_delta(mut self, value: i64) -> Self {
        self.below_trailing_delta = Some(value);
        self
    }
    /// Set the provider `belowTimeInForce` parameter.
    #[must_use]
    pub fn below_time_in_force(mut self, value: impl Into<String>) -> Self {
        self.below_time_in_force = Some(value.into());
        self
    }
    /// Set the provider `belowStrategyId` parameter.
    #[must_use]
    pub fn below_strategy_id(mut self, value: i64) -> Self {
        self.below_strategy_id = Some(value);
        self
    }
    /// Set the provider `belowStrategyType` parameter.
    #[must_use]
    pub fn below_strategy_type(mut self, value: i64) -> Self {
        self.below_strategy_type = Some(value);
        self
    }
    /// Set the provider `belowPegPriceType` parameter.
    #[must_use]
    pub fn below_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.below_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `belowPegOffsetType` parameter.
    #[must_use]
    pub fn below_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.below_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `belowPegOffsetValue` parameter.
    #[must_use]
    pub fn below_peg_offset_value(mut self, value: i64) -> Self {
        self.below_peg_offset_value = Some(value);
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
impl Request for OrderListOco {
    type Response = super::rest_models::OrderListOcoResponse;
    const OP: Operation = Operation {
        name: "orderListOco",
        path: "/api/v3/orderList/oco",
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
            &["aboveType", "belowType", "quantity", "side", "symbol"],
            &[
                ("side", &["BUY", "SELL"]),
                (
                    "aboveType",
                    &[
                        "STOP_LOSS_LIMIT",
                        "STOP_LOSS",
                        "LIMIT_MAKER",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                    ],
                ),
                ("aboveTimeInForce", &["GTC", "IOC", "FOK"]),
                ("abovePegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("abovePegOffsetType", &["PRICE_LEVEL"]),
                (
                    "belowType",
                    &[
                        "STOP_LOSS",
                        "STOP_LOSS_LIMIT",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                    ],
                ),
                ("belowTimeInForce", &["GTC", "IOC", "FOK"]),
                ("belowPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("belowPegOffsetType", &["PRICE_LEVEL"]),
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
            ],
            &[],
        )?;
        super::validation::validate("orderListOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderListOpo`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-opo).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderListOpo {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
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
    working_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "workingPrice", skip_serializing_if = "Option::is_none")]
    working_price: Option<Decimal>,
    #[serde(rename = "workingQuantity", skip_serializing_if = "Option::is_none")]
    working_quantity: Option<Decimal>,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    working_iceberg_qty: Option<Decimal>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    working_strategy_id: Option<i64>,
    #[serde(
        rename = "workingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    working_strategy_type: Option<i64>,
    #[serde(
        rename = "workingPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_price_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_value: Option<i64>,
    #[serde(rename = "pendingType", skip_serializing_if = "Option::is_none")]
    pending_type: Option<String>,
    #[serde(rename = "pendingSide", skip_serializing_if = "Option::is_none")]
    pending_side: Option<String>,
    #[serde(
        rename = "pendingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "pendingPrice", skip_serializing_if = "Option::is_none")]
    pending_price: Option<Decimal>,
    #[serde(rename = "pendingStopPrice", skip_serializing_if = "Option::is_none")]
    pending_stop_price: Option<Decimal>,
    #[serde(
        rename = "pendingTrailingDelta",
        skip_serializing_if = "Option::is_none"
    )]
    pending_trailing_delta: Option<Decimal>,
    #[serde(rename = "pendingIcebergQty", skip_serializing_if = "Option::is_none")]
    pending_iceberg_qty: Option<Decimal>,
    #[serde(rename = "pendingTimeInForce", skip_serializing_if = "Option::is_none")]
    pending_time_in_force: Option<String>,
    #[serde(rename = "pendingStrategyId", skip_serializing_if = "Option::is_none")]
    pending_strategy_id: Option<i64>,
    #[serde(
        rename = "pendingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_strategy_type: Option<i64>,
    #[serde(
        rename = "pendingPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_peg_price_type: Option<String>,
    #[serde(
        rename = "pendingPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_peg_offset_type: Option<String>,
    #[serde(
        rename = "pendingPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    pending_peg_offset_value: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderListOpo {
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
    pub fn working_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `workingStrategyId` parameter.
    #[must_use]
    pub fn working_strategy_id(mut self, value: i64) -> Self {
        self.working_strategy_id = Some(value);
        self
    }
    /// Set the provider `workingStrategyType` parameter.
    #[must_use]
    pub fn working_strategy_type(mut self, value: i64) -> Self {
        self.working_strategy_type = Some(value);
        self
    }
    /// Set the provider `workingPegPriceType` parameter.
    #[must_use]
    pub fn working_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetType` parameter.
    #[must_use]
    pub fn working_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetValue` parameter.
    #[must_use]
    pub fn working_peg_offset_value(mut self, value: i64) -> Self {
        self.working_peg_offset_value = Some(value);
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
    pub fn pending_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `pendingStrategyId` parameter.
    #[must_use]
    pub fn pending_strategy_id(mut self, value: i64) -> Self {
        self.pending_strategy_id = Some(value);
        self
    }
    /// Set the provider `pendingStrategyType` parameter.
    #[must_use]
    pub fn pending_strategy_type(mut self, value: i64) -> Self {
        self.pending_strategy_type = Some(value);
        self
    }
    /// Set the provider `pendingPegPriceType` parameter.
    #[must_use]
    pub fn pending_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.pending_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pendingPegOffsetType` parameter.
    #[must_use]
    pub fn pending_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.pending_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `pendingPegOffsetValue` parameter.
    #[must_use]
    pub fn pending_peg_offset_value(mut self, value: i64) -> Self {
        self.pending_peg_offset_value = Some(value);
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
impl Request for OrderListOpo {
    type Response = super::rest_models::OrderListOpoResponse;
    const OP: Operation = Operation {
        name: "orderListOpo",
        path: "/api/v3/orderList/opo",
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
            &[
                "pendingSide",
                "pendingType",
                "symbol",
                "workingPrice",
                "workingQuantity",
                "workingSide",
                "workingType",
            ],
            &[
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
                ("workingType", &["LIMIT", "LIMIT_MAKER"]),
                ("workingSide", &["BUY", "SELL"]),
                ("workingTimeInForce", &["GTC", "IOC", "FOK"]),
                ("workingPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("workingPegOffsetType", &["PRICE_LEVEL"]),
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
                ("pendingPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pendingPegOffsetType", &["PRICE_LEVEL"]),
            ],
            &[],
        )?;
        super::validation::validate("orderListOpo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderListOpoco`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-opoco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderListOpoco {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
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
    working_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "workingPrice", skip_serializing_if = "Option::is_none")]
    working_price: Option<Decimal>,
    #[serde(rename = "workingQuantity", skip_serializing_if = "Option::is_none")]
    working_quantity: Option<Decimal>,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    working_iceberg_qty: Option<Decimal>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    working_strategy_id: Option<i64>,
    #[serde(
        rename = "workingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    working_strategy_type: Option<i64>,
    #[serde(
        rename = "workingPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_price_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_value: Option<i64>,
    #[serde(rename = "pendingSide", skip_serializing_if = "Option::is_none")]
    pending_side: Option<String>,
    #[serde(rename = "pendingAboveType", skip_serializing_if = "Option::is_none")]
    pending_above_type: Option<String>,
    #[serde(
        rename = "pendingAboveClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_client_order_id: Option<ClientOrderId>,
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
    #[serde(
        rename = "pendingAboveStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_strategy_id: Option<i64>,
    #[serde(
        rename = "pendingAboveStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_strategy_type: Option<i64>,
    #[serde(
        rename = "pendingAbovePegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_peg_price_type: Option<String>,
    #[serde(
        rename = "pendingAbovePegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_peg_offset_type: Option<String>,
    #[serde(
        rename = "pendingAbovePegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_peg_offset_value: Option<i64>,
    #[serde(rename = "pendingBelowType", skip_serializing_if = "Option::is_none")]
    pending_below_type: Option<String>,
    #[serde(
        rename = "pendingBelowClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_client_order_id: Option<ClientOrderId>,
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
    #[serde(
        rename = "pendingBelowStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_strategy_id: Option<i64>,
    #[serde(
        rename = "pendingBelowStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_strategy_type: Option<i64>,
    #[serde(
        rename = "pendingBelowPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_peg_price_type: Option<String>,
    #[serde(
        rename = "pendingBelowPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_peg_offset_type: Option<String>,
    #[serde(
        rename = "pendingBelowPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_peg_offset_value: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderListOpoco {
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
    pub fn working_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `workingStrategyId` parameter.
    #[must_use]
    pub fn working_strategy_id(mut self, value: i64) -> Self {
        self.working_strategy_id = Some(value);
        self
    }
    /// Set the provider `workingStrategyType` parameter.
    #[must_use]
    pub fn working_strategy_type(mut self, value: i64) -> Self {
        self.working_strategy_type = Some(value);
        self
    }
    /// Set the provider `workingPegPriceType` parameter.
    #[must_use]
    pub fn working_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetType` parameter.
    #[must_use]
    pub fn working_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetValue` parameter.
    #[must_use]
    pub fn working_peg_offset_value(mut self, value: i64) -> Self {
        self.working_peg_offset_value = Some(value);
        self
    }
    /// Set the provider `pendingSide` parameter.
    #[must_use]
    pub fn pending_side(mut self, value: impl Into<String>) -> Self {
        self.pending_side = Some(value.into());
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
    pub fn pending_above_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `pendingAboveStrategyId` parameter.
    #[must_use]
    pub fn pending_above_strategy_id(mut self, value: i64) -> Self {
        self.pending_above_strategy_id = Some(value);
        self
    }
    /// Set the provider `pendingAboveStrategyType` parameter.
    #[must_use]
    pub fn pending_above_strategy_type(mut self, value: i64) -> Self {
        self.pending_above_strategy_type = Some(value);
        self
    }
    /// Set the provider `pendingAbovePegPriceType` parameter.
    #[must_use]
    pub fn pending_above_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.pending_above_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pendingAbovePegOffsetType` parameter.
    #[must_use]
    pub fn pending_above_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.pending_above_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `pendingAbovePegOffsetValue` parameter.
    #[must_use]
    pub fn pending_above_peg_offset_value(mut self, value: i64) -> Self {
        self.pending_above_peg_offset_value = Some(value);
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
    pub fn pending_below_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `pendingBelowStrategyId` parameter.
    #[must_use]
    pub fn pending_below_strategy_id(mut self, value: i64) -> Self {
        self.pending_below_strategy_id = Some(value);
        self
    }
    /// Set the provider `pendingBelowStrategyType` parameter.
    #[must_use]
    pub fn pending_below_strategy_type(mut self, value: i64) -> Self {
        self.pending_below_strategy_type = Some(value);
        self
    }
    /// Set the provider `pendingBelowPegPriceType` parameter.
    #[must_use]
    pub fn pending_below_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.pending_below_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pendingBelowPegOffsetType` parameter.
    #[must_use]
    pub fn pending_below_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.pending_below_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `pendingBelowPegOffsetValue` parameter.
    #[must_use]
    pub fn pending_below_peg_offset_value(mut self, value: i64) -> Self {
        self.pending_below_peg_offset_value = Some(value);
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
impl Request for OrderListOpoco {
    type Response = super::rest_models::OrderListOpocoResponse;
    const OP: Operation = Operation {
        name: "orderListOpoco",
        path: "/api/v3/orderList/opoco",
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
            &[
                "pendingAboveType",
                "pendingSide",
                "symbol",
                "workingPrice",
                "workingQuantity",
                "workingSide",
                "workingType",
            ],
            &[
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
                ("workingType", &["LIMIT", "LIMIT_MAKER"]),
                ("workingSide", &["BUY", "SELL"]),
                ("workingTimeInForce", &["GTC", "IOC", "FOK"]),
                ("workingPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("workingPegOffsetType", &["PRICE_LEVEL"]),
                ("pendingSide", &["BUY", "SELL"]),
                (
                    "pendingAboveType",
                    &[
                        "STOP_LOSS_LIMIT",
                        "STOP_LOSS",
                        "LIMIT_MAKER",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                    ],
                ),
                ("pendingAboveTimeInForce", &["GTC", "IOC", "FOK"]),
                ("pendingAbovePegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pendingAbovePegOffsetType", &["PRICE_LEVEL"]),
                (
                    "pendingBelowType",
                    &[
                        "STOP_LOSS",
                        "STOP_LOSS_LIMIT",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                    ],
                ),
                ("pendingBelowTimeInForce", &["GTC", "IOC", "FOK"]),
                ("pendingBelowPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pendingBelowPegOffsetType", &["PRICE_LEVEL"]),
            ],
            &[],
        )?;
        super::validation::validate("orderListOpoco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderListOto`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-oto).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderListOto {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
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
    working_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "workingPrice", skip_serializing_if = "Option::is_none")]
    working_price: Option<Decimal>,
    #[serde(rename = "workingQuantity", skip_serializing_if = "Option::is_none")]
    working_quantity: Option<Decimal>,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    working_iceberg_qty: Option<Decimal>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    working_strategy_id: Option<i64>,
    #[serde(
        rename = "workingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    working_strategy_type: Option<i64>,
    #[serde(rename = "pendingType", skip_serializing_if = "Option::is_none")]
    pending_type: Option<String>,
    #[serde(
        rename = "workingPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_price_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_value: Option<i64>,
    #[serde(rename = "pendingSide", skip_serializing_if = "Option::is_none")]
    pending_side: Option<String>,
    #[serde(
        rename = "pendingClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_client_order_id: Option<ClientOrderId>,
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
    #[serde(rename = "pendingStrategyId", skip_serializing_if = "Option::is_none")]
    pending_strategy_id: Option<i64>,
    #[serde(
        rename = "pendingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_strategy_type: Option<i64>,
    #[serde(
        rename = "pendingPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_peg_price_type: Option<String>,
    #[serde(
        rename = "pendingPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_peg_offset_type: Option<String>,
    #[serde(
        rename = "pendingPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    pending_peg_offset_value: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderListOto {
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
    pub fn working_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `workingStrategyId` parameter.
    #[must_use]
    pub fn working_strategy_id(mut self, value: i64) -> Self {
        self.working_strategy_id = Some(value);
        self
    }
    /// Set the provider `workingStrategyType` parameter.
    #[must_use]
    pub fn working_strategy_type(mut self, value: i64) -> Self {
        self.working_strategy_type = Some(value);
        self
    }
    /// Set the provider `pendingType` parameter.
    #[must_use]
    pub fn pending_type(mut self, value: impl Into<String>) -> Self {
        self.pending_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegPriceType` parameter.
    #[must_use]
    pub fn working_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetType` parameter.
    #[must_use]
    pub fn working_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetValue` parameter.
    #[must_use]
    pub fn working_peg_offset_value(mut self, value: i64) -> Self {
        self.working_peg_offset_value = Some(value);
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
    pub fn pending_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `pendingStrategyId` parameter.
    #[must_use]
    pub fn pending_strategy_id(mut self, value: i64) -> Self {
        self.pending_strategy_id = Some(value);
        self
    }
    /// Set the provider `pendingStrategyType` parameter.
    #[must_use]
    pub fn pending_strategy_type(mut self, value: i64) -> Self {
        self.pending_strategy_type = Some(value);
        self
    }
    /// Set the provider `pendingPegPriceType` parameter.
    #[must_use]
    pub fn pending_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.pending_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pendingPegOffsetType` parameter.
    #[must_use]
    pub fn pending_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.pending_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `pendingPegOffsetValue` parameter.
    #[must_use]
    pub fn pending_peg_offset_value(mut self, value: i64) -> Self {
        self.pending_peg_offset_value = Some(value);
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
impl Request for OrderListOto {
    type Response = super::rest_models::OrderListOtoResponse;
    const OP: Operation = Operation {
        name: "orderListOto",
        path: "/api/v3/orderList/oto",
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
            &[
                "pendingQuantity",
                "pendingSide",
                "pendingType",
                "symbol",
                "workingPrice",
                "workingQuantity",
                "workingSide",
                "workingType",
            ],
            &[
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
                ("workingPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("workingPegOffsetType", &["PRICE_LEVEL"]),
                ("pendingSide", &["BUY", "SELL"]),
                ("pendingTimeInForce", &["GTC", "IOC", "FOK"]),
                ("pendingPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pendingPegOffsetType", &["PRICE_LEVEL"]),
            ],
            &[],
        )?;
        super::validation::validate("orderListOto", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderListOtoco`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-otoco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderListOtoco {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    list_client_order_id: Option<ClientOrderId>,
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
    working_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "workingPrice", skip_serializing_if = "Option::is_none")]
    working_price: Option<Decimal>,
    #[serde(rename = "workingQuantity", skip_serializing_if = "Option::is_none")]
    working_quantity: Option<Decimal>,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    working_iceberg_qty: Option<Decimal>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    working_strategy_id: Option<i64>,
    #[serde(
        rename = "workingStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    working_strategy_type: Option<i64>,
    #[serde(
        rename = "workingPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_price_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_type: Option<String>,
    #[serde(
        rename = "workingPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    working_peg_offset_value: Option<i64>,
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
    pending_above_client_order_id: Option<ClientOrderId>,
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
    #[serde(
        rename = "pendingAboveStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_strategy_id: Option<i64>,
    #[serde(
        rename = "pendingAboveStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_strategy_type: Option<i64>,
    #[serde(
        rename = "pendingAbovePegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_peg_price_type: Option<String>,
    #[serde(
        rename = "pendingAbovePegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_peg_offset_type: Option<String>,
    #[serde(
        rename = "pendingAbovePegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    pending_above_peg_offset_value: Option<i64>,
    #[serde(rename = "pendingBelowType", skip_serializing_if = "Option::is_none")]
    pending_below_type: Option<String>,
    #[serde(
        rename = "pendingBelowClientOrderId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_client_order_id: Option<ClientOrderId>,
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
    #[serde(
        rename = "pendingBelowStrategyId",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_strategy_id: Option<i64>,
    #[serde(
        rename = "pendingBelowStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_strategy_type: Option<i64>,
    #[serde(
        rename = "pendingBelowPegPriceType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_peg_price_type: Option<String>,
    #[serde(
        rename = "pendingBelowPegOffsetType",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_peg_offset_type: Option<String>,
    #[serde(
        rename = "pendingBelowPegOffsetValue",
        skip_serializing_if = "Option::is_none"
    )]
    pending_below_peg_offset_value: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderListOtoco {
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
    pub fn working_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `workingStrategyId` parameter.
    #[must_use]
    pub fn working_strategy_id(mut self, value: i64) -> Self {
        self.working_strategy_id = Some(value);
        self
    }
    /// Set the provider `workingStrategyType` parameter.
    #[must_use]
    pub fn working_strategy_type(mut self, value: i64) -> Self {
        self.working_strategy_type = Some(value);
        self
    }
    /// Set the provider `workingPegPriceType` parameter.
    #[must_use]
    pub fn working_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetType` parameter.
    #[must_use]
    pub fn working_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.working_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `workingPegOffsetValue` parameter.
    #[must_use]
    pub fn working_peg_offset_value(mut self, value: i64) -> Self {
        self.working_peg_offset_value = Some(value);
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
    pub fn pending_above_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `pendingAboveStrategyId` parameter.
    #[must_use]
    pub fn pending_above_strategy_id(mut self, value: i64) -> Self {
        self.pending_above_strategy_id = Some(value);
        self
    }
    /// Set the provider `pendingAboveStrategyType` parameter.
    #[must_use]
    pub fn pending_above_strategy_type(mut self, value: i64) -> Self {
        self.pending_above_strategy_type = Some(value);
        self
    }
    /// Set the provider `pendingAbovePegPriceType` parameter.
    #[must_use]
    pub fn pending_above_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.pending_above_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pendingAbovePegOffsetType` parameter.
    #[must_use]
    pub fn pending_above_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.pending_above_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `pendingAbovePegOffsetValue` parameter.
    #[must_use]
    pub fn pending_above_peg_offset_value(mut self, value: i64) -> Self {
        self.pending_above_peg_offset_value = Some(value);
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
    pub fn pending_below_client_order_id(mut self, value: ClientOrderId) -> Self {
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
    /// Set the provider `pendingBelowStrategyId` parameter.
    #[must_use]
    pub fn pending_below_strategy_id(mut self, value: i64) -> Self {
        self.pending_below_strategy_id = Some(value);
        self
    }
    /// Set the provider `pendingBelowStrategyType` parameter.
    #[must_use]
    pub fn pending_below_strategy_type(mut self, value: i64) -> Self {
        self.pending_below_strategy_type = Some(value);
        self
    }
    /// Set the provider `pendingBelowPegPriceType` parameter.
    #[must_use]
    pub fn pending_below_peg_price_type(mut self, value: impl Into<String>) -> Self {
        self.pending_below_peg_price_type = Some(value.into());
        self
    }
    /// Set the provider `pendingBelowPegOffsetType` parameter.
    #[must_use]
    pub fn pending_below_peg_offset_type(mut self, value: impl Into<String>) -> Self {
        self.pending_below_peg_offset_type = Some(value.into());
        self
    }
    /// Set the provider `pendingBelowPegOffsetValue` parameter.
    #[must_use]
    pub fn pending_below_peg_offset_value(mut self, value: i64) -> Self {
        self.pending_below_peg_offset_value = Some(value);
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
impl Request for OrderListOtoco {
    type Response = super::rest_models::OrderListOtocoResponse;
    const OP: Operation = Operation {
        name: "orderListOtoco",
        path: "/api/v3/orderList/otoco",
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
            &[
                "pendingAboveType",
                "pendingQuantity",
                "pendingSide",
                "symbol",
                "workingPrice",
                "workingQuantity",
                "workingSide",
                "workingType",
            ],
            &[
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
                ("workingType", &["LIMIT", "LIMIT_MAKER"]),
                ("workingSide", &["BUY", "SELL"]),
                ("workingTimeInForce", &["GTC", "IOC", "FOK"]),
                ("workingPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("workingPegOffsetType", &["PRICE_LEVEL"]),
                ("pendingSide", &["BUY", "SELL"]),
                (
                    "pendingAboveType",
                    &[
                        "STOP_LOSS_LIMIT",
                        "STOP_LOSS",
                        "LIMIT_MAKER",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                    ],
                ),
                ("pendingAboveTimeInForce", &["GTC", "IOC", "FOK"]),
                ("pendingAbovePegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pendingAbovePegOffsetType", &["PRICE_LEVEL"]),
                (
                    "pendingBelowType",
                    &[
                        "STOP_LOSS",
                        "STOP_LOSS_LIMIT",
                        "TAKE_PROFIT",
                        "TAKE_PROFIT_LIMIT",
                    ],
                ),
                ("pendingBelowTimeInForce", &["GTC", "IOC", "FOK"]),
                ("pendingBelowPegPriceType", &["PRIMARY_PEG", "MARKET_PEG"]),
                ("pendingBelowPegOffsetType", &["PRICE_LEVEL"]),
            ],
            &[],
        )?;
        super::validation::validate("orderListOtoco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderOco`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-oco).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderOco {
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
    #[serde(rename = "limitStrategyId", skip_serializing_if = "Option::is_none")]
    limit_strategy_id: Option<i64>,
    #[serde(rename = "limitStrategyType", skip_serializing_if = "Option::is_none")]
    limit_strategy_type: Option<i64>,
    #[serde(rename = "limitIcebergQty", skip_serializing_if = "Option::is_none")]
    limit_iceberg_qty: Option<Decimal>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    trailing_delta: Option<i64>,
    #[serde(rename = "stopClientOrderId", skip_serializing_if = "Option::is_none")]
    stop_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(rename = "stopStrategyId", skip_serializing_if = "Option::is_none")]
    stop_strategy_id: Option<i64>,
    #[serde(rename = "stopStrategyType", skip_serializing_if = "Option::is_none")]
    stop_strategy_type: Option<i64>,
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
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl OrderOco {
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
    /// Set the provider `limitStrategyId` parameter.
    #[must_use]
    pub fn limit_strategy_id(mut self, value: i64) -> Self {
        self.limit_strategy_id = Some(value);
        self
    }
    /// Set the provider `limitStrategyType` parameter.
    #[must_use]
    pub fn limit_strategy_type(mut self, value: i64) -> Self {
        self.limit_strategy_type = Some(value);
        self
    }
    /// Set the provider `limitIcebergQty` parameter.
    #[must_use]
    pub fn limit_iceberg_qty(mut self, value: Decimal) -> Self {
        self.limit_iceberg_qty = Some(value);
        self
    }
    /// Set the provider `trailingDelta` parameter.
    #[must_use]
    pub fn trailing_delta(mut self, value: i64) -> Self {
        self.trailing_delta = Some(value);
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
    /// Set the provider `stopStrategyId` parameter.
    #[must_use]
    pub fn stop_strategy_id(mut self, value: i64) -> Self {
        self.stop_strategy_id = Some(value);
        self
    }
    /// Set the provider `stopStrategyType` parameter.
    #[must_use]
    pub fn stop_strategy_type(mut self, value: i64) -> Self {
        self.stop_strategy_type = Some(value);
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
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
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
impl Request for OrderOco {
    type Response = super::rest_models::OrderOcoResponse;
    const OP: Operation = Operation {
        name: "orderOco",
        path: "/api/v3/order/oco",
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
            &["price", "quantity", "side", "stopPrice", "symbol"],
            &[
                ("side", &["BUY", "SELL"]),
                ("stopLimitTimeInForce", &["GTC", "IOC", "FOK"]),
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
            ],
            &[],
        )?;
        super::validation::validate("orderOco", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`sorOrder`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#sor-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SorOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl SorOrder {
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
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
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
impl Request for SorOrder {
    type Response = super::rest_models::SorOrderResponse;
    const OP: Operation = Operation {
        name: "sorOrder",
        path: "/api/v3/sor/order",
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
            &["quantity", "side", "symbol", "type"],
            &[
                ("side", &["BUY", "SELL"]),
                ("type", &["MARKET", "LIMIT"]),
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
            ],
            &[],
        )?;
        super::validation::validate("sorOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`sorOrderTest`](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#sor-order-test).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SorOrderTest {
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
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    new_client_order_id: Option<ClientOrderId>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    strategy_type: Option<i64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    iceberg_qty: Option<Decimal>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    self_trade_prevention_mode: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<Decimal>,
}
impl SorOrderTest {
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
    /// Set the provider `selfTradePreventionMode` parameter.
    #[must_use]
    pub fn self_trade_prevention_mode(mut self, value: impl Into<String>) -> Self {
        self.self_trade_prevention_mode = Some(value.into());
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
impl Request for SorOrderTest {
    type Response = super::rest_models::SorOrderTestResponse;
    const OP: Operation = Operation {
        name: "sorOrderTest",
        path: "/api/v3/sor/order/test",
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
            &["quantity", "side", "symbol", "type"],
            &[
                ("side", &["BUY", "SELL"]),
                ("type", &["MARKET", "LIMIT"]),
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
            ],
            &[],
        )?;
        super::validation::validate("sorOrderTest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [accountCommission](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_commission(
        &self,
        request: &AccountCommission,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountCommissionResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [allOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
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

    /// [getAccount](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_account(
        &self,
        request: &GetAccount,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetAccountResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_open_orders(
        &self,
        request: &GetOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [deleteOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn delete_open_orders(
        &self,
        request: &DeleteOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DeleteOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getOrder](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_order(
        &self,
        request: &GetOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [deleteOrder](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn delete_order(
        &self,
        request: &DeleteOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DeleteOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newOrder](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
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

    /// [myTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn my_trades(
        &self,
        request: &MyTrades,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MyTradesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [rateLimitOrder](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn rate_limit_order(
        &self,
        request: &RateLimitOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::RateLimitOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [exchangeInfo](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn exchange_info(
        &self,
        request: &ExchangeInfo,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ExchangeInfoResponse>, Error> {
        let response = self.inner.execute(request, deadline).await?;
        super::rate::adopt_stated_limits(&self.inner, &response.data)?;
        Ok(response)
    }

    /// [executionRules](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn execution_rules(
        &self,
        request: &ExecutionRules,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ExecutionRulesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [ping](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ping(
        &self,
        request: &Ping,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PingResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [time](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn time(
        &self,
        request: &Time,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TimeResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [aggTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn agg_trades(
        &self,
        request: &AggTrades,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AggTradesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [avgPrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn avg_price(
        &self,
        request: &AvgPrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AvgPriceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [depth](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn depth(
        &self,
        request: &Depth,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DepthResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_trades(
        &self,
        request: &GetTrades,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetTradesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [historicalTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn historical_trades(
        &self,
        request: &HistoricalTrades,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::HistoricalTradesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [historicalBlockTrades](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn historical_block_trades(
        &self,
        request: &HistoricalBlockTrades,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::HistoricalBlockTradesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [klines](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn klines(
        &self,
        request: &Klines,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::KlinesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [ticker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker(
        &self,
        request: &Ticker,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TickerResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [ticker24hr](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker24hr(
        &self,
        request: &Ticker24hr,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::Ticker24hrResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [tickerBookTicker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker_book_ticker(
        &self,
        request: &TickerBookTicker,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TickerBookTickerResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [tickerPrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker_price(
        &self,
        request: &TickerPrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TickerPriceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [tickerTradingDay](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ticker_trading_day(
        &self,
        request: &TickerTradingDay,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TickerTradingDayResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [uiKlines](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn ui_klines(
        &self,
        request: &UiKlines,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UiKlinesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [referencePrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn reference_price(
        &self,
        request: &ReferencePrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ReferencePriceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [referencePriceCalculation](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/market).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn reference_price_calculation(
        &self,
        request: &ReferencePriceCalculation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ReferencePriceCalculationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderTest](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_test(
        &self,
        request: &OrderTest,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderTestResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [allOrderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#all-order-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn all_order_list(
        &self,
        request: &AllOrderList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AllOrderListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getOrderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#get-order-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_order_list(
        &self,
        request: &GetOrderList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetOrderListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [deleteOrderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#delete-order-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn delete_order_list(
        &self,
        request: &DeleteOrderList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::DeleteOrderListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [myAllocations](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-allocations).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn my_allocations(
        &self,
        request: &MyAllocations,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MyAllocationsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [myFilters](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-filters).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn my_filters(
        &self,
        request: &MyFilters,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MyFiltersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [myPreventedMatches](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#my-prevented-matches).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn my_prevented_matches(
        &self,
        request: &MyPreventedMatches,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::MyPreventedMatchesResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [openOrderList](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#open-order-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn open_order_list(
        &self,
        request: &OpenOrderList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OpenOrderListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderAmendments](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#order-amendments).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_amendments(
        &self,
        request: &OrderAmendments,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderAmendmentsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderAmendKeepPriority](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-amend-keep-priority).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_amend_keep_priority(
        &self,
        request: &OrderAmendKeepPriority,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderAmendKeepPriorityResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderCancelReplace](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-cancel-replace).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_cancel_replace(
        &self,
        request: &OrderCancelReplace,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderCancelReplaceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderListOco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-oco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_list_oco(
        &self,
        request: &OrderListOco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderListOcoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderListOpo](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-opo).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_list_opo(
        &self,
        request: &OrderListOpo,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderListOpoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderListOpoco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-opoco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_list_opoco(
        &self,
        request: &OrderListOpoco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderListOpocoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderListOto](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-oto).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_list_oto(
        &self,
        request: &OrderListOto,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderListOtoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderListOtoco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-list-otoco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_list_otoco(
        &self,
        request: &OrderListOtoco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderListOtocoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderOco](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#order-oco).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_oco(
        &self,
        request: &OrderOco,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OrderOcoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [sorOrder](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#sor-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn sor_order(
        &self,
        request: &SorOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SorOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [sorOrderTest](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade#sor-order-test).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn sor_order_test(
        &self,
        request: &SorOrderTest,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SorOrderTestResponse>, Error> {
        self.inner.execute(request, deadline).await
    }
}
