// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation},
};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn cost(op: Operation, p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let (single, all) = match op.name {
        "portfolioMarginUmTradingQuantitativeRulesIndicators" => (1, 10),
        "cmAccountTradeList" | "queryAllCmOrders" => (20, 40),
        "queryAllCmConditionalOrders"
        | "queryAllCurrentCmOpenConditionalOrders"
        | "queryAllCurrentCmOpenOrders"
        | "queryAllCurrentUmOpenOrders"
        | "queryAllCurrentUmOpenAlgoOrders" => (1, 40),
        "queryUsersCmForceOrders" | "queryUsersUmForceOrders" => (20, 50),
        _ => (op.weight, op.weight),
    };
    let weight = if p.contains_key("symbol") {
        single
    } else {
        all
    };
    if weight == 0 {
        return Err(Error::Configuration("missing native endpoint weight"));
    }
    Ok(Cost {
        weight,
        rolling: match op.name {
            "bnbTransfer" => Some(crate::core::rate::RollingLimit {
                endpoint: op.path,
                window_millis: 600_000,
                requests: 10,
            }),
            "fundAutoCollection" => Some(crate::core::rate::RollingLimit {
                endpoint: op.path,
                window_millis: 3_600_000,
                requests: 500,
            }),
            _ => None,
        },
        orders60: match op.name {
            "newCmOrder"
            | "newUmOrder"
            | "newMarginOrder"
            | "newCmConditionalOrder"
            | "newUmAlgoOrder"
            | "modifyCmOrder"
            | "modifyUmOrder" => 1,
            "marginAccountNewOco" => 2,
            _ => 0,
        },
        download: match op.name {
            "getDownloadIdForUmFuturesOrderHistory" => 1,
            "getDownloadIdForUmFuturesTradeHistory" => 2,
            "getDownloadIdForUmFuturesTransactionHistory" => 3,
            _ => 0,
        },
        ..Cost::default()
    })
}
pub(crate) fn open_orders_cost(
    op: Operation,
    p: &BTreeMap<String, Value>,
    authority: Option<super::TradingSymbolCount>,
) -> Result<Cost, Error> {
    let mut cost = cost(op, p)?;
    if !p.contains_key("symbol") {
        let authority = authority.ok_or(Error::Validation(
            "current venue trading-symbol count required",
        ))?;
        cost.weight = op
            .weight
            .checked_mul(authority.count())
            .ok_or(Error::Validation("all-symbol request weight overflow"))?;
        cost.authority_expiry = Some(authority.expires_at_millis());
    }
    Ok(cost)
}
