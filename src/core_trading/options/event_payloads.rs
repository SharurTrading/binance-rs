// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Options stream dispatch and retained future fields.
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

use crate::Error;

/// Provider-native market evidence, without consumer policy.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum MarketPayload {
    /// Documented `IndexPriceStreams` event.
    IndexPriceStreams(Box<Vec<super::stream_models::IndexPriceStreamsEventItem>>),
    /// Documented `KlineCandlestickStreams` event.
    KlineCandlestickStreams(Box<super::stream_models::KlineCandlestickStreamsEvent>),
    /// Documented `OptionMarkPrice` event.
    OptionMarkPrice(Box<Vec<super::stream_models::OptionMarkPriceEventItem>>),
    /// Documented `NewSymbolInfo` event.
    NewSymbolInfo(Box<super::stream_models::NewSymbolInfoEvent>),
    /// Documented `OpenInterest` event.
    OpenInterest(Box<Vec<super::stream_models::OpenInterestEventItem>>),
    /// Documented `DiffBookDepthStreams` event.
    DiffBookDepthStreams(Box<super::stream_models::DiffBookDepthStreamsEvent>),
    /// Documented `IndividualSymbolBookTickerStreams` event.
    IndividualSymbolBookTickerStreams(
        Box<super::stream_models::IndividualSymbolBookTickerStreamsEvent>,
    ),
    /// Documented `PartialBookDepthStreams` event.
    PartialBookDepthStreams(Box<super::stream_models::PartialBookDepthStreamsEvent>),
    /// Documented `Hour24Ticker` event.
    Hour24Ticker(Box<super::stream_models::Hour24TickerEvent>),
    /// Documented `TradeStreams` event.
    TradeStreams(Box<super::stream_models::TradeStreamsEvent>),
    /// Future event evidence, retained with redacted diagnostics.
    Unknown(UnknownMessage),
}

/// Provider-native user evidence, without consumer policy.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum UserPayload {
    /// Documented `AccountUpdate` event.
    AccountUpdate(Box<super::stream_models::AccountUpdateEvent>),
    /// Documented `BalancePositionUpdate` event.
    BalancePositionUpdate(Box<super::stream_models::BalancePositionUpdateEvent>),
    /// Documented `GreekUpdate` event.
    GreekUpdate(Box<super::stream_models::GreekUpdateEvent>),
    /// Documented `ListenKeyExpired` event.
    ListenKeyExpired(Box<super::stream_models::ListenKeyExpiredEvent>),
    /// Documented `OrderTradeUpdate` event.
    OrderTradeUpdate(Box<super::stream_models::OrderTradeUpdateEvent>),
    /// Documented `RiskLevelChange` event.
    RiskLevelChange(Box<super::stream_models::RiskLevelChangeEvent>),
    /// Future event evidence, retained with redacted diagnostics.
    Unknown(UnknownMessage),
}

/// Decode a documented user-data payload or its stream/data envelope.
/// Partial balance and position updates contain only changed entries.
///
/// # Errors
/// Returns a continuity gap for malformed known records or missing event kind.
pub fn user_payload(value: Value) -> Result<UserPayload, Error> {
    let value = if value.get("stream").is_some() {
        value
            .get("data")
            .cloned()
            .ok_or(Error::Gap("Options private stream envelope"))?
    } else {
        value
    };
    let event = value
        .get("e")
        .and_then(Value::as_str)
        .ok_or(Error::Gap("Options user event kind"))?;
    match event {
        "ACCOUNT_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::AccountUpdate(Box::new(v)))
        }
        "BALANCE_POSITION_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::BalancePositionUpdate(Box::new(v)))
        }
        "GREEK_UPDATE" => {
            serde_json::from_value(value).map(|v| UserPayload::GreekUpdate(Box::new(v)))
        }
        "listenKeyExpired" => {
            serde_json::from_value(value).map(|v| UserPayload::ListenKeyExpired(Box::new(v)))
        }
        "ORDER_TRADE_UPDATE" => {
            let record: super::stream_models::OrderTradeUpdateEvent = serde_json::from_value(value)
                .map_err(|_| Error::Gap("malformed Options order event"))?;
            if record.o.n != crate::Decimal::ZERO && record.o.upper_n.is_none() {
                return Err(Error::Gap("Options commission asset required"));
            }
            Ok(UserPayload::OrderTradeUpdate(Box::new(record)))
        }
        "RISK_LEVEL_CHANGE" => {
            serde_json::from_value(value).map(|v| UserPayload::RiskLevelChange(Box::new(v)))
        }
        _ => return Ok(UserPayload::Unknown(value.into())),
    }
    .map_err(|_| Error::Gap("malformed Options user event"))
}

pub(crate) fn market_payload(kind: &str, value: Value) -> Result<MarketPayload, Error> {
    match kind {
        "indexPriceStreams" => {
            serde_json::from_value(value).map(|v| MarketPayload::IndexPriceStreams(Box::new(v)))
        }
        "klineCandlestickStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::KlineCandlestickStreams(Box::new(v))),
        "optionMarkPrice" => {
            serde_json::from_value(value).map(|v| MarketPayload::OptionMarkPrice(Box::new(v)))
        }
        "newSymbolInfo" => {
            serde_json::from_value(value).map(|v| MarketPayload::NewSymbolInfo(Box::new(v)))
        }
        "openInterest" => {
            serde_json::from_value(value).map(|v| MarketPayload::OpenInterest(Box::new(v)))
        }
        "diffBookDepthStreams" => {
            serde_json::from_value(value).map(|v| MarketPayload::DiffBookDepthStreams(Box::new(v)))
        }
        "individualSymbolBookTickerStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::IndividualSymbolBookTickerStreams(Box::new(v))),
        "partialBookDepthStreams" => serde_json::from_value(value)
            .map(|v| MarketPayload::PartialBookDepthStreams(Box::new(v))),
        "hour24Ticker" => {
            serde_json::from_value(value).map(|v| MarketPayload::Hour24Ticker(Box::new(v)))
        }
        "tradeStreams" => {
            serde_json::from_value(value).map(|v| MarketPayload::TradeStreams(Box::new(v)))
        }
        _ => return Ok(MarketPayload::Unknown(value.into())),
    }
    .map_err(|_| Error::Gap("malformed Options market event"))
}

/// Asset for monetary valuations explicitly denominated by Options documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValuationAsset {
    /// USDT, as specified for `ACCOUNT_UPDATE` valuations.
    Usdt,
}
impl super::stream_models::AccountUpdateEvent {
    /// Documented valuation asset; no currency is selected or converted.
    #[must_use]
    pub fn valuation_asset(&self) -> ValuationAsset {
        ValuationAsset::Usdt
    }
}
