// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use crate::Error;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`getFuturesLeadTraderStatus`](https://developers.binance.com/en/docs/catalog/advanced-trading-copy-trading/api/rest-api/future-copy-trading#get-futures-lead-trader-status).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesLeadTraderStatus {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetFuturesLeadTraderStatus {
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
impl Request for GetFuturesLeadTraderStatus {
    type Response = super::rest_models::GetFuturesLeadTraderStatusResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getFuturesLeadTraderStatus",
        path: "/sapi/v1/copyTrading/futures/userStatus",
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
        super::validation::validate("getFuturesLeadTraderStatus", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFuturesLeadTradingSymbolWhitelist`](https://developers.binance.com/en/docs/catalog/advanced-trading-copy-trading/api/rest-api/future-copy-trading#get-futures-lead-trading-symbol-whitelist).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesLeadTradingSymbolWhitelist {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetFuturesLeadTradingSymbolWhitelist {
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
impl Request for GetFuturesLeadTradingSymbolWhitelist {
    type Response = super::rest_models::GetFuturesLeadTradingSymbolWhitelistResponse;
    fn response_failed(value: &serde_json::Value) -> bool {
        super::validation::response_failed(Self::OP.name, value)
    }
    const OP: Operation = Operation {
        name: "getFuturesLeadTradingSymbolWhitelist",
        path: "/sapi/v1/copyTrading/futures/leadSymbol",
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
        super::validation::validate("getFuturesLeadTradingSymbolWhitelist", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [getFuturesLeadTraderStatus](https://developers.binance.com/en/docs/catalog/advanced-trading-copy-trading/api/rest-api/future-copy-trading#get-futures-lead-trader-status).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_futures_lead_trader_status(
        &self,
        request: &GetFuturesLeadTraderStatus,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetFuturesLeadTraderStatusResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getFuturesLeadTradingSymbolWhitelist](https://developers.binance.com/en/docs/catalog/advanced-trading-copy-trading/api/rest-api/future-copy-trading#get-futures-lead-trading-symbol-whitelist).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_futures_lead_trading_symbol_whitelist(
        &self,
        request: &GetFuturesLeadTradingSymbolWhitelist,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetFuturesLeadTradingSymbolWhitelistResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }
}
