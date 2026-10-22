// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! USDⓈ-M Futures REST transport; clones share transport pools and venue budgets.

use super::config::Config;

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
