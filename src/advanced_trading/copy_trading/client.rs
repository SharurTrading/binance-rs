// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Copy Trading protocol marker, endpoints and REST transport.

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
