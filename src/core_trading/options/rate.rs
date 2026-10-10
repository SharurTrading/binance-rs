// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{BudgetLimits, Error, core::Cost};
use std::collections::BTreeMap;

/// Translate authoritative Options exchange-information rate limits.
/// `REQUEST_WEIGHT` and `ORDERS` authority is required; no Futures ceilings are borrowed.
/// The caller may share the resulting owner with all Options clients on one IP/account.
///
/// # Errors
/// Refuses missing limits, invalid limits or intervals the shared limiter cannot enforce.
pub fn budget_limits(
    info: &super::rest_models::ExchangeInformationResponse,
) -> Result<BudgetLimits, Error> {
    let mut limits = BudgetLimits {
        weight_per_minute: u64::MAX,
        ws_weight_per_minute: u64::MAX,
        orders_per_ten_seconds: u64::MAX,
        orders_per_minute: u64::MAX,
        orders_per_day: None,
        raw_requests_per_five_minutes: None,
        connections_per_five_minutes: None,
        shared_request_weight: false,
    };
    let (mut weight, mut orders) = (false, false);
    for rule in &info.rate_limits {
        let kind = rule
            .rate_limit_type
            .as_deref()
            .ok_or(Error::Configuration("Options rate limit kind"))?;
        let interval = rule
            .interval
            .as_deref()
            .ok_or(Error::Configuration("Options rate limit interval"))?;
        let number = rule
            .interval_num
            .ok_or(Error::Configuration("Options rate interval count"))?;
        let value = rule
            .limit
            .and_then(|v| u64::try_from(v).ok())
            .filter(|v| *v > 0)
            .ok_or(Error::Configuration("Options positive rate limit"))?;
        match (kind, interval, number) {
            ("REQUEST_WEIGHT", "MINUTE", 1) => {
                limits.weight_per_minute = limits.weight_per_minute.min(value);
                weight = true;
            }
            ("ORDERS" | "ORDER", "SECOND", 10) => {
                limits.orders_per_ten_seconds = limits.orders_per_ten_seconds.min(value);
                orders = true;
            }
            ("ORDERS" | "ORDER", "MINUTE", 1) => {
                limits.orders_per_minute = limits.orders_per_minute.min(value);
                orders = true;
            }
            ("ORDERS" | "ORDER", "DAY", 1) => {
                limits.orders_per_day = Some(limits.orders_per_day.map_or(value, |v| v.min(value)));
                orders = true;
            }
            ("RAW_REQUESTS" | "RAW_REQUEST", "MINUTE", 5) => {
                limits.raw_requests_per_five_minutes = Some(
                    limits
                        .raw_requests_per_five_minutes
                        .map_or(value, |v| v.min(value)),
                );
            }
            _ => {
                return Err(Error::Configuration(
                    "unsupported Options rate limit interval",
                ));
            }
        }
    }
    if !weight || !orders {
        return Err(Error::Configuration(
            "Options weight and order budget authority required",
        ));
    }
    Ok(limits)
}

pub(crate) fn cost(
    op: crate::core::Operation,
    p: &BTreeMap<String, serde_json::Value>,
) -> Result<Cost, Error> {
    let weight = match op.name {
        "orderBook" => match p
            .get("limit")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(100)
        {
            5 | 10 | 20 | 50 => 1,
            100 => 5,
            500 => 10,
            1000 => 20,
            _ => return Err(Error::Validation("Options depth limit")),
        },
        "ticker24hrPriceChangeStatistics" | "queryCurrentOpenOptionOrders" => {
            if p.contains_key("symbol") { 1 } else { 40 }
        }
        _ => op.weight,
    };
    let orders = match op.name {
        "newOrder" | "newBlockTradeOrder" => 1,
        "placeMultipleOrders" => u64::try_from(
            p.get("orders")
                .and_then(serde_json::Value::as_array)
                .ok_or(Error::Validation("Options orders"))?
                .len(),
        )
        .map_err(|_| Error::Validation("Options order count"))?,
        _ => 0,
    };
    Ok(Cost {
        weight,
        orders10: orders,
        orders60: orders,
        orders_day: orders,
        raw_requests: 1,
        ..Cost::default()
    })
}
