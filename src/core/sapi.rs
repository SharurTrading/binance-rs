// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! SAPI endpoint budgets: Wallet/Convert general-info defines independent
//! endpoint counters, IP 12,000/minute or UID 180,000/minute.
use super::{Budgets, Cost, Error, RateEvidence};
use std::time::Duration;
#[derive(Clone, Copy)]
pub(crate) struct SapiCost {
    pub endpoint: &'static str,
    pub uid: bool,
    pub weight: u64,
    pub requests_per_second: Option<u64>,
}
impl Budgets {
    pub(crate) fn admit_sapi(&self, cost: SapiCost, now: u64) -> Result<(), Error> {
        {
            let ip = self
                .ip
                .lock()
                .map_err(|_| Error::Configuration("IP budget poisoned"))?;
            ip.check_cooldown(now)?;
        }
        let owner = if cost.uid { &self.account } else { &self.ip };
        let mut state = owner
            .lock()
            .map_err(|_| Error::Configuration("SAPI budget poisoned"))?;
        if let Some(&until) = state.endpoint_cooldown.get(cost.endpoint)
            && now < until
        {
            return Err(Error::Admission {
                retry_after: Duration::from_millis(until - now),
            });
        }
        let mut windows = vec![(60_000, if cost.uid { 180_000 } else { 12_000 }, cost.weight)];
        // The bucket width is derived from the pinned unit of the provider's
        // annotation: `x-requests-per-second` is documented per second, so the
        // interval window is one 1000 ms bucket charging one request.
        if let Some(limit) = cost.requests_per_second {
            windows.push((1000, limit, 1));
        }
        for &(window, limit, amount) in &windows {
            let (bucket, count) = state
                .endpoints
                .get(&(cost.endpoint, window))
                .copied()
                .unwrap_or_default();
            let count = if bucket == now / window { count } else { 0 };
            if count.checked_add(amount).is_none_or(|v| v > limit) {
                return Err(Error::Admission {
                    retry_after: Duration::from_millis(window - now % window),
                });
            }
        }
        for (window, _, amount) in windows {
            let current = state.endpoints.entry((cost.endpoint, window)).or_default();
            if current.0 != now / window {
                *current = (now / window, 0);
            }
            current.1 = current
                .1
                .checked_add(amount)
                .ok_or(Error::Configuration("SAPI budget overflow"))?;
        }
        Ok(())
    }
    pub(crate) fn observe_cost(
        &self,
        cost: Cost,
        e: &RateEvidence,
        now: u64,
        status: u16,
    ) -> Result<(), Error> {
        self.observe_ban(status, e)?;
        let Some(cost) = cost.sapi else {
            return self.observe(e, now, false);
        };
        if status == 418 {
            self.observe(e, now, false)?;
        }
        for (uid, owner) in [(false, &self.ip), (true, &self.account)] {
            let mut state = owner
                .lock()
                .map_err(|_| Error::Configuration("SAPI budget poisoned"))?;
            let header = if uid {
                "x-sapi-used-uid-weight-1m"
            } else {
                "x-sapi-used-ip-weight-1m"
            };
            if let Some(&count) = e.counters.get(header) {
                let current = state.endpoints.entry((cost.endpoint, 60_000)).or_default();
                if current.0 != now / 60_000 {
                    *current = (now / 60_000, 0);
                }
                current.1 = current.1.max(count);
            }
            if cost.uid == uid
                && let Some(delay) = e.retry_after
            {
                // Venue retry timing is never dropped and never narrowed. An
                // unrepresentable delay saturates at the widest one; erroring here
                // would discard an answer the venue has already sent.
                let delay = u64::try_from(delay.as_nanos().div_ceil(1_000_000)).unwrap_or(u64::MAX);
                let until = now.saturating_add(delay);
                let current = state.endpoint_cooldown.entry(cost.endpoint).or_default();
                *current = (*current).max(until);
            }
        }
        Ok(())
    }
}

/// Configuration for production SAPI REST products; no documented demo fallback.
/// Callers own credentials, time, IP/account budget scopes, and the async runtime.
#[derive(Clone)]
pub struct Config {
    pub(crate) rest: url::Url,
    pub(crate) credentials: Option<super::Credentials>,
    pub(crate) clock: std::sync::Arc<dyn super::Clock>,
    pub(crate) budgets: Budgets,
    pub(crate) timeout: Duration,
    pub(crate) proxy: Option<reqwest::Proxy>,
}
impl Config {
    /// Configure the documented production SAPI host without network I/O.
    ///
    /// # Errors
    /// Returns invalid endpoint or budget configuration errors.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            rest: super::validate_url("https://api.binance.com", false)?,
            credentials: None,
            clock: std::sync::Arc::new(super::SystemClock),
            budgets: Budgets::sapi()?,
            timeout: Duration::from_secs(10),
            proxy: None,
        })
    }
    /// Attach caller-acquired signing credentials.
    #[must_use]
    pub fn credentials(mut self, value: super::Credentials) -> Self {
        self.credentials = Some(value);
        self
    }
    /// Inject a millisecond clock for signing, expiry and endpoint quotas.
    #[must_use]
    pub fn clock(mut self, value: std::sync::Arc<dyn super::Clock>) -> Self {
        self.clock = value;
        self
    }
    /// Share IP/UID scopes across every Wallet/Convert client for this account.
    #[must_use]
    pub fn budgets(mut self, value: Budgets) -> Self {
        self.budgets = value;
        self
    }
    /// Set the single-attempt transport timeout.
    ///
    /// # Errors
    /// Refuses a zero or unrepresentable timeout.
    pub fn timeout(mut self, value: Duration) -> Result<Self, Error> {
        if value.is_zero() || tokio::time::Instant::now().checked_add(value).is_none() {
            return Err(Error::Configuration("zero or unrepresentable timeout"));
        }
        self.timeout = value;
        Ok(self)
    }
    /// Override the endpoint with TLS or an exact loopback fixture host.
    ///
    /// # Errors
    /// Refuses insecure remote hosts and URL credentials.
    pub fn rest_url(mut self, value: &str) -> Result<Self, Error> {
        self.rest = super::validate_url(value, false)?;
        Ok(self)
    }
    /// Use an explicit proxy; ambient proxy settings are ignored.
    #[must_use]
    pub fn http_proxy(mut self, value: reqwest::Proxy) -> Self {
        self.proxy = Some(value);
        self
    }
}
impl Budgets {
    /// Create SAPI endpoint scopes. Endpoint contracts govern every admission;
    /// legacy Spot/Futures aggregate counters are not charged by SAPI requests.
    /// `for_account` preserves the shared IP owner and creates a distinct UID owner.
    ///
    /// # Errors
    /// Returns configuration errors when constructing the shared owners.
    pub fn sapi() -> Result<Self, Error> {
        Self::new(super::BudgetLimits {
            weight_per_minute: u64::MAX,
            ws_weight_per_minute: u64::MAX,
            orders_per_ten_seconds: u64::MAX,
            orders_per_minute: u64::MAX,
            orders_per_day: None,
            raw_requests_per_five_minutes: None,
            connections_per_five_minutes: None,
            shared_request_weight: false,
        })
    }
}
