// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Decimal, Error};
use jiff::{Timestamp, ToSpan, tz::TimeZone};
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
    let millis =
        i64::try_from(now).map_err(|_| Error::Validation("Options history clock range"))?;
    let current = Timestamp::from_millisecond(millis)
        .map_err(|_| Error::Validation("Options history clock range"))?;
    // Binance documents months without a month-end day convention. Refuse only
    // dates certainly older than three months; the venue owns the boundary day.
    // Preserve the supplied timestamp exactly, with no operator-input rounding.
    let oldest = current
        .to_zoned_datetime(TimeZone::UTC)
        .date()
        .first_of_month()
        .checked_sub(3.months())
        .and_then(|date| date.to_zoned_datetime(TimeZone::UTC))
        .map_err(|_| Error::Validation("Options history calendar range"))?
        .timestamp()
        .as_millisecond();
    let oldest = u64::try_from(oldest.max(0))
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
                    if key == "orderIds" {
                        serde_json::from_value::<super::OrderId>(v.clone()).is_err()
                    } else {
                        serde_json::from_value::<super::ClientOrderId>(v.clone()).is_err()
                    }
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
        ) {
            let amount = super::wire::parse_decimal(value)
                .map_err(|_| Error::Validation("Options financial parameter"))?;
            if key != "price" && amount <= Decimal::ZERO {
                return Err(Error::Validation("positive Options amount"));
            }
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

#[cfg(test)]
mod tests {
    use super::{validate_cancel_batch, validate_three_month_history};
    use crate::Error;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn three_month_lookback_keeps_the_full_earliest_calendar_month() {
        const MAY_31_NOON: u64 = 1_717_156_800_000;
        const FEBRUARY_1: u64 = 1_706_745_600_000;
        let at = |value| BTreeMap::from([("startTime".to_owned(), json!(value))]);
        assert!(validate_three_month_history(&at(FEBRUARY_1), MAY_31_NOON).is_ok());
        assert!(matches!(
            validate_three_month_history(&at(1_706_745_599_999), MAY_31_NOON),
            Err(Error::Validation("Options documented history lookback"))
        ));
    }

    #[test]
    fn cancellation_members_require_native_identity_evidence() {
        for (key, malformed) in [
            ("orderIds", json!("not-an-integer")),
            ("orderIds", json!(1.5)),
            ("orderIds", json!(null)),
            ("clientOrderIds", json!("")),
            ("clientOrderIds", json!("caller id")),
            ("clientOrderIds", json!("caller\n")),
            ("clientOrderIds", json!(5)),
            ("clientOrderIds", json!(null)),
        ] {
            let members = BTreeMap::from([(key.to_owned(), json!([malformed]))]);
            assert!(
                validate_cancel_batch(&members).is_err(),
                "{key}: {members:?}"
            );
        }
        for (key, valid) in [
            ("orderIds", json!([0, -1, "0007"])),
            ("clientOrderIds", json!(["caller-1", "caller-2"])),
        ] {
            let members = BTreeMap::from([(key.to_owned(), valid)]);
            assert!(validate_cancel_batch(&members).is_ok());
        }
    }
}
