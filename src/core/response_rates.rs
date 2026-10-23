// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Received rate evidence is applied only at a valid receipt-time observation.

use crate::core::{Budgets, Clock, Cost, Error, Outcome, ResponseMeta, TransportFailure};

#[derive(Clone, Copy)]
pub(crate) enum Transport {
    Http(Cost),
    WebSocket,
}

pub(crate) fn observe(
    budgets: &Budgets,
    clock: &dyn Clock,
    meta: &ResponseMeta,
    outcome: Outcome,
    transport: Transport,
) -> Result<u64, Error> {
    let failed = || {
        crate::core::http::transport_after_response(
            meta,
            outcome,
            TransportFailure::RateEvidenceFailed,
        )
    };
    let Ok(now) = clock.now_millis() else {
        budgets.hold_unknown_rate_timing().map_err(|_| failed())?;
        return Err(failed());
    };
    match transport {
        Transport::Http(cost) => budgets.observe_cost(cost, &meta.rates, now, meta.status),
        Transport::WebSocket => budgets
            .observe_ban(meta.status, &meta.rates)
            .and_then(|()| budgets.observe(&meta.rates, now, true)),
    }
    .map_err(|_| failed())?;
    Ok(now)
}
