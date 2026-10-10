// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! SAPI endpoint budgets: Wallet/Convert/Margin general-info defines independent
//! endpoint counters, IP 12,000/minute or UID 180,000/minute.
use super::{Budgets, Cost, Error, RateEvidence};
use std::time::Duration;
#[derive(Clone, Copy)]
pub(crate) struct SapiCost {
    pub endpoint: &'static str,
    pub uid: bool,
    pub weight: u64,
    pub requests_per_second: Option<u64>,
    /// Additional per-IP request cap, independent of the endpoint weight scope.
    pub requests_per_minute: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sapi_orders_require_authority_and_refuse_without_partial_weight_charge() {
        let budgets = Budgets::sapi().unwrap();
        let cost = SapiCost {
            endpoint: "/sapi/v1/margin/order",
            uid: true,
            weight: 6,
            requests_per_second: None,
            requests_per_minute: None,
        };
        let order = Cost {
            orders10: 1,
            ..Cost::default()
        };
        assert!(matches!(
            budgets.admit_sapi(cost, order, 120_000),
            Err(Error::Configuration(_))
        ));
        assert!(budgets.account.lock().unwrap().endpoints.is_empty());
        budgets
            .observe_sapi_order_windows(&[(10_000, 0, 1)], 120_000)
            .unwrap();
        budgets.admit_sapi(cost, order, 120_001).unwrap();
        let clone = budgets.clone();
        assert!(matches!(
            clone.admit_sapi(cost, order, 120_002),
            Err(Error::Admission { .. })
        ));
        assert_eq!(
            budgets.account.lock().unwrap().endpoints[&(cost.endpoint, 60_000)].1,
            6
        );
        let other = budgets.for_account();
        assert!(matches!(
            other.admit_sapi(cost, order, 120_002),
            Err(Error::Configuration(_))
        ));
        // Invalid observations never replace valid evidence or partly install quotas.
        assert!(
            budgets
                .observe_sapi_order_windows(&[(60_000, 0, 100), (0, 0, 1)], 120_003)
                .is_err()
        );
        assert_eq!(budgets.account.lock().unwrap().sapi_order_windows.len(), 1);
        let evidence = RateEvidence {
            counters: [("x-mbx-order-count-10s".into(), 1)].into(),
            ..RateEvidence::default()
        };
        budgets.admit_sapi(cost, order, 130_000).unwrap();
        budgets
            .observe_cost(
                Cost {
                    sapi: Some(cost),
                    ..order
                },
                &evidence,
                140_000,
                200,
            )
            .unwrap();
        assert!(matches!(
            budgets.admit_sapi(cost, order, 140_001),
            Err(Error::Admission { .. })
        ));
    }

    #[test]
    fn uid_weight_and_ip_request_cap_are_independent_and_atomic() {
        let budgets = Budgets::sapi().unwrap();
        let cost = SapiCost {
            endpoint: "/sapi/v1/margin/max-leverage",
            uid: true,
            weight: 3000,
            requests_per_second: None,
            requests_per_minute: Some(1),
        };
        budgets.admit_sapi(cost, Cost::default(), 120_000).unwrap();
        let other_account = budgets.for_account();
        assert!(matches!(
            other_account.admit_sapi(cost, Cost::default(), 120_001),
            Err(Error::Admission { .. })
        ));
        assert!(other_account.account.lock().unwrap().endpoints.is_empty());
        let account = budgets.account.lock().unwrap();
        assert_eq!(account.endpoints[&(cost.endpoint, 60_000)].1, 3000);
        drop(account);
        budgets.admit_sapi(cost, Cost::default(), 180_000).unwrap();
    }
}
impl Budgets {
    pub(crate) fn admit_sapi(&self, cost: SapiCost, attempt: Cost, now: u64) -> Result<(), Error> {
        // Same lock order as aggregate admission. Validate both authorities before
        // charging either; a UID weight reservation cannot bypass an IP request cap.
        let mut ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        let mut account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("SAPI budget poisoned"))?;
        ip.check_cooldown(now)?;
        let placements = attempt
            .orders10
            .max(attempt.orders60)
            .max(attempt.orders_day);
        if placements > 0 && account.sapi_order_windows.is_empty() {
            return Err(Error::Configuration(
                "Margin order quota authority required",
            ));
        }
        for (&window, &(bucket, count, limit)) in &account.sapi_order_windows {
            let count = if bucket == now / window { count } else { 0 };
            if placements > 0 && count.checked_add(placements).is_none_or(|v| v > limit) {
                return Err(Error::Admission {
                    retry_after: Duration::from_millis(window - now % window),
                });
            }
        }
        if let Some(limit) = cost.requests_per_minute {
            if limit == 0 {
                return Err(Error::Configuration("zero SAPI request cap"));
            }
            let (bucket, count) = ip
                .endpoint_ip_requests
                .get(cost.endpoint)
                .copied()
                .unwrap_or_default();
            if bucket == now / 60_000 && count >= limit {
                return Err(Error::Admission {
                    retry_after: Duration::from_millis(60_000 - now % 60_000),
                });
            }
        }
        let state = if cost.uid { &mut account } else { &mut ip };
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
        if cost.requests_per_minute.is_some() {
            let current = ip.endpoint_ip_requests.entry(cost.endpoint).or_default();
            if current.0 != now / 60_000 {
                *current = (now / 60_000, 0);
            }
            current.1 = current
                .1
                .checked_add(1)
                .ok_or(Error::Configuration("SAPI request budget overflow"))?;
        }
        if placements > 0 {
            for (&window, current) in &mut account.sapi_order_windows {
                if current.0 != now / window {
                    current.0 = now / window;
                    current.1 = 0;
                }
                current.1 = current
                    .1
                    .checked_add(placements)
                    .ok_or(Error::Configuration("Margin order budget overflow"))?;
            }
        }
        Ok(())
    }
    /// Install native Margin order quotas only after validating the entire observation.
    pub(crate) fn observe_sapi_order_windows(
        &self,
        windows: &[(u64, u64, u64)],
        now: u64,
    ) -> Result<(), Error> {
        if windows.is_empty()
            || windows
                .iter()
                .any(|(window, _, limit)| *window == 0 || *limit == 0)
        {
            return Err(Error::Gap("missing Margin order quota evidence"));
        }
        let mut account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("account budget poisoned"))?;
        let mut observed = std::collections::BTreeMap::<u64, (u64, u64)>::new();
        for &(window, count, limit) in windows {
            let entry = observed.entry(window).or_insert((count, limit));
            entry.0 = entry.0.max(count);
            entry.1 = entry.1.min(limit);
        }
        for (window, (count, limit)) in observed {
            let previous = account
                .sapi_order_windows
                .get(&window)
                .filter(|(bucket, _, _)| *bucket == now / window)
                .map_or(0, |(_, count, _)| *count);
            account
                .sapi_order_windows
                .insert(window, (now / window, count.max(previous), limit));
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
        let mut account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("account budget poisoned"))?;
        for (name, &count) in &e.counters {
            let Some(interval) = name.strip_prefix("x-mbx-order-count-") else {
                continue;
            };
            let unit = match interval.as_bytes().last() {
                Some(b's') => 1000_u64,
                Some(b'm') => 60_000,
                Some(b'h') => 3_600_000,
                Some(b'd') => 86_400_000,
                _ => continue, // Unknown header evidence remains in ResponseMeta.
            };
            let Some(window) = interval
                .get(..interval.len().saturating_sub(1))
                .and_then(|n| n.parse::<u64>().ok())
                .and_then(|n| n.checked_mul(unit))
                .filter(|window| *window > 0)
            else {
                continue;
            };
            if let Some(current) = account.sapi_order_windows.get_mut(&window) {
                if current.0 != now / window {
                    current.0 = now / window;
                    current.1 = 0;
                }
                current.1 = current.1.max(count);
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
    /// Share IP/UID scopes across every Wallet/Convert/Margin client for this account.
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
