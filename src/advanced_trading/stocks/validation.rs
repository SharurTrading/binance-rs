// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Decimal, Error,
    advanced_trading::validation::{Parameters, common, either, forbidden, required, text},
};
pub(crate) fn validate(op: &str, p: &Parameters) -> Result<(), Error> {
    common(p)?;
    if let Some(symbol) = text(p, "symbol")
        && !symbol
            .chars()
            .all(|c| c.is_ascii_uppercase() || matches!(c, '.' | '-'))
    {
        return Err(Error::Validation(
            "Stocks uses uppercase native equity tickers",
        ));
    }
    if op == "equityOrderDetail" {
        either(p, "orderId", "clientOrderId")?;
    }
    if op == "placeEquityOrder" {
        if text(p, "timeInForce") == Some("GTC") && text(p, "orderType") != Some("LIMIT") {
            return Err(Error::Validation("Stocks GTC is limited to LIMIT orders"));
        }
        match (text(p, "side"), text(p, "orderType")) {
            (Some("BUY" | "SELL"), Some("LIMIT")) => {
                required(p, &["price", "quantity", "tradingSession"])?;
                forbidden(p, &["notional"])?;
            }
            (Some("BUY"), Some("MARKET")) => {
                required(p, &["notional"])?;
                forbidden(p, &["price", "quantity", "tradingSession"])?;
            }
            (Some("SELL"), Some("MARKET")) => {
                required(p, &["quantity"])?;
                forbidden(p, &["price", "notional", "tradingSession"])?;
            }
            _ => return Err(Error::Validation("Stocks native side/order type")),
        }
        if let Some(price) = p.get("price") {
            let price = super::wire::parse_decimal(price)
                .map_err(|_| Error::Validation("Stocks limit price"))?;
            if price.normalize().scale() > 2 {
                return Err(Error::Validation(
                    "Stocks limit price exceeds two decimal places",
                ));
            }
        }
        let fractional = p.contains_key("notional")
            || p.get("quantity")
                .map(super::wire::parse_decimal)
                .transpose()
                .map_err(|_| Error::Validation("Stocks quantity"))?
                .is_some_and(|q| !q.fract().is_zero());
        if fractional
            && text(p, "timeInForce") == Some("GTC")
            && !matches!(text(p, "tradingSession"), Some("EXTENDED" | "24H"))
        {
            return Err(Error::Validation("Stocks fractional GTC session"));
        }
    }
    for key in ["notional", "underlyingAssetAmount", "tokenizedAssetAmount"] {
        if let Some(v) = p.get(key)
            && super::wire::parse_decimal(v)
                .map_err(|_| Error::Validation("Stocks native amount"))?
                <= Decimal::ZERO
        {
            return Err(Error::Validation("Stocks positive native amount"));
        }
    }
    Ok(())
}
pub(crate) fn validate_time(p: &Parameters, _now: u64) -> Result<(), Error> {
    crate::core::validate_parameters(p, &[], &[], &[])
}

// Product-local validation/authentication refusals; unknown and 5xx remain ambiguous.
pub(crate) fn definitive(status: u16, value: &serde_json::Value) -> bool {
    status < 500
        && (value.get("status").and_then(serde_json::Value::as_str) == Some("F")
            || value
                .get("code")
                .and_then(serde_json::Value::as_i64)
                .is_some_and(|code| {
                    matches!(
                        code,
                        -26004
                            | 486_405
                            | 486_410
                            | -2015
                            | -2014
                            | -1131
                            | -1130
                            | -1128
                            | -1127
                            | -1121
                            | -1111
                            | -1106
                            | -1105
                            | -1104
                            | -1103
                            | -1102
                            | -1101
                            | -1100
                            | -1022
                            | -1021
                            | -1020
                            | -1002
                    )
                }))
}

pub(crate) fn response_failed(op: &str, value: &serde_json::Value) -> bool {
    value.get("success") == Some(&serde_json::Value::Bool(false))
        || value
            .get("code")
            .and_then(serde_json::Value::as_i64)
            .is_some_and(|code| code != 0 && code != 200)
        || match op {
            "placeEquityOrder" | "cancelEquityOrder" => value
                .get("status")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|s| s != "S"),
            "tokenizedMint" | "tokenizedRedeem" => value
                .get("status")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|s| !matches!(s, "S" | "P")),
            _ => false,
        }
}

// Stocks reports only aggregate partial-cancellation evidence, without leg IDs.
pub(crate) fn partial(_status: u16, value: &serde_json::Value) -> Option<crate::PartialOperation> {
    (value.get("code").and_then(serde_json::Value::as_i64) == Some(486_447)).then(|| {
        crate::PartialOperation {
            legs: std::collections::BTreeMap::new(),
            outcome: crate::Outcome::Partial,
        }
    })
}
