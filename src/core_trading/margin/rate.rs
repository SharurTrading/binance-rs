// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation, sapi::SapiCost},
};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn cost(op: Operation, p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let (uid, weight) = match op.name {
        "queryCrossMarginFeeData" => (false, if p.contains_key("coin") { 1 } else { 5 }),
        "queryIsolatedMarginFeeData" => (false, if p.contains_key("symbol") { 1 } else { 10 }),
        "queryMarginAccountsOpenOrders" => (false, if p.contains_key("symbol") { 10 } else { 40 }),
        "marginAccountNewOrder"
        | "marginAccountNewOco"
        | "marginAccountNewOto"
        | "marginAccountNewOtoco" => (
            true,
            if p.get("sideEffectType")
                .and_then(Value::as_str)
                .is_some_and(|s| matches!(s, "MARGIN_BUY" | "AUTO_BORROW_REPAY"))
            {
                1500
            } else {
                6
            },
        ),
        "queryMaxBorrow"
        | "adjustCrossMarginMaxLeverage"
        | "disableIsolatedMarginAccount"
        | "enableIsolatedMarginAccount"
        | "marginAccountBorrowRepay"
        | "queryMarginAvailableInventory"
        | "closeUserDataStream"
        | "keepaliveUserDataStream"
        | "startUserDataStream"
        | "createSpecialKey"
        | "deleteSpecialKey"
        | "querySpecialKey"
        | "editIpForSpecialKey"
        | "exitSpecialKeyMode"
        | "smallLiabilityExchange"
        | "getSmallLiabilityExchangeHistory"
        | "marginAccountCancelOco"
        | "marginManualLiquidation"
        | "queryMarginAccountsOpenOtootocoOrderLists"
        | "querySpecialKeyList"
        | "queryLiquidationLoan"
        | "liquidationLoanRepay"
        | "queryLiquidationLoanRepayHistory"
        | "createUserListenToken" => (true, op.weight),
        _ => (false, op.weight),
    };
    if weight == 0 {
        return Err(Error::Configuration("missing Margin quota evidence"));
    }
    Ok(Cost {
        sapi: Some(SapiCost {
            endpoint: op.path,
            uid,
            weight,
            requests_per_second: None,
            requests_per_minute: op.requests_per_minute,
        }),
        orders10: match op.name {
            "marginAccountNewOrder" => 1,
            "marginAccountNewOco" | "marginAccountNewOto" => 2,
            "marginAccountNewOtoco" => 3,
            _ => 0,
        },
        ..Cost::default()
    })
}

/// Parse native account order authority completely before shared state is changed.
pub(crate) fn order_windows(
    response: &super::rest_models::QueryCurrentMarginOrderCountUsageResponse,
) -> Result<Vec<(u64, u64, u64)>, Error> {
    if response.is_empty() {
        return Err(Error::Validation("missing Margin order quota evidence"));
    }
    let mut windows = std::collections::BTreeMap::new();
    for item in response {
        if item.rate_limit_type != "ORDERS" {
            return Err(Error::Validation("unknown Margin quota type"));
        }
        let interval = match item.interval.as_str() {
            "SECOND" => 1000_u64,
            "MINUTE" => 60_000,
            "HOUR" => 3_600_000,
            "DAY" => 86_400_000,
            _ => return Err(Error::Validation("unknown Margin quota interval")),
        };
        let multiplier = u64::try_from(item.interval_num)
            .ok()
            .filter(|n| *n > 0)
            .ok_or(Error::Validation("Margin quota interval"))?;
        let window = interval
            .checked_mul(multiplier)
            .ok_or(Error::Validation("Margin quota interval overflow"))?;
        let count =
            u64::try_from(item.count).map_err(|_| Error::Validation("Margin order count"))?;
        let limit = u64::try_from(item.limit)
            .ok()
            .filter(|n| *n > 0)
            .ok_or(Error::Validation("Margin order limit"))?;
        if windows.insert(window, (count, limit)).is_some() {
            return Err(Error::Validation("duplicate Margin quota interval"));
        }
    }
    Ok(windows
        .into_iter()
        .map(|(window, (count, limit))| (window, count, limit))
        .collect())
}
