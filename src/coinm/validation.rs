// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
use serde_json::Value;
use std::collections::BTreeMap;

fn str_field<'a>(p: &'a BTreeMap<String, Value>, name: &str) -> Option<&'a str> {
    p.get(name).and_then(Value::as_str)
}
fn truth(p: &BTreeMap<String, Value>, name: &str) -> bool {
    p.get(name)
        .is_some_and(|v| v == &Value::Bool(true) || v.as_str() == Some("true"))
}
fn required(p: &BTreeMap<String, Value>, names: &[&str]) -> Result<(), Error> {
    if names.iter().any(|n| !p.contains_key(*n)) {
        return Err(Error::Validation("conditional required parameter"));
    }
    Ok(())
}
pub(crate) fn validate(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])?;
    if p.get("recvWindow").is_some_and(|v| {
        crate::core::request::text(v)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .is_none()
    }) {
        return Err(Error::Validation(
            "Futures recvWindow must be integer milliseconds",
        ));
    }
    validate_order_enums(op, p)?;
    validate_batch(op, p)?;
    if matches!(op, "newOrder" | "testOrder") {
        validate_standard_order(op, p)?;
    }
    if op == "rpiOrderBook" && p.get("limit").is_some_and(|v| v.as_i64() != Some(1000)) {
        return Err(Error::Validation("RPI depth supports only 1000 levels"));
    }
    if op == "newAlgoOrder" {
        required(p, &["algoType", "symbol", "side", "type", "clientAlgoId"])?;
        if str_field(p, "algoType") != Some("CONDITIONAL") {
            return Err(Error::Validation("algoType"));
        }
        match str_field(p, "type") {
            Some("STOP" | "TAKE_PROFIT") => {
                required(p, &["quantity", "triggerPrice"])?;
                if !p.contains_key("price") && !p.contains_key("priceMatch") {
                    return Err(Error::Validation("conditional limit price"));
                }
            }
            Some("STOP_MARKET" | "TAKE_PROFIT_MARKET") => {
                required(p, &["triggerPrice"])?;
                if !truth(p, "closePosition") {
                    required(p, &["quantity"])?;
                }
            }
            Some("TRAILING_STOP_MARKET") => required(p, &["quantity", "callbackRate"])?,
            _ => return Err(Error::Validation("algo order type")),
        }
    }
    if op == "modifyOrder" {
        required(p, &["symbol", "side", "quantity", "price"])?;
        if !p.contains_key("price") && !p.contains_key("priceMatch") {
            return Err(Error::Validation("modify price"));
        }
    }
    if matches!(
        op,
        "cancelOrder" | "queryOrder" | "queryCurrentOpenOrder" | "modifyOrder"
    ) && !p.contains_key("orderId")
        && !p.contains_key("origClientOrderId")
    {
        return Err(Error::Validation("order identity required"));
    }
    if matches!(op, "cancelAlgoOrder" | "queryAlgoOrder")
        && !p.contains_key("algoId")
        && !p.contains_key("clientAlgoId")
    {
        return Err(Error::Validation("algo identity required"));
    }
    if p.contains_key("price") && p.contains_key("priceMatch") {
        return Err(Error::Validation("price conflicts with priceMatch"));
    }
    if str_field(p, "positionSide").is_some_and(|s| s != "BOTH") && p.contains_key("reduceOnly") {
        return Err(Error::Validation("reduceOnly is forbidden in hedge mode"));
    }
    if truth(p, "closePosition") && (p.contains_key("quantity") || p.contains_key("reduceOnly")) {
        return Err(Error::Validation(
            "closePosition conflicts with quantity/reduceOnly",
        ));
    }
    if str_field(p, "timeInForce") == Some("GTD") {
        required(p, &["goodTillDate"])?;
    }
    if op == "sendQuoteRequest" && p.contains_key("fromAmount") == p.contains_key("toAmount") {
        return Err(Error::Validation("exactly one conversion amount"));
    }
    if op == "cancelMultipleOrders"
        && !p.contains_key("orderIdList")
        && !p.contains_key("origClientOrderIdList")
    {
        return Err(Error::Validation("batch cancellation identities"));
    }
    Ok(())
}

fn validate_batch(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    if matches!(op, "placeMultipleOrders" | "modifyMultipleOrders") {
        let batch = p
            .get("batchOrders")
            .and_then(Value::as_array)
            .ok_or(Error::Validation("batchOrders"))?;
        if batch.is_empty() || batch.len() > 5 {
            return Err(Error::Validation("documented five-order batch maximum"));
        }
        let mut ids = std::collections::BTreeSet::new();
        for member in batch {
            let map = member
                .as_object()
                .ok_or(Error::Validation("batch member"))?;
            let member: BTreeMap<_, _> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            validate(
                if op == "placeMultipleOrders" {
                    "newOrder"
                } else {
                    "modifyOrder"
                },
                &member,
            )?;
            if let Some(id) = str_field(&member, "newClientOrderId")
                && !ids.insert(id.to_owned())
            {
                return Err(Error::Validation("duplicate batch client ID"));
            }
        }
    }
    Ok(())
}

fn validate_order_enums(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    crate::core::validate_parameters(
        p,
        &[],
        &[
            ("side", &["BUY", "SELL"]),
            ("positionSide", &["BOTH", "LONG", "SHORT"]),
            ("timeInForce", &["GTC", "IOC", "FOK", "GTX", "GTD", "RPI"]),
            ("reduceOnly", &["true", "false"]),
            ("closePosition", &["true", "false"]),
            (
                "priceMatch",
                &[
                    "NONE",
                    "OPPONENT",
                    "OPPONENT_5",
                    "OPPONENT_10",
                    "OPPONENT_20",
                    "QUEUE",
                    "QUEUE_5",
                    "QUEUE_10",
                    "QUEUE_20",
                ],
            ),
            ("newOrderRespType", &["ACK", "RESULT"]),
        ],
        &[],
    )?;
    if let Some(value) = p.get("callbackRate") {
        let value =
            super::wire::parse_decimal(value).map_err(|_| Error::Validation("callbackRate"))?;
        if value < crate::Decimal::new(1, 1)
            || value
                > if op == "testOrder" {
                    crate::Decimal::new(5, 0)
                } else {
                    crate::Decimal::TEN
                }
        {
            return Err(Error::Validation("callbackRate must be between 0.1 and 10"));
        }
    }
    Ok(())
}

fn validate_standard_order(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    required(p, &["symbol", "side", "type", "newClientOrderId"])?;
    match str_field(p, "type") {
        Some("LIMIT") => {
            required(p, &["timeInForce", "quantity"])?;
            if !p.contains_key("price") && !p.contains_key("priceMatch") {
                return Err(Error::Validation("LIMIT needs price or priceMatch"));
            }
        }
        Some("MARKET") => required(p, &["quantity"])?,
        Some("STOP" | "TAKE_PROFIT") if op == "testOrder" => {
            required(p, &["quantity", "stopPrice"])?;
            if !p.contains_key("price") && !p.contains_key("priceMatch") {
                return Err(Error::Validation("conditional test price"));
            }
        }
        Some("STOP_MARKET" | "TAKE_PROFIT_MARKET") if op == "testOrder" => {
            required(p, &["stopPrice"])?;
            if !truth(p, "closePosition") {
                required(p, &["quantity"])?;
            }
        }
        Some("TRAILING_STOP_MARKET") if op == "testOrder" => {
            required(p, &["quantity", "callbackRate"])?;
        }
        _ => {
            return Err(Error::Validation(
                "conditional execution uses the algo endpoint",
            ));
        }
    }
    Ok(())
}

// GTD rules are checked against the injected venue clock immediately before sending.
// Source: the New Order (TRADE) goodTillDate contract in the pinned catalog.
pub(crate) fn validate_time(p: &BTreeMap<String, Value>, now: u64) -> Result<(), Error> {
    if let Some(batch) = p.get("batchOrders").and_then(Value::as_array) {
        for member in batch {
            let member = member
                .as_object()
                .ok_or(Error::Validation("batch member"))?
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            validate_time(&member, now)?;
        }
    }
    if str_field(p, "timeInForce") == Some("GTD") {
        let expiry = p
            .get("goodTillDate")
            .and_then(Value::as_u64)
            .ok_or(Error::Validation("GTD expiry"))?;
        let earliest = now
            .checked_div(1000)
            .and_then(|n| n.checked_add(600))
            .ok_or(Error::Validation("GTD timestamp overflow"))?;
        if expiry / 1000 <= earliest || expiry >= 253_402_300_799_000 {
            return Err(Error::Validation(
                "GTD expiry must exceed venue time by 600 seconds and precede year 10000",
            ));
        }
    }
    Ok(())
}

// Current Futures general-info 503 variants and error-code reference.
pub(crate) fn definitive(status: u16, value: &serde_json::Value) -> bool {
    crate::core::error::futures_definitive(status, value)
}
