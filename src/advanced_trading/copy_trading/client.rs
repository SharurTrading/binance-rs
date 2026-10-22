// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native product protocol, configuration and shared transport aliases.

use crate::advanced_trading::config;

/// Fixed Copy Trading protocol marker; not caller-implementable.
#[derive(Clone)]
pub enum Protocol {}

impl config::sealed::Sealed for Protocol {}

impl config::Product for Protocol {
    const REST: &'static str = "https://api.binance.com";
    const STREAMS: Option<&'static str> = None;
    const POOL: crate::core::VenuePool = crate::core::VenuePool::Sapi;
}

/// Explicit production endpoints and shared services for Copy Trading.
pub type Config = config::Config<Protocol>;

/// REST transport whose clones share native configuration and quota ownership.
pub type RestClient = config::RestClient<Protocol>;
