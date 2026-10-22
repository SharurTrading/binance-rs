// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native REST transport and shared client configuration.

use super::Config;

/// Cloneable REST client sharing caller-supplied endpoint IP/UID budgets.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
}

impl RestClient {
    /// Construct a transport without network I/O or runtime ownership.
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
    /// Clear an IP ban the venue gave no timing for, on the SAPI pool this client
    /// draws on; see [`Config::release_unknown_ban`].
    ///
    /// # Errors
    /// Returns a configuration error if the pool's lock is poisoned.
    pub fn release_unknown_ban(&self) -> Result<(), crate::Error> {
        self.inner.release_unknown_ban()
    }
}
