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
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: Some(0),
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
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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
            &[],
        )?;
        super::validation::validate("orderTest", &p)
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
        self.inner.execute(request, deadline).await
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
}
