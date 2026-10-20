// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Portfolio Margin bindings. Mutations are sent once; resolve uncertainty with venue reads.
//! The caller owns credentials, runtime, deadlines and recovery.

pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub(super) mod validation;
mod wire {
    pub(crate) use crate::advanced_trading::wire::*;
}

/// Fixed Portfolio Margin protocol marker; not caller-implementable.
#[derive(Clone)]
pub enum Protocol {}
impl super::config::sealed::Sealed for Protocol {}
impl super::config::Product for Protocol {
    const REST: &'static str = "https://papi.binance.com";
    const STREAMS: Option<&'static str> = Some("wss://fstream.binance.com/pm");
    const POOL: crate::core::VenuePool = crate::core::VenuePool::PortfolioMargin;
}
/// Explicit production endpoints and shared services for Portfolio Margin.
pub type Config = super::config::Config<Protocol>;

/// REST transport whose clones share native configuration and quota ownership.
pub type RestClient = super::config::RestClient<Protocol>;

/// Caller-provided current exchange symbol-count authority for all-symbol reads.
pub use crate::core_trading::margin::TradingSymbolCount;

pub mod stream_models;
/// Native stream-name constructors for this product.
pub type Stream = super::streams::Stream<Protocol>;
/// Caller-driven native connections with typed source-ordered events.
pub type Streams = super::streams::Streams<Protocol>;
impl super::streams::StreamingProduct for Protocol {
    const MAX_GENERATION_AGE: Option<std::time::Duration> =
        Some(std::time::Duration::from_hours(24));
    type Payload = event_payloads::Payload;
    const MARKET: bool = false;
    const MARKET_WS_SUFFIX: bool = false;
    const LIST_CONTROL: crate::StreamControl = crate::StreamControl::ListSubscriptions;
    const PRIVATE_KIND: Option<&'static str> = None;
    const PRIVATE_SUFFIX: Option<&'static str> = Some("");
    fn decode(kind: Option<&str>, value: serde_json::Value) -> Result<Self::Payload, crate::Error> {
        event_payloads::decode(kind, value)
    }
}
