// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Binance Options EAPI and routed market/private streams.
//!
//! Native option contracts, underlying identities, settlement assets and Greeks
//! remain distinct from Futures and Spot models. All timestamps use milliseconds.
//! Each money-moving request is attempted once; uncertain outcomes require venue
//! reads. Streams expose generation boundaries and caller-owned driver teardown.
//! Private balance/position events update only the listed assets and contracts.

mod config;
pub mod event_payloads;
mod identity;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub mod stream_models;
mod stream_names;
pub mod streams;
mod validation;
pub mod wire;
pub use crate::core::socket::QueueStats;
pub use config::{Config, Environment};
pub use identity::{
    BlockOrderMatchingKey, BlockTradeSettlementKey, ClientOrderId, OrderId, RecordId, Symbol,
    TradeId,
};
pub use rate::budget_limits;
pub use streams::{ConnectionDriver, Route, Stream, StreamEvent, Streams};

/// Cloneable REST client sharing endpoint IP/account budgets.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
}
impl RestClient {
    /// Report the shared IP pool's windows at the client's current clock time.
    ///
    /// # Errors
    /// Returns clock failure or a poisoned pool error.
    pub fn pool_usage(&self) -> Result<crate::PoolUsage, crate::Error> {
        self.inner.pool_usage()
    }
    /// The venue pool this client draws on; see [`Config::pool_key`].
    #[must_use]
    pub fn pool_key(&self) -> Option<crate::PoolKey> {
        self.inner.pool_key()
    }
    /// Construct without network I/O or runtime ownership.
    ///
    /// # Errors
    /// Returns invalid transport configuration errors.
    pub fn new(config: Config) -> Result<Self, crate::Error> {
        Ok(Self {
            inner: crate::core::HttpClient::new(
                config.rest,
                config.credentials,
                config.clock,
                config.budgets,
                config.timeout,
                config.proxy,
            )?,
        })
    }
}
