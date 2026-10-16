// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native stream dispatch with retained future-event evidence.
pub use crate::core_trading::wallet::event_payloads::UnknownMessage;
/// A native event, without cross-product account or instrument normalization.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Payload {
    /// Native `accountConfigUpdate` evidence.
    AccountConfigUpdate(Box<super::stream_models::AccountConfigUpdateEvent>),
    /// Native `accountUpdate` evidence.
    AccountUpdate(Box<super::stream_models::AccountUpdateEvent>),
    /// Native `algoOrderUpdate` evidence.
    AlgoOrderUpdate(Box<super::stream_models::AlgoOrderUpdateEvent>),
    /// Native `balanceUpdate` evidence.
    BalanceUpdate(Box<super::stream_models::BalanceUpdateEvent>),
    /// Native `conditionalOrderTradeUpdate` evidence.
    ConditionalOrderTradeUpdate(Box<super::stream_models::ConditionalOrderTradeUpdateEvent>),
    /// Native `executionReport` evidence.
    ExecutionReport(Box<super::stream_models::ExecutionReportEvent>),
    /// Native `liabilityChange` evidence.
    LiabilityChange(Box<super::stream_models::LiabilityChangeEvent>),
    /// Native `listenKeyExpired` evidence.
    ListenKeyExpired(Box<super::stream_models::ListenKeyExpiredEvent>),
    /// Native `openOrderLoss` evidence.
    OpenOrderLoss(Box<super::stream_models::OpenOrderLossEvent>),
    /// Native `orderTradeUpdate` evidence.
    OrderTradeUpdate(Box<super::stream_models::OrderTradeUpdateEvent>),
    /// Native `outboundAccountPosition` evidence.
    OutboundAccountPosition(Box<super::stream_models::OutboundAccountPositionEvent>),
    /// Native `riskLevelChange` evidence.
    RiskLevelChange(Box<super::stream_models::RiskLevelChangeEvent>),
    /// Future native event or stream shape, retained with redacted diagnostics.
    Unknown(UnknownMessage),
}
/// Decode one named market stream, or a user event when `kind` is absent.
///
/// # Errors
/// Returns a gap for malformed known evidence, including financial parse failures.
pub fn decode(kind: Option<&str>, value: serde_json::Value) -> Result<Payload, crate::Error> {
    if kind.is_none() {
        let event = value
            .get("e")
            .and_then(serde_json::Value::as_str)
            .ok_or(crate::Error::Gap("native user event type required"))?;
        if event == "ACCOUNT_UPDATE"
            && value
                .get("a")
                .is_none_or(|a| a.get("B").is_none() && a.get("P").is_none())
        {
            return Err(crate::Error::Gap(
                "native account update has no balance or position evidence",
            ));
        }
        let commission = match event {
            "ORDER_TRADE_UPDATE" => value.get("o"),
            "executionReport" => Some(&value),
            _ => None,
        };
        if let Some(commission) = commission {
            if let Some(amount) = commission.get("n") {
                let amount = super::wire::parse_decimal(amount)
                    .map_err(|_| crate::Error::Gap("native commission amount"))?;
                if !amount.is_zero() && commission.get("N").is_none_or(serde_json::Value::is_null) {
                    return Err(crate::Error::Gap("native commission asset required"));
                }
            } else if commission.get("N").is_some_and(|asset| !asset.is_null()) {
                return Err(crate::Error::Gap("native commission amount required"));
            }
        }

        return match event {
            "ACCOUNT_CONFIG_UPDATE" => decode_value(value, Payload::AccountConfigUpdate),
            "ACCOUNT_UPDATE" => decode_value(value, Payload::AccountUpdate),
            "ALGO_UPDATE" => decode_value(value, Payload::AlgoOrderUpdate),
            "balanceUpdate" => decode_value(value, Payload::BalanceUpdate),
            "CONDITIONAL_ORDER_TRADE_UPDATE" => {
                decode_value(value, Payload::ConditionalOrderTradeUpdate)
            }
            "executionReport" => decode_value(value, Payload::ExecutionReport),
            "liabilityChange" => decode_value(value, Payload::LiabilityChange),
            "listenKeyExpired" => decode_value(value, Payload::ListenKeyExpired),
            "openOrderLoss" => decode_value(value, Payload::OpenOrderLoss),
            "ORDER_TRADE_UPDATE" => decode_value(value, Payload::OrderTradeUpdate),
            "outboundAccountPosition" => decode_value(value, Payload::OutboundAccountPosition),
            "riskLevelChange" => decode_value(value, Payload::RiskLevelChange),
            _ => Ok(Payload::Unknown(value.into())),
        };
    }
    match kind {
        Some("accountConfigUpdate") => decode_market(value, None, Payload::AccountConfigUpdate),
        Some("accountUpdate") => decode_market(value, None, Payload::AccountUpdate),
        Some("algoOrderUpdate") => decode_market(value, None, Payload::AlgoOrderUpdate),
        Some("balanceUpdate") => decode_market(value, None, Payload::BalanceUpdate),
        Some("conditionalOrderTradeUpdate") => {
            decode_market(value, None, Payload::ConditionalOrderTradeUpdate)
        }
        Some("executionReport") => decode_market(value, None, Payload::ExecutionReport),
        Some("liabilityChange") => decode_market(value, None, Payload::LiabilityChange),
        Some("listenKeyExpired") => decode_market(value, None, Payload::ListenKeyExpired),
        Some("openOrderLoss") => decode_market(value, None, Payload::OpenOrderLoss),
        Some("orderTradeUpdate") => decode_market(value, None, Payload::OrderTradeUpdate),
        Some("outboundAccountPosition") => {
            decode_market(value, None, Payload::OutboundAccountPosition)
        }
        Some("riskLevelChange") => decode_market(value, None, Payload::RiskLevelChange),
        _ => Ok(Payload::Unknown(value.into())),
    }
}

fn decode_value<T: serde::de::DeserializeOwned>(
    value: serde_json::Value,
    wrap: fn(Box<T>) -> Payload,
) -> Result<Payload, crate::Error> {
    serde_json::from_value(value)
        .map(|value| wrap(Box::new(value)))
        .map_err(|_| crate::Error::Gap("malformed native stream event"))
}
fn decode_market<T: serde::de::DeserializeOwned>(
    value: serde_json::Value,
    event: Option<&str>,
    wrap: fn(Box<T>) -> Payload,
) -> Result<Payload, crate::Error> {
    if event.is_some_and(|expected| {
        value
            .get("e")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|actual| actual != expected)
    }) {
        return Ok(Payload::Unknown(value.into()));
    }
    decode_value(value, wrap)
}
