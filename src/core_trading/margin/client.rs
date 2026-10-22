// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Margin REST transport; clones share caller-owned SAPI IP/account budgets.

use super::config::Config;

/// Cloneable REST client sharing caller-owned SAPI IP/account budgets.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
    budgets: crate::Budgets,
    clock: std::sync::Arc<dyn crate::Clock>,
}
impl RestClient {
    /// Construct a transport without network I/O or runtime ownership.
    ///
    /// # Errors
    /// Returns typed transport configuration errors.
    pub fn new(config: Config) -> Result<Self, crate::Error> {
        let config = config.inner;
        Ok(Self {
            budgets: config.budgets.clone(),
            clock: config.clock.clone(),
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
    /// Install fresh native order count/limit evidence on this shared account owner.
    /// Validate all records before mutation; clone clients immediately share authority.
    ///
    /// # Errors
    /// Refuses missing/unknown or malformed quota evidence, overflow and clock failures.
    pub fn observe_order_limits(
        &self,
        response: &super::rest_models::QueryCurrentMarginOrderCountUsageResponse,
    ) -> Result<(), crate::Error> {
        let windows = super::rate::order_windows(response)?;
        self.budgets
            .observe_sapi_order_windows(&windows, self.clock.now_millis()?)
    }
}
