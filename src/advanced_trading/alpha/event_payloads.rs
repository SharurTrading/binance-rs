// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native stream dispatch with retained future-event evidence.
pub use crate::core_trading::wallet::event_payloads::UnknownMessage;
/// A native event, without cross-product account or instrument normalization.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Payload {
    /// Native `aggregateTradeStream` evidence.
    AggregateTradeStream(Box<super::stream_models::AggregateTradeStreamEvent>),
    /// Native `allBookTickerStream` evidence.
    AllBookTickerStream(Box<super::stream_models::AllBookTickerStreamEvent>),
    /// Native `allMiniTickerStream` evidence.
    AllMiniTickerStream(Box<super::stream_models::AllMiniTickerStreamEvent>),
    /// Native `allTickerStream` evidence.
    AllTickerStream(Box<super::stream_models::AllTickerStreamEvent>),
    /// Native `allTokens24hTickerStream` evidence.
    AllTokens24hTickerStream(Box<super::stream_models::AllTokens24hTickerStreamEvent>),
    /// Native `bookTickerStream` evidence.
    BookTickerStream(Box<super::stream_models::BookTickerStreamEvent>),
    /// Native `partialDepthStream` evidence.
    PartialDepthStream(Box<super::stream_models::PartialDepthStreamEvent>),
    /// Native `fullDepthStream` evidence.
    FullDepthStream(Box<super::stream_models::FullDepthStreamEvent>),
    /// Native `klineStream` evidence.
    KlineStream(Box<super::stream_models::KlineStreamEvent>),
    /// Native `contractKlineStream` evidence.
    ContractKlineStream(Box<super::stream_models::ContractKlineStreamEvent>),
    /// Native `miniTickerStream` evidence.
    MiniTickerStream(Box<super::stream_models::MiniTickerStreamEvent>),
    /// Native `tickerStream` evidence.
    TickerStream(Box<super::stream_models::TickerStreamEvent>),
    /// Native `tradeStream` evidence.
    TradeStream(Box<super::stream_models::TradeStreamEvent>),
    /// Future native event or stream shape, retained with redacted diagnostics.
    Unknown(UnknownMessage),
}
/// Decode one named market stream, or a user event when `kind` is absent.
///
/// # Errors
/// Returns a gap for malformed known evidence, including financial parse failures.
pub fn decode(kind: Option<&str>, value: serde_json::Value) -> Result<Payload, crate::Error> {
    match kind {
        Some("aggregateTradeStream") => {
            decode_market(value, Some("aggTrade"), Payload::AggregateTradeStream)
        }
        Some("allBookTickerStream") => {
            decode_market(value, Some("bookTicker"), Payload::AllBookTickerStream)
        }
        Some("allMiniTickerStream") => {
            decode_market(value, Some("24hrMiniTicker"), Payload::AllMiniTickerStream)
        }
        Some("allTickerStream") => {
            decode_market(value, Some("24hrTicker"), Payload::AllTickerStream)
        }
        Some("allTokens24hTickerStream") => {
            decode_market(value, Some("tickerList"), Payload::AllTokens24hTickerStream)
        }
        Some("bookTickerStream") => {
            decode_market(value, Some("bookTicker"), Payload::BookTickerStream)
        }
        Some("partialDepthStream") => {
            decode_market(value, Some("depthUpdate"), Payload::PartialDepthStream)
        }
        Some("fullDepthStream") => {
            decode_market(value, Some("depthUpdate"), Payload::FullDepthStream)
        }
        Some("klineStream") => decode_market(value, Some("kline"), Payload::KlineStream),
        Some("contractKlineStream") => {
            decode_market(value, Some("kline"), Payload::ContractKlineStream)
        }
        Some("miniTickerStream") => {
            decode_market(value, Some("24hrMiniTicker"), Payload::MiniTickerStream)
        }
        Some("tickerStream") => decode_market(value, Some("24hrTicker"), Payload::TickerStream),
        Some("tradeStream") => decode_market(value, Some("trade"), Payload::TradeStream),
        _ => Ok(Payload::Unknown(value.into())),
    }
}

fn decode_value<T: serde::de::DeserializeOwned>(
    value: serde_json::Value,
    wrap: fn(Box<T>) -> Payload,
) -> Result<Payload, crate::Error> {
    serde_json::from_value(value)
        .map(|value| wrap(Box::new(value)))
        .map_err(|_| crate::Error::Gap("malformed native stream event"))
}
fn decode_market<T: serde::de::DeserializeOwned>(
    value: serde_json::Value,
    event: Option<&str>,
    wrap: fn(Box<T>) -> Payload,
) -> Result<Payload, crate::Error> {
    if event.is_some_and(|expected| {
        value
            .get("e")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|actual| actual != expected)
    }) {
        return Ok(Payload::Unknown(value.into()));
    }
    decode_value(value, wrap)
}
