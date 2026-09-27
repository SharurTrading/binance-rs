// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated ws request builders.

use crate::ClientOrderId;
use crate::Decimal;
use crate::Error;
use crate::Symbol;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`accountInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#account-information).
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
    type Response = super::ws_models::AccountInformationResponse;
    const OP: Operation = Operation {
        name: "accountInformation",
        path: "/account.status",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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

/// Validated request builder for [`accountInformationV2`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#account-information-v2).
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
}
impl Request for AccountInformationV2 {
    type Response = super::ws_models::AccountInformationV2Response;
    const OP: Operation = Operation {
        name: "accountInformationV2",
        path: "/v2/account.status",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("accountInformationV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresAccountBalance`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#futures-account-balance).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FuturesAccountBalance {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl FuturesAccountBalance {
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
impl Request for FuturesAccountBalance {
    type Response = super::ws_models::FuturesAccountBalanceResponse;
    const OP: Operation = Operation {
        name: "futuresAccountBalance",
        path: "/account.balance",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("futuresAccountBalance", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresAccountBalanceV2`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#futures-account-balance-v2).
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
}
impl Request for FuturesAccountBalanceV2 {
    type Response = super::ws_models::FuturesAccountBalanceV2Response;
    const OP: Operation = Operation {
        name: "futuresAccountBalanceV2",
        path: "/v2/account.balance",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("futuresAccountBalanceV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderBook`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/market-data#order-book).
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
}
impl Request for OrderBook {
    type Response = super::ws_models::OrderBookResponse;
    const OP: Operation = Operation {
        name: "orderBook",
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
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("orderBook", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolOrderBookTicker`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/market-data#symbol-order-book-ticker).
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
}
impl Request for SymbolOrderBookTicker {
    type Response = super::ws_models::SymbolOrderBookTickerResponse;
    const OP: Operation = Operation {
        name: "symbolOrderBookTicker",
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("symbolOrderBookTicker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolPriceTicker`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/market-data#symbol-price-ticker).
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
}
impl Request for SymbolPriceTicker {
    type Response = super::ws_models::SymbolPriceTickerResponse;
    const OP: Operation = Operation {
        name: "symbolPriceTicker",
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("symbolPriceTicker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAlgoOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#cancel-algo-order).
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
}
impl Request for CancelAlgoOrder {
    type Response = super::ws_models::CancelAlgoOrderResponse;
    const OP: Operation = Operation {
        name: "cancelAlgoOrder",
        path: "/algoOrder.cancel",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("cancelAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#cancel-order).
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
}
impl Request for CancelOrder {
    type Response = super::ws_models::CancelOrderResponse;
    const OP: Operation = Operation {
        name: "cancelOrder",
        path: "/order.cancel",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#modify-order).
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
}
impl Request for ModifyOrder {
    type Response = super::ws_models::ModifyOrderResponse;
    const OP: Operation = Operation {
        name: "modifyOrder",
        path: "/order.modify",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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
            &[],
        )?;
        super::validation::validate("modifyOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newAlgoOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#new-algo-order).
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
}
impl Request for NewAlgoOrder {
    type Response = super::ws_models::NewAlgoOrderResponse;
    const OP: Operation = Operation {
        name: "newAlgoOrder",
        path: "/algoOrder.place",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 0,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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
                        "STOP_MARKET",
                        "TAKE_PROFIT_MARKET",
                        "STOP",
                        "TAKE_PROFIT",
                        "TRAILING_STOP_MARKET",
                    ],
                ),
                ("timeInForce", &["IOC", "GTC", "FOK"]),
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

/// Validated request builder for [`newOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#new-order).
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
}
impl Request for NewOrder {
    type Response = super::ws_models::NewOrderResponse;
    const OP: Operation = Operation {
        name: "newOrder",
        path: "/order.place",
        method: "POST",
        security: Security::Signed,
        mutation: true,
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
                ("positionSide", &["BOTH", "LONG", "SHORT"]),
                ("type", &["LIMIT", "MARKET"]),
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
                    &["NONE", "EXPIRE_TAKER", "EXPIRE_BOTH", "EXPIRE_MAKER"],
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

/// Validated request builder for [`positionInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#position-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PositionInformation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PositionInformation {
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
impl Request for PositionInformation {
    type Response = super::ws_models::PositionInformationResponse;
    const OP: Operation = Operation {
        name: "positionInformation",
        path: "/account.position",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("positionInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`positionInformationV2`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#position-information-v2).
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
}
impl Request for PositionInformationV2 {
    type Response = super::ws_models::PositionInformationV2Response;
    const OP: Operation = Operation {
        name: "positionInformationV2",
        path: "/v2/account.position",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("positionInformationV2", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#query-order).
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
}
impl Request for QueryOrder {
    type Response = super::ws_models::QueryOrderResponse;
    const OP: Operation = Operation {
        name: "queryOrder",
        path: "/order.status",
        method: "POST",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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

/// Validated request builder for [`closeUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/user-data-streams#close-user-data-stream).
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
    type Response = super::ws_models::CloseUserDataStreamResponse;
    const OP: Operation = Operation {
        name: "closeUserDataStream",
        path: "/userDataStream.stop",
        method: "POST",
        security: Security::Key,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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

/// Validated request builder for [`keepaliveUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/user-data-streams#keepalive-user-data-stream).
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
    type Response = super::ws_models::KeepaliveUserDataStreamResponse;
    const OP: Operation = Operation {
        name: "keepaliveUserDataStream",
        path: "/userDataStream.ping",
        method: "POST",
        security: Security::Key,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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

/// Validated request builder for [`startUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/user-data-streams#start-user-data-stream).
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
    type Response = super::ws_models::StartUserDataStreamResponse;
    const OP: Operation = Operation {
        name: "startUserDataStream",
        path: "/userDataStream.start",
        method: "POST",
        security: Security::Key,
        mutation: true,
        weight: 1,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
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

impl super::WsClient {
    /// [accountInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#account-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_information(
        &self,
        request: &AccountInformation,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::AccountInformationResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [accountInformationV2](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#account-information-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_information_v2(
        &self,
        request: &AccountInformationV2,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::AccountInformationV2Response>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [futuresAccountBalance](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#futures-account-balance).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_account_balance(
        &self,
        request: &FuturesAccountBalance,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::FuturesAccountBalanceResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [futuresAccountBalanceV2](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/account#futures-account-balance-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_account_balance_v2(
        &self,
        request: &FuturesAccountBalanceV2,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::FuturesAccountBalanceV2Response>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [orderBook](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/market-data#order-book).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn order_book(
        &self,
        request: &OrderBook,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::OrderBookResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [symbolOrderBookTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/market-data#symbol-order-book-ticker).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn symbol_order_book_ticker(
        &self,
        request: &SymbolOrderBookTicker,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::SymbolOrderBookTickerResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [symbolPriceTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/market-data#symbol-price-ticker).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn symbol_price_ticker(
        &self,
        request: &SymbolPriceTicker,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::SymbolPriceTickerResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [cancelAlgoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#cancel-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_algo_order(
        &self,
        request: &CancelAlgoOrder,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::CancelAlgoOrderResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [cancelOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#cancel-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_order(
        &self,
        request: &CancelOrder,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::CancelOrderResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [modifyOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#modify-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn modify_order(
        &self,
        request: &ModifyOrder,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::ModifyOrderResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [newAlgoOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#new-algo-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_algo_order(
        &self,
        request: &NewAlgoOrder,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::NewAlgoOrderResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [newOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#new-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_order(
        &self,
        request: &NewOrder,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::NewOrderResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [positionInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#position-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn position_information(
        &self,
        request: &PositionInformation,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::PositionInformationResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [positionInformationV2](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#position-information-v2).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn position_information_v2(
        &self,
        request: &PositionInformationV2,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::PositionInformationV2Response>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [queryOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/trade#query-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_order(
        &self,
        request: &QueryOrder,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::QueryOrderResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [closeUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/user-data-streams#close-user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn close_user_data_stream(
        &self,
        request: &CloseUserDataStream,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::CloseUserDataStreamResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [keepaliveUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/user-data-streams#keepalive-user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn keepalive_user_data_stream(
        &self,
        request: &KeepaliveUserDataStream,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::KeepaliveUserDataStreamResponse>, Error> {
        self.execute(request, id, deadline).await
    }

    /// [startUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-api/user-data-streams#start-user-data-stream).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn start_user_data_stream(
        &self,
        request: &StartUserDataStream,
        id: crate::RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::ws_models::StartUserDataStreamResponse>, Error> {
        self.execute(request, id, deadline).await
    }
}
