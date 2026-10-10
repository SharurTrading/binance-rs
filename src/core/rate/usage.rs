// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! What an IP pool reports of itself: each window's limit and what it has spent.
//!
//! The report reads the same state admission reads; it neither paces nor admits.

use super::{Budgets, State};
use crate::core::Error;
use std::time::Duration;

/// Where a window's limit comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LimitSource {
    /// The venue's documented figure, which stands until an exchange information
    /// read by a client of the pool states one.
    Documented,
    /// The latest exchange information `rateLimits` entry a client of the pool read.
    Stated,
}

/// One counted IP window of a pool, as it stood when read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct WindowUsage {
    /// The window's length; windows align to the epoch, as the venue's do.
    pub interval: Duration,
    /// The most the window admits.
    pub limit: u64,
    /// Whether the venue stated `limit` or it is the documented figure.
    pub source: LimitSource,
    /// Spent in the current window: the pool's own count of admitted requests,
    /// raised to the venue's usage header whenever a reply reported more.
    pub used: u64,
    /// Time until the current window ends and its count starts again from zero.
    pub resets_in: Duration,
}

/// The IP windows of the pool a client draws on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PoolUsage {
    /// `REQUEST_WEIGHT` per minute. On a pool that counts REST and WebSocket API
    /// weight together, which every pool drawn from [`crate::WeightPools`] does,
    /// it counts both.
    pub request_weight: WindowUsage,
    /// `RAW_REQUESTS` per five minutes, where the pool counts them (Spot).
    pub raw_requests: Option<WindowUsage>,
}

impl Budgets {
    /// The pool's minute weight limit: the venue's latest statement, or the
    /// documented figure until one is read. Admission and the report both use it.
    pub(super) fn weight_limit(&self, ip: &State) -> (u64, LimitSource) {
        match ip.stated_weight_per_minute {
            Some(limit) => (limit, LimitSource::Stated),
            None => (self.limits.weight_per_minute, LimitSource::Documented),
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
        let window = |name: &str, window: u64, (limit, source): (u64, LimitSource)| WindowUsage {
            interval: Duration::from_millis(window),
            limit,
            source,
            used: ip.current(name, window, now),
            resets_in: Duration::from_millis(window - now % window),
        };
        Ok(PoolUsage {
            request_weight: window("weight", 60_000, self.weight_limit(&ip)),
            raw_requests: self
                .raw_request_limit(&ip)
                .map(|limit| window("raw", 300_000, limit)),
        })
    }
}
