// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Product-documented endpoint request ceilings in account-owned rolling windows.
use super::{Error, State};
use std::time::Duration;

#[derive(Clone, Copy)]
pub(crate) struct RollingLimit {
    pub endpoint: &'static str,
    pub window_millis: u64,
    pub requests: usize,
}
impl State {
    pub(in crate::core) fn check_rolling(
        &self,
        limit: Option<RollingLimit>,
        now: u64,
    ) -> Result<(), Error> {
        let Some(limit) = limit else { return Ok(()) };
        if limit.window_millis == 0 || limit.requests == 0 {
            return Err(Error::Configuration("invalid rolling endpoint quota"));
        }
        let Some(times) = self.rolling_requests.get(limit.endpoint) else {
            return Ok(());
        };
        if times.back().is_some_and(|t| *t > now) {
            return Err(Error::Configuration("rolling quota clock moved backwards"));
        }
        let active: Vec<_> = times
            .iter()
            .filter(|t| now.saturating_sub(**t) < limit.window_millis)
            .collect();
        if active.len() >= limit.requests {
            let oldest = active
                .first()
                .ok_or(Error::Configuration("rolling quota evidence"))?;
            return Err(Error::Admission {
                retry_after: Duration::from_millis(limit.window_millis - (now - **oldest)),
            });
        }
        Ok(())
    }
    pub(in crate::core) fn charge_rolling(&mut self, limit: Option<RollingLimit>, now: u64) {
        if let Some(limit) = limit {
            let times = self.rolling_requests.entry(limit.endpoint).or_default();
            while times
                .front()
                .is_some_and(|t| now.saturating_sub(*t) >= limit.window_millis)
            {
                times.pop_front();
            }
            times.push_back(now);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{BudgetLimits, Budgets, Cost, sapi::SapiCost};
    #[test]
    fn rolling_refusals_are_atomic_across_clients_and_boundary_is_exact() {
        for sapi in [false, true] {
            let budgets = if sapi {
                Budgets::sapi().unwrap()
            } else {
                Budgets::new(BudgetLimits::portfolio_margin()).unwrap()
            };
            let cost = Cost {
                weight: 750,
                rolling: Some(RollingLimit {
                    endpoint: "/synthetic/transfer",
                    window_millis: 600_000,
                    requests: 2,
                }),
                sapi: sapi.then_some(SapiCost {
                    endpoint: "/synthetic/transfer",
                    uid: false,
                    weight: 1500,
                    requests_per_second: None,
                    requests_per_minute: None,
                }),
                ..Cost::default()
            };
            budgets.admit(cost, 59_999).unwrap();
            budgets.clone().admit(cost, 60_000).unwrap();
            let before = budgets.account.lock().unwrap().rolling_requests.clone();
            assert!(
                matches!(budgets.admit(cost,659_998),Err(Error::Admission{retry_after}) if retry_after==Duration::from_millis(1))
            );
            assert_eq!(budgets.account.lock().unwrap().rolling_requests, before);
            budgets.for_account().admit(cost, 659_998).unwrap();
            budgets.admit(cost, 659_999).unwrap();
            assert!(budgets.admit(cost, 659_999).is_err());
            budgets.admit(cost, 660_000).unwrap();
        }
    }
}
