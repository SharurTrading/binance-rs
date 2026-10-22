// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native REST transport and shared client configuration.

use super::Config;

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
