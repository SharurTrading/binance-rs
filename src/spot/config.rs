// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{BudgetLimits, Budgets, Clock, Credentials, Error, SystemClock};
use std::{sync::Arc, time::Duration};

/// Explicit endpoint environments; no silent production fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Environment {
    /// Current Binance Spot demo environment.
    Demo,
    /// Binance production environment.
    Production,
}

/// Provider endpoints and shared services. The caller owns the Tokio runtime.
#[derive(Clone)]
pub struct Config {
    pub(crate) rest: url::Url,
    pub(crate) time_unit: crate::TimeUnit,
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
                "https://demo-api.binance.com",
                "wss://demo-ws-api.binance.com/ws-api/v3",
                "wss://demo-stream.binance.com",
                6000,
            ),
            Environment::Production => (
                "https://api.binance.com",
                "wss://ws-api.binance.com/ws-api/v3",
                "wss://stream.binance.com:9443",
                6000,
            ),
        };
        Ok(Self {
            rest: crate::core::validate_url(rest, false)?,
            time_unit: crate::TimeUnit::Milliseconds,
            websocket: crate::core::validate_url(websocket, true)?,
            streams: crate::core::validate_url(streams, true)?,
            credentials: None,
            clock: Arc::new(SystemClock),
            budgets: Budgets::new(BudgetLimits::spot().weight_per_minute(weight))?,
            timeout: Duration::from_secs(10),
            proxy: None,
        })
    }
    /// Select explicit timestamp units for REST responses, API replies, and streams.
    /// Signing uses the same units; `recvWindow` always remains milliseconds.
    #[must_use]
    pub fn time_unit(mut self, unit: crate::TimeUnit) -> Self {
        self.time_unit = unit;
        self
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
    /// Refuses a zero timeout.
    pub fn timeout(mut self, timeout: Duration) -> Result<Self, Error> {
        if timeout.is_zero() {
            return Err(Error::Configuration("zero timeout"));
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
    /// Override the market streaming endpoint root.
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

pub(super) fn apply_time_unit(url: &mut url::Url, unit: crate::TimeUnit) {
    let parameters: Vec<_> = url
        .query_pairs()
        .filter(|(key, _)| key != "timeUnit")
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    url.set_query(None);
    if !parameters.is_empty() {
        url.query_pairs_mut().extend_pairs(parameters);
    }
    if unit == crate::TimeUnit::Microseconds {
        url.query_pairs_mut().append_pair("timeUnit", "MICROSECOND");
    }
}
#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "synthetic endpoint assertions")]
mod tests {
    #[test]
    fn configured_timestamp_provenance_overrides_endpoint_query() {
        let mut url = url::Url::parse("ws://127.0.0.1/?timeUnit=MICROSECOND&other=a").unwrap();
        super::apply_time_unit(&mut url, crate::TimeUnit::Milliseconds);
        assert!(!url.query_pairs().any(|(key, _)| key == "timeUnit"));
        let mut url =
            url::Url::parse("ws://127.0.0.1/?timeUnit=MICROSECOND&timeUnit=microsecond&other=a")
                .unwrap();
        super::apply_time_unit(&mut url, crate::TimeUnit::Microseconds);
        assert_eq!(
            url.query_pairs()
                .filter(|(key, _)| key == "timeUnit")
                .count(),
            1
        );
        assert!(
            url.query_pairs()
                .any(|(key, value)| key == "other" && value == "a")
        );
    }
}
