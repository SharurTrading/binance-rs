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
        exit: crate::core::rate::reserve::ExitEligibility::from_flag(matches!(
            op.name,
            "deleteOrder"
                | "deleteOpenOrders"
                | "deleteOrderList"
                | "orderCancel"
                | "openOrdersCancelAll"
                | "orderListCancel"
        )),
        weight: if rest { weight } else { 0 },
        ws_weight: if rest { 0 } else { weight },
        raw_requests: u64::from(rest),
        orders10: orders,
        orders_day: orders,
        ..Default::default()
    })
}

/// Hands an exchange information reply's `rateLimits` to the client's IP pool.
pub(crate) fn adopt_stated_limits(
    client: &crate::core::HttpClient,
    reply: &super::rest_models::ExchangeInfoResponse,
) -> Result<(), Error> {
    client.adopt_stated_limits(reply.rate_limits.iter().flatten().map(stated))
}

fn stated(
    item: &super::rest_models::ExchangeInfoResponseRateLimitsItem,
) -> crate::core::StatedLimit<'_> {
    crate::core::StatedLimit {
        kind: item
            .rate_limit_type
            .as_ref()
            .map(super::enums::RateLimitType::as_str),
        interval: item
            .interval
            .as_ref()
            .map(super::enums::RateLimitInterval::as_str),
        interval_num: item.interval_num,
        limit: item.limit,
    }
}

/// Adopt the same counted windows from a WebSocket `exchangeInfo` result as REST.
/// Validates the entire statement before changing either limit.
pub(crate) fn adopt_stated_ws_limits(
    client: &super::WsClient,
    reply: &super::ws_models::ExchangeInfoResponse,
) -> Result<(), Error> {
    client
        .budgets
        .adopt_stated(reply.rate_limits.iter().flatten().map(|item| {
            crate::core::StatedLimit {
                kind: item
                    .rate_limit_type
                    .as_ref()
                    .map(super::enums::RateLimitType::as_str),
                interval: item
                    .interval
                    .as_ref()
                    .map(super::enums::RateLimitInterval::as_str),
                interval_num: item.interval_num,
                limit: item.limit,
            }
        }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_pure_cancellations_are_eligible_without_placing_orders() {
        use crate::core::Request;
        let empty = std::collections::BTreeMap::new();
        for op in [
            crate::spot::rest_requests::DeleteOrder::OP,
            crate::spot::rest_requests::DeleteOpenOrders::OP,
            crate::spot::rest_requests::DeleteOrderList::OP,
            crate::spot::ws_requests::OrderCancel::OP,
            crate::spot::ws_requests::OpenOrdersCancelAll::OP,
            crate::spot::ws_requests::OrderListCancel::OP,
        ] {
            let cost = super::cost(op, &empty).unwrap();
            assert!(cost.exit.is_eligible(), "{}", op.name);
            assert_eq!(cost.orders10, 0, "{}", op.name);
            assert_eq!(cost.orders60, 0, "{}", op.name);
        }
    }

    #[test]
    fn cancel_replace_and_spot_sells_remain_ordinary() {
        use crate::core::Request;
        let p = std::collections::BTreeMap::from([("side".into(), serde_json::json!("SELL"))]);
        assert!(
            !super::cost(crate::spot::rest_requests::OrderCancelReplace::OP, &p)
                .unwrap()
                .exit
                .is_eligible()
        );
        assert!(
            !super::cost(crate::spot::rest_requests::NewOrder::OP, &p)
                .unwrap()
                .exit
                .is_eligible()
        );
    }
}
