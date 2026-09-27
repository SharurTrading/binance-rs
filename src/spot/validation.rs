// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Decimal, Error};
use serde_json::Value;
use std::collections::BTreeMap;

fn required(p: &BTreeMap<String, Value>, fields: &[&str]) -> Result<(), Error> {
    if fields.iter().any(|f| !p.contains_key(*f)) {
        return Err(Error::Validation("required Spot parameter"));
    }
    Ok(())
}
fn text<'a>(p: &'a BTreeMap<String, Value>, key: &str) -> Option<&'a str> {
    p.get(key).and_then(Value::as_str)
}
pub(crate) fn validate(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])?;
    for key in [
        "quantity",
        "quoteOrderQty",
        "price",
        "stopPrice",
        "icebergQty",
    ] {
        if let Some(v) = p.get(key) {
            let d =
                super::wire::parse_decimal(v).map_err(|_| Error::Validation("Spot magnitude"))?;
            if d <= Decimal::ZERO {
                return Err(Error::Validation("positive Spot magnitude"));
            }
        }
    }
    if p.contains_key("symbol") && p.contains_key("symbols") {
        return Err(Error::Validation("symbol conflicts with symbols"));
    }
    if matches!(op, "ticker" | "tickerTradingDay")
        && !p.contains_key("symbol")
        && !p.contains_key("symbols")
    {
        return Err(Error::Validation("ticker symbol selection required"));
    }
    if let Some(symbols) = p.get("symbols") {
        let symbols = symbols
            .as_array()
            .ok_or(Error::Validation("symbols array"))?;
        if symbols.is_empty() {
            return Err(Error::Validation("empty symbols"));
        }
        for s in symbols {
            crate::Symbol::new(s.as_str().ok_or(Error::Validation("symbol identity"))?)?;
        }
    }
    if p.get("limit")
        .is_some_and(|v| v.as_i64().is_none_or(|v| v < 1))
    {
        return Err(Error::Validation("positive limit"));
    }
    if let (Some(start), Some(end)) = (
        p.get("startTime").and_then(Value::as_i64),
        p.get("endTime").and_then(Value::as_i64),
    ) && (start < 0 || end < start)
    {
        return Err(Error::Validation("time range"));
    }
    if matches!(
        op,
        "getOrder" | "deleteOrder" | "orderStatus" | "orderCancel"
    ) && !p.contains_key("orderId")
        && !p.contains_key("origClientOrderId")
    {
        return Err(Error::Validation("Spot order identity required"));
    }
    if matches!(op, "newOrder" | "orderPlace" | "orderTest") {
        validate_order(p)?;
    }
    if let Some(tz) = text(p, "timeZone") {
        let (negative, value) = tz.strip_prefix('-').map_or((false, tz), |v| (true, v));
        let value = value.strip_prefix('+').unwrap_or(value);
        let (hour, minute) = value.split_once(':').unwrap_or((value, "0"));
        let hour = hour
            .parse::<i32>()
            .map_err(|_| Error::Validation("timeZone"))?;
        let minute = minute
            .parse::<i32>()
            .map_err(|_| Error::Validation("timeZone"))?;
        let minutes = hour
            .checked_mul(60)
            .and_then(|v| v.checked_add(minute))
            .ok_or(Error::Validation("timeZone"))?;
        if hour < 0 || !(0..60).contains(&minute) || minutes > if negative { 720 } else { 840 } {
            return Err(Error::Validation("timeZone"));
        }
    }
    Ok(())
}
fn validate_order(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    required(p, &["symbol", "side", "type", "newClientOrderId"])?;
    let quantity = p.contains_key("quantity");
    let quote = p.contains_key("quoteOrderQty");
    if quantity && quote {
        return Err(Error::Validation(
            "base quantity conflicts with quote spend",
        ));
    }
    let pegged = p.contains_key("pegPriceType");
    if p.contains_key("pegOffsetValue") != p.contains_key("pegOffsetType") {
        return Err(Error::Validation("peg offset pair"));
    }
    if !pegged && (p.contains_key("pegOffsetValue") || p.contains_key("pegOffsetType")) {
        return Err(Error::Validation("peg offset needs pegPriceType"));
    }
    let price = || {
        if pegged || p.contains_key("price") {
            Ok(())
        } else {
            Err(Error::Validation("Spot limit price required"))
        }
    };
    match text(p, "type") {
        Some("MARKET") => {
            if quantity == quote {
                return Err(Error::Validation(
                    "MARKET requires base quantity or quote spend",
                ));
            }
            if p.contains_key("price") || pegged {
                return Err(Error::Validation("MARKET price/peg"));
            }
        }
        Some("LIMIT") => {
            required(p, &["quantity", "timeInForce"])?;
            price()?;
        }
        Some("LIMIT_MAKER") => {
            required(p, &["quantity"])?;
            price()?;
        }
        Some("STOP_LOSS" | "TAKE_PROFIT" | "STOP_LOSS_LIMIT" | "TAKE_PROFIT_LIMIT") => {
            required(p, &["quantity"])?;
            if !p.contains_key("stopPrice") && !p.contains_key("trailingDelta") {
                return Err(Error::Validation("Spot stop trigger required"));
            }
            if matches!(
                text(p, "type"),
                Some("STOP_LOSS_LIMIT" | "TAKE_PROFIT_LIMIT")
            ) {
                required(p, &["timeInForce"])?;
                price()?;
            }
        }
        _ => return Err(Error::Validation("Spot order type")),
    }
    if quote && text(p, "type") != Some("MARKET") {
        return Err(Error::Validation("quote spend requires MARKET"));
    }
    if p.contains_key("icebergQty") && text(p, "timeInForce") != Some("GTC") {
        return Err(Error::Validation("iceberg requires GTC"));
    }
    if p.get("trailingDelta")
        .is_some_and(|v| v.as_i64().is_none_or(|v| v <= 0))
    {
        return Err(Error::Validation("positive trailing delta"));
    }
    Ok(())
}
// Signing uses millisecond timestamps; receive windows preserve three fractional digits.
pub(crate) fn validate_time(p: &BTreeMap<String, Value>, _now: u64) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])
}

// Spot documents every 5xx as execution-unknown; Futures 503 message rules do not apply.
pub(crate) fn definitive(status: u16, value: &serde_json::Value) -> bool {
    if status >= 500 {
        return false;
    }
    value.get("code").and_then(Value::as_i64).is_some_and(|c| {
        matches!(
            c,
            -1013
                | -1015
                | -1021
                | -1022
                | -1100
                | -1101
                | -1102
                | -1103
                | -1104
                | -1105
                | -1106
                | -1111
                | -1114
                | -1115
                | -1116
                | -1117
                | -1118
                | -1119
                | -1120
                | -1121
                | -1127
                | -1128
                | -1130
                | -2010
                | -2011
                | -2014
                | -2015
        )
    })
}
