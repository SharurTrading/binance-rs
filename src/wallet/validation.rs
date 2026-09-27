// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Decimal, Error};
use serde_json::Value;
use std::collections::BTreeMap;
fn required(p: &BTreeMap<String, Value>, keys: &[&str]) -> Result<(), Error> {
    if keys.iter().any(|k| !p.contains_key(*k)) {
        return Err(Error::Validation("required Wallet parameter"));
    }
    Ok(())
}
pub(crate) fn validate(op: &str, p: &BTreeMap<String, Value>) -> Result<(), Error> {
    for (key, value) in p {
        if value.as_str().is_some_and(str::is_empty) {
            return Err(Error::Validation("empty Wallet parameter"));
        }
        if key == "asset"
            && let Some(values) = value.as_array()
        {
            if values.is_empty() {
                return Err(Error::Validation("empty Wallet assets"));
            }
            for value in values {
                crate::Asset::new(value.as_str().ok_or(Error::Validation("asset identity"))?)?;
            }
        }
        if key.ends_with("Price")
            && super::wire::parse_decimal(value).map_err(|_| Error::Validation("Wallet price"))?
                <= Decimal::ZERO
        {
            return Err(Error::Validation("positive Wallet price"));
        }
    }
    if let (Some(start), Some(end)) = (
        p.get("startTime").and_then(Value::as_i64),
        p.get("endTime").and_then(Value::as_i64),
    ) && (start < 0 || end < start)
    {
        return Err(Error::Validation("Wallet time range"));
    }
    match op {
        "withdraw" | "withdrawTravelRule" | "brokerWithdraw" => required(p, &["withdrawOrderId"]),
        "dustConvert" => required(p, &["clientId", "targetAsset"]),
        "queryUserWalletBalance" => required(p, &["quoteAsset"]),
        "oneClickArrivalDepositApply"
            if !p.contains_key("depositId") && !p.contains_key("txId") =>
        {
            Err(Error::Validation("deposit identity required"))
        }
        "userUniversalTransfer" | "queryUserUniversalTransferHistory" => {
            match p.get("type").and_then(Value::as_str) {
                Some("ISOLATEDMARGIN_MARGIN") => required(p, &["fromSymbol"]),
                Some("MARGIN_ISOLATEDMARGIN") => required(p, &["toSymbol"]),
                Some("ISOLATEDMARGIN_ISOLATEDMARGIN") => required(p, &["fromSymbol", "toSymbol"]),
                _ => Ok(()),
            }
        }
        _ => Ok(()),
    }
}
pub(crate) fn validate_time(p: &BTreeMap<String, Value>, _now: u64) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])
}
// Wallet error-code documentation is independent evidence; unknown/5xx remain ambiguous.
pub(crate) fn definitive(status: u16, value: &Value) -> bool {
    status < 500
        && value
            .get("code")
            .and_then(Value::as_i64)
            .is_some_and(|code| {
                matches!(
                    code,
                    -1002
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
                        | -1121
                        | -1127
                        | -1128
                        | -1130
                        | -1131
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
