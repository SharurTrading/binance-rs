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
    if op == "userDataFilter" {
        return validate_filter(p);
    }
    crate::core::validate_parameters(p, &[], &[], &[])?;
    for key in [
        "quantity",
        "quoteOrderQty",
        "icebergQty",
        "quoteNotionalLimit",
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
    if matches!(
        op,
        "userDataStreamSubscribe" | "userDataStreamSubscribeSignature"
    ) && let Some(filter) = p.get("filter")
    {
        validate_filter(
            filter
                .as_object()
                .ok_or(Error::Validation("user data filter"))?,
        )?;
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
    validate_advanced(op, p)
}
// A subscription filter names events by their own short fields. The venue's
// validation table refuses an empty filter, an empty or repeated value list, an
// unknown key and an unrecognized event type; execution types are the venue's to judge:
// https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-api.md#user-data-stream-subscription-filters
fn validate_filter<'a>(
    filter: impl IntoIterator<Item = (&'a String, &'a Value)>,
) -> Result<(), Error> {
    const EVENT_TYPES: [&str; 5] = [
        "executionReport",
        "balanceUpdate",
        "outboundAccountPosition",
        "listStatus",
        "externalLockUpdate",
    ];
    let mut filter = filter.into_iter().peekable();
    if filter.peek().is_none() {
        return Err(Error::Validation("empty user data filter"));
    }
    for (key, values) in filter {
        let values = values
            .as_array()
            .ok_or(Error::Validation("user data filter values"))?;
        if values.is_empty() {
            return Err(Error::Validation("empty user data filter values"));
        }
        let mut seen = std::collections::BTreeSet::new();
        for value in values {
            let value = value
                .as_str()
                .ok_or(Error::Validation("user data filter value"))?;
            if !seen.insert(value) {
                return Err(Error::Validation("duplicate user data filter value"));
            }
            let accepted = match key.as_str() {
                "a" => crate::Asset::new(value).is_ok(),
                "e" => EVENT_TYPES.contains(&value),
                "s" => crate::Symbol::new(value).is_ok(),
                "x" => true,
                _ => return Err(Error::Validation("user data filter key")),
            };
            if !accepted {
                return Err(Error::Validation("user data filter value"));
            }
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
    // A quote notional limit caps what a BUY order may spend. The venue refuses it
    // on SELL orders, beside a quote spend, and on LIMIT_MAKER orders:
    // https://github.com/binance/binance-spot-api-docs/blob/master/faqs/quoteNotionalLimit.md
    if p.contains_key("quoteNotionalLimit") {
        if text(p, "side") != Some("BUY") {
            return Err(Error::Validation("quote notional limit requires BUY"));
        }
        if quote {
            return Err(Error::Validation(
                "quote notional limit conflicts with quote spend",
            ));
        }
        if text(p, "type") == Some("LIMIT_MAKER") {
            return Err(Error::Validation(
                "quote notional limit conflicts with LIMIT_MAKER",
            ));
        }
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
    value
        .get("code")
        .and_then(Value::as_i64)
        .is_some_and(definitive_code)
}

// -1018 PARAM_OVERFLOW and -1108 BAD_ASSET are left out: the error page states for
// neither, as it does for -1013, that the request did not reach the matching engine.
pub(crate) fn definitive_code(c: i64) -> bool {
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
}
// Partial response semantics are specific to Spot cancel/replace. Keep the shared
// transport unaware of product codes and never retain sensitive leg bodies in errors.
pub(crate) fn partial(status: u16, value: &Value) -> Option<crate::PartialOperation> {
    use crate::{OperationLeg, Outcome, PartialOperation};
    let data = value.get("data").unwrap_or(value);
    let leg = |name: &str| -> Option<OperationLeg> {
        let result = data.get(format!("{name}Result"))?.as_str()?;
        let response = data.get(format!("{name}Response"));
        let code = response.and_then(|v| v.get("code")).and_then(Value::as_i64);
        let order_id = response
            .and_then(|v| v.get("orderId"))
            .and_then(Value::as_i64);
        let client_order_id = response
            .and_then(|v| v.get("clientOrderId"))
            .and_then(Value::as_str)
            .map(str::to_owned);
        let valid_ack = response.is_some_and(|v| {
            if name == "cancel" {
                serde_json::from_value::<
                        super::rest_models::OrderCancelReplaceResponseCancelResponse,
                    >(v.clone())
                    .is_ok()
            } else {
                serde_json::from_value::<
                    super::rest_models::OrderCancelReplaceResponseNewOrderResponseValue,
                >(v.clone())
                .is_ok()
            }
        });
        let outcome = match result {
            "SUCCESS"
                if order_id.is_some()
                    && client_order_id.is_some()
                    && code.is_none()
                    && valid_ack =>
            {
                Outcome::Accepted
            }
            "FAILURE" if response.is_some_and(|v| definitive(status, v)) => Outcome::Rejected,
            "NOT_ATTEMPTED" if name == "newOrder" && response.is_none_or(Value::is_null) => {
                Outcome::NotSent
            }
            _ => Outcome::Unknown,
        };
        Some(OperationLeg {
            outcome,
            code,
            order_id,
            client_order_id,
        })
    };
    let cancel = leg("cancel")?;
    let new_order = leg("newOrder")?;
    let known_status = matches!(status, 400 | 409 | 429);
    let outcome =
        if !known_status || [cancel.outcome, new_order.outcome].contains(&Outcome::Unknown) {
            Outcome::Unknown
        } else {
            match (cancel.outcome, new_order.outcome) {
                (Outcome::Accepted, Outcome::Rejected) | (Outcome::Rejected, Outcome::Accepted) => {
                    Outcome::Partial
                }
                (Outcome::Rejected, Outcome::Rejected | Outcome::NotSent) => Outcome::Rejected,
                _ => Outcome::Unknown,
            }
        };
    Some(PartialOperation {
        legs: BTreeMap::from([("cancel".into(), cancel), ("newOrder".into(), new_order)]),
        outcome,
    })
}

fn validate_advanced(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    for (name, value) in p {
        if name.to_ascii_lowercase().ends_with("clientorderid") {
            super::ClientOrderId::new(
                value
                    .as_str()
                    .ok_or(Error::Validation("Spot caller identity"))?,
            )?;
        }
        if ["Qty", "Quantity", "Price", "QuoteNotionalLimit"]
            .iter()
            .any(|suffix| name.ends_with(suffix))
        {
            let amount = super::wire::parse_decimal(value)
                .map_err(|_| Error::Validation("Spot magnitude"))?;
            if !name.ends_with("Price") && amount <= Decimal::ZERO {
                return Err(Error::Validation("positive Spot magnitude"));
            }
        }
    }
    match op {
        "orderCancelReplace" => {
            required(p, &["cancelNewClientOrderId"])?;
            if !p.contains_key("cancelOrderId") && !p.contains_key("cancelOrigClientOrderId") {
                return Err(Error::Validation("cancel identity required"));
            }
            validate_order(p)?;
        }
        "orderAmendKeepPriority" => {
            required(p, &["newClientOrderId"])?;
            if !p.contains_key("orderId") && !p.contains_key("origClientOrderId") {
                return Err(Error::Validation("amend identity required"));
            }
        }
        "getOrderList" | "orderListStatus"
            if !p.contains_key("orderListId") && !p.contains_key("origClientOrderId") =>
        {
            return Err(Error::Validation("list identity required"));
        }
        "deleteOrderList" | "orderListCancel" => {
            required(p, &["newClientOrderId"])?;
            if !p.contains_key("orderListId") && !p.contains_key("listClientOrderId") {
                return Err(Error::Validation("list identity required"));
            }
        }
        "sorOrder" | "sorOrderPlace" | "sorOrderTest" => {
            validate_order(p)?;
            if !matches!(text(p, "type"), Some("LIMIT" | "MARKET"))
                || p.contains_key("quoteOrderQty")
            {
                return Err(Error::Validation("SOR order type/amount"));
            }
        }
        "orderOco" | "orderListPlace" => {
            required(
                p,
                &[
                    "listClientOrderId",
                    "limitClientOrderId",
                    "stopClientOrderId",
                ],
            )?;
            if p.contains_key("stopLimitPrice") {
                required(p, &["stopLimitTimeInForce"])?;
            }
        }
        name if name.starts_with("orderListO") || name.starts_with("orderListPlaceO") => {
            validate_list(p)?;
        }
        "myPreventedMatches"
            if !p.contains_key("orderId") && !p.contains_key("preventedMatchId") =>
        {
            return Err(Error::Validation("prevented match identity required"));
        }
        "allOrderList" | "allOrderLists"
            if p.contains_key("fromId")
                && (p.contains_key("startTime") || p.contains_key("endTime")) =>
        {
            return Err(Error::Validation("fromId conflicts with time range"));
        }
        _ => (),
    }
    Ok(())
}

fn validate_list(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    required(p, &["listClientOrderId"])?;
    if p.contains_key("workingType") {
        required(p, &["workingClientOrderId"])?;
        validate_leg(p, "working", "workingSide", "workingQuantity")?;
        if p.contains_key("pendingAboveType") {
            required(
                p,
                &[
                    "pendingAboveClientOrderId",
                    "pendingBelowClientOrderId",
                    "pendingBelowType",
                ],
            )?;
            validate_leg(
                p,
                "pendingAbove",
                "pendingSide",
                if p.contains_key("pendingQuantity") {
                    "pendingQuantity"
                } else {
                    "workingQuantity"
                },
            )?;
            validate_leg(
                p,
                "pendingBelow",
                "pendingSide",
                if p.contains_key("pendingQuantity") {
                    "pendingQuantity"
                } else {
                    "workingQuantity"
                },
            )?;
        } else {
            required(p, &["pendingClientOrderId"])?;
            validate_leg(
                p,
                "pending",
                "pendingSide",
                if p.contains_key("pendingQuantity") {
                    "pendingQuantity"
                } else {
                    "workingQuantity"
                },
            )?;
        }
    } else {
        required(p, &["aboveClientOrderId", "belowClientOrderId"])?;
        validate_leg(p, "above", "side", "quantity")?;
        validate_leg(p, "below", "side", "quantity")?;
    }
    Ok(())
}
fn validate_leg(
    p: &BTreeMap<String, Value>,
    prefix: &str,
    side: &str,
    quantity: &str,
) -> Result<(), Error> {
    let mut leg = BTreeMap::new();
    for key in ["symbol", "quantity", "side"] {
        let source = match key {
            "quantity" => quantity,
            "side" => side,
            _ => key,
        };
        if let Some(v) = p.get(source) {
            leg.insert(key.into(), v.clone());
        }
    }
    for (key, value) in p {
        if let Some(suffix) = key.strip_prefix(prefix)
            && !suffix.is_empty()
        {
            let mut chars = suffix.chars();
            if let Some(first) = chars.next() {
                let name = format!("{}{}", first.to_ascii_lowercase(), chars.as_str());
                let name = if name == "clientOrderId" {
                    "newClientOrderId".into()
                } else {
                    name
                };
                leg.insert(name, value.clone());
            }
        }
    }
    validate_order(&leg)
}
