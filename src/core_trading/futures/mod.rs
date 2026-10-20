// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared transport plumbing for native linear and inverse Futures products.
//! Wire payloads, endpoint catalogs and rate tables remain product-specific.

mod config;
pub(crate) mod streams;
pub(crate) mod validation;
mod websocket;
pub use config::{Config, Environment};
pub use websocket::{ApiEvent, ApiEvents, ConnectionDriver, WsClient};

pub(crate) mod sealed {
    pub trait Sealed {}
}
/// Fixed transport facts implemented only by native Futures product markers.
pub trait Product: sealed::Sealed + Clone {
    /// REST, WebSocket API and stream endpoints in the demo environment.
    const DEMO: [&'static str; 3];
    /// REST, WebSocket API and stream endpoints in production.
    const PRODUCTION: [&'static str; 3];
    /// Apply native REST/WebSocket counter sharing to the supplied owner.
    fn configure_budgets(budgets: crate::Budgets) -> crate::Budgets;
}
/// Native WebSocket API payload and protocol validation boundary.
pub trait ApiProduct: Product {
    /// Product-native late-response body.
    type ApiPayload: std::fmt::Debug;
    /// Product-native authentication session receipt.
    type SessionStatus: serde::de::DeserializeOwned;
    /// Decode without converting linear and inverse account wire models.
    ///
    /// # Errors
    /// Returns malformed native response evidence.
    fn api_payload(name: &str, value: serde_json::Value) -> Result<Self::ApiPayload, crate::Error>;
    /// Check time-sensitive native request authority immediately before sending.
    ///
    /// # Errors
    /// Returns the product's validation refusal.
    fn validate_time(
        parameters: &std::collections::BTreeMap<String, serde_json::Value>,
        now: u64,
    ) -> Result<(), crate::Error>;
    /// Whether this product documents a refusal as definitive.
    fn definitive(status: u16, value: &serde_json::Value) -> bool;
}

/// Cloneable REST client; clones share transport pools and venue budgets.
#[derive(Clone)]
pub struct RestClient<P: Product> {
    product: std::marker::PhantomData<P>,
    pub(crate) inner: crate::core::HttpClient,
}
impl<P: Product> RestClient<P> {
    /// The IP windows of the pool this client draws on, read at its clock; see
    /// [`Config::pool_usage`].
    ///
    /// # Errors
    /// Returns the clock's error, or a configuration error if the pool's lock is
    /// poisoned.
    pub fn pool_usage(&self) -> Result<crate::PoolUsage, crate::Error> {
        self.inner.pool_usage()
    }
    /// This account owner's order windows at this client's clock;
    /// see [`Config::account_usage`].
    ///
    /// # Errors
    /// Returns the clock's error, or a configuration error if a budget lock is poisoned.
    pub fn account_usage(&self) -> Result<crate::AccountUsage, crate::Error> {
        self.inner.account_usage()
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
    /// Create a transport without starting a runtime or performing network I/O.
    ///
    /// # Errors
    /// Returns configuration errors from the HTTP transport.
    pub fn new(config: Config<P>) -> Result<Self, crate::Error> {
        Ok(Self {
            product: std::marker::PhantomData,
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
