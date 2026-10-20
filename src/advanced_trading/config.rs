// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared configuration machinery; product aliases select native endpoints and pools.
use crate::core::{HttpClient, PoolEnvironment, VenuePool};
use crate::{AccountKey, Budgets, Clock, Credentials, Error, SystemClock, WeightPools};
use std::{marker::PhantomData, sync::Arc, time::Duration};

pub(crate) mod sealed {
    pub trait Sealed {}
}
/// Fixed protocol facts implemented only by this crate's product markers.
pub trait Product: sealed::Sealed + Clone {
    /// Official production REST host.
    const REST: &'static str;
    /// Official stream route, when documented.
    const STREAMS: Option<&'static str>;
    /// Independent venue budget scope.
    const POOL: VenuePool;
}

/// Production product configuration. Construction performs no network I/O.
/// Credentials, clock and runtime are caller-owned. Clones share all budgets.
#[derive(Clone)]
pub struct Config<P: Product> {
    pub(crate) rest: url::Url,
    pub(crate) streams: Option<url::Url>,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) clock: Arc<dyn Clock>,
    pub(crate) budgets: Budgets,
    pub(crate) timeout: Duration,
    proxy: Option<reqwest::Proxy>,
    product: PhantomData<P>,
}
impl<P: Product> std::fmt::Debug for Config<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("credentials", &self.credentials)
            .field("budgets", &self.budgets)
            .field("timeout", &self.timeout)
            .finish_non_exhaustive()
    }
}
impl<P: Product> Config<P> {
    /// Select this product's production endpoints and process IP pool.
    ///
    /// # Errors
    /// Returns endpoint or budget configuration errors.
    pub fn new() -> Result<Self, Error> {
        Self::with_pools(WeightPools::process())
    }
    /// Select a registry. New configurations have distinct account owners;
    /// clients sharing a registry must use the same clock.
    ///
    /// # Errors
    /// Returns invalid endpoint or poisoned registry errors.
    pub fn with_pools(pools: &WeightPools) -> Result<Self, Error> {
        Self::drawn(pools, None)
    }
    /// Share the named account's counters in this product's process pool.
    ///
    /// # Errors
    /// Returns configuration errors.
    pub fn new_for_account(account: &AccountKey) -> Result<Self, Error> {
        Self::with_pools_for_account(WeightPools::process(), account)
    }
    /// Share account and IP counters through an explicit registry.
    ///
    /// # Errors
    /// Returns configuration errors.
    pub fn with_pools_for_account(
        pools: &WeightPools,
        account: &AccountKey,
    ) -> Result<Self, Error> {
        Self::drawn(pools, Some(account))
    }
    fn drawn(pools: &WeightPools, account: Option<&AccountKey>) -> Result<Self, Error> {
        Ok(Self {
            rest: crate::core::validate_url(P::REST, false)?,
            streams: P::STREAMS
                .map(|s| crate::core::validate_url(s, true))
                .transpose()?,
            credentials: None,
            clock: Arc::new(SystemClock),
            budgets: pools.draw(P::POOL, PoolEnvironment::Production, account)?,
            timeout: Duration::from_secs(10),
            proxy: None,
            product: PhantomData,
        })
    }
    /// Attach caller-acquired credentials. Debug output redacts signing material.
    #[must_use]
    pub fn credentials(mut self, value: Credentials) -> Self {
        self.credentials = Some(value);
        self
    }
    /// Use one clock across every client sharing a registry.
    #[must_use]
    pub fn clock(mut self, value: Arc<dyn Clock>) -> Self {
        self.clock = value;
        self
    }
    /// Replace the explicit budget owner, preserving its accepted evidence.
    #[must_use]
    pub fn budgets(mut self, value: Budgets) -> Self {
        self.budgets = value;
        self
    }
    /// Current pool identity, absent after replacing the budget owner.
    #[must_use]
    pub fn pool_key(&self) -> Option<crate::PoolKey> {
        self.budgets.pool_key()
    }
    /// Current aggregate IP usage and limits.
    ///
    /// # Errors
    /// Returns clock or poisoned budget errors.
    pub fn pool_usage(&self) -> Result<crate::PoolUsage, Error> {
        self.budgets.usage(self.clock.now_millis()?)
    }
    /// Current native account order counters.
    ///
    /// # Errors
    /// Returns clock or poisoned budget errors.
    pub fn account_usage(&self) -> Result<crate::AccountUsage, Error> {
        self.budgets.account_usage(self.clock.now_millis()?)
    }
    /// Replace the shared caller-selected exit reserve.
    ///
    /// # Errors
    /// Returns a poisoned budget error without partial policy mutation.
    pub fn set_exit_reserve(&self, value: crate::ExitReserve) -> Result<(), Error> {
        self.budgets.set_exit_reserve(value)
    }
    /// Release an untimed venue ban after the caller resolves it with the venue.
    /// Known cooldown timing continues to bind.
    ///
    /// # Errors
    /// Returns a poisoned budget error.
    pub fn release_unknown_ban(&self) -> Result<(), Error> {
        self.budgets.release_unknown_ban()
    }
    /// Override the REST host; insecure transport is permitted only on exact loopback.
    ///
    /// # Errors
    /// Refuses insecure remote and credential-bearing URLs.
    pub fn rest_url(mut self, value: &str) -> Result<Self, Error> {
        self.rest = crate::core::validate_url(value, false)?;
        Ok(self)
    }
    /// Override a documented stream endpoint for caller routing or local fixtures.
    ///
    /// # Errors
    /// Refuses products without streams and insecure remote URLs.
    pub fn streams_url(mut self, value: &str) -> Result<Self, Error> {
        if P::STREAMS.is_none() {
            return Err(Error::Configuration("product has no documented streams"));
        }
        self.streams = Some(crate::core::validate_url(value, true)?);
        Ok(self)
    }
    /// Set an explicit HTTP proxy; ambient proxy variables are ignored.
    ///
    /// # Errors
    /// Refuses invalid proxy configuration.
    pub fn proxy(mut self, value: &str) -> Result<Self, Error> {
        self.proxy =
            Some(reqwest::Proxy::all(value).map_err(|_| Error::Configuration("HTTP proxy"))?);
        Ok(self)
    }
    /// Set the transport timeout.
    ///
    /// # Errors
    /// Refuses zero or unrepresentable durations.
    pub fn timeout(mut self, value: Duration) -> Result<Self, Error> {
        if value.is_zero() || tokio::time::Instant::now().checked_add(value).is_none() {
            return Err(Error::Configuration("transport timeout"));
        }
        self.timeout = value;
        Ok(self)
    }
    pub(crate) fn check_stream_transport(&self) -> Result<(), Error> {
        if self.proxy.is_some() {
            return Err(Error::Configuration(
                "explicit HTTP proxies are not supported by stream transport",
            ));
        }
        Ok(())
    }
    pub(crate) fn http(self) -> Result<HttpClient, Error> {
        HttpClient::new(
            self.rest,
            self.credentials,
            self.clock,
            self.budgets,
            self.timeout,
            self.proxy,
        )
    }
}

/// REST transport whose clones share product configuration and quota ownership.
#[derive(Clone)]
pub struct RestClient<P: Product> {
    pub(crate) inner: HttpClient,
    product: PhantomData<P>,
}
impl<P: Product> RestClient<P> {
    /// Construct without network I/O or a hidden runtime.
    ///
    /// # Errors
    /// Returns invalid HTTP transport configuration.
    pub fn new(config: Config<P>) -> Result<Self, Error> {
        Ok(Self {
            inner: config.http()?,
            product: PhantomData,
        })
    }
}
