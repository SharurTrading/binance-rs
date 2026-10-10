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

/// Validated request builder for [`accountInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#account-information).
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
impl Request for AccountInformation {
    type Response = super::rest_models::AccountInformationResponse;
    const OP: Operation = Operation {
        name: "accountInformation",
        path: "/dapi/v1/account",
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
        super::validation::validate("accountInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`futuresAccountBalance`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#futures-account-balance).
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
impl Request for FuturesAccountBalance {
    type Response = super::rest_models::FuturesAccountBalanceResponse;
    const OP: Operation = Operation {
        name: "futuresAccountBalance",
        path: "/dapi/v1/balance",
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
        super::validation::validate("futuresAccountBalance", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getCurrentPositionMode`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-current-position-mode).
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
        path: "/dapi/v1/positionSide/dual",
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
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("getCurrentPositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changePositionMode`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-position-mode).
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
        path: "/dapi/v1/positionSide/dual",
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
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("changePositionMode", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForFuturesOrderHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-order-history).
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
        path: "/dapi/v1/order/asyn",
        method: "GET",
        security: Security::Signed,
        mutation: true,
        weight: 1000,
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
        super::validation::validate("getDownloadIdForFuturesOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForFuturesTradeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-trade-history).
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
        path: "/dapi/v1/trade/asyn",
        method: "GET",
        security: Security::Signed,
        mutation: true,
        weight: 1000,
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
        super::validation::validate("getDownloadIdForFuturesTradeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getDownloadIdForFuturesTransactionHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-transaction-history).
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
        path: "/dapi/v1/income/asyn",
        method: "GET",
        security: Security::Signed,
        mutation: true,
        weight: 1000,
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
        super::validation::validate("getDownloadIdForFuturesTransactionHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFuturesOrderHistoryDownloadLinkById`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-order-history-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesOrderHistoryDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<Decimal>,
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
    pub fn download_id(mut self, value: Decimal) -> Self {
        self.download_id = Some(value);
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
        path: "/dapi/v1/order/asyn/id",
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

/// Validated request builder for [`getFuturesTradeDownloadLinkById`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-trade-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesTradeDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<Decimal>,
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
    pub fn download_id(mut self, value: Decimal) -> Self {
        self.download_id = Some(value);
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
        path: "/dapi/v1/trade/asyn/id",
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

/// Validated request builder for [`getFuturesTransactionHistoryDownloadLinkById`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-transaction-history-download-link-by-id).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFuturesTransactionHistoryDownloadLinkById {
    #[serde(rename = "downloadId", skip_serializing_if = "Option::is_none")]
    download_id: Option<Decimal>,
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
    pub fn download_id(mut self, value: Decimal) -> Self {
        self.download_id = Some(value);
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
        path: "/dapi/v1/income/asyn/id",
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

/// Validated request builder for [`getIncomeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-income-history).
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
        path: "/dapi/v1/income",
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

/// Validated request builder for [`notionalBracketForPair`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#notional-bracket-for-pair).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NotionalBracketForPair {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NotionalBracketForPair {
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
impl Request for NotionalBracketForPair {
    type Response = super::rest_models::NotionalBracketForPairResponse;
    const OP: Operation = Operation {
        name: "notionalBracketForPair",
        path: "/dapi/v1/leverageBracket",
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
        super::validation::validate("notionalBracketForPair", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`notionalBracketForSymbol`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#notional-bracket-for-symbol).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NotionalBracketForSymbol {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NotionalBracketForSymbol {
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
impl Request for NotionalBracketForSymbol {
    type Response = super::rest_models::NotionalBracketForSymbolResponse;
    const OP: Operation = Operation {
        name: "notionalBracketForSymbol",
        path: "/dapi/v2/leverageBracket",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("notionalBracketForSymbol", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userCommissionRate`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#user-commission-rate).
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
        path: "/dapi/v1/commissionRate",
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

/// Validated request builder for [`basis`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#basis).
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

/// Validated request builder for [`checkServerTime`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#check-server-time).
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
        path: "/dapi/v1/time",
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
        super::validation::validate("checkServerTime", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`compressedAggregateTradesList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#compressed-aggregate-trades-list).
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
        path: "/dapi/v1/aggTrades",
        method: "GET",
        security: Security::Public,
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

/// Validated request builder for [`continuousContractKlineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#continuous-contract-kline-candlestick-data).
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
        path: "/dapi/v1/continuousKlines",
        method: "GET",
        security: Security::Public,
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
            &["contractType", "interval", "pair"],
            &[
                (
                    "contractType",
                    &["PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER"],
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

/// Validated request builder for [`exchangeInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#exchange-information).
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
        path: "/dapi/v1/exchangeInfo",
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
        super::validation::validate("exchangeInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getFundingRateHistoryOfPerpetualFutures`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#get-funding-rate-history-of-perpetual-futures).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFundingRateHistoryOfPerpetualFutures {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl GetFundingRateHistoryOfPerpetualFutures {
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
impl Request for GetFundingRateHistoryOfPerpetualFutures {
    type Response = super::rest_models::GetFundingRateHistoryOfPerpetualFuturesResponse;
    const OP: Operation = Operation {
        name: "getFundingRateHistoryOfPerpetualFutures",
        path: "/dapi/v1/fundingRate",
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
        validate_parameters(
            &p,
            &["symbol"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("getFundingRateHistoryOfPerpetualFutures", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`fundingInfo`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#get-funding-rate-info).
#[derive(Clone, Debug, Default, Serialize)]
pub struct FundingInfo {}
impl FundingInfo {
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
impl Request for FundingInfo {
    type Response = super::rest_models::FundingInfoResponse;
    const OP: Operation = Operation {
        name: "fundingInfo",
        path: "/dapi/v1/fundingInfo",
        method: "GET",
        security: Security::Public,
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
        super::validation::validate("fundingInfo", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`indexPriceAndMarkPrice`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#index-price-and-mark-price).
#[derive(Clone, Debug, Default, Serialize)]
pub struct IndexPriceAndMarkPrice {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
}
impl IndexPriceAndMarkPrice {
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
impl Request for IndexPriceAndMarkPrice {
    type Response = super::rest_models::IndexPriceAndMarkPriceResponse;
    const OP: Operation = Operation {
        name: "indexPriceAndMarkPrice",
        path: "/dapi/v1/premiumIndex",
        method: "GET",
        security: Security::Public,
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("indexPriceAndMarkPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`indexPriceKlineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#index-price-kline-candlestick-data).
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
        path: "/dapi/v1/indexPriceKlines",
        method: "GET",
        security: Security::Public,
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

/// Validated request builder for [`klineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#kline-candlestick-data).
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
        path: "/dapi/v1/klines",
        method: "GET",
        security: Security::Public,
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

/// Validated request builder for [`longShortRatio`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#long-short-ratio).
#[derive(Clone, Debug, Default, Serialize)]
pub struct LongShortRatio {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    contract_type: Option<String>,
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
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
        self
    }
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }
    /// Set the provider `contractType` parameter.
    #[must_use]
    pub fn contract_type(mut self, value: impl Into<String>) -> Self {
        self.contract_type = Some(value.into());
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
            &["pair", "period"],
            &[
                (
                    "period",
                    &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
                ),
                (
                    "contractType",
                    &["PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER"],
                ),
            ],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("longShortRatio", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`markPriceKlineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#mark-price-kline-candlestick-data).
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
        path: "/dapi/v1/markPriceKlines",
        method: "GET",
        security: Security::Public,
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

/// Validated request builder for [`oldTradesLookup`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#old-trades-lookup).
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
        path: "/dapi/v1/historicalTrades",
        method: "GET",
        security: Security::Key,
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
            &[],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("oldTradesLookup", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openInterest`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#open-interest).
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
        path: "/dapi/v1/openInterest",
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
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("openInterest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openInterestStatistics`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#open-interest-statistics).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenInterestStatistics {
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
impl OpenInterestStatistics {
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
impl Request for OpenInterestStatistics {
    type Response = super::rest_models::OpenInterestStatisticsResponse;
    const OP: Operation = Operation {
        name: "openInterestStatistics",
        path: "/futures/data/openInterestHist",
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
        validate_parameters(
            &p,
            &["pair", "period"],
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
        super::validation::validate("openInterestStatistics", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderBook`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#order-book).
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
        path: "/dapi/v1/depth",
        method: "GET",
        security: Security::Public,
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

/// Validated request builder for [`premiumIndexKlineData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#premium-index-kline-data).
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
        path: "/dapi/v1/premiumIndexKlines",
        method: "GET",
        security: Security::Public,
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

/// Validated request builder for [`queryIndexPriceConstituents`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#query-index-price-constituents).
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
        path: "/dapi/v1/constituents",
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
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("queryIndexPriceConstituents", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`recentTradesList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#recent-trades-list).
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
        path: "/dapi/v1/trades",
        method: "GET",
        security: Security::Public,
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
        super::validation::validate("recentTradesList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolOrderBookTicker`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#symbol-order-book-ticker).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SymbolOrderBookTicker {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
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
impl Request for SymbolOrderBookTicker {
    type Response = super::rest_models::SymbolOrderBookTickerResponse;
    const OP: Operation = Operation {
        name: "symbolOrderBookTicker",
        path: "/dapi/v1/ticker/bookTicker",
        method: "GET",
        security: Security::Public,
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
        super::validation::validate("symbolOrderBookTicker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`symbolPriceTicker`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#symbol-price-ticker).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SymbolPriceTicker {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
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
impl Request for SymbolPriceTicker {
    type Response = super::rest_models::SymbolPriceTickerResponse;
    const OP: Operation = Operation {
        name: "symbolPriceTicker",
        path: "/dapi/v1/ticker/price",
        method: "GET",
        security: Security::Public,
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
        super::validation::validate("symbolPriceTicker", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`takerBuySellVolume`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#taker-buy-sell-volume).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TakerBuySellVolume {
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
impl TakerBuySellVolume {
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
impl Request for TakerBuySellVolume {
    type Response = super::rest_models::TakerBuySellVolumeResponse;
    const OP: Operation = Operation {
        name: "takerBuySellVolume",
        path: "/futures/data/takerBuySellVol",
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
        validate_parameters(
            &p,
            &["contractType", "pair", "period"],
            &[
                (
                    "contractType",
                    &["ALL", "PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER"],
                ),
                (
                    "period",
                    &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
                ),
            ],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("takerBuySellVolume", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`testConnectivity`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#test-connectivity).
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
        path: "/dapi/v1/ping",
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

/// Validated request builder for [`ticker24hrPriceChangeStatistics`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#ticker24hr-price-change-statistics).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Ticker24hrPriceChangeStatistics {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
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
impl Request for Ticker24hrPriceChangeStatistics {
    type Response = super::rest_models::Ticker24hrPriceChangeStatisticsResponse;
    const OP: Operation = Operation {
        name: "ticker24hrPriceChangeStatistics",
        path: "/dapi/v1/ticker/24hr",
        method: "GET",
        security: Security::Public,
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
        super::validation::validate("ticker24hrPriceChangeStatistics", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`topTraderLongShortRatioAccounts`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-accounts).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TopTraderLongShortRatioAccounts {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    contract_type: Option<String>,
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
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
        self
    }
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }
    /// Set the provider `contractType` parameter.
    #[must_use]
    pub fn contract_type(mut self, value: impl Into<String>) -> Self {
        self.contract_type = Some(value.into());
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
        validate_parameters(
            &p,
            &["pair", "period"],
            &[
                (
                    "period",
                    &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
                ),
                (
                    "contractType",
                    &["PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER"],
                ),
            ],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("topTraderLongShortRatioAccounts", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`topTraderLongShortRatioPositions`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-positions).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TopTraderLongShortRatioPositions {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    period: Option<String>,
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    contract_type: Option<String>,
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
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
        self
    }
    /// Set the provider `period` parameter.
    #[must_use]
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }
    /// Set the provider `contractType` parameter.
    #[must_use]
    pub fn contract_type(mut self, value: impl Into<String>) -> Self {
        self.contract_type = Some(value.into());
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
        validate_parameters(
            &p,
            &["pair", "period"],
            &[
                (
                    "period",
                    &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"],
                ),
                (
                    "contractType",
                    &["PERPETUAL", "CURRENT_QUARTER", "NEXT_QUARTER"],
                ),
            ],
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("topTraderLongShortRatioPositions", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountTradeList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#account-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountTradeList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
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
    /// Set the provider `pair` parameter.
    #[must_use]
    pub fn pair(mut self, value: Symbol) -> Self {
        self.pair = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
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
        path: "/dapi/v1/userTrades",
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

/// Validated request builder for [`allOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#all-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AllOrders {
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
        path: "/dapi/v1/allOrders",
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
            &[
                ("limit", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("allOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`autoCancelAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#auto-cancel-all-open-orders).
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
        path: "/dapi/v1/countdownCancelAll",
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

/// Validated request builder for [`cancelAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-all-open-orders).
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
        path: "/dapi/v1/allOpenOrders",
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
        super::validation::validate("cancelAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelMultipleOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-multiple-orders).
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
        path: "/dapi/v1/batchOrders",
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
        super::validation::validate("cancelMultipleOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyMultipleOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-multiple-orders).
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
        path: "/dapi/v1/batchOrders",
        method: "PUT",
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

/// Validated request builder for [`placeMultipleOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#place-multiple-orders).
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
        path: "/dapi/v1/batchOrders",
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

/// Validated request builder for [`cancelOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-order).
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
        path: "/dapi/v1/order",
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
        super::validation::validate("cancelOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-order).
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
        path: "/dapi/v1/order",
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
            &["side", "symbol"],
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("modifyOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#new-order).
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
        path: "/dapi/v1/order",
        method: "POST",
        security: Security::Signed,
        mutation: true,
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
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX"]),
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("newOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#query-order).
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
        path: "/dapi/v1/order",
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
        super::validation::validate("queryOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeInitialLeverage`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-initial-leverage).
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
        path: "/dapi/v1/leverage",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("changeInitialLeverage", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`changeMarginType`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-margin-type).
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
        path: "/dapi/v1/marginType",
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

/// Validated request builder for [`currentAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#current-all-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CurrentAllOpenOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
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
        path: "/dapi/v1/openOrders",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("currentAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getOrderModifyHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#get-order-modify-history).
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
        path: "/dapi/v1/orderAmendment",
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

/// Validated request builder for [`getPositionMarginChangeHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#get-position-margin-change-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetPositionMarginChangeHistory {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<i64>,
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
    pub fn type_value(mut self, value: i64) -> Self {
        self.type_value = Some(value);
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
        path: "/dapi/v1/positionMargin/history",
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
        super::validation::validate("getPositionMarginChangeHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`modifyIsolatedPositionMargin`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-isolated-position-margin).
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
        path: "/dapi/v1/positionMargin",
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
            &["amount", "symbol", "type"],
            &[("positionSide", &["BOTH", "LONG", "SHORT"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("modifyIsolatedPositionMargin", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`positionAdlQuantileEstimation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#position-adl-quantile-estimation).
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
        path: "/dapi/v1/adlQuantile",
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
        super::validation::validate("positionAdlQuantileEstimation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`positionInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#position-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PositionInformation {
    #[serde(rename = "marginAsset", skip_serializing_if = "Option::is_none")]
    margin_asset: Option<String>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pair: Option<Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PositionInformation {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `marginAsset` parameter.
    #[must_use]
    pub fn margin_asset(mut self, value: impl Into<String>) -> Self {
        self.margin_asset = Some(value.into());
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
impl Request for PositionInformation {
    type Response = super::rest_models::PositionInformationResponse;
    const OP: Operation = Operation {
        name: "positionInformation",
        path: "/dapi/v1/positionRisk",
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
        super::validation::validate("positionInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentOpenOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#query-current-open-order).
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
        path: "/dapi/v1/openOrder",
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
        super::validation::validate("queryCurrentOpenOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`usersForceOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#users-force-orders).
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
        path: "/dapi/v1/forceOrders",
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
            &[
                ("limit", -9_223_372_036_854_775_808, 100),
                ("recvWindow", -9_223_372_036_854_775_808, 60_000),
            ],
        )?;
        super::validation::validate("usersForceOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`closeUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#close-user-data-stream).
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
        path: "/dapi/v1/listenKey",
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

/// Validated request builder for [`keepaliveUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#keepalive-user-data-stream).
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
        path: "/dapi/v1/listenKey",
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

/// Validated request builder for [`startUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#start-user-data-stream).
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
        path: "/dapi/v1/listenKey",
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

/// Validated request builder for [`newAlgoOrder`](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice).
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
        path: "/dapi/v1/algoOrder",
        method: "POST",
        security: Security::Signed,
        mutation: true,
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
            &["algoType", "clientAlgoId", "side", "symbol", "type"],
            &[
                ("algoType", &["CONDITIONAL"]),
                ("side", &["BUY", "SELL"]),
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
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX", "GTD"]),
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
                (
                    "selfTradePreventionMode",
                    &["EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH", "NONE"],
                ),
            ],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("newAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAlgoOrder`](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice).
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
        path: "/dapi/v1/algoOrder",
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
        super::validation::validate("cancelAlgoOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`openAlgoOrders`](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenAlgoOrders {
    #[serde(rename = "algoType", skip_serializing_if = "Option::is_none")]
    algo_type: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    algo_id: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl OpenAlgoOrders {
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
impl Request for OpenAlgoOrders {
    type Response = super::rest_models::OpenAlgoOrdersResponse;
    const OP: Operation = Operation {
        name: "openAlgoOrders",
        path: "/dapi/v1/openAlgoOrders",
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
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("openAlgoOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

impl super::RestClient {
    /// [accountInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#account-information).
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

    /// [futuresAccountBalance](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#futures-account-balance).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn futures_account_balance(
        &self,
        request: &FuturesAccountBalance,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FuturesAccountBalanceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [getCurrentPositionMode](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-current-position-mode).
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

    /// [changePositionMode](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-position-mode).
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

    /// [getDownloadIdForFuturesOrderHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-order-history).
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

    /// [getDownloadIdForFuturesTradeHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-trade-history).
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

    /// [getDownloadIdForFuturesTransactionHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-download-id-for-futures-transaction-history).
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

    /// [getFuturesOrderHistoryDownloadLinkById](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-order-history-download-link-by-id).
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

    /// [getFuturesTradeDownloadLinkById](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-trade-download-link-by-id).
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

    /// [getFuturesTransactionHistoryDownloadLinkById](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-futures-transaction-history-download-link-by-id).
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

    /// [getIncomeHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#get-income-history).
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

    /// [notionalBracketForPair](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#notional-bracket-for-pair).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn notional_bracket_for_pair(
        &self,
        request: &NotionalBracketForPair,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NotionalBracketForPairResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [notionalBracketForSymbol](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#notional-bracket-for-symbol).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn notional_bracket_for_symbol(
        &self,
        request: &NotionalBracketForSymbol,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NotionalBracketForSymbolResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [userCommissionRate](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#user-commission-rate).
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

    /// [basis](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#basis).
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

    /// [checkServerTime](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#check-server-time).
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

    /// [compressedAggregateTradesList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#compressed-aggregate-trades-list).
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

    /// [continuousContractKlineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#continuous-contract-kline-candlestick-data).
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

    /// [exchangeInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#exchange-information).
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

    /// [getFundingRateHistoryOfPerpetualFutures](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#get-funding-rate-history-of-perpetual-futures).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_funding_rate_history_of_perpetual_futures(
        &self,
        request: &GetFundingRateHistoryOfPerpetualFutures,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::GetFundingRateHistoryOfPerpetualFuturesResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [fundingInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#get-funding-rate-info).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn funding_info(
        &self,
        request: &FundingInfo,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::FundingInfoResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [indexPriceAndMarkPrice](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#index-price-and-mark-price).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn index_price_and_mark_price(
        &self,
        request: &IndexPriceAndMarkPrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::IndexPriceAndMarkPriceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [indexPriceKlineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#index-price-kline-candlestick-data).
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

    /// [klineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#kline-candlestick-data).
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

    /// [longShortRatio](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#long-short-ratio).
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

    /// [markPriceKlineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#mark-price-kline-candlestick-data).
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

    /// [oldTradesLookup](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#old-trades-lookup).
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

    /// [openInterest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#open-interest).
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

    /// [openInterestStatistics](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#open-interest-statistics).
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

    /// [orderBook](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#order-book).
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

    /// [premiumIndexKlineData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#premium-index-kline-data).
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

    /// [queryIndexPriceConstituents](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#query-index-price-constituents).
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

    /// [recentTradesList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#recent-trades-list).
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

    /// [symbolOrderBookTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#symbol-order-book-ticker).
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

    /// [symbolPriceTicker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#symbol-price-ticker).
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

    /// [takerBuySellVolume](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#taker-buy-sell-volume).
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

    /// [testConnectivity](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#test-connectivity).
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

    /// [ticker24hrPriceChangeStatistics](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#ticker24hr-price-change-statistics).
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

    /// [topTraderLongShortRatioAccounts](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-accounts).
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

    /// [topTraderLongShortRatioPositions](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/market-data#top-trader-long-short-ratio-positions).
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

    /// [accountTradeList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#account-trade-list).
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

    /// [allOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#all-orders).
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

    /// [autoCancelAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#auto-cancel-all-open-orders).
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

    /// [cancelAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-all-open-orders).
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

    /// [cancelMultipleOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-multiple-orders).
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

    /// [modifyMultipleOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-multiple-orders).
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

    /// [placeMultipleOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#place-multiple-orders).
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

    /// [cancelOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#cancel-order).
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

    /// [modifyOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-order).
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

    /// [newOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#new-order).
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

    /// [queryOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#query-order).
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

    /// [changeInitialLeverage](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-initial-leverage).
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

    /// [changeMarginType](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#change-margin-type).
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

    /// [currentAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#current-all-open-orders).
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

    /// [getOrderModifyHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#get-order-modify-history).
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

    /// [getPositionMarginChangeHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#get-position-margin-change-history).
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

    /// [modifyIsolatedPositionMargin](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#modify-isolated-position-margin).
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

    /// [positionAdlQuantileEstimation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#position-adl-quantile-estimation).
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

    /// [positionInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#position-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn position_information(
        &self,
        request: &PositionInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::PositionInformationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentOpenOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#query-current-open-order).
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

    /// [usersForceOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#users-force-orders).
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

    /// [closeUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#close-user-data-stream).
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

    /// [keepaliveUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#keepalive-user-data-stream).
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

    /// [startUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/user-data-streams#start-user-data-stream).
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

    /// [newAlgoOrder](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice).
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

    /// [cancelAlgoOrder](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice).
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

    /// [openAlgoOrders](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn open_algo_orders(
        &self,
        request: &OpenAlgoOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OpenAlgoOrdersResponse>, Error> {
        self.inner.execute(request, deadline).await
    }
}
