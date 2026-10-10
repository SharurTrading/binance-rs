// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Decimal, Error};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
fn text<'a>(p: &'a BTreeMap<String, Value>, k: &str) -> Option<&'a str> {
    p.get(k).and_then(Value::as_str)
}
fn required(p: &BTreeMap<String, Value>, keys: &[&str]) -> Result<(), Error> {
    if keys.iter().any(|k| !p.contains_key(*k)) {
        return Err(Error::Validation("required Margin parameter"));
    }
    Ok(())
}
fn order(p: &BTreeMap<String, Value>, prefix: &str, has_quote: bool) -> Result<(), Error> {
    let key = |plain: &str, capitalized: &str| {
        if prefix.is_empty() {
            plain.to_owned()
        } else {
            format!("{prefix}{capitalized}")
        }
    };
    let kind = key("type", "Type");
    let quantity = key("quantity", "Quantity");
    let price = key("price", "Price");
    let tif = key("timeInForce", "TimeInForce");
    let stop = key("stopPrice", "StopPrice");
    let trailing = key("trailingDelta", "TrailingDelta");
    let iceberg = key("icebergQty", "IcebergQty");
    let quote = has_quote && p.contains_key("quoteOrderQty");
    if quote && p.contains_key(&quantity) {
        return Err(Error::Validation(
            "Margin base quantity conflicts with quote spend",
        ));
    }
    match text(p, &kind) {
        Some("MARKET") => {
            if p.contains_key(&quantity) == quote {
                return Err(Error::Validation("Margin MARKET amount direction"));
            }
            if p.contains_key(&price) || p.contains_key(&tif) {
                return Err(Error::Validation("Margin MARKET price or timeInForce"));
            }
        }
        Some("LIMIT") => required(p, &[&quantity, &price, &tif])?,
        Some("LIMIT_MAKER") => required(p, &[&quantity, &price])?,
        Some("STOP_LOSS" | "TAKE_PROFIT" | "STOP_LOSS_LIMIT" | "TAKE_PROFIT_LIMIT") => {
            required(p, &[&quantity])?;
            if !p.contains_key(&stop) && !p.contains_key(&trailing) {
                return Err(Error::Validation("Margin stop trigger required"));
            }
            if matches!(
                text(p, &kind),
                Some("STOP_LOSS_LIMIT" | "TAKE_PROFIT_LIMIT")
            ) {
                required(p, &[&price, &tif])?;
            }
        }
        _ => return Err(Error::Validation("Margin order type")),
    }
    if quote && text(p, &kind) != Some("MARKET") {
        return Err(Error::Validation("Margin quote spend requires MARKET"));
    }
    if p.contains_key(&iceberg) && text(p, &tif) != Some("GTC") {
        return Err(Error::Validation("Margin iceberg requires GTC"));
    }
    Ok(())
}
fn validate_magnitudes(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])?;
    for (k, v) in p {
        if v.as_str().is_some_and(str::is_empty) {
            return Err(Error::Validation("empty Margin parameter"));
        }
        if matches!(
            k.as_str(),
            "amount"
                | "quantity"
                | "quoteOrderQty"
                | "price"
                | "stopPrice"
                | "stopLimitPrice"
                | "icebergQty"
                | "limitIcebergQty"
                | "stopIcebergQty"
                | "workingPrice"
                | "workingQuantity"
                | "workingIcebergQty"
                | "pendingPrice"
                | "pendingStopPrice"
                | "pendingQuantity"
                | "pendingIcebergQty"
                | "pendingAbovePrice"
                | "pendingAboveStopPrice"
                | "pendingAboveIcebergQty"
                | "pendingBelowPrice"
                | "pendingBelowStopPrice"
                | "pendingBelowIcebergQty"
        ) && super::wire::parse_decimal(v).map_err(|_| Error::Validation("Margin magnitude"))?
            <= Decimal::ZERO
        {
            return Err(Error::Validation("positive Margin magnitude"));
        }
        if matches!(
            k.as_str(),
            "limit"
                | "size"
                | "current"
                | "tier"
                | "trailingDelta"
                | "pendingTrailingDelta"
                | "pendingAboveTrailingDelta"
                | "pendingBelowTrailingDelta"
        ) && v.as_i64().is_none_or(|n| n <= 0)
        {
            return Err(Error::Validation("positive Margin integer"));
        }
        if matches!(k.as_str(), "startTime" | "endTime" | "vipLevel")
            && v.as_i64().is_none_or(|n| n < 0)
        {
            return Err(Error::Validation("nonnegative Margin integer"));
        }
    }
    Ok(())
}

fn validate_range(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    if let (Some(s), Some(e)) = (
        p.get("startTime").and_then(Value::as_i64),
        p.get("endTime").and_then(Value::as_i64),
    ) {
        let duration = e
            .checked_sub(s)
            .ok_or(Error::Validation("Margin time range"))?;
        let max = match op {
            "queryCrossIsolatedMarginCapitalFlow" => Some(604_800_000),
            "queryMarginAccountsAllOrders" | "queryMarginAccountsTradeList" => Some(86_399_999),
            "getCrossMarginTransferHistory" => Some(2_592_000_000),
            _ => None,
        };
        if duration < 0 || max.is_some_and(|m| duration > m) {
            return Err(Error::Validation("Margin time range"));
        }
    }
    Ok(())
}

fn validate_scope_and_lists(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    if (text(p, "isIsolated") == Some("TRUE") || p.get("isIsolated") == Some(&Value::Bool(true)))
        && !p.contains_key("symbol")
    {
        return Err(Error::Validation("isolated Margin symbol required"));
    }
    for (key, max) in [
        ("symbols", Some(5)),
        ("assets", Some(20)),
        ("assetNames", None),
    ] {
        if let Some(v) = text(p, key) {
            let members = v.split(',').collect::<Vec<_>>();
            let mut seen = BTreeSet::new();
            if max.is_some_and(|m| members.len() > m) {
                return Err(Error::Validation("Margin list limit"));
            }
            for v in members {
                if v.is_empty() || v.trim() != v || !seen.insert(v) {
                    return Err(Error::Validation("Margin identity list"));
                }
                if key == "symbols" {
                    crate::Symbol::new(v)?;
                } else {
                    crate::Asset::new(v)?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn validate(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    validate_magnitudes(p)?;
    validate_range(op, p)?;
    validate_scope_and_lists(p)?;
    if op == "createUserListenToken" {
        return Err(Error::Configuration(
            "Margin token issuance security evidence unresolved",
        ));
    }
    match op {
        "createUserListenToken"
            if p.contains_key("symbol") && p.get("isIsolated") != Some(&Value::Bool(true)) =>
        {
            return Err(Error::Validation(
                "Margin cross token cannot select isolated symbol",
            ));
        }
        "marginAccountNewOrder" => order(p, "", true)?,
        "marginAccountNewOco" => {
            if p.contains_key("stopLimitPrice") != p.contains_key("stopLimitTimeInForce") {
                return Err(Error::Validation(
                    "Margin stop limit price/timeInForce pair",
                ));
            }
            if p.contains_key("stopIcebergQty") && text(p, "stopLimitTimeInForce") != Some("GTC") {
                return Err(Error::Validation("Margin stop iceberg requires GTC"));
            }
        }
        "marginAccountNewOto" => {
            order(p, "working", false)?;
            order(p, "pending", false)?;
            if p.contains_key("pendingTrailingDelta") {
                required(p, &["pendingPrice"])?;
            }
        }
        "marginAccountNewOtoco" => {
            order(p, "working", false)?;
            required(p, &["pendingBelowType"])?;
            for prefix in ["pendingAbove", "pendingBelow"] {
                let mut leg = p.clone();
                leg.insert(
                    format!("{prefix}Quantity"),
                    p.get("pendingQuantity")
                        .cloned()
                        .ok_or(Error::Validation("Margin pending quantity"))?,
                );
                order(&leg, prefix, false)?;
                if p.contains_key(&format!("{prefix}TrailingDelta")) {
                    required(p, &[&format!("{prefix}Price")])?;
                }
            }
        }
        "queryMarginAccountsOrder" | "marginAccountCancelOrder"
            if !p.contains_key("orderId") && !p.contains_key("origClientOrderId") =>
        {
            return Err(Error::Validation("Margin order identity required"));
        }
        "queryMarginAccountsOco"
            if !p.contains_key("orderListId") && !p.contains_key("origClientOrderId") =>
        {
            return Err(Error::Validation("Margin OCO identity required"));
        }
        "marginAccountCancelOco"
            if !p.contains_key("orderListId") && !p.contains_key("listClientOrderId") =>
        {
            return Err(Error::Validation("Margin OCO identity required"));
        }
        "queryMarginAccountsAllOco"
            if p.contains_key("fromId")
                && (p.contains_key("startTime") || p.contains_key("endTime")) =>
        {
            return Err(Error::Validation("Margin OCO ID/time conflict"));
        }
        "queryPreventedMatches"
            if p.contains_key("orderId") == p.contains_key("preventedMatchId")
                || (p.contains_key("fromPreventedMatchId") && !p.contains_key("orderId")) =>
        {
            return Err(Error::Validation("Margin prevented match selection"));
        }
        "deleteSpecialKey" if p.contains_key("apiKey") == p.contains_key("apiName") => {
            return Err(Error::Validation("Margin special key selection"));
        }
        "adjustCrossMarginMaxLeverage"
            if !p
                .get("maxLeverage")
                .and_then(Value::as_i64)
                .is_some_and(|n| matches!(n, 3 | 5 | 10)) =>
        {
            return Err(Error::Validation("Margin maximum leverage"));
        }
        "marginManualLiquidation"
            if text(p, "type") == Some("ISOLATED") && !p.contains_key("symbol") =>
        {
            return Err(Error::Validation("isolated liquidation symbol required"));
        }
        _ => (),
    }
    Ok(())
}
pub(crate) fn validate_time(p: &BTreeMap<String, Value>, _now: u64) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])
}
// Product-specific pinned refusal codes; never classify a 5xx or future code as refusal.
pub(crate) fn definitive(status: u16, value: &Value) -> bool {
    status < 500
        && value.get("code").and_then(Value::as_i64).is_some_and(|c| {
            matches!(
                c,
                -1002
                    | -1014
                    | -1020
                    | -1021
                    | -1022
                    | -1100
                    | -1101
                    | -1102
                    | -1103
                    | -1104
                    | -1105
                    | -1106
                    | -1109
                    | -1110
                    | -1111
                    | -1114
                    | -1115
                    | -1116
                    | -1117
                    | -1118
                    | -1119
                    | -1120
                    | -1121
                    | -1122
                    | -1123
                    | -1124
                    | -1125
                    | -1127
                    | -1128
                    | -1130
                    | -1131
                    | -1134
                    | -1145
                    | -1151
                    | -1209
                    | -2010
                    | -2011
                    | -2013
                    | -2014
                    | -2015
                    | -2016
                    | -2017
                    | -2018
                    | -3001
                    | -3002
                    | -3003
                    | -3004
                    | -3005
                    | -3006
                    | -3008
                    | -3009
                    | -3010
                    | -3011
                    | -3012
                    | -3013
                    | -3014
                    | -3015
                    | -3016
                    | -3017
                    | -3018
                    | -3019
                    | -3020
                    | -3021
                    | -3022
                    | -3023
                    | -3024
                    | -3025
                    | -3026
                    | -3027
                    | -3028
            )
        })
}

pub(crate) fn future_open_order_lists(p: &BTreeMap<String, Value>, now: u64) -> Result<(), Error> {
    if now < 1_791_936_000_000 {
        return Err(Error::Validation(
            "Margin open OTO/OTOCO endpoint effective 2026-10-14 UTC",
        ));
    }
    validate_time(p, now)
}
