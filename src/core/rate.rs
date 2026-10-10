// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{Error, RateEvidence};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

/// Venue-sourced initial rate limits; stated exchange information limits replace the
/// minute weight and raw-request figures.
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
    /// COIN-M's current shared UM/CM limits after the June 2026 integration; the
    /// futures pool every USDⓈ-M and COIN-M `Config::new` draws on starts here.
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
impl Cost {
    /// The weight this cost charges a pool's minute weight window.
    ///
    /// REST requests carry their weight in `weight`; WebSocket API requests in
    /// `ws_weight`, which a pool counting both together charges the same window.
    pub(crate) fn request_weight(&self) -> u64 {
        self.weight.max(self.ws_weight)
    }
}

#[derive(Default)]
pub(super) struct State {
    // Each counter is re-derivable: release it at the end of its aligned venue interval.
    counts: BTreeMap<&'static str, (u64, u64)>,
    pub(super) cooldown: u64,
    pub(super) cooldown_timing_unknown: bool,
    observed_weight: (u64, u64),
    order_windows: BTreeMap<u64, (u64, u64, u64)>,
    pub(super) endpoints: BTreeMap<(&'static str, u64), (u64, u64)>,
    pub(super) endpoint_cooldown: BTreeMap<&'static str, u64>,
    // The latest limits the venue stated for this pool; the baseline stands until then.
    stated_weight_per_minute: Option<u64>,
    stated_raw_requests_per_five_minutes: Option<u64>,
}
impl State {
    /// The count of `name` in the window of `window` milliseconds holding `now`.
    fn current(&self, name: &str, window: u64, now: u64) -> u64 {
        match self.counts.get(name) {
            Some(&(bucket, count)) if bucket == now / window => count,
            _ => 0,
        }
    }
    pub(super) fn check_cooldown(&self, now: u64) -> Result<(), Error> {
        if self.cooldown_timing_unknown {
            return Err(Error::CooldownTimingUnknown);
        }
        if now < self.cooldown {
            return Err(Error::Admission {
                retry_after: Duration::from_millis(self.cooldown - now),
            });
        }
        Ok(())
    }
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
    fn ip_cost(&self, ip: &State, c: Cost) -> Vec<(&'static str, u64, u64, u64)> {
        let weight = if self.limits.shared_request_weight {
            c.request_weight()
        } else {
            c.weight
        };
        let (weight_limit, _) = self.weight_limit(ip);
        let mut ip_cost = vec![("weight", 60_000, weight_limit, weight)];
        if let Some((limit, _)) = self.raw_request_limit(ip) {
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
        ip.check_cooldown(now)?;
        let ip_cost = self.ip_cost(&ip, c);
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
        let placements = c.orders10.max(c.orders60).max(c.orders_day);
        order_windows(&mut account, placements, now, false)?;
        let monthly = monthly_download(&account, c.download, now)?;
        for (state, costs) in [
            (&ip, &ip_cost),
            (&ws_ip, &ws_cost),
            (&account, &account_cost),
        ] {
            for &(name, window, limit, amount) in costs {
                let count = state.current(name, window, now);
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
        order_windows(&mut account, placements, now, true)?;
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
    // FIX LimitResponse uses explicit interval/count/max, independently of HTTP headers.
    // Hold a counter floor for a full interval after observation: no reset origin is guessed.
    pub(crate) fn observe_order_window(
        &self,
        window: u64,
        count: u64,
        limit: u64,
        now: u64,
    ) -> Result<(), Error> {
        if window == 0 {
            return Err(Error::Gap("FIX order limit interval missing"));
        }
        let until = now
            .checked_add(window)
            .ok_or(Error::Configuration("FIX order limit overflow"))?;
        let mut account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("account budget poisoned"))?;
        let previous = account
            .order_windows
            .get(&window)
            .filter(|(expiry, _, _)| *expiry > now)
            .map_or(0, |(_, count, _)| *count);
        account
            .order_windows
            .insert(window, (until, count.max(previous), limit));
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
    pub(crate) fn observe_ban(&self, status: u16, evidence: &RateEvidence) -> Result<(), Error> {
        // A ban, or any venue retry timing this client cannot use, leaves the owner
        // without an expiry. Refuse every send rather than invent one.
        if evidence.retry_after.is_none() && (status == 418 || evidence.retry_after_unusable) {
            self.ip
                .lock()
                .map_err(|_| Error::Configuration("IP budget poisoned"))?
                .cooldown_timing_unknown = true;
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
            // Venue retry timing is never dropped and never narrowed. A delay or deadline
            // beyond the representable range saturates at the widest one, which refuses
            // longer than the venue asked; an error here would discard an answer the
            // venue has already sent.
            let delay = u64::try_from(delay.as_nanos().div_ceil(1_000_000)).unwrap_or(u64::MAX);
            ip.cooldown = ip.cooldown.max(now.saturating_add(delay));
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

/// One `rateLimits` entry of a venue's exchange information, as stated.
#[derive(Clone, Copy, Debug)]
pub(crate) struct StatedLimit<'a> {
    pub kind: Option<&'a str>,
    pub interval: Option<&'a str>,
    pub interval_num: Option<i64>,
    pub limit: Option<i64>,
}

impl Budgets {
    /// Adopt the minute request weight and five-minute raw-request limits a venue
    /// reply states, for every client of this IP owner. `ORDERS` entries are account
    /// limits and other windows are not counted here; neither is adopted.
    ///
    /// # Errors
    /// A counted window stated without a positive limit is refused as a gap and
    /// leaves the owner's limits unchanged.
    pub(crate) fn adopt_stated<'a>(
        &self,
        stated: impl IntoIterator<Item = StatedLimit<'a>>,
    ) -> Result<(), Error> {
        let mut weight = None;
        let mut raw = None;
        for entry in stated {
            let slot = match (entry.kind, entry.interval, entry.interval_num) {
                (Some("REQUEST_WEIGHT"), Some("MINUTE"), Some(1)) => &mut weight,
                (Some("RAW_REQUESTS"), Some("MINUTE"), Some(5)) => &mut raw,
                _ => continue,
            };
            let limit = entry
                .limit
                .and_then(|limit| u64::try_from(limit).ok())
                .filter(|limit| *limit > 0)
                .ok_or(Error::Gap("stated rate limit without a positive limit"))?;
            // Two statements of one window: the stricter binds.
            *slot = Some(slot.map_or(limit, |current: u64| current.min(limit)));
        }
        let mut ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        if weight.is_some() {
            ip.stated_weight_per_minute = weight;
        }
        if raw.is_some() {
            ip.stated_raw_requests_per_five_minutes = raw;
        }
        Ok(())
    }
}

/// The venue IP limit a product's requests count against.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum VenuePool {
    /// Spot's own IP limits.
    Spot,
    /// USDⓈ-M and COIN-M, which share one IP limit since the UM/CM integration.
    Futures,
}

/// The environment a pool counts; demo and production never share a pool.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PoolEnvironment {
    /// The venue's demo environment.
    Demo,
    /// The venue's production environment.
    Production,
}

/// One IP weight pool per venue pool and environment: Spot's own, and USDⓈ-M with
/// COIN-M together. `Config::new` draws on the process's registry; a registry built
/// here is independent of it and of every other.
#[derive(Default)]
pub struct WeightPools {
    pools: Mutex<BTreeMap<(VenuePool, PoolEnvironment), Budgets>>,
}
impl std::fmt::Debug for WeightPools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("WeightPools")
    }
}
impl WeightPools {
    /// An empty registry, independent of the process's.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn process() -> &'static Self {
        static PROCESS: OnceLock<WeightPools> = OnceLock::new();
        PROCESS.get_or_init(Self::new)
    }
    /// A budget counting IP traffic against `pool` in `environment`, with its own
    /// account owner. The pool starts at the venue's documented baseline.
    pub(crate) fn draw(
        &self,
        pool: VenuePool,
        environment: PoolEnvironment,
    ) -> Result<Budgets, Error> {
        let mut pools = self
            .pools
            .lock()
            .map_err(|_| Error::Configuration("weight pools poisoned"))?;
        if let Some(owner) = pools.get(&(pool, environment)) {
            return Ok(owner.for_account());
        }
        let owner = Budgets::new(match pool {
            VenuePool::Spot => BudgetLimits::spot(),
            VenuePool::Futures => BudgetLimits::coinm(),
        })?;
        let drawn = owner.for_account();
        pools.insert((pool, environment), owner);
        Ok(drawn)
    }
}

fn order_windows(account: &mut State, amount: u64, now: u64, apply: bool) -> Result<(), Error> {
    for (&window, counter) in &mut account.order_windows {
        if counter.0 <= now {
            *counter = (
                now.checked_add(window)
                    .ok_or(Error::Configuration("order window overflow"))?,
                0,
                counter.2,
            );
        }
        let next = counter
            .1
            .checked_add(amount)
            .ok_or(Error::Configuration("order window overflow"))?;
        if next > counter.2 && amount != 0 {
            return Err(Error::Admission {
                retry_after: Duration::from_millis(counter.0 - now),
            });
        }
        if apply {
            counter.1 = next;
        }
    }
    Ok(())
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

mod usage;
pub use usage::{LimitSource, PoolUsage, WindowUsage};

#[cfg(test)]
mod capacity_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_order_floor_is_shared_and_does_not_guess_the_venue_reset_origin() {
        let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
        budgets.observe_order_window(10_000, 2, 2, 9000).unwrap();
        let order = Cost {
            orders10: 1,
            orders60: 1,
            orders_day: 1,
            ..Cost::default()
        };
        assert!(matches!(
            budgets.clone().admit(order, 10_001),
            Err(Error::Admission { .. })
        ));
        budgets.admit(order, 19_000).unwrap();
        budgets.admit(order, 19_000).unwrap();
        assert!(matches!(
            budgets.admit(order, 19_000),
            Err(Error::Admission { .. })
        ));
    }

    #[test]
    fn sub_millisecond_cooldown_cannot_authorize_an_early_attempt() {
        let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
        let evidence = RateEvidence {
            retry_after: Some(Duration::from_micros(1)),
            ..RateEvidence::default()
        };
        budgets.observe(&evidence, 1000, true).unwrap();
        assert!(matches!(
            budgets.admit(Cost::default(), 1000),
            Err(Error::Admission { .. })
        ));
        budgets.admit(Cost::default(), 1001).unwrap();
    }

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
