// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation},
};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn cost(op: Operation, _p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let weight = op.weight;
    if weight == 0 {
        return Err(Error::Configuration("missing native endpoint weight"));
    }
    Ok(Cost {
        sapi: Some(crate::core::sapi::SapiCost {
            endpoint: op.path,
            uid: false,
            weight,
            requests_per_second: op.requests_per_second,
            requests_per_minute: op.requests_per_minute,
        }),
        weight: 0,
        rolling: match op.name {
            "placeEquityOrder" => Some(crate::core::rate::RollingLimit {
                endpoint: op.path,
                window_millis: 60_000,
                requests: 200,
            }),
            "tokenizedMint" | "tokenizedRedeem" => Some(crate::core::rate::RollingLimit {
                endpoint: op.path,
                window_millis: 60_000,
                requests: 50,
            }),
            _ => None,
        },
        ..Cost::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advanced_trading::stocks::rest_requests::{
        PlaceEquityOrder, TokenizedMint, TokenizedRedeem,
    };
    use crate::core::{Budgets, Request};
    #[test]
    fn stocks_uid_caps_hold_across_clones_and_roll_past_the_minute_boundary() {
        for (op, limit) in [
            (PlaceEquityOrder::OP, 200),
            (TokenizedMint::OP, 50),
            (TokenizedRedeem::OP, 50),
        ] {
            let budgets = Budgets::sapi().unwrap();
            let cost = cost(op, &BTreeMap::new()).unwrap();
            for _ in 0..limit {
                budgets.clone().admit(cost, 59_999).unwrap();
            }
            assert!(matches!(
                budgets.admit(cost, 60_000),
                Err(Error::Admission { .. })
            ));
            budgets.for_account().admit(cost, 60_000).unwrap();
            budgets.admit(cost, 119_999).unwrap();
        }
    }
}
