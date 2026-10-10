// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{AccountKey, Budgets, Clock, Credentials, Error, WeightPools};
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
    /// Draw independent SAPI endpoint IP authority from the explicit registry.
    /// Configurations sharing `pools` share SAPI endpoint quotas and cooldown evidence.
    ///
    /// # Errors
    /// Returns endpoint or shared-pool configuration failures.
    pub fn with_pools(pools: &WeightPools) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::sapi::Config::with_pools(pools)?,
        })
    }
    /// Configure the production SAPI route like [`Config::new`], with the account
    /// owner the process's registry keeps for `account` in the SAPI pool. Every
    /// Margin, Wallet and Convert configuration given an equal key shares its native
    /// order windows and UID weight, so order windows that [`Config::order_limits`]
    /// installs on one hold for all of them.
    ///
    /// # Errors
    /// Returns endpoint or budget configuration failures.
    pub fn new_for_account(account: &AccountKey) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::sapi::Config::new_for_account(account)?,
        })
    }
    /// Configure like [`Config::new_for_account`], drawing both the SAPI IP scopes
    /// and the keyed account owner from `pools` instead of the process's registry.
    ///
    /// # Errors
    /// Returns endpoint or shared-pool configuration failures.
    pub fn with_pools_for_account(
        pools: &WeightPools,
        account: &AccountKey,
    ) -> Result<Self, Error> {
        Ok(Self {
            inner: crate::core::sapi::Config::with_pools_for_account(pools, account)?,
        })
    }
    /// Attach caller-acquired signing credentials.
    #[must_use]
    pub fn credentials(mut self, value: Credentials) -> Self {
        self.inner = self.inner.credentials(value);
        self
    }
    /// Inject a millisecond clock for signing and native quotas.
    ///
    /// Clients sharing one [`WeightPools`](crate::WeightPools) registry must share one
    /// clock (the same `Arc<dyn Clock>`, or wall time for all): each charges the pool's
    /// fixed, aligned windows at the instant its own clock reports.
    #[must_use]
    pub fn clock(mut self, value: Arc<dyn Clock>) -> Self {
        self.inner = self.inner.clock(value);
        self
    }
    /// The venue pool this configuration's clients draw on, the production SAPI
    /// pool; `None` once [`Config::budgets`] replaces it with an explicit owner.
    #[must_use]
    pub fn pool_key(&self) -> Option<crate::PoolKey> {
        self.inner.pool_key()
    }
    /// Clear an IP ban the venue gave no timing for, on the SAPI pool this
    /// configuration's clients draw on; see [`crate::Budgets::release_unknown_ban`].
    ///
    /// # Errors
    /// Returns a configuration error if the pool's lock is poisoned.
    pub fn release_unknown_ban(&self) -> Result<(), Error> {
        self.inner.release_unknown_ban()
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
