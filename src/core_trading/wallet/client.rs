// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native REST transport and shared client configuration.

use super::Config;

/// REST client whose clones share endpoint budgets and transport configuration.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
}

impl RestClient {
    /// Create a client without network I/O or a hidden runtime.
    ///
    /// # Errors
    /// Returns configuration errors from HTTP construction.
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
    /// Clear an IP ban the venue gave no timing for, on the SAPI pool this client
    /// draws on; see [`Config::release_unknown_ban`].
    ///
    /// # Errors
    /// Returns a configuration error if the pool's lock is poisoned.
    pub fn release_unknown_ban(&self) -> Result<(), crate::Error> {
        self.inner.release_unknown_ban()
    }
}
