// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Decimal, Error};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
fn required(p: &BTreeMap<String, Value>, keys: &[&str]) -> Result<(), Error> {
    crate::core::validate_parameters(p, keys, &[], &[])
}
pub(crate) fn validate(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    validate_common(p)?;
    match op {
        "newOrder" => {
            required(
                p,
                &[
                    "symbol",
                    "side",
                    "type",
                    "quantity",
                    "price",
                    "clientOrderId",
                ],
            )?;
            crate::core::validate_parameters(
                p,
                &[],
                &[
                    ("type", &["LIMIT"]),
                    (
                        "selfTradePreventionMode",
                        &["NONE", "EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH"],
                    ),
                ],
                &[],
            )
        }
        "cancelOptionOrder" | "querySingleOrder"
            if !p.contains_key("orderId") && !p.contains_key("clientOrderId") =>
        {
            Err(Error::Validation("Options order identity required"))
        }
        "cancelMultipleOptionOrders" => validate_cancel_batch(p),
        "placeMultipleOrders" => validate_place_batch(p),
        "newBlockTradeOrder" => validate_block_order(p),
        "setAutoCancelAllOpenOrders"
            if p.get("countdownTime")
                .and_then(Value::as_i64)
                .is_none_or(|v| v != 0 && v < 5000) =>
        {
            Err(Error::Validation(
                "Options countdown zero or at least 5000 milliseconds",
            ))
        }
        "autoCancelAllOpenOrders" => {
            let names = p
                .get("underlyings")
                .and_then(Value::as_str)
                .ok_or(Error::Validation("Options heartbeat underlyings"))?;
            for name in names.split(',') {
                crate::Symbol::new(name)?;
            }
            Ok(())
        }
        "orderBook"
            if p.get("limit")
                .and_then(Value::as_i64)
                .is_some_and(|v| ![5, 10, 20, 50, 100, 500, 1000].contains(&v)) =>
        {
            Err(Error::Validation("Options depth limit"))
        }
        _ => Ok(()),
    }
}
pub(crate) fn validate_time(p: &BTreeMap<String, Value>, _now: u64) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])
}
/// Only the explicitly documented Options invalid-input/access/rejection codes
/// prove refusal. Internal, network, timeout and future codes remain ambiguous.
pub(crate) fn definitive(status: u16, value: &Value) -> bool {
    status < 500
        && value.get("code").and_then(Value::as_i64).is_some_and(|c| {
            matches!(
                c,
                -1002
                    | -1014
                    | -1015
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
                    | -1111
                    | -1115
                    | -1116
                    | -1117
                    | -1118
                    | -1119
                    | -1120
                    | -1121
                    | -1125
                    | -1127
                    | -1128
                    | -1129
                    | -1130
                    | -1131
                    | -2010
                    | -2013
                    | -2014
                    | -2015
                    | -2018
                    | -2027
                    | -4001
                    | -4002
                    | -4003
                    | -4004
                    | -4005
                    | -4013
                    | -4029
                    | -4030
                    | -4055
                    | -4056
                    | -5010
                    | -6001
                    | -6002
                    | -6003
                    | -6004
                    | -6005
                    | -6006
                    | -6008
                    | -6009
                    | -6011
                    | -6012
                    | -6013
                    | -6073
            )
        })
}

pub(crate) fn validate_three_month_history(
    p: &BTreeMap<String, Value>,
    now: u64,
) -> Result<(), Error> {
    let current = time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(now) * 1_000_000)
        .map_err(|_| Error::Validation("Options history clock range"))?;
    let (year, month, _) = current.date().to_calendar_date();
    let ordinal = year
        .checked_mul(12)
        .and_then(|v| v.checked_add(i32::from(u8::from(month)) - 1))
        .and_then(|v| v.checked_sub(3))
        .ok_or(Error::Validation("Options history calendar range"))?;
    let year = ordinal.div_euclid(12);
    let month = time::Month::try_from(
        u8::try_from(ordinal.rem_euclid(12) + 1)
            .map_err(|_| Error::Validation("Options history month"))?,
    )
    .map_err(|_| Error::Validation("Options history month"))?;
    // Binance documents months without a month-end day convention. Refuse only
    // dates certainly older than three months; the venue owns the boundary day.
    // Preserve the supplied timestamp exactly, with no operator-input rounding.
    let date = time::Date::from_calendar_date(year, month, 1)
        .map_err(|_| Error::Validation("Options history calendar date"))?;
    let oldest_nanos = date
        .with_time(time::Time::MIDNIGHT)
        .assume_utc()
        .unix_timestamp_nanos();
    let oldest = u64::try_from(oldest_nanos.max(0) / 1_000_000)
        .map_err(|_| Error::Validation("Options history timestamp range"))?;
    validate_history_window(p, oldest, now)
}
pub(crate) fn validate_five_day_history(
    p: &BTreeMap<String, Value>,
    now: u64,
) -> Result<(), Error> {
    validate_history_window(p, now.saturating_sub(5 * 24 * 60 * 60 * 1000), now)
}
fn validate_history_window(
    p: &BTreeMap<String, Value>,
    oldest: u64,
    now: u64,
) -> Result<(), Error> {
    for name in ["startTime", "endTime"] {
        if let Some(value) = p.get(name) {
            let value = value
                .as_u64()
                .ok_or(Error::Validation("Options history timestamp"))?;
            if value < oldest || value > now {
                return Err(Error::Validation("Options documented history lookback"));
            }
        }
    }
    Ok(())
}

fn validate_cancel_batch(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    // The current official schema's x-parameters-notes-cn explicitly forbids
    // sending both lists. The English note only says at least one is required.
    // Both facts are retained in schema/options-rest.json for this operation.
    if p.contains_key("orderIds") == p.contains_key("clientOrderIds") {
        return Err(Error::Validation(
            "exactly one Options cancellation identity list",
        ));
    }
    let mut any = false;
    for key in ["orderIds", "clientOrderIds"] {
        if let Some(value) = p.get(key) {
            let values = value
                .as_array()
                .ok_or(Error::Validation("Options cancellation identities"))?;
            if values.is_empty()
                || values.iter().any(|v| {
                    key == "orderIds"
                        && serde_json::from_value::<super::OrderId>(v.clone()).is_err()
                })
            {
                return Err(Error::Validation("Options cancellation identities"));
            }
            any = true;
        }
    }
    if any {
        Ok(())
    } else {
        Err(Error::Validation(
            "Options cancellation identities required",
        ))
    }
}

fn validate_place_batch(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    let values = p
        .get("orders")
        .and_then(Value::as_array)
        .ok_or(Error::Validation("Options batch orders"))?;
    if values.is_empty() || values.len() > 10 {
        return Err(Error::Validation(
            "documented Options ten-order batch limit",
        ));
    }
    let mut identities = BTreeSet::new();
    for value in values {
        let member: BTreeMap<_, _> = value
            .as_object()
            .ok_or(Error::Validation("Options batch member"))?
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        validate("newOrder", &member)?;
        let id = member
            .get("clientOrderId")
            .and_then(Value::as_str)
            .ok_or(Error::Validation("Options batch caller identity"))?;
        if !identities.insert(id.to_owned()) {
            return Err(Error::Validation("duplicate Options batch identity"));
        }
    }
    Ok(())
}

fn validate_block_order(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    let legs = p
        .get("legs")
        .and_then(Value::as_array)
        .ok_or(Error::Validation("Options block legs"))?;
    if legs.len() != 1 {
        return Err(Error::Validation("documented single Options block leg"));
    }
    let leg: BTreeMap<_, _> = legs[0]
        .as_object()
        .ok_or(Error::Validation("Options block leg"))?
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    required(&leg, &["symbol", "side", "type", "quantity", "price"])?;
    validate("blockLeg", &leg)?;
    crate::core::validate_parameters(&leg, &[], &[("type", &["LIMIT"])], &[])
}

fn validate_common(p: &BTreeMap<String, Value>) -> Result<(), Error> {
    for (key, value) in p {
        if value.as_str().is_some_and(str::is_empty) {
            return Err(Error::Validation("empty Options parameter"));
        }
        if matches!(
            key.as_str(),
            "quantity" | "price" | "qtyLimit" | "deltaLimit"
        ) && super::wire::parse_decimal(value)
            .map_err(|_| Error::Validation("Options financial parameter"))?
            <= Decimal::ZERO
        {
            return Err(Error::Validation("positive Options amount"));
        }
        if matches!(
            key.as_str(),
            "startTime" | "endTime" | "frozenTimeInMilliseconds"
        ) && value.as_i64().is_some_and(|v| v < 0)
        {
            return Err(Error::Validation("negative Options time"));
        }
        if key == "recvWindow" && value.as_i64().is_none_or(|v| !(1..=60_000).contains(&v)) {
            return Err(Error::Validation("Options integer recvWindow milliseconds"));
        }
    }
    crate::core::validate_parameters(
        p,
        &[],
        &[
            ("side", &["BUY", "SELL"]),
            ("timeInForce", &["GTC", "IOC", "FOK", "GTX"]),
            ("newOrderRespType", &["ACK", "RESULT"]),
        ],
        &[("limit", 1, 1500)],
    )?;
    if let (Some(start), Some(end)) = (
        p.get("startTime").and_then(Value::as_i64),
        p.get("endTime").and_then(Value::as_i64),
    ) && end < start
    {
        return Err(Error::Validation("Options history time range"));
    }
    Ok(())
}
