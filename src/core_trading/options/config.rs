// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Budgets, Clock, Credentials, Error, SystemClock};
use std::{sync::Arc, time::Duration};

/// Explicit endpoint environments; no silent production fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Environment {
    /// Current Binance Options demo environment.
    Demo,
    /// Binance production environment.
    Production,
}

/// Provider endpoints and shared services. The caller owns the Tokio runtime.
#[derive(Clone)]
pub struct Config {
    pub(crate) rest: url::Url,
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
    /// Select documented demo or production endpoints with caller-owned venue budgets.
    /// Use `budget_limits` to validate Options exchange-information rate evidence;
    /// share the same owner across clients using the same IP/account.
    ///
    /// # Errors
    /// Returns a configuration error if an endpoint or venue budget is invalid.
    pub fn new(environment: Environment, budgets: Budgets) -> Result<Self, Error> {
        let (rest, streams) = match environment {
            Environment::Demo => (
                "https://demo-fapi.binance.com",
                "wss://demo-fstream.binance.com",
            ),
            Environment::Production => ("https://eapi.binance.com", "wss://fstream.binance.com"),
        };
        Ok(Self {
            rest: crate::core::validate_url(rest, false)?,
            streams: crate::core::validate_url(streams, true)?,
            credentials: None,
            clock: Arc::new(SystemClock),
            budgets,
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
    #[must_use]
    pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }
    /// Share documented IP/account budgets across HTTP and WebSocket clients.
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
