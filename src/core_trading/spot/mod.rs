// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Spot Trading: asset balances, order amounts, and transport semantics.
//!
//! JSON catalog coverage includes advanced order lists and partial cancel/replace.
//! Native FIX/SBE protocols have separate modules; remaining evidence is tracked in
//! [issue #11](https://github.com/SharurTrading/binance-rs/issues/11).
//!
//! All operations use caller deadlines. The library never retries a mutation.
//! Construct `Config` with an explicit demo/production environment; it draws on
//! the process's IP weight pool for that environment. WebSocket drivers run on
//! caller-owned tasks.

mod config;
pub mod enums;
pub mod event_payloads;
pub mod fix;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub mod sbe;
pub mod stream_models;
mod stream_names;
pub mod streams;
mod validation;
mod websocket;
pub mod wire;
pub mod ws_models;
pub mod ws_requests;

pub use crate::core::socket::QueueStats;
pub use config::{Config, Environment};
mod identity;
pub use identity::ClientOrderId;
pub use streams::{Route, Stream, StreamEvent, Streams};
pub use websocket::{ApiEvent, ApiEvents, ConnectionDriver, WsClient};

/// Cloneable REST client; clones share transport pools and venue budgets.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
}
impl RestClient {
    /// The IP windows of the pool this client draws on, read at its clock; see
    /// [`Config::pool_usage`].
    ///
    /// # Errors
    /// Returns the clock's error, or a configuration error if the pool's lock is
    /// poisoned.
    pub fn pool_usage(&self) -> Result<crate::PoolUsage, crate::Error> {
        self.inner.pool_usage()
    }
    /// Clear an IP ban the venue gave no timing for, on the pool this client draws on;
    /// see [`Config::release_unknown_ban`].
    ///
    /// # Errors
    /// Returns a configuration error if the pool's lock is poisoned.
    pub fn release_unknown_ban(&self) -> Result<(), crate::Error> {
        self.inner.release_unknown_ban()
    }
    /// The venue pool this client draws on; see [`Config::pool_key`].
    #[must_use]
    pub fn pool_key(&self) -> Option<crate::PoolKey> {
        self.inner.pool_key()
    }
    /// Receive production SBE schema 3:4 into the same native response models.
    /// SBE timestamps/signing use microseconds; receive windows remain milliseconds.
    /// JSON negotiation errors retain status/rates; successful JSON fallback is refused.
    ///
    /// # Errors
    /// Returns configuration errors without I/O. No retries or runtime are started.
    pub fn new_sbe(config: Config) -> Result<Self, crate::Error> {
        let mut client = Self::new(config.time_unit(crate::TimeUnit::Microseconds))?;
        client.inner = client
            .inner
            .binary_responses("3:4", sbe::schema::decode_api_value);
        Ok(client)
    }
    /// Create a transport without starting a runtime or performing network I/O.
    ///
    /// # Errors
    /// Returns configuration errors from the HTTP transport.
    pub fn new(config: Config) -> Result<Self, crate::Error> {
        Ok(Self {
            inner: crate::core::HttpClient::new(
                config.rest,
                config.credentials,
                config.clock,
                config.budgets,
                config.timeout,
                config.proxy,
            )?
            .time_unit(config.time_unit),
        })
    }
}
