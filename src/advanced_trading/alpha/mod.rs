// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Alpha Trading bindings. Mutations are sent once; resolve uncertainty with venue reads.
//! The caller owns credentials, runtime, deadlines and recovery.

pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub(super) mod validation;
mod wire {
    pub(crate) use crate::advanced_trading::wire::*;
}

/// Fixed Alpha Trading protocol marker; not caller-implementable.
#[derive(Clone)]
pub enum Protocol {}
impl super::config::sealed::Sealed for Protocol {}
impl super::config::Product for Protocol {
    const REST: &'static str = "https://www.binance.com";
    const STREAMS: Option<&'static str> = Some("wss://nbstream.binance.com/w3w/wsa/stream");
    const POOL: crate::core::VenuePool = crate::core::VenuePool::Alpha;
}
/// Explicit production endpoints and shared services for Alpha Trading.
pub type Config = super::config::Config<Protocol>;

/// REST transport whose clones share native configuration and quota ownership.
pub type RestClient = super::config::RestClient<Protocol>;

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
    const MARKET_WS_SUFFIX: bool = false;
    const LIST_CONTROL: crate::StreamControl = crate::StreamControl::ListSubscription;
    const PRIVATE_KIND: Option<&'static str> = None;
    const PRIVATE_SUFFIX: Option<&'static str> = None;
    fn decode(kind: Option<&str>, value: serde_json::Value) -> Result<Self::Payload, crate::Error> {
        event_payloads::decode(kind, value)
    }
}

/// Exact native depth levels and Alpha's twelve-column klines.
pub use super::wire::{Kline, PriceLevel};
