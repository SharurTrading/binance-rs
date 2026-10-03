// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Decimal, Error};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn validate(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    for (key, value) in p {
        if value.as_str().is_some_and(str::is_empty) {
            return Err(Error::Validation("empty Convert parameter"));
        }
        if matches!(key.as_str(), "baseAmount" | "quoteAmount" | "limitPrice")
            && super::wire::parse_decimal(value).map_err(|_| Error::Validation("Convert amount"))?
                <= Decimal::ZERO
        {
            return Err(Error::Validation("positive Convert amount"));
        }
    }
    match op {
        "sendQuoteRequest" => {
            if p.contains_key("fromAmount") == p.contains_key("toAmount") {
                return Err(Error::Validation("exactly one Convert amount"));
            }
            if p.get("fromAsset") == p.get("toAsset") {
                return Err(Error::Validation("distinct Convert assets"));
            }
        }
        "placeLimitOrder" => {
            if p.contains_key("baseAmount") == p.contains_key("quoteAmount") {
                return Err(Error::Validation("exactly one Convert limit amount"));
            }
            if p.get("baseAsset") == p.get("quoteAsset") {
                return Err(Error::Validation("distinct Convert limit assets"));
            }
        }
        "orderStatus" if !p.contains_key("orderId") && !p.contains_key("quoteId") => {
            return Err(Error::Validation("Convert order or quote identity"));
        }
        "getConvertTradeHistory" => {
            let start = p
                .get("startTime")
                .and_then(Value::as_i64)
                .ok_or(Error::Validation("Convert history start"))?;
            let end = p
                .get("endTime")
                .and_then(Value::as_i64)
                .ok_or(Error::Validation("Convert history end"))?;
            if start < 0 || end < start || end.checked_sub(start).is_none_or(|v| v > 2_592_000_000)
            {
                return Err(Error::Validation("Convert history maximum 30 days"));
            }
        }
        _ => (),
    }
    Ok(())
}
pub(crate) fn validate_time(p: &BTreeMap<String, Value>, _now: u64) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])
}
// Convert's own error-code page documents these definitive request refusals;
// its general-info keeps all 5xx outcomes unknown, regardless of a familiar
// code. Pinned in schema/convert-error-codes.json against the product's
// documented error-code page (verified 2026-10-03), which no longer lists
// -1002 (folded into the ambiguous -1001 family). The matching-engine
// rejection codes -2010/-2011 are Convert's only divergence from Wallet.
pub(crate) fn definitive(status: u16, value: &Value) -> bool {
    status < 500
        && value
            .get("code")
            .and_then(Value::as_i64)
            .is_some_and(|code| {
                matches!(
                    code,
                    -1020
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
                        | -1121
                        | -1127
                        | -1128
                        | -1130
                        | -1131
                        | -2010
                        | -2011
                        | -2014
                        | -2015
                        | -3001
                        | -3002
                        | -3004
                        | -3005
                        | -3011
                        | -3018
                        | -3019
                        | -3020
                        | -3026
                )
            })
}
