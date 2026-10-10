// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::core::{PoolEnvironment, VenuePool};
use crate::{AccountKey, Budgets, Clock, Credentials, Error, SystemClock, WeightPools};
use std::{sync::Arc, time::Duration};

/// Explicit endpoint environments; no silent production fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Environment {
    /// Current Binance Futures demo environment.
    Demo,
    /// Binance production environment.
    Production,
}

/// Provider endpoints and shared services. The caller owns the Tokio runtime.
#[derive(Clone)]
pub struct Config {
    pub(crate) rest: url::Url,
    pub(crate) websocket: url::Url,
    pub(crate) streams: url::Url,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) clock: Arc<dyn Clock>,
    pub(crate) budgets: Budgets,
    pub(crate) timeout: Duration,
    pub(crate) proxy: Option<reqwest::Proxy>,
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("credentials", &self.credentials)
            .field("budgets", &self.budgets)
            .field("timeout", &self.timeout)
            .finish_non_exhaustive()
    }
}
impl Config {
    /// Select demo or production endpoints explicitly.
    ///
    /// The IP budget is the process's pool for this venue pool and environment,
    /// shared with every other client drawn from it; the account owner is this
    /// configuration's own, unless [`Config::new_for_account`] names a shared one.
    /// [`Config::budgets`] replaces both.
    ///
    /// # Errors
    /// Returns a configuration error if an endpoint or venue budget is invalid.
    pub fn new(environment: Environment) -> Result<Self, Error> {
        Self::with_pools(environment, WeightPools::process())
    }
    /// Select endpoints like [`Config::new`], drawing the IP budget from `pools`
    /// instead of the process's registry.
    ///
    /// # Errors
    /// Returns a configuration error if an endpoint or venue budget is invalid.
    pub fn with_pools(environment: Environment, pools: &WeightPools) -> Result<Self, Error> {
        Self::drawn(environment, pools, None)
    }
    /// Select endpoints like [`Config::new`], with the account owner the process's
    /// registry keeps for `account` in this venue pool and environment: every
    /// configuration given an equal key shares its order counters and other
    /// account limits, while the IP budget stays the pool's.
    ///
    /// # Errors
    /// Returns a configuration error if an endpoint or venue budget is invalid.
    pub fn new_for_account(environment: Environment, account: &AccountKey) -> Result<Self, Error> {
        Self::with_pools_for_account(environment, WeightPools::process(), account)
    }
    /// Select endpoints like [`Config::new_for_account`], drawing both the IP budget
    /// and the keyed account owner from `pools` instead of the process's registry.
    ///
    /// # Errors
    /// Returns a configuration error if an endpoint or venue budget is invalid.
    pub fn with_pools_for_account(
        environment: Environment,
        pools: &WeightPools,
        account: &AccountKey,
    ) -> Result<Self, Error> {
        Self::drawn(environment, pools, Some(account))
    }
    fn drawn(
        environment: Environment,
        pools: &WeightPools,
        account: Option<&AccountKey>,
    ) -> Result<Self, Error> {
        let (rest, websocket, streams, pool) = match environment {
            Environment::Demo => (
                "https://demo-dapi.binance.com",
                "wss://testnet.binancefuture.com/ws-dapi/v1",
                "wss://demo-dstream.binance.com",
                PoolEnvironment::Demo,
            ),
            Environment::Production => (
                "https://dapi.binance.com",
                "wss://ws-dapi.binance.com/ws-dapi/v1",
                "wss://dstream.binance.com",
                PoolEnvironment::Production,
            ),
        };
        Ok(Self {
            rest: crate::core::validate_url(rest, false)?,
            websocket: crate::core::validate_url(websocket, true)?,
            streams: crate::core::validate_url(streams, true)?,
            credentials: None,
            clock: Arc::new(SystemClock),
            budgets: pools.draw(VenuePool::Futures, pool, account)?,
            timeout: Duration::from_secs(10),
            proxy: None,
        })
    }
    /// Attach caller-acquired credentials.
    #[must_use]
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }
    /// Inject time for signing and venue budget accounting.
    ///
    /// Clients sharing one [`WeightPools`](crate::WeightPools) registry must share one
    /// clock (the same `Arc<dyn Clock>`, or wall time for all): each charges the pool's
    /// fixed, aligned windows at the instant its own clock reports.
    #[must_use]
    pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }
    /// The IP windows of the pool this configuration's clients draw on, read at
    /// its clock: each window's limit, whether the venue stated it, and what the
    /// pool has spent. It reports; admission alone decides what is sent.
    ///
    /// # Errors
    /// Returns the clock's error, or a configuration error if the pool's lock is
    /// poisoned.
    pub fn pool_usage(&self) -> Result<crate::PoolUsage, Error> {
        self.budgets.usage(self.clock.now_millis()?)
    }
    /// Clear an IP ban the venue gave no timing for, on the pool this configuration's
    /// clients draw on. Every client of the pool is admitted again; a known
    /// `Retry-After` cooldown still holds. See [`Budgets::release_unknown_ban`].
    ///
    /// # Errors
    /// Returns a configuration error if the pool's lock is poisoned.
    pub fn release_unknown_ban(&self) -> Result<(), Error> {
        self.budgets.release_unknown_ban()
    }
    /// The venue pool this configuration's clients draw on, to group clients that
    /// count against one venue limit; `None` once [`Config::budgets`] replaces it
    /// with an explicit owner.
    #[must_use]
    pub fn pool_key(&self) -> Option<crate::PoolKey> {
        self.budgets.pool_key()
    }
    /// Replace the drawn pool with an explicit IP/account owner, isolating this
    /// client from every pool; clone the owner to share it across clients.
    #[must_use]
    pub fn budgets(mut self, budgets: Budgets) -> Self {
        self.budgets = budgets;
        self
    }
    /// Set the per-attempt transport timeout. No automatic retry is performed.
    ///
    /// # Errors
    /// Refuses a zero or unrepresentable timeout.
    pub fn timeout(mut self, timeout: Duration) -> Result<Self, Error> {
        if timeout.is_zero() || tokio::time::Instant::now().checked_add(timeout).is_none() {
            return Err(Error::Configuration("zero or unrepresentable timeout"));
        }
        self.timeout = timeout;
        Ok(self)
    }
    /// Override a REST endpoint (TLS, or exact loopback for fixtures).
    ///
    /// # Errors
    /// Refuses insecure remote URLs and URL credentials.
    pub fn rest_url(mut self, url: &str) -> Result<Self, Error> {
        self.rest = crate::core::validate_url(url, false)?;
        Ok(self)
    }
    /// Override the WebSocket API endpoint.
    ///
    /// # Errors
    /// Refuses insecure remote URLs and URL credentials.
    pub fn websocket_url(mut self, url: &str) -> Result<Self, Error> {
        self.websocket = crate::core::validate_url(url, true)?;
        Ok(self)
    }
    /// Override the routed market/private streaming endpoint root.
    ///
    /// # Errors
    /// Refuses insecure remote URLs and URL credentials.
    pub fn streams_url(mut self, url: &str) -> Result<Self, Error> {
        self.streams = crate::core::validate_url(url, true)?;
        Ok(self)
    }
    /// Explicit HTTP proxy. Ambient proxy variables are always ignored.
    ///
    /// WebSocket proxy support is intentionally absent; it is never silently inherited.
    #[must_use]
    pub fn http_proxy(mut self, proxy: reqwest::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
