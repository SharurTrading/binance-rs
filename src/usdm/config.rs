// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{BudgetLimits, Budgets, Clock, Credentials, Error, SystemClock};
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
    /// # Errors
    /// Returns a configuration error if an endpoint or venue budget is invalid.
    pub fn new(environment: Environment) -> Result<Self, Error> {
        let (rest, websocket, streams, weight) = match environment {
            Environment::Demo => (
                "https://demo-fapi.binance.com",
                "wss://testnet.binancefuture.com/ws-fapi/v1",
                "wss://demo-fstream.binance.com",
                6000,
            ),
            Environment::Production => (
                "https://fapi.binance.com",
                "wss://ws-fapi.binance.com/ws-fapi/v1",
                "wss://fstream.binance.com",
                2400,
            ),
        };
        Ok(Self {
            rest: crate::core::validate_url(rest, false)?,
            websocket: crate::core::validate_url(websocket, true)?,
            streams: crate::core::validate_url(streams, true)?,
            credentials: None,
            clock: Arc::new(SystemClock),
            budgets: Budgets::new(BudgetLimits::usdm().weight_per_minute(weight))?,
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
