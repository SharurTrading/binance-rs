// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{Error, RateEvidence};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

/// Venue-sourced initial rate limits. Replace with exchangeInfo evidence when available.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct BudgetLimits {
    /// IP request weight per minute.
    pub weight_per_minute: u64,
    /// WebSocket API IP weight per minute, distinct from ordinary REST weight.
    pub ws_weight_per_minute: u64,
    /// Account order count per ten seconds.
    pub orders_per_ten_seconds: u64,
    /// Account order count per minute.
    pub orders_per_minute: u64,
    /// Optional account daily order limit; absent for Futures.
    pub orders_per_day: Option<u64>,
    /// Optional REST raw-request limit per five minutes.
    pub raw_requests_per_five_minutes: Option<u64>,
    /// Optional WebSocket connection-attempt limit per five minutes.
    pub connections_per_five_minutes: Option<u64>,
    /// Whether REST and WebSocket request weight share one IP counter.
    pub shared_request_weight: bool,
}
impl BudgetLimits {
    /// Binance's documented production USDⓈ-M limits (conservative schema baseline).
    #[must_use]
    pub fn usdm() -> Self {
        Self {
            weight_per_minute: 2400,
            ws_weight_per_minute: 2400,
            orders_per_ten_seconds: 300,
            orders_per_minute: 1200,
            orders_per_day: None,
            raw_requests_per_five_minutes: None,
            connections_per_five_minutes: None,
            shared_request_weight: false,
        }
    }
    /// COIN-M's current shared UM/CM limits after the June 2026 integration.
    /// Reuse the same `Budgets` owner for both Futures products on one IP/account.
    #[must_use]
    pub fn coinm() -> Self {
        Self {
            shared_request_weight: true,
            ..Self::usdm()
        }
    }
    /// Spot's documented baseline; replace account limits with exchange evidence.
    /// Sources: Spot WebSocket rate limits and the March 2026 `RAW_REQUESTS` update.
    #[must_use]
    pub fn spot() -> Self {
        Self {
            weight_per_minute: 6000,
            ws_weight_per_minute: 6000,
            orders_per_ten_seconds: 50,
            orders_per_minute: u64::MAX,
            orders_per_day: Some(160_000),
            raw_requests_per_five_minutes: Some(300_000),
            connections_per_five_minutes: Some(300),
            shared_request_weight: true,
        }
    }
    /// Set a venue-reported IP minute weight limit.
    #[must_use]
    pub fn weight_per_minute(mut self, value: u64) -> Self {
        self.weight_per_minute = value;
        self
    }
    /// Set a venue-reported WebSocket API minute weight limit.
    #[must_use]
    pub fn ws_weight_per_minute(mut self, value: u64) -> Self {
        self.ws_weight_per_minute = value;
        self
    }
    /// Set venue-reported account order limits.
    #[must_use]
    pub fn orders(mut self, ten_seconds: u64, minute: u64) -> Self {
        self.orders_per_ten_seconds = ten_seconds;
        self.orders_per_minute = minute;
        self
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Cost {
    pub sapi: Option<super::sapi::SapiCost>,
    pub weight: u64,
    pub orders10: u64,
    pub orders60: u64,
    pub orders_day: u64,
    pub raw_requests: u64,
    pub connections: u64,
    pub ws_weight: u64,
    pub funding: bool,
    pub history: bool,
    pub quote: bool,
    pub download: u8,
}

#[derive(Default)]
pub(super) struct State {
    // Each counter is re-derivable: release it at the end of its aligned venue interval.
    counts: BTreeMap<&'static str, (u64, u64)>,
    cooldown: u64,
    observed_weight: (u64, u64),
    pub(super) endpoints: BTreeMap<(&'static str, u64), (u64, u64)>,
    pub(super) endpoint_cooldown: BTreeMap<&'static str, u64>,
}

/// Shared venue budgets. Clones share both IP and account evidence.
///
/// Keep one IP owner per venue budget scope and derive account owners with
/// `for_account`; constructing independent budgets on the same IP undercounts traffic.
#[derive(Clone)]
pub struct Budgets {
    pub(super) ip: Arc<Mutex<State>>,
    ws_ip: Arc<Mutex<State>>,
    pub(super) account: Arc<Mutex<State>>,
    limits: Arc<BudgetLimits>,
}
impl std::fmt::Debug for Budgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Budgets(shared IP/account)")
    }
}
impl Budgets {
    /// Construct an explicit venue budget scope.
    ///
    /// # Errors
    /// Refuses zero budgets.
    pub fn new(limits: BudgetLimits) -> Result<Self, Error> {
        if limits.ws_weight_per_minute == 0
            || limits.weight_per_minute == 0
            || limits.orders_per_ten_seconds == 0
            || limits.orders_per_minute == 0
            || limits.orders_per_day == Some(0)
            || limits.raw_requests_per_five_minutes == Some(0)
            || limits.connections_per_five_minutes == Some(0)
        {
            return Err(Error::Configuration("zero rate budget"));
        }
        Ok(Self {
            ip: Arc::default(),
            ws_ip: Arc::default(),
            account: Arc::default(),
            limits: Arc::new(limits),
        })
    }
    /// A distinct account sharing this IP budget. Reuse this returned owner for
    /// every client and credential that accesses that same account.
    #[must_use]
    pub fn for_account(&self) -> Self {
        Self {
            ip: self.ip.clone(),
            ws_ip: self.ws_ip.clone(),
            account: Arc::default(),
            limits: self.limits.clone(),
        }
    }
    fn ip_cost(&self, c: Cost) -> Vec<(&'static str, u64, u64, u64)> {
        let weight = if self.limits.shared_request_weight {
            c.weight.max(c.ws_weight)
        } else {
            c.weight
        };
        let mut ip_cost = vec![("weight", 60_000, self.limits.weight_per_minute, weight)];
        if let Some(limit) = self.limits.raw_requests_per_five_minutes {
            ip_cost.push(("raw", 300_000, limit, c.raw_requests));
        }
        if let Some(limit) = self.limits.connections_per_five_minutes {
            ip_cost.push(("connections", 300_000, limit, c.connections));
        }
        if c.funding {
            ip_cost.push(("funding", 300_000, 500, 1));
        }
        if c.history {
            ip_cost.push(("history", 300_000, 1000, 1));
        }
        ip_cost
    }
    pub(crate) fn admit(&self, c: Cost, now: u64) -> Result<(), Error> {
        if let Some(cost) = c.sapi {
            return self.admit_sapi(cost, now);
        }
        let mut ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        let mut ws_ip = self
            .ws_ip
            .lock()
            .map_err(|_| Error::Configuration("WS IP budget poisoned"))?;
        let mut account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("account budget poisoned"))?;
        if now < ip.cooldown {
            return Err(Error::Admission {
                retry_after: Duration::from_millis(ip.cooldown - now),
            });
        }
        let ip_cost = self.ip_cost(c);
        let ws_cost = vec![(
            "weight",
            60_000,
            self.limits.ws_weight_per_minute,
            if self.limits.shared_request_weight {
                0
            } else {
                c.ws_weight
            },
        )];
        let mut account_cost = vec![
            (
                "orders10",
                10_000,
                self.limits.orders_per_ten_seconds,
                c.orders10,
            ),
            (
                "orders60",
                60_000,
                self.limits.orders_per_minute,
                c.orders60,
            ),
        ];
        if let Some(limit) = self.limits.orders_per_day {
            account_cost.push(("ordersDay", 86_400_000, limit, c.orders_day));
        }
        if c.quote {
            account_cost.extend([
                ("quoteHour", 3_600_000, 360, 1),
                ("quoteDay", 86_400_000, 500, 1),
            ]);
        }
        let monthly = monthly_download(&account, c.download, now)?;
        for (state, costs) in [
            (&ip, &ip_cost),
            (&ws_ip, &ws_cost),
            (&account, &account_cost),
        ] {
            for &(name, window, limit, amount) in costs {
                let (start, count) = state.counts.get(name).copied().unwrap_or_default();
                let bucket = now / window;
                let count = if start == bucket { count } else { 0 };
                if count.checked_add(amount).is_none_or(|v| v > limit) {
                    return Err(Error::Admission {
                        retry_after: Duration::from_millis(window - now % window),
                    });
                }
            }
        }
        for (state, costs) in [
            (&mut ip, &ip_cost),
            (&mut ws_ip, &ws_cost),
            (&mut account, &account_cost),
        ] {
            for &(name, window, _, amount) in costs {
                let counter = state.counts.entry(name).or_default();
                if counter.0 != now / window {
                    *counter = (now / window, 0);
                }
                counter.1 = counter
                    .1
                    .checked_add(amount)
                    .ok_or(Error::Configuration("budget overflow"))?;
            }
        }
        if let Some((key, bucket)) = monthly {
            let current = account.counts.entry(key).or_default();
            if current.0 != bucket {
                *current = (bucket, 0);
            }
            current.1 = current
                .1
                .checked_add(1)
                .ok_or(Error::Configuration("budget overflow"))?;
        }
        Ok(())
    }
    // Refund only venue-documented successful zero-weight operations, never below evidence.
    pub(crate) fn refund_weight(
        &self,
        amount: u64,
        now: u64,
        websocket: bool,
    ) -> Result<(), Error> {
        let owner = if websocket && !self.limits.shared_request_weight {
            &self.ws_ip
        } else {
            &self.ip
        };
        let mut state = owner
            .lock()
            .map_err(|_| Error::Configuration("weight owner poisoned"))?;
        let bucket = now / 60_000;
        let floor = if state.observed_weight.0 == bucket {
            state.observed_weight.1
        } else {
            0
        };
        if let Some(current) = state.counts.get_mut("weight")
            && current.0 == bucket
        {
            current.1 = current.1.saturating_sub(amount).max(floor);
        }
        Ok(())
    }
    pub(crate) fn observe(&self, e: &RateEvidence, now: u64, websocket: bool) -> Result<(), Error> {
        let mut ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        let mut ws_ip = self
            .ws_ip
            .lock()
            .map_err(|_| Error::Configuration("WS IP budget poisoned"))?;
        let mut account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("account budget poisoned"))?;
        if let Some(delay) = e.retry_after {
            let delay = u64::try_from(delay.as_millis())
                .map_err(|_| Error::Configuration("cooldown overflow"))?;
            ip.cooldown = ip.cooldown.max(
                now.checked_add(delay)
                    .ok_or(Error::Configuration("cooldown overflow"))?,
            );
        }
        for (name, count) in &e.counters {
            let (state, key, window) = match name.as_str() {
                "x-mbx-used-weight-1m" => (
                    if websocket && !self.limits.shared_request_weight {
                        &mut ws_ip
                    } else {
                        &mut ip
                    },
                    "weight",
                    60_000,
                ),
                "x-mbx-order-count-10s" => (&mut account, "orders10", 10_000),
                "x-mbx-order-count-1m" => (&mut account, "orders60", 60_000),
                "x-mbx-order-count-1d" => (&mut account, "ordersDay", 86_400_000),
                _ => continue,
            };
            if key == "weight" {
                if state.observed_weight.0 != now / window {
                    state.observed_weight = (now / window, 0);
                }
                state.observed_weight.1 = state.observed_weight.1.max(*count);
            }
            let current = state.counts.entry(key).or_default();
            if current.0 != now / window {
                *current = (now / window, 0);
            }
            current.1 = current.1.max(*count);
        }
        Ok(())
    }
}

fn monthly_download(
    account: &State,
    kind: u8,
    now: u64,
) -> Result<Option<(&'static str, u64)>, Error> {
    if kind == 0 {
        return Ok(None);
    }
    let seconds =
        i64::try_from(now / 1000).map_err(|_| Error::Configuration("calendar timestamp"))?;
    let dt = time::OffsetDateTime::from_unix_timestamp(seconds)
        .map_err(|_| Error::Configuration("calendar timestamp"))?;
    let bucket = u64::try_from(dt.year())
        .map_err(|_| Error::Configuration("calendar year"))?
        .checked_mul(12)
        .and_then(|v| v.checked_add(u64::from(dt.month() as u8)))
        .ok_or(Error::Configuration("calendar overflow"))?;
    let key = match kind {
        1 => "downloadOrders",
        2 => "downloadTrades",
        3 => "downloadIncome",
        4 => "downloadCMOrders",
        5 => "downloadCMTrades",
        _ => "downloadCMIncome",
    };
    let limit = match kind {
        1 => 10,
        4..=6 => 8,
        _ => 5,
    };
    let current = account.counts.get(key).copied().unwrap_or_default();
    if current.0 == bucket && current.1 >= limit {
        let (year, month) = if dt.month() == time::Month::December {
            (
                dt.year()
                    .checked_add(1)
                    .ok_or(Error::Configuration("calendar overflow"))?,
                time::Month::January,
            )
        } else {
            (dt.year(), dt.month().next())
        };
        let next = time::Date::from_calendar_date(year, month, 1)
            .map_err(|_| Error::Configuration("calendar"))?
            .midnight()
            .assume_utc()
            .unix_timestamp();
        let delay = u64::try_from(
            next.checked_sub(seconds)
                .ok_or(Error::Configuration("calendar overflow"))?,
        )
        .map_err(|_| Error::Configuration("calendar overflow"))?;
        return Err(Error::Admission {
            retry_after: Duration::from_secs(delay),
        });
    }
    Ok(Some((key, bucket)))
}

#[cfg(test)]
mod capacity_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rest_ws_ip_scopes_are_distinct_but_account_orders_are_shared() {
        let budget = Budgets::new(
            BudgetLimits::usdm()
                .weight_per_minute(1)
                .ws_weight_per_minute(1)
                .orders(1, 1),
        )
        .unwrap();
        budget
            .admit(
                Cost {
                    weight: 1,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        budget
            .admit(
                Cost {
                    ws_weight: 1,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        assert!(
            budget
                .admit(
                    Cost {
                        weight: 1,
                        ..Cost::default()
                    },
                    1000
                )
                .is_err()
        );
        // Refused combined admission consumes no account order slot.
        assert!(
            budget
                .admit(
                    Cost {
                        weight: 1,
                        orders10: 1,
                        orders60: 1,
                        ..Cost::default()
                    },
                    1000
                )
                .is_err()
        );
        budget
            .admit(
                Cost {
                    orders10: 1,
                    orders60: 1,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        assert!(
            budget
                .clone()
                .admit(
                    Cost {
                        orders10: 1,
                        ..Cost::default()
                    },
                    1000
                )
                .is_err()
        );
        budget
            .for_account()
            .admit(
                Cost {
                    orders10: 1,
                    orders60: 1,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        budget
            .admit(
                Cost {
                    weight: 1,
                    ws_weight: 1,
                    orders10: 1,
                    orders60: 1,
                    ..Cost::default()
                },
                60_000,
            )
            .unwrap();
    }

    #[test]
    fn monthly_download_jobs_reset_at_calendar_month_not_elapsed_30_days() {
        let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
        let january = u64::try_from(
            time::Date::from_calendar_date(2026, time::Month::January, 31)
                .unwrap()
                .midnight()
                .assume_utc()
                .unix_timestamp(),
        )
        .unwrap()
            * 1000;
        let february = january + 86_400_000;
        for _ in 0..10 {
            budgets
                .admit(
                    Cost {
                        download: 1,
                        ..Cost::default()
                    },
                    january,
                )
                .unwrap();
        }
        assert!(
            matches!(budgets.admit(Cost {download:1,..Cost::default()},january),Err(Error::Admission {retry_after}) if retry_after==Duration::from_hours(24))
        );
        budgets
            .admit(
                Cost {
                    download: 1,
                    ..Cost::default()
                },
                february,
            )
            .unwrap();
        for _ in 0..5 {
            budgets
                .admit(
                    Cost {
                        download: 2,
                        ..Cost::default()
                    },
                    february,
                )
                .unwrap();
        }
        assert!(
            budgets
                .admit(
                    Cost {
                        download: 2,
                        ..Cost::default()
                    },
                    february
                )
                .is_err()
        );
        budgets
            .admit(
                Cost {
                    download: 3,
                    ..Cost::default()
                },
                february,
            )
            .unwrap();
    }

    #[test]
    fn funding_history_and_conversion_apply_their_additional_venue_limits() {
        let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
        for _ in 0..500 {
            budgets
                .admit(
                    Cost {
                        funding: true,
                        ..Cost::default()
                    },
                    1000,
                )
                .unwrap();
        }
        assert!(
            budgets
                .admit(
                    Cost {
                        funding: true,
                        ..Cost::default()
                    },
                    1000
                )
                .is_err()
        );
        budgets
            .admit(
                Cost {
                    funding: true,
                    ..Cost::default()
                },
                300_000,
            )
            .unwrap();
        for _ in 0..360 {
            budgets
                .admit(
                    Cost {
                        quote: true,
                        ..Cost::default()
                    },
                    1000,
                )
                .unwrap();
        }
        assert!(
            budgets
                .admit(
                    Cost {
                        quote: true,
                        ..Cost::default()
                    },
                    1000
                )
                .is_err()
        );
        budgets
            .admit(
                Cost {
                    quote: true,
                    ..Cost::default()
                },
                3_600_000,
            )
            .unwrap();
    }
}
