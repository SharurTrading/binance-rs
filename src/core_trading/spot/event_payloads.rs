// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Typed payload dispatch for API and market/user-data events.

use crate::{Error, SensitiveString};
use serde_json::Value;

/// Unknown future payloads are retained, with redacted Debug output.
#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UnknownMessage(Value);
impl UnknownMessage {
    /// Explicit provider payload access; do not log sensitive account data.
    #[must_use]
    pub fn as_value(&self) -> &Value {
        &self.0
    }
}
impl std::fmt::Debug for UnknownMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UnknownMessage([REDACTED])")
    }
}
impl From<Value> for UnknownMessage {
    fn from(v: Value) -> Self {
        Self(v)
    }
}

/// A late WebSocket API response, decoded using its original operation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ApiPayload {
    /// `accountCommission` response.
    AccountCommission(Box<super::ws_models::AccountCommissionResponse>),
    /// `accountRateLimitsOrders` response.
    AccountRateLimitsOrders(Box<super::ws_models::AccountRateLimitsOrdersResponse>),
    /// `accountStatus` response.
    AccountStatus(Box<super::ws_models::AccountStatusResponse>),
    /// `allOrders` response.
    AllOrders(Box<super::ws_models::AllOrdersResponse>),
    /// `myTrades` response.
    MyTrades(Box<super::ws_models::MyTradesResponse>),
    /// `openOrdersStatus` response.
    OpenOrdersStatus(Box<super::ws_models::OpenOrdersStatusResponse>),
    /// `orderStatus` response.
    OrderStatus(Box<super::ws_models::OrderStatusResponse>),
    /// `exchangeInfo` response.
    ExchangeInfo(Box<super::ws_models::ExchangeInfoResponse>),
    /// `executionRules` response.
    ExecutionRules(Box<super::ws_models::ExecutionRulesResponse>),
    /// `ping` response.
    Ping(Box<super::ws_models::PingResponse>),
    /// `time` response.
    Time(Box<super::ws_models::TimeResponse>),
    /// `avgPrice` response.
    AvgPrice(Box<super::ws_models::AvgPriceResponse>),
    /// `depth` response.
    Depth(Box<super::ws_models::DepthResponse>),
    /// `klines` response.
    Klines(Box<super::ws_models::KlinesResponse>),
    /// `ticker` response.
    Ticker(Box<super::ws_models::TickerResponse>),
    /// `ticker24hr` response.
    Ticker24hr(Box<super::ws_models::Ticker24hrResponse>),
    /// `tickerBook` response.
    TickerBook(Box<super::ws_models::TickerBookResponse>),
    /// `tickerPrice` response.
    TickerPrice(Box<super::ws_models::TickerPriceResponse>),
    /// `tickerTradingDay` response.
    TickerTradingDay(Box<super::ws_models::TickerTradingDayResponse>),
    /// `tradesAggregate` response.
    TradesAggregate(Box<super::ws_models::TradesAggregateResponse>),
    /// `tradesHistorical` response.
    TradesHistorical(Box<super::ws_models::TradesHistoricalResponse>),
    /// `blockTradesHistorical` response.
    BlockTradesHistorical(Box<super::ws_models::BlockTradesHistoricalResponse>),
    /// `tradesRecent` response.
    TradesRecent(Box<super::ws_models::TradesRecentResponse>),
    /// `uiKlines` response.
    UiKlines(Box<super::ws_models::UiKlinesResponse>),
    /// `referencePrice` response.
    ReferencePrice(Box<super::ws_models::ReferencePriceResponse>),
    /// `referencePriceCalculation` response.
    ReferencePriceCalculation(Box<super::ws_models::ReferencePriceCalculationResponse>),
    /// `openOrdersCancelAll` response.
    OpenOrdersCancelAll(Box<super::ws_models::OpenOrdersCancelAllResponse>),
    /// `orderCancel` response.
    OrderCancel(Box<super::ws_models::OrderCancelResponse>),
    /// `orderPlace` response.
    OrderPlace(Box<super::ws_models::OrderPlaceResponse>),
    /// `orderTest` response.
    OrderTest(Box<super::ws_models::OrderTestResponse>),
    /// `sessionSubscriptions` response.
    SessionSubscriptions(Box<super::ws_models::SessionSubscriptionsResponse>),
    /// `userDataStreamSubscribe` response.
    UserDataStreamSubscribe(Box<super::ws_models::UserDataStreamSubscribeResponse>),
    /// `userDataStreamSubscribeSignature` response.
    UserDataStreamSubscribeSignature(
        Box<super::ws_models::UserDataStreamSubscribeSignatureResponse>,
    ),
    /// `userDataStreamUnsubscribe` response.
    UserDataStreamUnsubscribe(Box<super::ws_models::UserDataStreamUnsubscribeResponse>),
    /// `allOrderLists` response.
    AllOrderLists(Box<super::ws_models::AllOrderListsResponse>),
    /// `myAllocations` response.
    MyAllocations(Box<super::ws_models::MyAllocationsResponse>),
    /// `myFilters` response.
    MyFilters(Box<super::ws_models::MyFiltersResponse>),
    /// `myPreventedMatches` response.
    MyPreventedMatches(Box<super::ws_models::MyPreventedMatchesResponse>),
    /// `openOrderListsStatus` response.
    OpenOrderListsStatus(Box<super::ws_models::OpenOrderListsStatusResponse>),
    /// `orderAmendments` response.
    OrderAmendments(Box<super::ws_models::OrderAmendmentsResponse>),
    /// `orderListStatus` response.
    OrderListStatus(Box<super::ws_models::OrderListStatusResponse>),
    /// `orderAmendKeepPriority` response.
    OrderAmendKeepPriority(Box<super::ws_models::OrderAmendKeepPriorityResponse>),
    /// `orderCancelReplace` response.
    OrderCancelReplace(Box<super::ws_models::OrderCancelReplaceResponse>),
    /// `orderListCancel` response.
    OrderListCancel(Box<super::ws_models::OrderListCancelResponse>),
    /// `orderListPlace` response.
    OrderListPlace(Box<super::ws_models::OrderListPlaceResponse>),
    /// `orderListPlaceOco` response.
    OrderListPlaceOco(Box<super::ws_models::OrderListPlaceOcoResponse>),
    /// `orderListPlaceOpo` response.
    OrderListPlaceOpo(Box<super::ws_models::OrderListPlaceOpoResponse>),
    /// `orderListPlaceOpoco` response.
    OrderListPlaceOpoco(Box<super::ws_models::OrderListPlaceOpocoResponse>),
    /// `orderListPlaceOto` response.
    OrderListPlaceOto(Box<super::ws_models::OrderListPlaceOtoResponse>),
    /// `orderListPlaceOtoco` response.
    OrderListPlaceOtoco(Box<super::ws_models::OrderListPlaceOtocoResponse>),
    /// `sorOrderPlace` response.
    SorOrderPlace(Box<super::ws_models::SorOrderPlaceResponse>),
    /// `sorOrderTest` response.
    SorOrderTest(Box<super::ws_models::SorOrderTestResponse>),
    /// Session authentication/status response.
    Session(SessionStatus),
}

/// Provider session evidence, never an assertion about order truth.
#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[non_exhaustive]
pub struct SessionStatus {
    /// Connection timestamp.
    #[serde(rename = "connectedSince")]
    pub connected_since: Option<i64>,
    /// Authentication timestamp.
    #[serde(rename = "authorizedSince")]
    pub authorized_since: Option<i64>,
    /// Current server timestamp.
    #[serde(rename = "serverTime")]
    pub server_time: Option<i64>,
    /// Authenticated API key; redacted by default.
    #[serde(rename = "apiKey")]
    pub api_key: Option<SensitiveString>,
    /// Unknown future session fields.
    #[serde(flatten)]
    pub extra: UnknownMessage,
}

pub(crate) fn api_payload(operation: &str, value: Value) -> Result<ApiPayload, Error> {
    match operation {
        "accountCommission"
        | "accountRateLimitsOrders"
        | "accountStatus"
        | "allOrders"
        | "myTrades"
        | "openOrdersStatus"
        | "orderStatus"
        | "exchangeInfo"
        | "executionRules"
        | "ping"
        | "time"
        | "avgPrice"
        | "depth"
        | "klines"
        | "ticker"
        | "ticker24hr"
        | "tickerBook"
        | "tickerPrice"
        | "tickerTradingDay"
        | "tradesAggregate" => return api_payload_0(operation, value),
        "tradesHistorical"
        | "blockTradesHistorical"
        | "tradesRecent"
        | "uiKlines"
        | "referencePrice"
        | "referencePriceCalculation"
        | "openOrdersCancelAll"
        | "orderCancel"
        | "orderPlace"
        | "orderTest"
        | "sessionSubscriptions"
        | "userDataStreamSubscribe"
        | "userDataStreamSubscribeSignature"
        | "userDataStreamUnsubscribe"
        | "allOrderLists"
        | "myAllocations"
        | "myFilters"
        | "myPreventedMatches"
        | "openOrderListsStatus"
        | "orderAmendments" => return api_payload_1(operation, value),
        "orderListStatus"
        | "orderAmendKeepPriority"
        | "orderCancelReplace"
        | "orderListCancel"
        | "orderListPlace"
        | "orderListPlaceOco"
        | "orderListPlaceOpo"
        | "orderListPlaceOpoco"
        | "orderListPlaceOto"
        | "orderListPlaceOtoco"
        | "sorOrderPlace"
        | "sorOrderTest" => return api_payload_2(operation, value),
        "sessionLogon" | "sessionStatus" | "sessionLogout" => {
            serde_json::from_value(value).map(ApiPayload::Session)
        }
        _ => return Err(Error::Gap("unrecognized correlated API operation")),
    }
    .map_err(|_| Error::Gap("malformed correlated API response"))
}

fn api_payload_0(operation: &str, value: Value) -> Result<ApiPayload, Error> {
    match operation {
        "accountCommission" => {
            serde_json::from_value(value).map(|v| ApiPayload::AccountCommission(Box::new(v)))
        }
        "accountRateLimitsOrders" => {
            serde_json::from_value(value).map(|v| ApiPayload::AccountRateLimitsOrders(Box::new(v)))
        }
        "accountStatus" => {
            serde_json::from_value(value).map(|v| ApiPayload::AccountStatus(Box::new(v)))
        }
        "allOrders" => serde_json::from_value(value).map(|v| ApiPayload::AllOrders(Box::new(v))),
        "myTrades" => serde_json::from_value(value).map(|v| ApiPayload::MyTrades(Box::new(v))),
        "openOrdersStatus" => {
            serde_json::from_value(value).map(|v| ApiPayload::OpenOrdersStatus(Box::new(v)))
        }
        "orderStatus" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderStatus(Box::new(v)))
        }
        "exchangeInfo" => {
            serde_json::from_value(value).map(|v| ApiPayload::ExchangeInfo(Box::new(v)))
        }
        "executionRules" => {
            serde_json::from_value(value).map(|v| ApiPayload::ExecutionRules(Box::new(v)))
        }
        "ping" => serde_json::from_value(value).map(|v| ApiPayload::Ping(Box::new(v))),
        "time" => serde_json::from_value(value).map(|v| ApiPayload::Time(Box::new(v))),
        "avgPrice" => serde_json::from_value(value).map(|v| ApiPayload::AvgPrice(Box::new(v))),
        "depth" => serde_json::from_value(value).map(|v| ApiPayload::Depth(Box::new(v))),
        "klines" => serde_json::from_value(value).map(|v| ApiPayload::Klines(Box::new(v))),
        "ticker" => serde_json::from_value(value).map(|v| ApiPayload::Ticker(Box::new(v))),
        "ticker24hr" => serde_json::from_value(value).map(|v| ApiPayload::Ticker24hr(Box::new(v))),
        "tickerBook" => serde_json::from_value(value).map(|v| ApiPayload::TickerBook(Box::new(v))),
        "tickerPrice" => {
            serde_json::from_value(value).map(|v| ApiPayload::TickerPrice(Box::new(v)))
        }
        "tickerTradingDay" => {
            serde_json::from_value(value).map(|v| ApiPayload::TickerTradingDay(Box::new(v)))
        }
        "tradesAggregate" => {
            serde_json::from_value(value).map(|v| ApiPayload::TradesAggregate(Box::new(v)))
        }
        _ => return Err(Error::Gap("unrecognized correlated API operation")),
    }
    .map_err(|_| Error::Gap("malformed correlated API response"))
}
fn api_payload_1(operation: &str, value: Value) -> Result<ApiPayload, Error> {
    match operation {
        "tradesHistorical" => {
            serde_json::from_value(value).map(|v| ApiPayload::TradesHistorical(Box::new(v)))
        }
        "blockTradesHistorical" => {
            serde_json::from_value(value).map(|v| ApiPayload::BlockTradesHistorical(Box::new(v)))
        }
        "tradesRecent" => {
            serde_json::from_value(value).map(|v| ApiPayload::TradesRecent(Box::new(v)))
        }
        "uiKlines" => serde_json::from_value(value).map(|v| ApiPayload::UiKlines(Box::new(v))),
        "referencePrice" => {
            serde_json::from_value(value).map(|v| ApiPayload::ReferencePrice(Box::new(v)))
        }
        "referencePriceCalculation" => serde_json::from_value(value)
            .map(|v| ApiPayload::ReferencePriceCalculation(Box::new(v))),
        "openOrdersCancelAll" => {
            serde_json::from_value(value).map(|v| ApiPayload::OpenOrdersCancelAll(Box::new(v)))
        }
        "orderCancel" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderCancel(Box::new(v)))
        }
        "orderPlace" => serde_json::from_value(value).map(|v| ApiPayload::OrderPlace(Box::new(v))),
        "orderTest" => serde_json::from_value(value).map(|v| ApiPayload::OrderTest(Box::new(v))),
        "sessionSubscriptions" => {
            serde_json::from_value(value).map(|v| ApiPayload::SessionSubscriptions(Box::new(v)))
        }
        "userDataStreamSubscribe" => {
            serde_json::from_value(value).map(|v| ApiPayload::UserDataStreamSubscribe(Box::new(v)))
        }
        "userDataStreamSubscribeSignature" => serde_json::from_value(value)
            .map(|v| ApiPayload::UserDataStreamSubscribeSignature(Box::new(v))),
        "userDataStreamUnsubscribe" => serde_json::from_value(value)
            .map(|v| ApiPayload::UserDataStreamUnsubscribe(Box::new(v))),
        "allOrderLists" => {
            serde_json::from_value(value).map(|v| ApiPayload::AllOrderLists(Box::new(v)))
        }
        "myAllocations" => {
            serde_json::from_value(value).map(|v| ApiPayload::MyAllocations(Box::new(v)))
        }
        "myFilters" => serde_json::from_value(value).map(|v| ApiPayload::MyFilters(Box::new(v))),
        "myPreventedMatches" => {
            serde_json::from_value(value).map(|v| ApiPayload::MyPreventedMatches(Box::new(v)))
        }
        "openOrderListsStatus" => {
            serde_json::from_value(value).map(|v| ApiPayload::OpenOrderListsStatus(Box::new(v)))
        }
        "orderAmendments" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderAmendments(Box::new(v)))
        }
        _ => return Err(Error::Gap("unrecognized correlated API operation")),
    }
    .map_err(|_| Error::Gap("malformed correlated API response"))
}
fn api_payload_2(operation: &str, value: Value) -> Result<ApiPayload, Error> {
    match operation {
        "orderListStatus" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListStatus(Box::new(v)))
        }
        "orderAmendKeepPriority" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderAmendKeepPriority(Box::new(v)))
        }
        "orderCancelReplace" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderCancelReplace(Box::new(v)))
        }
        "orderListCancel" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListCancel(Box::new(v)))
        }
        "orderListPlace" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListPlace(Box::new(v)))
        }
        "orderListPlaceOco" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListPlaceOco(Box::new(v)))
        }
        "orderListPlaceOpo" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListPlaceOpo(Box::new(v)))
        }
        "orderListPlaceOpoco" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListPlaceOpoco(Box::new(v)))
        }
        "orderListPlaceOto" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListPlaceOto(Box::new(v)))
        }
        "orderListPlaceOtoco" => {
            serde_json::from_value(value).map(|v| ApiPayload::OrderListPlaceOtoco(Box::new(v)))
        }
        "sorOrderPlace" => {
            serde_json::from_value(value).map(|v| ApiPayload::SorOrderPlace(Box::new(v)))
        }
        "sorOrderTest" => {
            serde_json::from_value(value).map(|v| ApiPayload::SorOrderTest(Box::new(v)))
        }
        _ => return Err(Error::Gap("unrecognized correlated API operation")),
    }
    .map_err(|_| Error::Gap("malformed correlated API response"))
}
/// Every documented market-stream payload, with provider distinctions intact.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum MarketPayload {
    /// `AggTrade` events.
    AggTrade(Box<super::stream_models::AggTradeEvent>),
    /// `AllMarketRollingWindowTicker` events.
    AllMarketRollingWindowTicker(
        Box<Vec<super::stream_models::AllMarketRollingWindowTickerEventItem>>,
    ),
    /// `AllMiniTicker` events.
    AllMiniTicker(Box<Vec<super::stream_models::AllMiniTickerEventItem>>),
    /// `AvgPrice` events.
    AvgPrice(Box<super::stream_models::AvgPriceEvent>),
    /// `BookTicker` events.
    BookTicker(Box<super::stream_models::BookTickerEvent>),
    /// `DiffBookDepth` events.
    DiffBookDepth(Box<super::stream_models::DiffBookDepthEvent>),
    /// `Kline` events.
    Kline(Box<super::stream_models::KlineEvent>),
    /// `KlineOffset` events.
    KlineOffset(Box<super::stream_models::KlineOffsetEvent>),
    /// `MiniTicker` events.
    MiniTicker(Box<super::stream_models::MiniTickerEvent>),
    /// `PartialBookDepth` events.
    PartialBookDepth(Box<super::stream_models::PartialBookDepthEvent>),
    /// `ReferencePrice` events.
    ReferencePrice(Box<super::stream_models::ReferencePriceEvent>),
    /// `RollingWindowTicker` events.
    RollingWindowTicker(Box<super::stream_models::RollingWindowTickerEvent>),
    /// `Ticker` events.
    Ticker(Box<super::stream_models::TickerEvent>),
    /// `Trade` events.
    Trade(Box<super::stream_models::TradeEvent>),
    /// `BlockTrade` events.
    BlockTrade(Box<super::stream_models::BlockTradeEvent>),
    /// A future stream name or kind this build does not model, preserved
    /// with redacted Debug for explicit consumer handling.
    Unknown(UnknownMessage),
}

pub(crate) fn market_payload(kind: &str, value: Value) -> Result<MarketPayload, Error> {
    match kind {
        "aggTrade" => serde_json::from_value(value).map(|v| MarketPayload::AggTrade(Box::new(v))),
        "allMarketRollingWindowTicker" => serde_json::from_value(value)
            .map(|v| MarketPayload::AllMarketRollingWindowTicker(Box::new(v))),
        "allMiniTicker" => {
            serde_json::from_value(value).map(|v| MarketPayload::AllMiniTicker(Box::new(v)))
        }
        "avgPrice" => serde_json::from_value(value).map(|v| MarketPayload::AvgPrice(Box::new(v))),
        "bookTicker" => {
            serde_json::from_value(value).map(|v| MarketPayload::BookTicker(Box::new(v)))
        }
        "diffBookDepth" => {
            serde_json::from_value(value).map(|v| MarketPayload::DiffBookDepth(Box::new(v)))
        }
        "kline" => serde_json::from_value(value).map(|v| MarketPayload::Kline(Box::new(v))),
        "klineOffset" => {
            serde_json::from_value(value).map(|v| MarketPayload::KlineOffset(Box::new(v)))
        }
        "miniTicker" => {
            serde_json::from_value(value).map(|v| MarketPayload::MiniTicker(Box::new(v)))
        }
        "partialBookDepth" => {
            serde_json::from_value(value).map(|v| MarketPayload::PartialBookDepth(Box::new(v)))
        }
        "referencePrice" => {
            serde_json::from_value(value).map(|v| MarketPayload::ReferencePrice(Box::new(v)))
        }
        "rollingWindowTicker" => {
            serde_json::from_value(value).map(|v| MarketPayload::RollingWindowTicker(Box::new(v)))
        }
        "ticker" => serde_json::from_value(value).map(|v| MarketPayload::Ticker(Box::new(v))),
        "trade" => serde_json::from_value(value).map(|v| MarketPayload::Trade(Box::new(v))),
        "blockTrade" => {
            serde_json::from_value(value).map(|v| MarketPayload::BlockTrade(Box::new(v)))
        }
        // An unrecognized stream kind is retained evidence, not a continuity
        // break; `Error::Gap` is reserved for malformed payloads of known kinds.
        _ => return Ok(MarketPayload::Unknown(value.into())),
    }
    .map_err(|_| Error::Gap("malformed market payload"))
}

/// Every documented user-data event, plus an explicit unknown-future alternative.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum UserPayload {
    /// Provider `balanceUpdate` event.
    BalanceUpdate(Box<super::stream_models::BalanceUpdateEvent>),
    /// Provider `eventStreamTerminated` event.
    EventStreamTerminated(Box<super::stream_models::EventStreamTerminatedEvent>),
    /// Provider `executionReport` event.
    ExecutionReport(Box<super::stream_models::ExecutionReportEvent>),
    /// Provider `externalLockUpdate` event.
    ExternalLockUpdate(Box<super::stream_models::ExternalLockUpdateEvent>),
    /// Provider `listStatus` event.
    ListStatus(Box<super::stream_models::ListStatusEvent>),
    /// Provider `outboundAccountPosition` event.
    OutboundAccountPosition(Box<super::stream_models::OutboundAccountPositionEvent>),
    /// A future event type, preserved for explicit consumer handling.
    Unknown(UnknownMessage),
}

pub(crate) fn user_payload(value: Value) -> Result<UserPayload, Error> {
    let event = value
        .get("e")
        .and_then(Value::as_str)
        .ok_or(Error::Gap("user event type"))?;
    match event {
        "balanceUpdate" => {
            serde_json::from_value(value).map(|v| UserPayload::BalanceUpdate(Box::new(v)))
        }
        "eventStreamTerminated" => {
            serde_json::from_value(value).map(|v| UserPayload::EventStreamTerminated(Box::new(v)))
        }
        "executionReport" => {
            serde_json::from_value(value).map(|v| UserPayload::ExecutionReport(Box::new(v)))
        }
        "externalLockUpdate" => {
            serde_json::from_value(value).map(|v| UserPayload::ExternalLockUpdate(Box::new(v)))
        }
        "listStatus" => serde_json::from_value(value).map(|v| UserPayload::ListStatus(Box::new(v))),
        "outboundAccountPosition" => {
            serde_json::from_value(value).map(|v| UserPayload::OutboundAccountPosition(Box::new(v)))
        }
        _ => return Ok(UserPayload::Unknown(value.into())),
    }
    .map_err(|_| Error::Gap("malformed execution/account event"))
}
