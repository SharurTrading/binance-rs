// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native product protocol, configuration and shared transport aliases.

use super::event_payloads;
use crate::advanced_trading::{config, streams};

/// Fixed Stocks Trading protocol marker; not caller-implementable.
#[derive(Clone)]
pub enum Protocol {}

impl config::sealed::Sealed for Protocol {}

impl config::Product for Protocol {
    const REST: &'static str = "https://api.binance.com";
    const STREAMS: Option<&'static str> = Some("wss://nbstream.binance.com/equity");
    const POOL: crate::core::VenuePool = crate::core::VenuePool::Sapi;
}

/// Explicit production endpoints and shared services for Stocks Trading.
pub type Config = config::Config<Protocol>;

/// REST transport whose clones share native configuration and quota ownership.
pub type RestClient = config::RestClient<Protocol>;

/// Native stream-name constructors for this product.
pub type Stream = streams::Stream<Protocol>;

/// Caller-driven native connections with typed source-ordered events.
pub type Streams = streams::Streams<Protocol>;

impl streams::StreamingProduct for Protocol {
    const MAX_GENERATION_AGE: Option<std::time::Duration> = None;
    type Payload = event_payloads::Payload;
    const MARKET: bool = true;
    const MARKET_WS_SUFFIX: bool = true;
    const LIST_CONTROL: crate::StreamControl = crate::StreamControl::ListSubscriptions;
    const PRIVATE_KIND: Option<&'static str> = Some("orderReportStream");
    const PRIVATE_SUFFIX: Option<&'static str> = Some("@orderReport");
    fn decode(kind: Option<&str>, value: serde_json::Value) -> Result<Self::Payload, crate::Error> {
        event_payloads::decode(kind, value)
    }
}
