// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    advanced_trading::validation::{Parameters, common, either, text},
};
use serde_json::Value;
pub(crate) fn validate(op: &str, p: &Parameters) -> Result<(), Error> {
    common(p)?;
    if op.starts_with("cancelAlgoOrder") {
        either(p, "algoId", "clientAlgoId")?;
    }
    if text(p, "positionSide").is_some_and(|s| s != "BOTH") && p.contains_key("reduceOnly") {
        return Err(Error::Validation(
            "Algo reduceOnly is forbidden in Hedge Mode",
        ));
    }
    if p.get("algoId")
        .and_then(Value::as_i64)
        .is_some_and(|id| id <= 0)
    {
        return Err(Error::Validation("Algo order identity"));
    }
    Ok(())
}
pub(crate) use crate::advanced_trading::validation::validate_time;

// Product-local validation/authentication refusals; unknown and 5xx remain ambiguous.
pub(crate) fn definitive(status: u16, value: &serde_json::Value) -> bool {
    // Native refusal codes pinned in schema/algo-error-codes.json.
    crate::advanced_trading::validation::definitive(
        status,
        value,
        &[
            -2015, -2014, -1131, -1130, -1128, -1127, -1121, -1111, -1106, -1105, -1104, -1103,
            -1102, -1101, -1100, -1022, -1021, -1020, -1002,
        ],
    )
}

pub(crate) fn response_failed(_op: &str, value: &serde_json::Value) -> bool {
    value.get("success") == Some(&serde_json::Value::Bool(false))
        || value
            .get("code")
            .and_then(serde_json::Value::as_i64)
            .is_some_and(|code| code != 0)
}
