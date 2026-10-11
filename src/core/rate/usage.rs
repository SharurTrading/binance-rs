// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Pool and account window limits and counts, read without admitting work.
//!
//! The report reads the same state admission reads; it neither paces nor admits.

use super::{Budgets, PoolKey, State};
use crate::core::Error;
use std::time::Duration;

/// Where a window's limit comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LimitSource {
    /// The venue's documented figure, which stands until an applicable
    /// venue statement is read.
    Documented,
    /// The strictest applicable venue statement: an exchange
    /// information `rateLimits` entry, a WebSocket reply's `rateLimits`, or an
    /// account order query.
    Stated,
}

/// One counted pool or account window, as it stood when read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct WindowUsage {
    /// The window's length; windows align to the epoch, as the venue's do.
    pub interval: Duration,
    /// The most the window admits.
    pub limit: u64,
    /// Whether the venue stated `limit` or it is the documented figure.
    pub source: LimitSource,
    /// Spent in the current window: the owner's count of admitted requests/orders,
    /// raised to the venue's usage evidence — `X-MBX-USED-WEIGHT-1M`, or a
    /// WebSocket reply's `rateLimits` count for the separate window — whenever
    /// a reply reported more. Account counts also follow `X-MBX-ORDER-COUNT-*`
    /// and account order queries, and are never reduced by lower evidence.
    pub used: u64,
    /// Time until the current window ends and its count starts again from zero.
    pub resets_in: Duration,
}

/// The IP windows of the pool a client draws on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PoolUsage {
    /// REST `REQUEST_WEIGHT` per minute. Spot and Options count WebSocket API
    /// weight here too; the futures pool counts COIN-M WebSocket API weight here,
    /// while USDⓈ-M WebSocket API weight has a counter of its own, reported as
    /// [`Self::ws_request_weight`].
    pub request_weight: WindowUsage,
    /// `RAW_REQUESTS` per five minutes, where the pool counts them (Spot).
    pub raw_requests: Option<WindowUsage>,
    /// The USDⓈ-M WebSocket API's own `REQUEST_WEIGHT` minute window, where the
    /// pool counts one apart from REST weight (the futures pool, and explicit
    /// owners built from `BudgetLimits::usdm`); `None` where the pool counts
    /// WebSocket API weight together with REST, in which `request_weight`
    /// already includes it.
    pub ws_request_weight: Option<WindowUsage>,
    /// The venue pool read; `None` for an explicit [`Budgets`] owner.
    pub key: Option<PoolKey>,
}

/// The order windows of this client's account owner at its clock.
///
/// Exchange information states common pool limits. API responses and order
/// queries state this account owner's limits. Where both state a limit, the
/// stricter binds. Usage belongs only to this owner (shared clones or
/// configurations using the same account key).
/// FIX's observation-relative windows and Margin quotas have separate semantics
/// and are not included in this aligned Spot/Futures/Options report.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AccountUsage {
    /// Counted `ORDERS` windows in increasing interval order. Undocumented
    /// windows without a venue statement are omitted, rather than reported as
    /// an invented ceiling. Counts reset on epoch-aligned window boundaries.
    pub orders: Vec<WindowUsage>,
}

impl Budgets {
    /// The same effective account limits used by admission and reporting.
    pub(super) fn order_limits(&self, ip: &State, account: &State) -> [(u64, LimitSource); 3] {
        let limit = |pool: Option<u64>, owner: Option<u64>, documented: u64| match (pool, owner) {
            (Some(pool), Some(owner)) => (pool.min(owner), LimitSource::Stated),
            (Some(value), None) | (None, Some(value)) => (value, LimitSource::Stated),
            (None, None) => (documented, LimitSource::Documented),
        };
        [
            limit(
                ip.stated_orders_per_ten_seconds,
                account.stated_orders_per_ten_seconds,
                self.limits.orders_per_ten_seconds,
            ),
            limit(
                ip.stated_orders_per_minute,
                account.stated_orders_per_minute,
                self.limits.orders_per_minute,
            ),
            limit(
                ip.stated_orders_per_day,
                account.stated_orders_per_day,
                self.limits.orders_per_day.unwrap_or(u64::MAX),
            ),
        ]
    }

    /// Read this account's aligned order windows without modifying counters.
    pub(crate) fn account_usage(&self, now: u64) -> Result<AccountUsage, Error> {
        let ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        let account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("account budget poisoned"))?;
        let orders = [
            ("orders10", 10_000),
            ("orders60", 60_000),
            ("ordersDay", 86_400_000),
        ]
        .into_iter()
        .zip(self.order_limits(&ip, &account))
        .filter(|(_, (limit, source))| *limit != u64::MAX || *source == LimitSource::Stated)
        .map(|((name, window), (limit, source))| WindowUsage {
            interval: Duration::from_millis(window),
            limit,
            source,
            used: account.current(name, window, now),
            resets_in: Duration::from_millis(window - now % window),
        })
        .collect();
        Ok(AccountUsage { orders })
    }

    /// The pool's minute weight limit: the venue's strictest statement, or the
    /// documented figure until one is read. Admission and the report both use it.
    pub(super) fn weight_limit(&self, ip: &State) -> (u64, LimitSource) {
        match ip.stated_weight_per_minute {
            Some(limit) => (limit, LimitSource::Stated),
            None => (self.limits.weight_per_minute, LimitSource::Documented),
        }
    }

    /// The pool's WebSocket API minute weight limit, where it counts one apart
    /// from REST: the venue's strictest statement in a WebSocket reply's
    /// `rateLimits`, or the documented figure until one is read. Admission and
    /// the report both use it.
    pub(super) fn ws_weight_limit(&self, ws: &State) -> (u64, LimitSource) {
        match ws.stated_weight_per_minute {
            Some(limit) => (limit, LimitSource::Stated),
            None => (self.limits.ws_weight_per_minute, LimitSource::Documented),
        }
    }

    /// The pool's five-minute raw-request limit, where it counts one.
    pub(super) fn raw_request_limit(&self, ip: &State) -> Option<(u64, LimitSource)> {
        match (
            ip.stated_raw_requests_per_five_minutes,
            self.limits.raw_requests_per_five_minutes,
        ) {
            (Some(limit), _) => Some((limit, LimitSource::Stated)),
            (None, Some(limit)) => Some((limit, LimitSource::Documented)),
            (None, None) => None,
        }
    }

    /// The pool's IP windows at `now` (Unix milliseconds).
    ///
    /// # Errors
    /// Returns a configuration error if the pool's lock is poisoned.
    pub(crate) fn usage(&self, now: u64) -> Result<PoolUsage, Error> {
        let ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        let ws_ip = self
            .ws_ip
            .lock()
            .map_err(|_| Error::Configuration("WS IP budget poisoned"))?;
        let window = |state: &State,
                      name: &str,
                      window: u64,
                      (limit, source): (u64, LimitSource)| WindowUsage {
            interval: Duration::from_millis(window),
            limit,
            source,
            used: state.current(name, window, now),
            resets_in: Duration::from_millis(window - now % window),
        };
        Ok(PoolUsage {
            request_weight: window(&ip, "weight", 60_000, self.weight_limit(&ip)),
            raw_requests: self
                .raw_request_limit(&ip)
                .map(|limit| window(&ip, "raw", 300_000, limit)),
            ws_request_weight: (!self.limits.shared_request_weight)
                .then(|| window(&ws_ip, "weight", 60_000, self.ws_weight_limit(&ws_ip))),
            key: self.key,
        })
    }
}
