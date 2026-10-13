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

/// Apply venue-stated IP limits to every Options client drawing on this pool.
pub(crate) fn adopt_stated_limits(
    client: &crate::core::HttpClient,
    reply: &super::rest_models::ExchangeInformationResponse,
) -> Result<(), Error> {
    client.adopt_stated_limits(
        reply
            .rate_limits
            .iter()
            .map(|item| crate::core::StatedLimit {
                kind: item.rate_limit_type.as_deref(),
                interval: item.interval.as_deref(),
                interval_num: item.interval_num,
                limit: item.limit,
            }),
    )
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
        exit: crate::core::rate::reserve::ExitEligibility::from_flag(
            matches!(
                op.name,
                "cancelOptionOrder"
                    | "cancelMultipleOptionOrders"
                    | "cancelAllOptionOrdersOnSpecificSymbol"
                    | "cancelAllOptionOrdersByUnderlying"
                    | "cancelBlockTradeOrder"
            ) || crate::core::rate::reserve::reducing_placement(op.name, p, false, "orders"),
        ),
        weight,
        orders10: orders,
        orders60: orders,
        orders_day: orders,
        raw_requests: 1,
        ..Cost::default()
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_pure_cancellations_are_eligible_without_placing_orders() {
        use crate::core::Request;
        let empty = std::collections::BTreeMap::new();
        for op in [
            crate::options::rest_requests::CancelOptionOrder::OP,
            crate::options::rest_requests::CancelMultipleOptionOrders::OP,
            crate::options::rest_requests::CancelAllOptionOrdersOnSpecificSymbol::OP,
            crate::options::rest_requests::CancelAllOptionOrdersByUnderlying::OP,
            crate::options::rest_requests::CancelBlockTradeOrder::OP,
        ] {
            let cost = super::cost(op, &empty).unwrap();
            assert!(cost.exit.is_eligible(), "{}", op.name);
            assert_eq!(cost.orders10, 0, "{}", op.name);
            assert_eq!(cost.orders60, 0, "{}", op.name);
        }
    }

    #[test]
    fn creation_uses_reserve_only_with_explicit_reducing_flags() {
        use crate::core::Request;
        use serde_json::json;
        use std::collections::BTreeMap;
        let op = crate::options::rest_requests::NewOrder::OP;
        assert!(
            !super::cost(op, &BTreeMap::new())
                .unwrap()
                .exit
                .is_eligible()
        );
        let p = BTreeMap::from([("reduceOnly".into(), json!("true"))]);
        assert!(super::cost(op, &p).unwrap().exit.is_eligible());
        let op = crate::options::rest_requests::PlaceMultipleOrders::OP;
        let reducing = BTreeMap::from([(
            "orders".into(),
            json!([{"reduceOnly":true},{"reduceOnly":"true"}]),
        )]);
        assert!(super::cost(op, &reducing).unwrap().exit.is_eligible());
        let mixed = BTreeMap::from([(
            "orders".into(),
            json!([{"reduceOnly":true},{"reduceOnly":false}]),
        )]);
        assert!(!super::cost(op, &mixed).unwrap().exit.is_eligible());
        let close = BTreeMap::from([("closePosition".into(), json!("true"))]);
        assert!(
            !super::cost(
                op,
                &BTreeMap::from([("orders".into(), json!([{ "closePosition": true }]))])
            )
            .unwrap()
            .exit
            .is_eligible()
        );
        assert!(
            !super::cost(crate::options::rest_requests::NewOrder::OP, &close)
                .unwrap()
                .exit
                .is_eligible()
        );
    }
}
