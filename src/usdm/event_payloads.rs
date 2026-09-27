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
    /// `accountInformation` response.
    AccountInformation(Box<super::ws_models::AccountInformationResponse>),
    /// `accountInformationV2` response.
    AccountInformationV2(Box<super::ws_models::AccountInformationV2Response>),
    /// `futuresAccountBalance` response.
    FuturesAccountBalance(Box<super::ws_models::FuturesAccountBalanceResponse>),
    /// `futuresAccountBalanceV2` response.
    FuturesAccountBalanceV2(Box<super::ws_models::FuturesAccountBalanceV2Response>),
    /// `orderBook` response.
    OrderBook(Box<super::ws_models::OrderBookResponse>),
    /// `symbolOrderBookTicker` response.
    SymbolOrderBookTicker(Box<super::ws_models::SymbolOrderBookTickerResponse>),
    /// `symbolPriceTicker` response.
    SymbolPriceTicker(Box<super::ws_models::SymbolPriceTickerResponse>),
    /// `cancelAlgoOrder` response.
    CancelAlgoOrder(Box<super::ws_models::CancelAlgoOrderResponse>),
    /// `cancelOrder` response.
    CancelOrder(Box<super::ws_models::CancelOrderResponse>),
    /// `modifyOrder` response.
    ModifyOrder(Box<super::ws_models::ModifyOrderResponse>),
    /// `newAlgoOrder` response.
    NewAlgoOrder(Box<super::ws_models::NewAlgoOrderResponse>),
    /// `newOrder` response.
    NewOrder(Box<super::ws_models::NewOrderResponse>),
    /// `positionInformation` response.
    PositionInformation(Box<super::ws_models::PositionInformationResponse>),
    /// `positionInformationV2` response.
    PositionInformationV2(Box<super::ws_models::PositionInformationV2Response>),
    /// `queryOrder` response.
    QueryOrder(Box<super::ws_models::QueryOrderResponse>),
    /// `closeUserDataStream` response.
    CloseUserDataStream(Box<super::ws_models::CloseUserDataStreamResponse>),
    /// `keepaliveUserDataStream` response.
    KeepaliveUserDataStream(Box<super::ws_models::KeepaliveUserDataStreamResponse>),
    /// `startUserDataStream` response.
    StartUserDataStream(Box<super::ws_models::StartUserDataStreamResponse>),
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
        "accountInformation"
        | "accountInformationV2"
        | "futuresAccountBalance"
        | "futuresAccountBalanceV2"
        | "orderBook"
        | "symbolOrderBookTicker"
        | "symbolPriceTicker"
        | "cancelAlgoOrder"
        | "cancelOrder"
        | "modifyOrder"
        | "newAlgoOrder"
        | "newOrder"
        | "positionInformation"
        | "positionInformationV2"
        | "queryOrder"
        | "closeUserDataStream"
        | "keepaliveUserDataStream"
        | "startUserDataStream" => return api_payload_0(operation, value),
        "sessionLogon" | "sessionStatus" | "sessionLogout" => {
            serde_json::from_value(value).map(ApiPayload::Session)
        }
        _ => return Err(Error::Gap("unrecognized correlated API operation")),
    }
    .map_err(|_| Error::Gap("malformed correlated API response"))
}

fn api_payload_0(operation: &str, value: Value) -> Result<ApiPayload, Error> {
    match operation {
        "accountInformation" => {
            serde_json::from_value(value).map(|v| ApiPayload::AccountInformation(Box::new(v)))
        }
        "accountInformationV2" => {
            serde_json::from_value(value).map(|v| ApiPayload::AccountInformationV2(Box::new(v)))
        }
        "futuresAccountBalance" => {
            serde_json::from_value(value).map(|v| ApiPayload::FuturesAccountBalance(Box::new(v)))
        }
        "futuresAccountBalanceV2" => {
            serde_json::from_value(value).map(|v| ApiPayload::FuturesAccountBalanceV2(Box::new(v)))
        }
        "orderBook" => serde_json::from_value(value).map(|v| ApiPayload::OrderBook(Box::new(v))),
        "symbolOrderBookTicker" => {
            serde_json::from_value(value).map(|v| ApiPayload::SymbolOrderBookTicker(Box::new(v)))
        }
        "symbolPriceTicker" => {
            serde_json::from_value(value).map(|v| ApiPayload::SymbolPriceTicker(Box::new(v)))
        }
        "cancelAlgoOrder" => {
            serde_json::from_value(value).map(|v| ApiPayload::CancelAlgoOrder(Box::new(v)))
        }
        "cancelOrder" => {
            serde_json::from_value(value).map(|v| ApiPayload::CancelOrder(Box::new(v)))
        }
        "modifyOrder" => {
            serde_json::from_value(value).map(|v| ApiPayload::ModifyOrder(Box::new(v)))
        }
        "newAlgoOrder" => {
            serde_json::from_value(value).map(|v| ApiPayload::NewAlgoOrder(Box::new(v)))
        }
        "newOrder" => serde_json::from_value(value).map(|v| ApiPayload::NewOrder(Box::new(v))),
        "positionInformation" => {
            serde_json::from_value(value).map(|v| ApiPayload::PositionInformation(Box::new(v)))
        }
        "positionInformationV2" => {
            serde_json::from_value(value).map(|v| ApiPayload::PositionInformationV2(Box::new(v)))
        }
        "queryOrder" => serde_json::from_value(value).map(|v| ApiPayload::QueryOrder(Box::new(v))),
        "closeUserDataStream" => {
            serde_json::from_value(value).map(|v| ApiPayload::CloseUserDataStream(Box::new(v)))
        }
        "keepaliveUserDataStream" => {
            serde_json::from_value(value).map(|v| ApiPayload::KeepaliveUserDataStream(Box::new(v)))
        }
        "startUserDataStream" => {
            serde_json::from_value(value).map(|v| ApiPayload::StartUserDataStream(Box::new(v)))
        }
        _ => return Err(Error::Gap("unrecognized correlated API operation")),
    }
    .map_err(|_| Error::Gap("malformed correlated API response"))
}
/// Every documented market-stream payload, with provider distinctions intact.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum MarketPayload {
    /// `AggregateTradeStreams` events.
    AggregateTradeStreams(Box<super::stream_models::AggregateTradeStreamsEvent>),
    /// `AllMarketLiquidationOrderStreams` events.
    AllMarketLiquidationOrderStreams(
        Box<super::stream_models::AllMarketLiquidationOrderStreamsEvent>,
    ),
    /// `AllMarketMiniTickersStream` events.
    AllMarketMiniTickersStream(Box<Vec<super::stream_models::AllMarketMiniTickersStreamEventItem>>),
    /// `AllMarketTickersStreams` events.
    AllMarketTickersStreams(Box<Vec<super::stream_models::AllMarketTickersStreamsEventItem>>),
    /// `CompositeIndexSymbolInformationStreams` events.
    CompositeIndexSymbolInformationStreams(
        Box<super::stream_models::CompositeIndexSymbolInformationStreamsEvent>,
    ),
    /// `ContinuousContractKlineCandlestickStreams` events.
    ContinuousContractKlineCandlestickStreams(
        Box<super::stream_models::ContinuousContractKlineCandlestickStreamsEvent>,
    ),
    /// `ContractInfoStream` events.
    ContractInfoStream(Box<super::stream_models::ContractInfoStreamEvent>),
    /// `IndividualSymbolMiniTickerStream` events.
    IndividualSymbolMiniTickerStream(
        Box<super::stream_models::IndividualSymbolMiniTickerStreamEvent>,
    ),
    /// `IndividualSymbolTickerStreams` events.
    IndividualSymbolTickerStreams(Box<super::stream_models::IndividualSymbolTickerStreamsEvent>),
    /// `KlineCandlestickStreams` events.
    KlineCandlestickStreams(Box<super::stream_models::KlineCandlestickStreamsEvent>),
    /// `LiquidationOrderStreams` events.
    LiquidationOrderStreams(Box<super::stream_models::LiquidationOrderStreamsEvent>),
    /// `MarkPriceStream` events.
    MarkPriceStream(Box<super::stream_models::MarkPriceStreamEvent>),
    /// `MarkPriceStreamForAllMarket` events.
    MarkPriceStreamForAllMarket(
        Box<Vec<super::stream_models::MarkPriceStreamForAllMarketEventItem>>,
    ),
    /// `AssetIndex` events.
    AssetIndex(Box<Vec<super::stream_models::AssetIndexEventItem>>),
    /// `TradingSessionStream` events.
    TradingSessionStream(Box<super::stream_models::TradingSessionStreamEvent>),
    /// `AllBookTickersStream` events.
    AllBookTickersStream(Box<super::stream_models::AllBookTickersStreamEvent>),
    /// `DiffBookDepthStreams` events.
    DiffBookDepthStreams(Box<super::stream_models::DiffBookDepthStreamsEvent>),
    /// `IndividualSymbolBookTickerStreams` events.
    IndividualSymbolBookTickerStreams(
        Box<super::stream_models::IndividualSymbolBookTickerStreamsEvent>,
    ),
    /// `PartialBookDepthStreams` events.
    PartialBookDepthStreams(Box<super::stream_models::PartialBookDepthStreamsEvent>),
    /// `RpiDiffBookDepthStreams` events.
    RpiDiffBookDepthStreams(Box<super::stream_models::RpiDiffBookDepthStreamsEvent>),
}

pub(crate) fn market_payload(kind: &str, value: Value) -> Result<MarketPayload, Error> {
    match kind {
        "aggregateTradeStreams" => {
            serde_json::from_value(value).map(|v| MarketPayload::AggregateTradeStreams(Box::new(v)))
        }
        "allMarketLiquidationOrderStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::AllMarketLiquidationOrderStreams(Box::new(v))),
        "allMarketMiniTickersStream" => serde_json::from_value(value)
            .map(|v| MarketPayload::AllMarketMiniTickersStream(Box::new(v))),
        "allMarketTickersStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::AllMarketTickersStreams(Box::new(v))),
        "compositeIndexSymbolInformationStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::CompositeIndexSymbolInformationStreams(Box::new(v))),
        "continuousContractKlineCandlestickStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::ContinuousContractKlineCandlestickStreams(Box::new(v))),
        "contractInfoStream" => {
            serde_json::from_value(value).map(|v| MarketPayload::ContractInfoStream(Box::new(v)))
        }
        "individualSymbolMiniTickerStream" => serde_json::from_value(value)
            .map(|v| MarketPayload::IndividualSymbolMiniTickerStream(Box::new(v))),
        "individualSymbolTickerStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::IndividualSymbolTickerStreams(Box::new(v))),
        "klineCandlestickStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::KlineCandlestickStreams(Box::new(v))),
        "liquidationOrderStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::LiquidationOrderStreams(Box::new(v))),
        "markPriceStream" => {
            serde_json::from_value(value).map(|v| MarketPayload::MarkPriceStream(Box::new(v)))
        }
        "markPriceStreamForAllMarket" => serde_json::from_value(value)
            .map(|v| MarketPayload::MarkPriceStreamForAllMarket(Box::new(v))),
        "assetIndex" => {
            serde_json::from_value(value).map(|v| MarketPayload::AssetIndex(Box::new(v)))
        }
        "tradingSessionStream" => {
            serde_json::from_value(value).map(|v| MarketPayload::TradingSessionStream(Box::new(v)))
        }
        "allBookTickersStream" => {
            serde_json::from_value(value).map(|v| MarketPayload::AllBookTickersStream(Box::new(v)))
        }
        "diffBookDepthStreams" => {
            serde_json::from_value(value).map(|v| MarketPayload::DiffBookDepthStreams(Box::new(v)))
        }
        "individualSymbolBookTickerStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::IndividualSymbolBookTickerStreams(Box::new(v))),
        "partialBookDepthStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::PartialBookDepthStreams(Box::new(v))),
        "rpiDiffBookDepthStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::RpiDiffBookDepthStreams(Box::new(v))),
        _ => return Err(Error::Gap("unrecognized subscribed stream")),
    }
    .map_err(|_| Error::Gap("malformed market payload"))
}

/// Every documented user-data event, plus an explicit unknown-future alternative.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum UserPayload {
    /// Provider `accountConfigUpdate` event.
    AccountConfigUpdate(Box<super::stream_models::AccountConfigUpdateEvent>),
    /// Provider `accountUpdate` event.
    AccountUpdate(Box<super::stream_models::AccountUpdateEvent>),
    /// Provider `algoUpdate` event.
    AlgoUpdate(Box<super::stream_models::AlgoUpdateEvent>),
    /// Provider `conditionalOrderTriggerReject` event.
    ConditionalOrderTriggerReject(Box<super::stream_models::ConditionalOrderTriggerRejectEvent>),
    /// Provider `gridUpdate` event.
    GridUpdate(Box<super::stream_models::GridUpdateEvent>),
    /// Provider `listenKeyExpired` event.
    ListenKeyExpired(Box<super::stream_models::ListenKeyExpiredEvent>),
    /// Provider `marginCall` event.
    MarginCall(Box<super::stream_models::MarginCallEvent>),
    /// Provider `orderTradeUpdate` event.
    OrderTradeUpdate(Box<super::stream_models::OrderTradeUpdateEvent>),
    /// Provider `strategyUpdate` event.
    StrategyUpdate(Box<super::stream_models::StrategyUpdateEvent>),
    /// Provider `tradeLite` event.
    TradeLite(Box<super::stream_models::TradeLiteEvent>),
    /// A future event type, preserved for explicit consumer handling.
    Unknown(UnknownMessage),
}

pub(crate) fn user_payload(value: Value) -> Result<UserPayload, Error> {
    let event = value
        .get("e")
        .and_then(Value::as_str)
        .ok_or(Error::Gap("user event type"))?;
    if event == "ACCOUNT_UPDATE"
        && value
            .get("a")
            .is_none_or(|a| a.get("B").is_none() && a.get("P").is_none())
    {
        return Err(Error::Gap("account event has no balance/position evidence"));
    }
    if event == "ACCOUNT_CONFIG_UPDATE" && value.get("ac").is_none() && value.get("ai").is_none() {
        return Err(Error::Gap("account configuration event has no evidence"));
    }
    match event {
        "ACCOUNT_CONFIG_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::AccountConfigUpdate(Box::new(v)))
        }
        "ACCOUNT_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::AccountUpdate(Box::new(v)))
        }
        "ALGO_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::AlgoUpdate(Box::new(v)))
        }
        "CONDITIONAL_ORDER_TRIGGER_REJECT" => serde_json::from_value(value)
            .map(|v| UserPayload::ConditionalOrderTriggerReject(Box::new(v))),
        "GRID_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::GridUpdate(Box::new(v)))
        }
        "listenKeyExpired" => {
            serde_json::from_value(value).map(|v| UserPayload::ListenKeyExpired(Box::new(v)))
        }
        "MARGIN_CALL" => {
            serde_json::from_value(value).map(|v| UserPayload::MarginCall(Box::new(v)))
        }
        "ORDER_TRADE_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::OrderTradeUpdate(Box::new(v)))
        }
        "STRATEGY_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::StrategyUpdate(Box::new(v)))
        }
        "TRADE_LITE" => serde_json::from_value(value).map(|v| UserPayload::TradeLite(Box::new(v))),
        _ => return Ok(UserPayload::Unknown(value.into())),
    }
    .map_err(|_| Error::Gap("malformed execution/account event"))
}
