// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! A precharged upgrade remains valid only in its original native quota windows.

use crate::Error;
use crate::core::rate::{Budgets, Cost, State, WindowCharge, reserve};
use std::{collections::BTreeMap, time::Duration};

struct ChargedWindow {
    name: &'static str,
    interval: u64,
    bucket: u64,
    amount: u64,
}

pub(crate) struct HandshakeCharge {
    cost: Cost,
    admitted_at: u64,
    ip: Vec<ChargedWindow>,
    websocket: Vec<ChargedWindow>,
}

impl HandshakeCharge {
    fn new(cost: Cost, admitted_at: u64) -> Result<Self, Error> {
        let Cost {
            rolling: None,
            authority_expiry: None,
            sapi: None,
            orders10: 0,
            orders60: 0,
            orders_day: 0,
            funding: false,
            history: false,
            quote: false,
            download: 0,
            weight: _,
            ws_weight: _,
            raw_requests: _,
            connections: _,
            exit: _,
        } = cost
        else {
            return Err(Error::Configuration("unsupported WebSocket handshake cost"));
        };
        Ok(Self {
            cost,
            admitted_at,
            ip: Vec::new(),
            websocket: Vec::new(),
        })
    }

    pub(super) fn record(
        &mut self,
        ip: &[Option<WindowCharge>],
        websocket: &[Option<WindowCharge>],
    ) {
        self.ip = charged(ip, self.admitted_at);
        self.websocket = charged(websocket, self.admitted_at);
    }
}

fn charged(costs: &[Option<WindowCharge>], admitted_at: u64) -> Vec<ChargedWindow> {
    costs
        .iter()
        .flatten()
        .filter_map(|&(name, interval, _, amount)| {
            (amount > 0).then_some(ChargedWindow {
                name,
                interval,
                bucket: admitted_at / interval,
                amount,
            })
        })
        .collect()
}

impl Budgets {
    pub(crate) fn admit_handshake(&self, cost: Cost, now: u64) -> Result<HandshakeCharge, Error> {
        let mut charge = HandshakeCharge::new(cost, now)?;
        self.admit_native(cost, now, Some(&mut charge))?;
        Ok(charge)
    }

    pub(crate) fn validate_handshake_charge(
        &self,
        charge: &HandshakeCharge,
        now: u64,
    ) -> Result<(), Error> {
        if now < charge.admitted_at {
            return Err(invalid_charge());
        }
        let ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        let websocket = self
            .ws_ip
            .lock()
            .map_err(|_| Error::Configuration("WS IP budget poisoned"))?;
        ip.check_cooldown(now)?;
        validate(
            &charge.ip,
            &self.ip_cost(&ip, charge.cost),
            &ip,
            ip.exit_reserve.request_weight,
            charge.cost.exit.is_eligible(),
            now,
        )?;
        validate(
            &charge.websocket,
            &self.ws_cost(&websocket, charge.cost),
            &websocket,
            websocket.exit_reserve.websocket_weight,
            charge.cost.exit.is_eligible(),
            now,
        )
    }
}

fn validate(
    charged: &[ChargedWindow],
    current: &[Option<WindowCharge>],
    state: &State,
    weight_reserve: u64,
    exit: bool,
    now: u64,
) -> Result<(), Error> {
    for charge in charged {
        if now / charge.interval != charge.bucket
            || state.current(charge.name, charge.interval, now) < charge.amount
            || !current
                .iter()
                .flatten()
                .any(|&(name, interval, _, amount)| {
                    name == charge.name && interval == charge.interval && amount == charge.amount
                })
        {
            return Err(invalid_charge());
        }
    }
    for &(name, interval, limit, amount) in current.iter().flatten() {
        if amount == 0 {
            continue;
        }
        let charge = charged
            .iter()
            .find(|charge| charge.name == name && charge.interval == interval)
            .ok_or_else(invalid_charge)?;
        if now / interval != charge.bucket {
            return Err(invalid_charge());
        }
        let count = state.current(name, interval, now);
        if count < charge.amount {
            return Err(invalid_charge());
        }
        let retained = match name {
            "weight" => weight_reserve,
            "raw" => state.exit_reserve.raw_requests,
            _ => 0,
        };
        let limit = reserve::admission_limit(limit, retained, exit);
        if count > limit {
            return Err(Error::Admission {
                retry_after: Duration::from_millis(interval - now % interval),
            });
        }
    }
    Ok(())
}

fn invalid_charge() -> Error {
    Error::NotSent {
        client_order_ids: BTreeMap::new(),
        operation: "connect",
        reason: "handshake quota admission is no longer valid",
    }
}
