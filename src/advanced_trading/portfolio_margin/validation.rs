// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    advanced_trading::validation::{
        Parameters, common, either, forbidden, range, required, text, truth,
    },
};
use serde_json::Value;
pub(crate) fn validate(op: &str, p: &Parameters) -> Result<(), Error> {
    common(p)?;
    if matches!(op, "newCmOrder" | "newUmOrder")
        && text(p, "newClientOrderId").is_some_and(|id| id.len() > 32)
    {
        return Err(Error::Validation(
            "Portfolio Margin UM/CM client order ID exceeds 32 characters",
        ));
    }
    if matches!(op, "newUmOrder" | "newCmOrder" | "newMarginOrder") {
        standard_order(op, p)?;
    }
    if matches!(
        op,
        "cancelCmOrder"
            | "modifyCmOrder"
            | "queryCmOrder"
            | "queryCurrentCmOpenOrder"
            | "queryCmModifyOrderHistory"
            | "cancelUmOrder"
            | "modifyUmOrder"
            | "queryUmOrder"
            | "queryCurrentUmOpenOrder"
            | "queryUmModifyOrderHistory"
            | "cancelMarginAccountOrder"
            | "queryMarginAccountOrder"
    ) {
        either(p, "orderId", "origClientOrderId")?;
    }
    if matches!(op, "newCmConditionalOrder" | "newUmAlgoOrder") {
        conditional_order(op, p)?;
    }
    if matches!(
        op,
        "cancelCmConditionalOrder"
            | "queryCmConditionalOrderHistory"
            | "queryCurrentCmOpenConditionalOrder"
    ) {
        either(p, "strategyId", "newClientStrategyId")?;
    }
    if matches!(op, "cancelUmAlgoOrder" | "queryCurrentUmOpenAlgoOrder") {
        either(p, "algoId", "clientAlgoId")?;
    }
    if matches!(
        op,
        "cancelMarginAccountOcoOrders" | "queryMarginAccountsOco"
    ) {
        either(p, "orderListId", "listClientOrderId")?;
    }
    if matches!(op, "queryMarginLoanRecord" | "queryMarginRepayRecord") {
        either(p, "txId", "startTime")?;
    }
    if op == "marginAccountNewOco" && p.contains_key("stopLimitPrice") {
        required(p, &["stopLimitTimeInForce"])?;
    }
    if op == "fundCollectionByAsset" && text(p, "asset") == Some("BNB") {
        return Err(Error::Validation(
            "Portfolio Margin BNB uses its dedicated transfer endpoint",
        ));
    }
    if matches!(op, "modifyUmOrder" | "modifyCmOrder") {
        either(p, "price", "priceMatch")?;
        if p.contains_key("priceMatch") {
            forbidden(p, &["price"])?;
        }
    }
    if let Some(value) = p.get("callbackRate") {
        let rate = super::wire::parse_decimal(value)
            .map_err(|_| Error::Validation("Portfolio Margin callback rate"))?;
        let maximum = if op == "newCmConditionalOrder" { 5 } else { 10 };
        if rate < crate::Decimal::new(1, 1) || rate > crate::Decimal::from(maximum) {
            return Err(Error::Validation(
                "Portfolio Margin native callback rate range",
            ));
        }
    }
    trade_history(op, p)
}
fn trade_history(op: &str, p: &Parameters) -> Result<(), Error> {
    if matches!(op, "cmAccountTradeList" | "queryAllCmOrders") {
        either(p, "symbol", "pair")?;
        if p.contains_key("symbol") && p.contains_key("pair") {
            return Err(Error::Validation(
                "Portfolio Margin symbol conflicts with pair",
            ));
        }
    }
    if op == "cmAccountTradeList" {
        if p.contains_key("pair") {
            forbidden(p, &["fromId", "orderId"])?;
        }
        range(p, 86_400_000, false)?;
    }
    if matches!(op, "cmAccountTradeList" | "umAccountTradeList") && p.contains_key("fromId") {
        forbidden(p, &["startTime", "endTime"])?;
    }
    let interval = match op {
        "getCmIncomeHistory" => Some((200, false)),
        "getMarginBorrowLoanInterestHistory"
        | "queryMarginLoanRecord"
        | "queryMarginRepayRecord"
        | "queryPortfolioMarginNegativeBalanceInterestHistory" => Some((30, false)),
        "queryAllCmConditionalOrders" | "queryAllUmOrders" | "queryUmAlgoOrderHistory" => {
            Some((7, true))
        }
        "umAccountTradeList" => Some((7, false)),
        "marginAccountTradeList" => Some((1, true)),
        "getDownloadIdForUmFuturesOrderHistory"
        | "getDownloadIdForUmFuturesTradeHistory"
        | "getDownloadIdForUmFuturesTransactionHistory" => Some((365, false)),
        _ => None,
    };
    if let Some((days, strict)) = interval {
        range(p, days * 86_400_000, strict)?;
    }
    Ok(())
}
fn standard_order(op: &str, p: &Parameters) -> Result<(), Error> {
    let margin = op == "newMarginOrder";
    match text(p, "type") {
        Some("LIMIT" | "STOP_LOSS_LIMIT" | "TAKE_PROFIT_LIMIT") => {
            required(p, &["quantity", "timeInForce"])?;
            if !p.contains_key("priceMatch") {
                required(p, &["price"])?;
            }
        }
        Some("LIMIT_MAKER") if margin => required(p, &["quantity", "price"])?,
        Some("MARKET") => {
            if margin {
                either(p, "quantity", "quoteOrderQty")?;
            } else {
                required(p, &["quantity"])?;
            }
        }
        Some("STOP_LOSS" | "TAKE_PROFIT") if margin => required(p, &["quantity", "stopPrice"])?,
        _ => return Err(Error::Validation("Portfolio Margin native order type")),
    }
    if margin
        && matches!(
            text(p, "type"),
            Some("STOP_LOSS_LIMIT" | "TAKE_PROFIT_LIMIT")
        )
    {
        required(p, &["stopPrice"])?;
    }
    if p.contains_key("priceMatch") {
        forbidden(p, &["price"])?;
    }
    if p.contains_key("quoteOrderQty") && p.contains_key("quantity") {
        return Err(Error::Validation(
            "base quantity conflicts with quote spend",
        ));
    }
    hedge(p)?;
    if text(p, "timeInForce") == Some("GTD") {
        required(p, &["goodTillDate"])?;
    }
    Ok(())
}
fn hedge(p: &Parameters) -> Result<(), Error> {
    if text(p, "positionSide").is_some_and(|s| s != "BOTH") && p.contains_key("reduceOnly") {
        return Err(Error::Validation(
            "Portfolio Margin reduceOnly in Hedge Mode",
        ));
    }
    if truth(p, "closePosition") {
        forbidden(p, &["quantity", "reduceOnly"])?;
        if matches!(
            (text(p, "positionSide"), text(p, "side")),
            (Some("LONG"), Some("BUY")) | (Some("SHORT"), Some("SELL"))
        ) {
            return Err(Error::Validation("Portfolio Margin hedge close direction"));
        }
    }
    Ok(())
}
fn conditional_order(op: &str, p: &Parameters) -> Result<(), Error> {
    let kind = text(
        p,
        if op == "newCmConditionalOrder" {
            "strategyType"
        } else {
            "type"
        },
    );
    let trigger = if op == "newCmConditionalOrder" {
        "stopPrice"
    } else {
        "triggerPrice"
    };
    match kind {
        Some("STOP" | "TAKE_PROFIT") => {
            required(p, &["quantity", trigger])?;
            if !p.contains_key("priceMatch") {
                required(p, &["price"])?;
            }
        }
        Some("STOP_MARKET" | "TAKE_PROFIT_MARKET") => {
            required(p, &[trigger])?;
            if !truth(p, "closePosition") {
                required(p, &["quantity"])?;
            }
        }
        Some("TRAILING_STOP_MARKET") => required(p, &["quantity", "callbackRate"])?,
        _ => return Err(Error::Validation("Portfolio Margin conditional order type")),
    }
    if p.contains_key("priceMatch") {
        forbidden(p, &["price"])?;
    }
    if truth(p, "closePosition") && !matches!(kind, Some("STOP_MARKET" | "TAKE_PROFIT_MARKET")) {
        return Err(Error::Validation(
            "Portfolio Margin closePosition order type",
        ));
    }
    hedge(p)
}
pub(crate) fn validate_time(p: &Parameters, now: u64) -> Result<(), Error> {
    if text(p, "timeInForce") == Some("GTD") {
        let date = p
            .get("goodTillDate")
            .and_then(Value::as_i64)
            .and_then(|n| u64::try_from(n).ok())
            .ok_or(Error::Validation("Portfolio Margin GTD timestamp"))?;
        let minimum = now
            .checked_add(600_000)
            .ok_or(Error::Validation("Portfolio Margin GTD overflow"))?;
        if date / 1000 <= minimum / 1000 || date >= 253_402_300_799_000 {
            return Err(Error::Validation("Portfolio Margin GTD timing"));
        }
    }
    Ok(())
}

// Product-local validation/authentication and documented server refusals.
pub(crate) fn definitive(status: u16, value: &serde_json::Value) -> bool {
    // Portfolio Margin General Info explicitly identifies these 503 variants as failures.
    (status == 503
        && (matches!(
            value.get("msg").and_then(Value::as_str),
            Some(
                "Service Unavailable."
                    | "Internal error; unable to process your request. Please try again."
            )
        ) || value.get("code").and_then(Value::as_i64) == Some(-1008)))
        || (status < 500
            && value
                .get("code")
                .and_then(serde_json::Value::as_i64)
                .is_some_and(|code| {
                    matches!(
                        code,
                        -2015
                            | -2014
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

pub(crate) fn response_failed(_op: &str, value: &serde_json::Value) -> bool {
    value.get("success") == Some(&serde_json::Value::Bool(false))
        || value
            .get("code")
            .is_some_and(|code| code.as_i64() != Some(200) && code.as_str() != Some("200"))
}
