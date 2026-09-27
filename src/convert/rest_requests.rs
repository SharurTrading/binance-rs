// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use crate::Decimal;
use crate::Error;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`listAllConvertPairs`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#list-all-convert-pairs).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ListAllConvertPairs {
    #[serde(rename = "fromAsset", skip_serializing_if = "Option::is_none")]
    from_asset: Option<crate::Asset>,
    #[serde(rename = "toAsset", skip_serializing_if = "Option::is_none")]
    to_asset: Option<crate::Asset>,
}
impl ListAllConvertPairs {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `fromAsset` parameter.
    #[must_use]
    pub fn from_asset(mut self, value: crate::Asset) -> Self {
        self.from_asset = Some(value);
        self
    }
    /// Set the provider `toAsset` parameter.
    #[must_use]
    pub fn to_asset(mut self, value: crate::Asset) -> Self {
        self.to_asset = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for ListAllConvertPairs {
    type Response = super::rest_models::ListAllConvertPairsResponse;
    const OP: Operation = Operation {
        name: "listAllConvertPairs",
        path: "/sapi/v1/convert/exchangeInfo",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 3000,
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

/// Validated request builder for [`queryOrderQuantityPrecisionPerAsset`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#query-order-quantity-precision-per-asset).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryOrderQuantityPrecisionPerAsset {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryOrderQuantityPrecisionPerAsset {
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
impl Request for QueryOrderQuantityPrecisionPerAsset {
    type Response = super::rest_models::QueryOrderQuantityPrecisionPerAssetResponse;
    const OP: Operation = Operation {
        name: "queryOrderQuantityPrecisionPerAsset",
        path: "/sapi/v1/convert/assetInfo",
        method: "GET",
        security: Security::Signed,
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
        super::validation::validate("queryOrderQuantityPrecisionPerAsset", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Canonical quote acceptance operation facts.
pub(crate) const ACCEPT_QUOTE_OPERATION: Operation = Operation {
    name: "acceptQuote",
    path: "/sapi/v1/convert/acceptQuote",
    method: "POST",
    security: Security::Signed,
    mutation: true,
    weight: 500,
    validate_time: super::validation::validate_time,
    definitive: super::validation::definitive,
    success_weight: None,
    partial: None,
};
pub use super::quote::AcceptQuote;

/// Validated request builder for [`cancelLimitOrder`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#cancel-limit-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelLimitOrder {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<super::OrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelLimitOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: super::OrderId) -> Self {
        self.order_id = Some(value);
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
impl Request for CancelLimitOrder {
    type Response = super::rest_models::CancelLimitOrderResponse;
    const OP: Operation = Operation {
        name: "cancelLimitOrder",
        path: "/sapi/v1/convert/limit/cancelOrder",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 200,
        validate_time: super::validation::validate_time,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["orderId"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelLimitOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getConvertTradeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#get-convert-trade-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetConvertTradeHistory {
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetConvertTradeHistory {
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
impl Request for GetConvertTradeHistory {
    type Response = super::rest_models::GetConvertTradeHistoryResponse;
    const OP: Operation = Operation {
        name: "getConvertTradeHistory",
        path: "/sapi/v1/convert/tradeFlow",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 3000,
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
                ("limit", -9_223_372_036_854_775_808, 1_000),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("getConvertTradeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderStatus`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#order-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderStatus {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<super::OrderId>,
    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    quote_id: Option<super::QuoteId>,
}
impl OrderStatus {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: super::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `quoteId` parameter.
    #[must_use]
    pub fn quote_id(mut self, value: super::QuoteId) -> Self {
        self.quote_id = Some(value);
        self
    }
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
    type Response = super::rest_models::OrderStatusResponse;
    const OP: Operation = Operation {
        name: "orderStatus",
        path: "/sapi/v1/convert/orderStatus",
        method: "GET",
        security: Security::Signed,
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
        super::validation::validate("orderStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`placeLimitOrder`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#place-limit-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaceLimitOrder {
    #[serde(rename = "baseAsset", skip_serializing_if = "Option::is_none")]
    base_asset: Option<crate::Asset>,
    #[serde(rename = "quoteAsset", skip_serializing_if = "Option::is_none")]
    quote_asset: Option<crate::Asset>,
    #[serde(rename = "limitPrice", skip_serializing_if = "Option::is_none")]
    limit_price: Option<Decimal>,
    #[serde(rename = "baseAmount", skip_serializing_if = "Option::is_none")]
    base_amount: Option<Decimal>,
    #[serde(rename = "quoteAmount", skip_serializing_if = "Option::is_none")]
    quote_amount: Option<Decimal>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "walletType", skip_serializing_if = "Option::is_none")]
    wallet_type: Option<String>,
    #[serde(rename = "expiredType", skip_serializing_if = "Option::is_none")]
    expired_type: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PlaceLimitOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `baseAsset` parameter.
    #[must_use]
    pub fn base_asset(mut self, value: crate::Asset) -> Self {
        self.base_asset = Some(value);
        self
    }
    /// Set the provider `quoteAsset` parameter.
    #[must_use]
    pub fn quote_asset(mut self, value: crate::Asset) -> Self {
        self.quote_asset = Some(value);
        self
    }
    /// Set the provider `limitPrice` parameter.
    #[must_use]
    pub fn limit_price(mut self, value: Decimal) -> Self {
        self.limit_price = Some(value);
        self
    }
    /// Set the provider `baseAmount` parameter.
    #[must_use]
    pub fn base_amount(mut self, value: Decimal) -> Self {
        self.base_amount = Some(value);
        self
    }
    /// Set the provider `quoteAmount` parameter.
    #[must_use]
    pub fn quote_amount(mut self, value: Decimal) -> Self {
        self.quote_amount = Some(value);
        self
    }
    /// Set the provider `side` parameter.
    #[must_use]
    pub fn side(mut self, value: impl Into<String>) -> Self {
        self.side = Some(value.into());
        self
    }
    /// Set the provider `walletType` parameter.
    #[must_use]
    pub fn wallet_type(mut self, value: impl Into<String>) -> Self {
        self.wallet_type = Some(value.into());
        self
    }
    /// Set the provider `expiredType` parameter.
    #[must_use]
    pub fn expired_type(mut self, value: impl Into<String>) -> Self {
        self.expired_type = Some(value.into());
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
impl Request for PlaceLimitOrder {
    type Response = super::rest_models::PlaceLimitOrderResponse;
    const OP: Operation = Operation {
        name: "placeLimitOrder",
        path: "/sapi/v1/convert/limit/placeOrder",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 500,
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
                "baseAsset",
                "expiredType",
                "limitPrice",
                "quoteAsset",
                "side",
            ],
            &[
                ("side", &["BUY", "SELL"]),
                (
                    "walletType",
                    &[
                        "SPOT",
                        "FUNDING",
                        "EARN",
                        "SPOT_FUNDING",
                        "FUNDING_EARN",
                        "SPOT_FUNDING_EARN",
                        "SPOT_EARN",
                    ],
                ),
                ("expiredType", &["1_D", "3_D", "7_D", "30_D"]),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("placeLimitOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryLimitOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#query-limit-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryLimitOpenOrders {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryLimitOpenOrders {
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
impl Request for QueryLimitOpenOrders {
    type Response = super::rest_models::QueryLimitOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "queryLimitOpenOrders",
        path: "/sapi/v1/convert/limit/queryOpenOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 3000,
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
        super::validation::validate("queryLimitOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`sendQuoteRequest`](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#send-quote-request).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SendQuoteRequest {
    #[serde(rename = "fromAsset", skip_serializing_if = "Option::is_none")]
    from_asset: Option<crate::Asset>,
    #[serde(rename = "toAsset", skip_serializing_if = "Option::is_none")]
    to_asset: Option<crate::Asset>,
    #[serde(rename = "fromAmount", skip_serializing_if = "Option::is_none")]
    from_amount: Option<Decimal>,
    #[serde(rename = "toAmount", skip_serializing_if = "Option::is_none")]
    to_amount: Option<Decimal>,
    #[serde(rename = "walletType", skip_serializing_if = "Option::is_none")]
    wallet_type: Option<String>,
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
    pub fn from_asset(mut self, value: crate::Asset) -> Self {
        self.from_asset = Some(value);
        self
    }
    /// Set the provider `toAsset` parameter.
    #[must_use]
    pub fn to_asset(mut self, value: crate::Asset) -> Self {
        self.to_asset = Some(value);
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
    /// Set the provider `walletType` parameter.
    #[must_use]
    pub fn wallet_type(mut self, value: impl Into<String>) -> Self {
        self.wallet_type = Some(value.into());
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
}
impl Request for SendQuoteRequest {
    type Response = super::rest_models::SendQuoteRequestResponse;
    const OP: Operation = Operation {
        name: "sendQuoteRequest",
        path: "/sapi/v1/convert/getQuote",
        method: "POST",
        security: Security::Signed,
        mutation: true,
        weight: 200,
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
            &[
                (
                    "walletType",
                    &[
                        "SPOT",
                        "FUNDING",
                        "EARN",
                        "SPOT_FUNDING",
                        "FUNDING_EARN",
                        "SPOT_FUNDING_EARN",
                        "SPOT_EARN",
                    ],
                ),
                ("validTime", &["10s", "30s", "1m"]),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("sendQuoteRequest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [listAllConvertPairs](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#list-all-convert-pairs).
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

    /// [queryOrderQuantityPrecisionPerAsset](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#query-order-quantity-precision-per-asset).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_order_quantity_precision_per_asset(
        &self,
        request: &QueryOrderQuantityPrecisionPerAsset,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::QueryOrderQuantityPrecisionPerAssetResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [acceptQuote](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#accept-quote).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn accept_quote(
        &self,
        request: &AcceptQuote,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::Acceptance>, Error> {
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::Acceptance {
                quotation: request.quotation().clone(),
                receipt: response.data,
            },
            meta: response.meta,
        })
    }

    /// [cancelLimitOrder](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#cancel-limit-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_limit_order(
        &self,
        request: &CancelLimitOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelLimitOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getConvertTradeHistory](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#get-convert-trade-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_convert_trade_history(
        &self,
        request: &GetConvertTradeHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetConvertTradeHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderStatus](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#order-status).
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

    /// [placeLimitOrder](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#place-limit-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn place_limit_order(
        &self,
        request: &PlaceLimitOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PlaceLimitOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryLimitOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#query-limit-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_limit_open_orders(
        &self,
        request: &QueryLimitOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryLimitOpenOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [sendQuoteRequest](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#send-quote-request).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn send_quote_request(
        &self,
        request: &SendQuoteRequest,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::Quotation>, Error> {
        let from_asset = request
            .from_asset
            .clone()
            .ok_or(Error::Validation("source asset required"))?;
        let to_asset = request
            .to_asset
            .clone()
            .ok_or(Error::Validation("target asset required"))?;
        let response = self.inner.execute(request, deadline).await?;
        Ok(crate::Response {
            data: super::Quotation {
                from_asset,
                to_asset,
                wallet_type: request.wallet_type.clone(),
                receipt: response.data,
            },
            meta: response.meta,
        })
    }
}
