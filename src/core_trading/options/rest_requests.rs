// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest request builders.

use super::rest_models::{NewBlockTradeOrderLegsInputItem, PlaceMultipleOrdersOrdersInputItem};
use crate::Decimal;
use crate::Error;
use crate::core::{Operation, Request, Security, parameters, validate_parameters};
use serde::Serialize;

/// Validated request builder for [`accountFundingFlow`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/account#account-funding-flow).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountFundingFlow {
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    currency: Option<crate::Asset>,
    #[serde(rename = "recordId", skip_serializing_if = "Option::is_none")]
    record_id: Option<crate::core_trading::options::RecordId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountFundingFlow {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `currency` parameter.
    #[must_use]
    pub fn currency(mut self, value: crate::Asset) -> Self {
        self.currency = Some(value);
        self
    }
    /// Set the provider `recordId` parameter.
    #[must_use]
    pub fn record_id(mut self, value: crate::core_trading::options::RecordId) -> Self {
        self.record_id = Some(value);
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
impl Request for AccountFundingFlow {
    type Response = super::rest_models::AccountFundingFlowResponse;
    const OP: Operation = Operation {
        name: "accountFundingFlow",
        path: "/eapi/v1/bill",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 1,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_three_month_history,
        definitive: super::validation::definitive,
        success_weight: None,
        partial: None,
    };
    fn validate(&self) -> Result<(), Error> {
        let p = parameters(self)?;
        validate_parameters(
            &p,
            &["currency"],
            &[],
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("accountFundingFlow", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`optionMarginAccountInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/account#option-margin-account-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OptionMarginAccountInformation {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl OptionMarginAccountInformation {
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
impl Request for OptionMarginAccountInformation {
    type Response = super::rest_models::OptionMarginAccountInformationResponse;
    const OP: Operation = Operation {
        name: "optionMarginAccountInformation",
        path: "/eapi/v1/marginAccount",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 3,
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
        super::validation::validate("optionMarginAccountInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`checkServerTime`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#check-server-time).
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
}
impl Request for CheckServerTime {
    type Response = super::rest_models::CheckServerTimeResponse;
    const OP: Operation = Operation {
        name: "checkServerTime",
        path: "/eapi/v1/time",
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

/// Validated request builder for [`exchangeInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#exchange-information).
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
}
impl Request for ExchangeInformation {
    type Response = super::rest_models::ExchangeInformationResponse;
    const OP: Operation = Operation {
        name: "exchangeInformation",
        path: "/eapi/v1/exchangeInfo",
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

/// Validated request builder for [`historicalExerciseRecords`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#historical-exercise-records).
#[derive(Clone, Debug, Default, Serialize)]
pub struct HistoricalExerciseRecords {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl HistoricalExerciseRecords {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
impl Request for HistoricalExerciseRecords {
    type Response = super::rest_models::HistoricalExerciseRecordsResponse;
    const OP: Operation = Operation {
        name: "historicalExerciseRecords",
        path: "/eapi/v1/exerciseHistory",
        method: "GET",
        security: Security::Public,
        mutation: false,
        weight: 3,
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
        super::validation::validate("historicalExerciseRecords", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`indexPrice`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#index-price).
#[derive(Clone, Debug, Default, Serialize)]
pub struct IndexPrice {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
}
impl IndexPrice {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for IndexPrice {
    type Response = super::rest_models::IndexPriceResponse;
    const OP: Operation = Operation {
        name: "indexPrice",
        path: "/eapi/v1/index",
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
        validate_parameters(&p, &["underlying"], &[], &[])?;
        super::validation::validate("indexPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`klineCandlestickData`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#kline-candlestick-data).
#[derive(Clone, Debug, Default, Serialize)]
pub struct KlineCandlestickData {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
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
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
}
impl Request for KlineCandlestickData {
    type Response = super::rest_models::KlineCandlestickDataResponse;
    const OP: Operation = Operation {
        name: "klineCandlestickData",
        path: "/eapi/v1/klines",
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

/// Validated request builder for [`openInterest`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#open-interest).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpenInterest {
    #[serde(rename = "underlyingAsset", skip_serializing_if = "Option::is_none")]
    underlying_asset: Option<crate::Asset>,
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    expiration: Option<String>,
}
impl OpenInterest {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlyingAsset` parameter.
    #[must_use]
    pub fn underlying_asset(mut self, value: crate::Asset) -> Self {
        self.underlying_asset = Some(value);
        self
    }
    /// Set the provider `expiration` parameter.
    #[must_use]
    pub fn expiration(mut self, value: impl Into<String>) -> Self {
        self.expiration = Some(value.into());
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for OpenInterest {
    type Response = super::rest_models::OpenInterestResponse;
    const OP: Operation = Operation {
        name: "openInterest",
        path: "/eapi/v1/openInterest",
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
        validate_parameters(&p, &["expiration", "underlyingAsset"], &[], &[])?;
        super::validation::validate("openInterest", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`optionMarkPrice`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#option-mark-price).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OptionMarkPrice {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
}
impl OptionMarkPrice {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
impl Request for OptionMarkPrice {
    type Response = super::rest_models::OptionMarkPriceResponse;
    const OP: Operation = Operation {
        name: "optionMarkPrice",
        path: "/eapi/v1/mark",
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
        validate_parameters(&p, &[], &[], &[])?;
        super::validation::validate("optionMarkPrice", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`orderBook`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#order-book).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OrderBook {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
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
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
    type Response = super::rest_models::OrderBookResponse;
    const OP: Operation = Operation {
        name: "orderBook",
        path: "/eapi/v1/depth",
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

/// Validated request builder for [`recentBlockTradesList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#recent-block-trades-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RecentBlockTradesList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}
impl RecentBlockTradesList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
impl Request for RecentBlockTradesList {
    type Response = super::rest_models::RecentBlockTradesListResponse;
    const OP: Operation = Operation {
        name: "recentBlockTradesList",
        path: "/eapi/v1/blockTrades",
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
        validate_parameters(&p, &[], &[], &[("limit", -9_223_372_036_854_775_808, 500)])?;
        super::validation::validate("recentBlockTradesList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`recentTradesList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#recent-trades-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RecentTradesList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
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
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
impl Request for RecentTradesList {
    type Response = super::rest_models::RecentTradesListResponse;
    const OP: Operation = Operation {
        name: "recentTradesList",
        path: "/eapi/v1/trades",
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
            &[("limit", -9_223_372_036_854_775_808, 500)],
        )?;
        super::validation::validate("recentTradesList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`testConnectivity`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#test-connectivity).
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
    const OP: Operation = Operation {
        name: "testConnectivity",
        path: "/eapi/v1/ping",
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

/// Validated request builder for [`ticker24hrPriceChangeStatistics`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#ticker24hr-price-change-statistics).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Ticker24hrPriceChangeStatistics {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
}
impl Ticker24hrPriceChangeStatistics {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
impl Request for Ticker24hrPriceChangeStatistics {
    type Response = super::rest_models::Ticker24hrPriceChangeStatisticsResponse;
    const OP: Operation = Operation {
        name: "ticker24hrPriceChangeStatistics",
        path: "/eapi/v1/ticker",
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

/// Validated request builder for [`acceptBlockTradeOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#accept-block-trade-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AcceptBlockTradeOrder {
    #[serde(
        rename = "blockOrderMatchingKey",
        skip_serializing_if = "Option::is_none"
    )]
    block_order_matching_key: Option<crate::core_trading::options::BlockOrderMatchingKey>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AcceptBlockTradeOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `blockOrderMatchingKey` parameter.
    #[must_use]
    pub fn block_order_matching_key(
        mut self,
        value: crate::core_trading::options::BlockOrderMatchingKey,
    ) -> Self {
        self.block_order_matching_key = Some(value);
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
impl Request for AcceptBlockTradeOrder {
    type Response = super::rest_models::AcceptBlockTradeOrderResponse;
    const OP: Operation = Operation {
        name: "acceptBlockTradeOrder",
        path: "/eapi/v1/block/order/execute",
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
            &["blockOrderMatchingKey"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("acceptBlockTradeOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryBlockTradeDetails`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#query-block-trade-details).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryBlockTradeDetails {
    #[serde(
        rename = "blockOrderMatchingKey",
        skip_serializing_if = "Option::is_none"
    )]
    block_order_matching_key: Option<crate::core_trading::options::BlockOrderMatchingKey>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryBlockTradeDetails {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `blockOrderMatchingKey` parameter.
    #[must_use]
    pub fn block_order_matching_key(
        mut self,
        value: crate::core_trading::options::BlockOrderMatchingKey,
    ) -> Self {
        self.block_order_matching_key = Some(value);
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
impl Request for QueryBlockTradeDetails {
    type Response = super::rest_models::QueryBlockTradeDetailsResponse;
    const OP: Operation = Operation {
        name: "queryBlockTradeDetails",
        path: "/eapi/v1/block/order/execute",
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
            &["blockOrderMatchingKey"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("queryBlockTradeDetails", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountBlockTradeList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#account-block-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountBlockTradeList {
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AccountBlockTradeList {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
impl Request for AccountBlockTradeList {
    type Response = super::rest_models::AccountBlockTradeListResponse;
    const OP: Operation = Operation {
        name: "accountBlockTradeList",
        path: "/eapi/v1/block/user-trades",
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
        super::validation::validate("accountBlockTradeList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelBlockTradeOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#cancel-block-trade-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelBlockTradeOrder {
    #[serde(
        rename = "blockOrderMatchingKey",
        skip_serializing_if = "Option::is_none"
    )]
    block_order_matching_key: Option<crate::core_trading::options::BlockOrderMatchingKey>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelBlockTradeOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `blockOrderMatchingKey` parameter.
    #[must_use]
    pub fn block_order_matching_key(
        mut self,
        value: crate::core_trading::options::BlockOrderMatchingKey,
    ) -> Self {
        self.block_order_matching_key = Some(value);
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
impl Request for CancelBlockTradeOrder {
    type Response = super::rest_models::CancelBlockTradeOrderResponse;
    const OP: Operation = Operation {
        name: "cancelBlockTradeOrder",
        path: "/eapi/v1/block/order/create",
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
            &["blockOrderMatchingKey"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("cancelBlockTradeOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`extendBlockTradeOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#extend-block-trade-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ExtendBlockTradeOrder {
    #[serde(
        rename = "blockOrderMatchingKey",
        skip_serializing_if = "Option::is_none"
    )]
    block_order_matching_key: Option<crate::core_trading::options::BlockOrderMatchingKey>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ExtendBlockTradeOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `blockOrderMatchingKey` parameter.
    #[must_use]
    pub fn block_order_matching_key(
        mut self,
        value: crate::core_trading::options::BlockOrderMatchingKey,
    ) -> Self {
        self.block_order_matching_key = Some(value);
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
impl Request for ExtendBlockTradeOrder {
    type Response = super::rest_models::ExtendBlockTradeOrderResponse;
    const OP: Operation = Operation {
        name: "extendBlockTradeOrder",
        path: "/eapi/v1/block/order/create",
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
            &["blockOrderMatchingKey"],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("extendBlockTradeOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newBlockTradeOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#new-block-trade-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewBlockTradeOrder {
    #[serde(rename = "liquidity", skip_serializing_if = "Option::is_none")]
    liquidity: Option<String>,
    #[serde(rename = "legs", skip_serializing_if = "Option::is_none")]
    legs: Option<Vec<NewBlockTradeOrderLegsInputItem>>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl NewBlockTradeOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `liquidity` parameter.
    #[must_use]
    pub fn liquidity(mut self, value: impl Into<String>) -> Self {
        self.liquidity = Some(value.into());
        self
    }
    /// Set the provider `legs` parameter.
    #[must_use]
    pub fn legs(mut self, value: Vec<NewBlockTradeOrderLegsInputItem>) -> Self {
        self.legs = Some(value);
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
impl Request for NewBlockTradeOrder {
    type Response = super::rest_models::NewBlockTradeOrderResponse;
    const OP: Operation = Operation {
        name: "newBlockTradeOrder",
        path: "/eapi/v1/block/order/create",
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
            &["legs", "liquidity"],
            &[("liquidity", &["MAKER", "TAKER"])],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("newBlockTradeOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryBlockTradeOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#query-block-trade-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryBlockTradeOrder {
    #[serde(
        rename = "blockOrderMatchingKey",
        skip_serializing_if = "Option::is_none"
    )]
    block_order_matching_key: Option<crate::core_trading::options::BlockOrderMatchingKey>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryBlockTradeOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `blockOrderMatchingKey` parameter.
    #[must_use]
    pub fn block_order_matching_key(
        mut self,
        value: crate::core_trading::options::BlockOrderMatchingKey,
    ) -> Self {
        self.block_order_matching_key = Some(value);
        self
    }
    /// Set the provider `endTime` parameter.
    #[must_use]
    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }
    /// Set the provider `startTime` parameter.
    #[must_use]
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
impl Request for QueryBlockTradeOrder {
    type Response = super::rest_models::QueryBlockTradeOrderResponse;
    const OP: Operation = Operation {
        name: "queryBlockTradeOrder",
        path: "/eapi/v1/block/order/orders",
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
        super::validation::validate("queryBlockTradeOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`autoCancelAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#auto-cancel-all-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AutoCancelAllOpenOrders {
    #[serde(rename = "underlyings", skip_serializing_if = "Option::is_none")]
    underlyings: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl AutoCancelAllOpenOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlyings` parameter.
    #[must_use]
    pub fn underlyings(mut self, value: impl Into<String>) -> Self {
        self.underlyings = Some(value.into());
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
impl Request for AutoCancelAllOpenOrders {
    type Response = super::rest_models::AutoCancelAllOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "autoCancelAllOpenOrders",
        path: "/eapi/v1/countdownCancelAllHeartBeat",
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
        validate_parameters(&p, &["underlyings"], &[], &[])?;
        super::validation::validate("autoCancelAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getAutoCancelAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#get-auto-cancel-all-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAutoCancelAllOpenOrders {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetAutoCancelAllOpenOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
impl Request for GetAutoCancelAllOpenOrders {
    type Response = super::rest_models::GetAutoCancelAllOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "getAutoCancelAllOpenOrders",
        path: "/eapi/v1/countdownCancelAll",
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
        super::validation::validate("getAutoCancelAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`setAutoCancelAllOpenOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#set-auto-cancel-all-open-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetAutoCancelAllOpenOrders {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "countdownTime", skip_serializing_if = "Option::is_none")]
    countdown_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl SetAutoCancelAllOpenOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
}
impl Request for SetAutoCancelAllOpenOrders {
    type Response = super::rest_models::SetAutoCancelAllOpenOrdersResponse;
    const OP: Operation = Operation {
        name: "setAutoCancelAllOpenOrders",
        path: "/eapi/v1/countdownCancelAll",
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
        validate_parameters(&p, &["countdownTime", "underlying"], &[], &[])?;
        super::validation::validate("setAutoCancelAllOpenOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`getMarketMakerProtectionConfig`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#get-market-maker-protection-config).
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetMarketMakerProtectionConfig {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl GetMarketMakerProtectionConfig {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
impl Request for GetMarketMakerProtectionConfig {
    type Response = super::rest_models::GetMarketMakerProtectionConfigResponse;
    const OP: Operation = Operation {
        name: "getMarketMakerProtectionConfig",
        path: "/eapi/v1/mmp",
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
        validate_parameters(&p, &["underlying"], &[], &[])?;
        super::validation::validate("getMarketMakerProtectionConfig", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`resetMarketMakerProtectionConfig`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#reset-market-maker-protection-config).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ResetMarketMakerProtectionConfig {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl ResetMarketMakerProtectionConfig {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
impl Request for ResetMarketMakerProtectionConfig {
    type Response = super::rest_models::ResetMarketMakerProtectionConfigResponse;
    const OP: Operation = Operation {
        name: "resetMarketMakerProtectionConfig",
        path: "/eapi/v1/mmpReset",
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
        validate_parameters(&p, &["underlying"], &[], &[])?;
        super::validation::validate("resetMarketMakerProtectionConfig", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`setMarketMakerProtectionConfig`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#set-market-maker-protection-config).
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetMarketMakerProtectionConfig {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(
        rename = "windowTimeInMilliseconds",
        skip_serializing_if = "Option::is_none"
    )]
    window_time_in_milliseconds: Option<i64>,
    #[serde(
        rename = "frozenTimeInMilliseconds",
        skip_serializing_if = "Option::is_none"
    )]
    frozen_time_in_milliseconds: Option<i64>,
    #[serde(rename = "qtyLimit", skip_serializing_if = "Option::is_none")]
    qty_limit: Option<Decimal>,
    #[serde(rename = "deltaLimit", skip_serializing_if = "Option::is_none")]
    delta_limit: Option<Decimal>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl SetMarketMakerProtectionConfig {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
        self
    }
    /// Set the provider `windowTimeInMilliseconds` parameter.
    #[must_use]
    pub fn window_time_in_milliseconds(mut self, value: i64) -> Self {
        self.window_time_in_milliseconds = Some(value);
        self
    }
    /// Set the provider `frozenTimeInMilliseconds` parameter.
    #[must_use]
    pub fn frozen_time_in_milliseconds(mut self, value: i64) -> Self {
        self.frozen_time_in_milliseconds = Some(value);
        self
    }
    /// Set the provider `qtyLimit` parameter.
    #[must_use]
    pub fn qty_limit(mut self, value: Decimal) -> Self {
        self.qty_limit = Some(value);
        self
    }
    /// Set the provider `deltaLimit` parameter.
    #[must_use]
    pub fn delta_limit(mut self, value: Decimal) -> Self {
        self.delta_limit = Some(value);
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
impl Request for SetMarketMakerProtectionConfig {
    type Response = super::rest_models::SetMarketMakerProtectionConfigResponse;
    const OP: Operation = Operation {
        name: "setMarketMakerProtectionConfig",
        path: "/eapi/v1/mmpSet",
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
                "deltaLimit",
                "frozenTimeInMilliseconds",
                "qtyLimit",
                "underlying",
                "windowTimeInMilliseconds",
            ],
            &[],
            &[("windowTimeInMilliseconds", 0, 5_000)],
        )?;
        super::validation::validate("setMarketMakerProtectionConfig", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`accountTradeList`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#account-trade-list).
#[derive(Clone, Debug, Default, Serialize)]
pub struct AccountTradeList {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    from_id: Option<crate::core_trading::options::TradeId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
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
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `fromId` parameter.
    #[must_use]
    pub fn from_id(mut self, value: crate::core_trading::options::TradeId) -> Self {
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
impl Request for AccountTradeList {
    type Response = super::rest_models::AccountTradeListResponse;
    const OP: Operation = Operation {
        name: "accountTradeList",
        path: "/eapi/v1/userTrades",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 5,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_three_month_history,
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
        super::validation::validate("accountTradeList", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllOptionOrdersByUnderlying`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-all-option-orders-by-underlying).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllOptionOrdersByUnderlying {
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    underlying: Option<crate::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllOptionOrdersByUnderlying {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `underlying` parameter.
    #[must_use]
    pub fn underlying(mut self, value: crate::Symbol) -> Self {
        self.underlying = Some(value);
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
impl Request for CancelAllOptionOrdersByUnderlying {
    type Response = super::rest_models::CancelAllOptionOrdersByUnderlyingResponse;
    const OP: Operation = Operation {
        name: "cancelAllOptionOrdersByUnderlying",
        path: "/eapi/v1/allOpenOrdersByUnderlying",
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
        validate_parameters(&p, &["underlying"], &[], &[])?;
        super::validation::validate("cancelAllOptionOrdersByUnderlying", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelAllOptionOrdersOnSpecificSymbol`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-all-option-orders-on-specific-symbol).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelAllOptionOrdersOnSpecificSymbol {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelAllOptionOrdersOnSpecificSymbol {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
impl Request for CancelAllOptionOrdersOnSpecificSymbol {
    type Response = super::rest_models::CancelAllOptionOrdersOnSpecificSymbolResponse;
    const OP: Operation = Operation {
        name: "cancelAllOptionOrdersOnSpecificSymbol",
        path: "/eapi/v1/allOpenOrders",
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
        super::validation::validate("cancelAllOptionOrdersOnSpecificSymbol", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelMultipleOptionOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-multiple-option-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelMultipleOptionOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "orderIds", skip_serializing_if = "Option::is_none")]
    order_ids: Option<Vec<crate::core_trading::options::OrderId>>,
    #[serde(rename = "clientOrderIds", skip_serializing_if = "Option::is_none")]
    client_order_ids: Option<Vec<crate::core_trading::options::ClientOrderId>>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelMultipleOptionOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `orderIds` parameter.
    #[must_use]
    pub fn order_ids(mut self, value: Vec<crate::core_trading::options::OrderId>) -> Self {
        self.order_ids = Some(value);
        self
    }
    /// Set the provider `clientOrderIds` parameter.
    #[must_use]
    pub fn client_order_ids(
        mut self,
        value: Vec<crate::core_trading::options::ClientOrderId>,
    ) -> Self {
        self.client_order_ids = Some(value);
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
impl Request for CancelMultipleOptionOrders {
    type Response = super::rest_models::CancelMultipleOptionOrdersResponse;
    const OP: Operation = Operation {
        name: "cancelMultipleOptionOrders",
        path: "/eapi/v1/batchOrders",
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
        validate_parameters(&p, &["symbol"], &[], &[])?;
        super::validation::validate("cancelMultipleOptionOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`placeMultipleOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#place-multiple-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct PlaceMultipleOrders {
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    orders: Option<Vec<PlaceMultipleOrdersOrdersInputItem>>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl PlaceMultipleOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `orders` parameter.
    #[must_use]
    pub fn orders(mut self, value: Vec<PlaceMultipleOrdersOrdersInputItem>) -> Self {
        self.orders = Some(value);
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
impl Request for PlaceMultipleOrders {
    type Response = super::rest_models::PlaceMultipleOrdersResponse;
    const OP: Operation = Operation {
        name: "placeMultipleOrders",
        path: "/eapi/v1/batchOrders",
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
        validate_parameters(&p, &["orders"], &[], &[])?;
        super::validation::validate("placeMultipleOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`cancelOptionOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-option-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelOptionOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::core_trading::options::OrderId>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    client_order_id: Option<crate::core_trading::options::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl CancelOptionOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::core_trading::options::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `clientOrderId` parameter.
    #[must_use]
    pub fn client_order_id(mut self, value: crate::core_trading::options::ClientOrderId) -> Self {
        self.client_order_id = Some(value);
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
impl Request for CancelOptionOrder {
    type Response = super::rest_models::CancelOptionOrderResponse;
    const OP: Operation = Operation {
        name: "cancelOptionOrder",
        path: "/eapi/v1/order",
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
        super::validation::validate("cancelOptionOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`newOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#new-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct NewOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_value: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    quantity: Option<Decimal>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    price: Option<Decimal>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    time_in_force: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    post_only: Option<bool>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    response_type: Option<String>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    client_order_id: Option<crate::core_trading::options::ClientOrderId>,
    #[serde(rename = "isMmp", skip_serializing_if = "Option::is_none")]
    is_mmp: Option<bool>,
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
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
    /// Set the provider `price` parameter.
    #[must_use]
    pub fn price(mut self, value: Decimal) -> Self {
        self.price = Some(value);
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
    pub fn reduce_only(mut self, value: bool) -> Self {
        self.reduce_only = Some(value);
        self
    }
    /// Set the provider `postOnly` parameter.
    #[must_use]
    pub fn post_only(mut self, value: bool) -> Self {
        self.post_only = Some(value);
        self
    }
    /// Set the provider `newOrderRespType` parameter.
    #[must_use]
    pub fn response_type(mut self, value: impl Into<String>) -> Self {
        self.response_type = Some(value.into());
        self
    }
    /// Set the provider `clientOrderId` parameter.
    #[must_use]
    pub fn client_order_id(mut self, value: crate::core_trading::options::ClientOrderId) -> Self {
        self.client_order_id = Some(value);
        self
    }
    /// Set the provider `isMmp` parameter.
    #[must_use]
    pub fn is_mmp(mut self, value: bool) -> Self {
        self.is_mmp = Some(value);
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
}
impl Request for NewOrder {
    type Response = super::rest_models::NewOrderResponse;
    const OP: Operation = Operation {
        name: "newOrder",
        path: "/eapi/v1/order",
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
            &[
                "clientOrderId",
                "price",
                "quantity",
                "side",
                "symbol",
                "type",
            ],
            &[
                ("side", &["BUY", "SELL"]),
                ("type", &["LIMIT"]),
                ("timeInForce", &["GTC", "IOC", "FOK", "GTX"]),
                ("newOrderRespType", &["ACK", "RESULT"]),
                (
                    "selfTradePreventionMode",
                    &["NONE", "EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH"],
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

/// Validated request builder for [`querySingleOrder`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-single-order).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QuerySingleOrder {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::core_trading::options::OrderId>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    client_order_id: Option<crate::core_trading::options::ClientOrderId>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QuerySingleOrder {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::core_trading::options::OrderId) -> Self {
        self.order_id = Some(value);
        self
    }
    /// Set the provider `clientOrderId` parameter.
    #[must_use]
    pub fn client_order_id(mut self, value: crate::core_trading::options::ClientOrderId) -> Self {
        self.client_order_id = Some(value);
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
impl Request for QuerySingleOrder {
    type Response = super::rest_models::QuerySingleOrderResponse;
    const OP: Operation = Operation {
        name: "querySingleOrder",
        path: "/eapi/v1/order",
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
        super::validation::validate("querySingleOrder", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`optionPositionInformation`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#option-position-information).
#[derive(Clone, Debug, Default, Serialize)]
pub struct OptionPositionInformation {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl OptionPositionInformation {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
impl Request for OptionPositionInformation {
    type Response = super::rest_models::OptionPositionInformationResponse;
    const OP: Operation = Operation {
        name: "optionPositionInformation",
        path: "/eapi/v1/position",
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
        super::validation::validate("optionPositionInformation", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryCurrentOpenOptionOrders`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-current-open-option-orders).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryCurrentOpenOptionOrders {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::core_trading::options::OrderId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryCurrentOpenOptionOrders {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::core_trading::options::OrderId) -> Self {
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
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for QueryCurrentOpenOptionOrders {
    type Response = super::rest_models::QueryCurrentOpenOptionOrdersResponse;
    const OP: Operation = Operation {
        name: "queryCurrentOpenOptionOrders",
        path: "/eapi/v1/openOrders",
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
        super::validation::validate("queryCurrentOpenOptionOrders", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`queryOptionOrderHistory`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-option-order-history).
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryOptionOrderHistory {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    order_id: Option<crate::core_trading::options::OrderId>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl QueryOptionOrderHistory {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
        self.symbol = Some(value);
        self
    }
    /// Set the provider `orderId` parameter.
    #[must_use]
    pub fn order_id(mut self, value: crate::core_trading::options::OrderId) -> Self {
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
impl Request for QueryOptionOrderHistory {
    type Response = super::rest_models::QueryOptionOrderHistoryResponse;
    const OP: Operation = Operation {
        name: "queryOptionOrderHistory",
        path: "/eapi/v1/historyOrders",
        method: "GET",
        security: Security::Signed,
        mutation: false,
        weight: 3,
        requests_per_second: None,
        requests_per_minute: None,
        validate_time: super::validation::validate_five_day_history,
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
        super::validation::validate("queryOptionOrderHistory", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`tradfiOptionsContract`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#tradfi-options-contract).
#[derive(Clone, Debug, Default, Serialize)]
pub struct TradfiOptionsContract {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl TradfiOptionsContract {
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
impl Request for TradfiOptionsContract {
    type Response = super::rest_models::TradfiOptionsContractResponse;
    const OP: Operation = Operation {
        name: "tradfiOptionsContract",
        path: "/eapi/v1/stock/contract",
        method: "POST",
        security: Security::Signed,
        mutation: true,
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
            &[],
            &[],
            &[("recvWindow", -9_223_372_036_854_775_808, 60_000)],
        )?;
        super::validation::validate("tradfiOptionsContract", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userCommission`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#user-commission).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserCommission {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UserCommission {
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
impl Request for UserCommission {
    type Response = super::rest_models::UserCommissionResponse;
    const OP: Operation = Operation {
        name: "userCommission",
        path: "/eapi/v1/commission",
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
        super::validation::validate("userCommission", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`userExerciseRecord`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#user-exercise-record).
#[derive(Clone, Debug, Default, Serialize)]
pub struct UserExerciseRecord {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    symbol: Option<crate::core_trading::options::Symbol>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
}
impl UserExerciseRecord {
    /// Start a request builder. Required inputs are checked by `build` and by dispatch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the provider `symbol` parameter.
    #[must_use]
    pub fn symbol(mut self, value: crate::core_trading::options::Symbol) -> Self {
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
    /// Set the provider `recvWindow` parameter.
    #[must_use]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate this request before dispatch.
    ///
    /// # Errors
    /// Refuses missing, invalid, or contradictory provider parameters.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
}
impl Request for UserExerciseRecord {
    type Response = super::rest_models::UserExerciseRecordResponse;
    const OP: Operation = Operation {
        name: "userExerciseRecord",
        path: "/eapi/v1/exerciseRecord",
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
            &[("limit", -9_223_372_036_854_775_808, 1_000)],
        )?;
        super::validation::validate("userExerciseRecord", &p)
    }
    fn cost(&self) -> Result<crate::core::Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
}

/// Validated request builder for [`closeUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#close-user-data-stream).
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
    const OP: Operation = Operation {
        name: "closeUserDataStream",
        path: "/eapi/v1/listenKey",
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

/// Validated request builder for [`keepaliveUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#keepalive-user-data-stream).
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
    const OP: Operation = Operation {
        name: "keepaliveUserDataStream",
        path: "/eapi/v1/listenKey",
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

/// Validated request builder for [`startUserDataStream`](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#start-user-data-stream).
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
        path: "/eapi/v1/listenKey",
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
    /// [accountFundingFlow](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/account#account-funding-flow).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_funding_flow(
        &self,
        request: &AccountFundingFlow,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountFundingFlowResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [optionMarginAccountInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/account#option-margin-account-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn option_margin_account_information(
        &self,
        request: &OptionMarginAccountInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OptionMarginAccountInformationResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [checkServerTime](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#check-server-time).
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

    /// [exchangeInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#exchange-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn exchange_information(
        &self,
        request: &ExchangeInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ExchangeInformationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [historicalExerciseRecords](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#historical-exercise-records).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn historical_exercise_records(
        &self,
        request: &HistoricalExerciseRecords,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::HistoricalExerciseRecordsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [indexPrice](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#index-price).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn index_price(
        &self,
        request: &IndexPrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::IndexPriceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [klineCandlestickData](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#kline-candlestick-data).
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

    /// [openInterest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#open-interest).
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

    /// [optionMarkPrice](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#option-mark-price).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn option_mark_price(
        &self,
        request: &OptionMarkPrice,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OptionMarkPriceResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [orderBook](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#order-book).
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

    /// [recentBlockTradesList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#recent-block-trades-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn recent_block_trades_list(
        &self,
        request: &RecentBlockTradesList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::RecentBlockTradesListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [recentTradesList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#recent-trades-list).
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

    /// [testConnectivity](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#test-connectivity).
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

    /// [ticker24hrPriceChangeStatistics](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-data#ticker24hr-price-change-statistics).
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

    /// [acceptBlockTradeOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#accept-block-trade-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn accept_block_trade_order(
        &self,
        request: &AcceptBlockTradeOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AcceptBlockTradeOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryBlockTradeDetails](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#query-block-trade-details).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_block_trade_details(
        &self,
        request: &QueryBlockTradeDetails,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryBlockTradeDetailsResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [accountBlockTradeList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#account-block-trade-list).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn account_block_trade_list(
        &self,
        request: &AccountBlockTradeList,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::AccountBlockTradeListResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [cancelBlockTradeOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#cancel-block-trade-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_block_trade_order(
        &self,
        request: &CancelBlockTradeOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelBlockTradeOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [extendBlockTradeOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#extend-block-trade-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn extend_block_trade_order(
        &self,
        request: &ExtendBlockTradeOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ExtendBlockTradeOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newBlockTradeOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#new-block-trade-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn new_block_trade_order(
        &self,
        request: &NewBlockTradeOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::NewBlockTradeOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryBlockTradeOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-block-trade#query-block-trade-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_block_trade_order(
        &self,
        request: &QueryBlockTradeOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryBlockTradeOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [autoCancelAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#auto-cancel-all-open-orders).
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

    /// [getAutoCancelAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#get-auto-cancel-all-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_auto_cancel_all_open_orders(
        &self,
        request: &GetAutoCancelAllOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetAutoCancelAllOpenOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [setAutoCancelAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#set-auto-cancel-all-open-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn set_auto_cancel_all_open_orders(
        &self,
        request: &SetAutoCancelAllOpenOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SetAutoCancelAllOpenOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [getMarketMakerProtectionConfig](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#get-market-maker-protection-config).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn get_market_maker_protection_config(
        &self,
        request: &GetMarketMakerProtectionConfig,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::GetMarketMakerProtectionConfigResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [resetMarketMakerProtectionConfig](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#reset-market-maker-protection-config).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn reset_market_maker_protection_config(
        &self,
        request: &ResetMarketMakerProtectionConfig,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::ResetMarketMakerProtectionConfigResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [setMarketMakerProtectionConfig](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/market-maker-endpoints#set-market-maker-protection-config).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn set_market_maker_protection_config(
        &self,
        request: &SetMarketMakerProtectionConfig,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::SetMarketMakerProtectionConfigResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [accountTradeList](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#account-trade-list).
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

    /// [cancelAllOptionOrdersByUnderlying](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-all-option-orders-by-underlying).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_option_orders_by_underlying(
        &self,
        request: &CancelAllOptionOrdersByUnderlying,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelAllOptionOrdersByUnderlyingResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [cancelAllOptionOrdersOnSpecificSymbol](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-all-option-orders-on-specific-symbol).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_all_option_orders_on_specific_symbol(
        &self,
        request: &CancelAllOptionOrdersOnSpecificSymbol,
        deadline: tokio::time::Instant,
    ) -> Result<
        crate::Response<super::rest_models::CancelAllOptionOrdersOnSpecificSymbolResponse>,
        Error,
    > {
        self.inner.execute(request, deadline).await
    }

    /// [cancelMultipleOptionOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-multiple-option-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_multiple_option_orders(
        &self,
        request: &CancelMultipleOptionOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelMultipleOptionOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [placeMultipleOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#place-multiple-orders).
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

    /// [cancelOptionOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#cancel-option-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn cancel_option_order(
        &self,
        request: &CancelOptionOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::CancelOptionOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [newOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#new-order).
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

    /// [querySingleOrder](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-single-order).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_single_order(
        &self,
        request: &QuerySingleOrder,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QuerySingleOrderResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [optionPositionInformation](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#option-position-information).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn option_position_information(
        &self,
        request: &OptionPositionInformation,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::OptionPositionInformationResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [queryCurrentOpenOptionOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-current-open-option-orders).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_current_open_option_orders(
        &self,
        request: &QueryCurrentOpenOptionOrders,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryCurrentOpenOptionOrdersResponse>, Error>
    {
        self.inner.execute(request, deadline).await
    }

    /// [queryOptionOrderHistory](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#query-option-order-history).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn query_option_order_history(
        &self,
        request: &QueryOptionOrderHistory,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::QueryOptionOrderHistoryResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [tradfiOptionsContract](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#tradfi-options-contract).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn tradfi_options_contract(
        &self,
        request: &TradfiOptionsContract,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::TradfiOptionsContractResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [userCommission](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#user-commission).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_commission(
        &self,
        request: &UserCommission,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UserCommissionResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [userExerciseRecord](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade#user-exercise-record).
    ///
    /// # Errors
    /// Returns input/admission errors before sending, or typed venue/transport evidence.
    pub async fn user_exercise_record(
        &self,
        request: &UserExerciseRecord,
        deadline: tokio::time::Instant,
    ) -> Result<crate::Response<super::rest_models::UserExerciseRecordResponse>, Error> {
        self.inner.execute(request, deadline).await
    }

    /// [closeUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#close-user-data-stream).
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

    /// [keepaliveUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#keepalive-user-data-stream).
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

    /// [startUserDataStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/user-data-streams#start-user-data-stream).
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
