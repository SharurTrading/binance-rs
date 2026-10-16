// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Stocks Trading bindings. Mutations are sent once; resolve uncertainty with venue reads.
//! The caller owns credentials, runtime, deadlines and recovery.

pub mod enums;
pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
mod validation;
mod wire {
    pub(crate) use crate::advanced_trading::wire::*;
}

/// Fixed Stocks Trading protocol marker; not caller-implementable.
#[derive(Clone)]
pub enum Protocol {}
impl super::config::sealed::Sealed for Protocol {}
impl super::config::Product for Protocol {
    const REST: &'static str = "https://api.binance.com";
    const STREAMS: Option<&'static str> = Some("wss://nbstream.binance.com/equity");
    const POOL: crate::core::VenuePool = crate::core::VenuePool::Sapi;
}
/// Explicit production endpoints and shared services for Stocks Trading.
pub type Config = super::config::Config<Protocol>;

/// REST transport whose clones share configuration and quota ownership.
#[derive(Clone)]
pub struct RestClient {
    inner: crate::core::HttpClient,
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

pub mod stream_models;
mod stream_names;
/// Native stream-name constructors for this product.
pub type Stream = super::streams::Stream<Protocol>;
/// Caller-driven native connections with typed source-ordered events.
pub type Streams = super::streams::Streams<Protocol>;
impl super::streams::StreamingProduct for Protocol {
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
mod identity;
pub use identity::ClientOrderId;
