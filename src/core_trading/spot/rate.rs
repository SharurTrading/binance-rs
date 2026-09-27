// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

// Spot endpoint weights: pinned catalog and the March 2026 success-weight change.
use crate::{
    Error,
    core::{Cost, Operation},
};
use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) fn cost(op: Operation, p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let count = if p.contains_key("symbol") {
        1
    } else {
        p.get("symbols")
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
    };
    let count = u64::try_from(count).map_err(|_| Error::Validation("symbol count"))?;
    let weight = match op.name {
        "depth" => match p.get("limit").and_then(Value::as_u64).unwrap_or(100) {
            1..=100 => 5,
            101..=500 => 25,
            501..=1000 => 50,
            1001..=5000 => 250,
            _ => return Err(Error::Validation("Spot depth limit")),
        },
        "getOpenOrders" | "openOrdersStatus" => {
            if count == 1 {
                6
            } else {
                80
            }
        }
        "ticker24hr" => match count {
            1..=20 => 2,
            21..=100 => 40,
            _ => 80,
        },
        "tickerPrice" | "tickerBookTicker" | "tickerBook" => {
            if p.contains_key("symbol") {
                2
            } else {
                4
            }
        }
        "ticker" | "tickerTradingDay" => count
            .checked_mul(4)
            .ok_or(Error::Validation("symbol weight"))?
            .min(200),
        "executionRules" => {
            if count == 0 {
                40
            } else {
                count
                    .checked_mul(2)
                    .ok_or(Error::Validation("symbol weight"))?
                    .min(40)
            }
        }
        "myTrades" => {
            if p.contains_key("orderId") {
                5
            } else {
                20
            }
        }
        "orderTest" | "sorOrderTest" => {
            if p.get("computeCommissionRates").and_then(Value::as_bool) == Some(true) {
                20
            } else {
                1
            }
        }
        "myPreventedMatches" => {
            if p.contains_key("orderId") {
                20
            } else {
                2
            }
        }
        _ => op.weight,
    };
    let rest = op.path.starts_with("/api/");
    let orders = match op.name {
        "newOrder" | "orderPlace" | "sorOrder" | "sorOrderPlace" | "orderCancelReplace" => 1,
        "orderListOco" | "orderOco" | "orderListPlace" | "orderListPlaceOco" | "orderListOto"
        | "orderListOpo" | "orderListPlaceOto" | "orderListPlaceOpo" => 2,
        "orderListOtoco" | "orderListOpoco" | "orderListPlaceOtoco" | "orderListPlaceOpoco" => 3,
        _ => 0,
    };
    Ok(Cost {
        weight: if rest { weight } else { 0 },
        ws_weight: if rest { 0 } else { weight },
        raw_requests: u64::from(rest),
        orders10: orders,
        orders_day: orders,
        ..Default::default()
    })
}
