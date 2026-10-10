// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{CompId, Role};
use crate::{AccountKey, Budgets, Error, PoolEnvironment, WeightPools, spot::Environment};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::Instant;

/// The account scope shared by every FIX session of one account: its FIX
/// connection limits, and the Spot account owner its orders count against.
///
/// Binance counts a Spot account's unfilled orders across all API keys and APIs, FIX
/// included, so the Spot owner should be the one the account's REST and WebSocket API
/// clients use. [`Self::new`] takes that owner explicitly;
/// [`Self::with_pools_for_account`] draws it for an [`AccountKey`], the owner every
/// Spot configuration given that key shares. Clone a scope to share it.
#[derive(Clone)]
pub struct AccountBudgets {
    pub(super) spot: Budgets,
    state: Arc<Mutex<State>>,
}
impl std::fmt::Debug for AccountBudgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccountBudgets(shared FIX/Spot)")
    }
}
#[derive(Default)]
struct State {
    attempts: BTreeMap<u8, VecDeque<Instant>>,
    leases: BTreeMap<u64, Slot>,
    next_lease: u64,
}
struct Slot {
    component: String,
    role: u8,
    closed: Option<Instant>,
    component_until: Option<Instant>,
}
pub(super) struct Lease {
    owner: AccountBudgets,
    ticket: u64,
    heartbeat: Duration,
}
impl AccountBudgets {
    /// Create one explicit FIX account scope using the account's existing Spot budget.
    #[must_use]
    pub fn new(spot: Budgets) -> Self {
        Self {
            spot,
            state: Arc::default(),
        }
    }
    /// The FIX account scope the process's registry keeps for `account` in the
    /// Spot pool of `environment`; see [`Self::with_pools_for_account`].
    ///
    /// # Errors
    /// Returns a configuration error if the registry's lock is poisoned.
    pub fn new_for_account(environment: Environment, account: &AccountKey) -> Result<Self, Error> {
        Self::with_pools_for_account(environment, WeightPools::process(), account)
    }
    /// The FIX account scope `pools` keeps for `account` in the Spot pool of
    /// `environment`.
    ///
    /// Its orders count against the account owner that
    /// [`spot::Config::with_pools_for_account`](crate::spot::Config::with_pools_for_account)
    /// draws for the same registry, environment and key: FIX, REST and WebSocket API
    /// orders share one count, and `LimitResponse` order limits a session observes
    /// hold the key's Spot clients too. It keeps the pool's shared IP state. Every
    /// scope drawn for the key shares one set of FIX connection limits. Choose the
    /// environment of the endpoint the sessions connect to.
    ///
    /// # Errors
    /// Returns a configuration error if the registry's lock is poisoned.
    pub fn with_pools_for_account(
        environment: Environment,
        pools: &WeightPools,
        account: &AccountKey,
    ) -> Result<Self, Error> {
        let pool = match environment {
            Environment::Demo => PoolEnvironment::Demo,
            Environment::Production => PoolEnvironment::Production,
        };
        pools.draw_fix(pool, account)
    }
    pub(super) fn connect(
        &self,
        role: Role,
        component: &CompId,
        heartbeat: Duration,
    ) -> Result<Lease, Error> {
        let key = role_key(role);
        let (attempts, interval, concurrent) = match role {
            Role::OrderEntry | Role::DropCopy => (15, Duration::from_secs(30), 10),
            Role::MarketData => (300, Duration::from_mins(5), 100),
        };
        let now = Instant::now();
        let mut state = self
            .state
            .lock()
            .map_err(|_| Error::Configuration("FIX account budget poisoned"))?;
        state.leases.retain(|_, slot| {
            slot.closed.is_none_or(|until| until > now)
                || slot.component_until.is_some_and(|until| until > now)
        });
        if state.leases.values().any(|slot| {
            slot.component == component.as_str()
                && (slot.closed.is_none() || slot.component_until.is_some_and(|until| until > now))
        }) {
            return Err(Error::Validation("FIX component identity still in use"));
        }
        if state
            .leases
            .values()
            .filter(|slot| slot.role == key && slot.closed.is_none_or(|until| until > now))
            .count()
            >= concurrent
        {
            return Err(Error::Admission {
                retry_after: heartbeat * 2,
            });
        }
        let recent = state.attempts.entry(key).or_default();
        while recent
            .front()
            .is_some_and(|at| now.duration_since(*at) >= interval)
        {
            recent.pop_front();
        }
        if recent.len() >= attempts {
            return Err(Error::Admission {
                retry_after: recent.front().map_or(interval, |at| {
                    interval.saturating_sub(now.duration_since(*at))
                }),
            });
        }
        recent.push_back(now);
        let ticket = state.next_lease;
        state.next_lease = ticket
            .checked_add(1)
            .ok_or(Error::Configuration("FIX lease identity overflow"))?;
        state.leases.insert(
            ticket,
            Slot {
                component: component.as_str().into(),
                role: key,
                closed: None,
                component_until: None,
            },
        );
        Ok(Lease {
            owner: self.clone(),
            ticket,
            heartbeat,
        })
    }
}
fn role_key(role: Role) -> u8 {
    match role {
        Role::OrderEntry => 0,
        Role::DropCopy => 1,
        Role::MarketData => 2,
    }
}
impl Lease {
    pub(super) fn retire(&mut self, clean: bool) -> Result<(), Error> {
        let mut state = self
            .owner
            .state
            .lock()
            .map_err(|_| Error::Configuration("FIX account budget poisoned"))?;
        if let Some(slot) = state.leases.get_mut(&self.ticket)
            && slot.closed.is_none()
        {
            let until = Instant::now()
                .checked_add(self.heartbeat * 2)
                .ok_or(Error::Configuration("FIX lease timer range"))?;
            slot.closed = Some(until);
            slot.component_until = if clean { None } else { Some(until) };
        }
        Ok(())
    }
}
impl Drop for Lease {
    fn drop(&mut self) {
        let _ = self.retire(false);
    }
}

pub(super) struct MessageBudget {
    count: VecDeque<Instant>,
    limit: usize,
    interval: Duration,
    observed: Option<(Instant, usize)>,
}
impl MessageBudget {
    pub(super) fn new(role: Role) -> Self {
        let (limit, seconds) = match role {
            Role::OrderEntry => (10_000, 10),
            Role::DropCopy => (60, 60),
            Role::MarketData => (2000, 60),
        };
        Self {
            count: VecDeque::new(),
            limit,
            interval: Duration::from_secs(seconds),
            observed: None,
        }
    }
    pub(super) fn observe(
        &mut self,
        interval: Duration,
        count: u64,
        limit: u64,
    ) -> Result<(), Error> {
        if interval.is_zero() {
            return Err(Error::Gap("FIX message limit interval missing"));
        }
        self.interval = interval;
        self.limit =
            usize::try_from(limit).map_err(|_| Error::Gap("FIX message limit overflow"))?;
        let count = usize::try_from(count).map_err(|_| Error::Gap("FIX message count overflow"))?;
        self.observed = Some((
            Instant::now()
                .checked_add(interval)
                .ok_or(Error::Gap("FIX message interval range"))?,
            count.max(self.count.len()),
        ));
        Ok(())
    }
    pub(super) fn admit(&mut self) -> Result<(), Error> {
        let now = Instant::now();
        while self
            .count
            .front()
            .is_some_and(|at| now.duration_since(*at) >= self.interval)
        {
            self.count.pop_front();
        }
        if self
            .observed
            .is_some_and(|(until, count)| until > now && count >= self.limit)
        {
            return Err(Error::Admission {
                retry_after: self.observed.map_or(self.interval, |(until, _)| {
                    until.saturating_duration_since(now)
                }),
            });
        }
        if self.count.len() >= self.limit {
            return Err(Error::Admission {
                retry_after: self.count.front().map_or(self.interval, |at| {
                    self.interval.saturating_sub(now.duration_since(*at))
                }),
            });
        }
        if let Some((until, count)) = &mut self.observed
            && *until > now
        {
            *count = count
                .checked_add(1)
                .ok_or(Error::Configuration("FIX message limit overflow"))?;
        }
        self.count.push_back(now);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BudgetLimits;
    #[tokio::test(start_paused = true)]
    async fn component_and_delayed_connection_count_remain_independent() {
        let budgets = AccountBudgets::new(Budgets::new(BudgetLimits::spot()).unwrap());
        let heartbeat = Duration::from_secs(5);
        let id = CompId::new("SAME").unwrap();
        let mut leases = vec![];
        for _ in 0..10 {
            let mut lease = budgets.connect(Role::OrderEntry, &id, heartbeat).unwrap();
            lease.retire(true).unwrap();
            leases.push(lease);
        }
        assert!(matches!(
            budgets.connect(Role::OrderEntry, &id, heartbeat),
            Err(Error::Admission { .. })
        ));
        tokio::time::advance(heartbeat * 2).await;
        let mut lease = budgets.connect(Role::OrderEntry, &id, heartbeat).unwrap();
        lease.retire(false).unwrap();
        assert!(matches!(
            budgets.connect(Role::OrderEntry, &id, heartbeat),
            Err(Error::Validation(_))
        ));
        tokio::time::advance(heartbeat * 2).await;
        budgets.connect(Role::OrderEntry, &id, heartbeat).unwrap();
    }
    #[tokio::test(start_paused = true)]
    async fn observed_message_floor_charges_control_and_survives_local_bucket_reset() {
        let mut budget = MessageBudget::new(Role::OrderEntry);
        budget.observe(Duration::from_secs(10), 2, 3).unwrap();
        budget.admit().unwrap();
        assert!(matches!(budget.admit(), Err(Error::Admission { .. })));
        tokio::time::advance(Duration::from_secs(10)).await;
        budget.admit().unwrap();
    }
}
