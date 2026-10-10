// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Budgets, Clock, Credentials, Error};
use std::{sync::Arc, time::Duration};
/// Margin production REST configuration with explicit native account order authority.
/// Reads need no order quota configuration. Placements refuse until `order_limits`
/// supplies current venue evidence on the shared account owner.
#[derive(Clone)]
pub struct Config {
    pub(crate) inner: crate::core::sapi::Config,
}
impl Config {
    /// Configure the production SAPI route without network I/O or tasks.
    ///
    /// # Errors
    /// Returns endpoint or budget configuration failures.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::sapi::Config::new()?,
        })
    }
    /// Attach caller-acquired signing credentials.
    #[must_use]
    pub fn credentials(mut self, value: Credentials) -> Self {
        self.inner = self.inner.credentials(value);
        self
    }
    /// Inject a millisecond clock for signing and native quotas.
    #[must_use]
    pub fn clock(mut self, value: Arc<dyn Clock>) -> Self {
        self.inner = self.inner.clock(value);
        self
    }
    /// Share the explicit IP and account owners with all clients in those scopes.
    /// Configure order evidence after selecting this owner.
    #[must_use]
    pub fn budgets(mut self, value: Budgets) -> Self {
        self.inner = self.inner.budgets(value);
        self
    }
    /// Install venue-reported account order windows and current counts atomically.
    /// The complete response is validated before changing shared authority. Counts
    /// are retained as floors within the reported venue interval.
    ///
    /// # Errors
    /// Refuses absent/unknown interval or type, zero limits, negative values, overflow
    /// and clock failures. Existing accepted evidence remains unchanged on failure.
    pub fn order_limits(
        self,
        response: &super::rest_models::QueryCurrentMarginOrderCountUsageResponse,
    ) -> Result<Self, Error> {
        let windows = super::rate::order_windows(response)?;
        self.inner
            .budgets
            .observe_sapi_order_windows(&windows, self.inner.clock.now_millis()?)?;
        Ok(self)
    }
    /// Set the single-attempt HTTP timeout.
    ///
    /// # Errors
    /// Refuses zero or unrepresentable timeouts.
    pub fn timeout(mut self, value: Duration) -> Result<Self, Error> {
        self.inner = self.inner.timeout(value)?;
        Ok(self)
    }
    /// Override the endpoint, permitting insecure exact loopback fixtures.
    ///
    /// # Errors
    /// Refuses insecure remote or credential-bearing URLs.
    pub fn rest_url(mut self, value: &str) -> Result<Self, Error> {
        self.inner = self.inner.rest_url(value)?;
        Ok(self)
    }
    /// Configure an explicit proxy; ambient proxy variables remain ignored.
    #[must_use]
    pub fn http_proxy(mut self, value: reqwest::Proxy) -> Self {
        self.inner = self.inner.http_proxy(value);
        self
    }
}
