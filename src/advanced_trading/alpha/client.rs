// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Alpha Trading protocol marker, endpoints, REST transport and stream adapters.

use super::event_payloads;
use crate::advanced_trading::{config, streams};

/// Fixed Alpha Trading protocol marker; not caller-implementable.
#[derive(Clone)]
pub enum Protocol {}
impl config::sealed::Sealed for Protocol {}
impl config::Product for Protocol {
    const REST: &'static str = "https://www.binance.com";
    const STREAMS: Option<&'static str> = Some("wss://nbstream.binance.com/w3w/wsa/stream");
    const POOL: crate::core::VenuePool = crate::core::VenuePool::Alpha;
}
/// Explicit production endpoints and shared services for Alpha Trading.
pub type Config = config::Config<Protocol>;

/// REST transport whose clones share configuration and quota ownership.
#[derive(Clone)]
pub struct RestClient {
    pub(super) inner: crate::core::HttpClient,
}
impl RestClient {
    /// Construct without network I/O or a hidden runtime.
    ///
    /// # Errors
    /// Returns invalid HTTP transport configuration.
    pub fn new(config: Config) -> Result<Self, crate::Error> {
        Ok(Self {
            inner: config.http()?,
        })
    }
}

/// Native stream-name constructors for this product.
pub type Stream = streams::Stream<Protocol>;
/// Caller-driven native connections with typed source-ordered events.
pub type Streams = streams::Streams<Protocol>;
impl streams::StreamingProduct for Protocol {
    const MAX_GENERATION_AGE: Option<std::time::Duration> = None;
    type Payload = event_payloads::Payload;
    const MARKET: bool = true;
    const MARKET_WS_SUFFIX: bool = false;
    const LIST_CONTROL: crate::StreamControl = crate::StreamControl::ListSubscription;
    const PRIVATE_KIND: Option<&'static str> = None;
    const PRIVATE_SUFFIX: Option<&'static str> = None;
    fn decode(kind: Option<&str>, value: serde_json::Value) -> Result<Self::Payload, crate::Error> {
        event_payloads::decode(kind, value)
    }
}
