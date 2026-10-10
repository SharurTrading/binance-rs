// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::core::{PoolEnvironment, VenuePool};
use crate::{AccountKey, Budgets, Clock, Credentials, Error, SystemClock, WeightPools};
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
    /// Select documented endpoints and the process's Options IP pool for this environment.
    /// Independently constructed clients share IP authority, with separate account owners.
    /// Clones share both owners; use [`Self::new_for_account`] or [`Self::budgets`] to
    /// share an account explicitly.
    ///
    /// # Errors
    /// Returns a configuration error if an endpoint or venue budget is invalid.
    pub fn new(environment: Environment) -> Result<Self, Error> {
        Self::with_pools(environment, WeightPools::process())
    }
    /// Select the Options IP pool from an explicit registry.
    /// Clients using this registry and environment share IP evidence. Every new
    /// configuration receives a separate account owner; its clones retain that owner.
    /// Documented host and rate sources are pinned in `schema/options-environments.json`.
    ///
    /// # Errors
    /// Returns a configuration error if endpoint validation or pool ownership fails.
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
        let (rest, streams, pool) = match environment {
            Environment::Demo => (
                "https://demo-fapi.binance.com",
                "wss://demo-fstream.binance.com",
                PoolEnvironment::Demo,
            ),
            Environment::Production => (
                "https://eapi.binance.com",
                "wss://fstream.binance.com",
                PoolEnvironment::Production,
            ),
        };
        Ok(Self {
            rest: crate::core::validate_url(rest, false)?,
            streams: crate::core::validate_url(streams, true)?,
            credentials: None,
            clock: Arc::new(SystemClock),
            budgets: pools.draw(VenuePool::Options, pool, account)?,
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
    /// Report this configuration's shared IP windows at its clock's current time.
    /// Reports preserve documented or venue-stated limits and admitted usage.
    ///
    /// # Errors
    /// Returns clock failure or a poisoned pool error.
    pub fn pool_usage(&self) -> Result<crate::PoolUsage, Error> {
        self.budgets.usage(self.clock.now_millis()?)
    }
    /// The venue pool this configuration's clients draw on, to group clients that
    /// count against one venue limit; `None` once [`Config::budgets`] replaces it
    /// with an explicit owner.
    #[must_use]
    pub fn pool_key(&self) -> Option<crate::PoolKey> {
        self.budgets.pool_key()
    }
    /// Replace the drawn pool with an explicit IP/account owner.
    /// Clone this owner to share both scopes, or use [`Budgets::for_account`] to
    /// share IP evidence with a separate account owner.
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

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "pinned protocol fixture assertions")]
mod tests {
    use super::{Config, Environment};
    use crate::WeightPools;

    #[test]
    fn native_options_environment_hosts_match_pinned_official_evidence() {
        let evidence: serde_json::Value =
            serde_json::from_str(include_str!("../../../schema/options-environments.json"))
                .unwrap();
        for (environment, key) in [
            (Environment::Demo, "Demo"),
            (Environment::Production, "Production"),
        ] {
            let config = Config::with_pools(environment, &WeightPools::new()).unwrap();
            let expected = &evidence["environments"][key];
            assert_eq!(
                config.rest.as_str().trim_end_matches('/'),
                expected["rest"].as_str().unwrap()
            );
            assert_eq!(
                config.streams.as_str().trim_end_matches('/'),
                expected["streams"].as_str().unwrap()
            );
            for path in ["/public/", "/market/", "/private/"] {
                let url = config.streams.join(path).unwrap();
                assert_eq!(url.path(), path);
                assert_eq!(url.host_str(), config.streams.host_str());
            }
        }
    }
}
