// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Retained future REST fields with redacted diagnostics.
use serde_json::Value;
/// Unknown future payloads are retained, with redacted Debug output.
#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UnknownMessage(Value);
impl UnknownMessage {
    /// Explicit provider payload access; do not log sensitive account data.
    #[must_use]
    pub fn as_value(&self) -> &Value {
        &self.0
    }
}
impl std::fmt::Debug for UnknownMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UnknownMessage([REDACTED])")
    }
}
impl From<Value> for UnknownMessage {
    fn from(v: Value) -> Self {
        Self(v)
    }
}

/// Native Margin execution and risk evidence, never projected into Spot account state.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum UserPayload {
    /// Cross-margin risk level and call status; no liquidation policy is selected.
    MarginLevelStatusChange(Box<super::stream_models::MarginLevelStatusChangeEvent>),
    /// Native asset principal and interest update, not a full account snapshot.
    UserLiabilityChange(Box<super::stream_models::UserLiabilityChangeEvent>),
    /// Partial balance delta with asset identity.
    BalanceUpdate(Box<super::stream_models::BalanceUpdateEvent>),
    /// Partial free/locked balance update, never a full account replacement.
    OutboundAccountPosition(Box<super::stream_models::OutboundAccountPositionEvent>),
    /// Order execution with client identity and distinct commission asset.
    ExecutionReport(Box<super::stream_models::ExecutionReportEvent>),
    /// Native order-list state; all legs retained.
    ListStatus(Box<super::stream_models::ListStatusEvent>),
    /// Expired risk listen key; no continuity is claimed across renewal.
    ListenKeyExpired(Box<super::stream_models::ListenKeyExpiredEvent>),
    /// Token subscription expired or explicitly terminated by the venue.
    EventStreamTerminated {
        /// Venue event time, in documented milliseconds.
        event_time: i64,
    },
    /// Future event evidence retained with redacted diagnostics.
    Unknown(UnknownMessage),
}
/// Decode a documented Margin event without inventing missing fields or decimals.
///
/// # Errors
/// Returns an explicit gap for malformed known events or a missing event discriminator.
pub fn user_payload(value: Value) -> Result<UserPayload, crate::Error> {
    let kind = value
        .get("e")
        .and_then(Value::as_str)
        .ok_or(crate::Error::Gap("Margin event type required"))?;
    match kind {
        "MARGIN_LEVEL_STATUS_CHANGE" => {
            serde_json::from_value(value).map(|v| UserPayload::MarginLevelStatusChange(Box::new(v)))
        }
        "USER_LIABILITY_CHANGE" => {
            serde_json::from_value(value).map(|v| UserPayload::UserLiabilityChange(Box::new(v)))
        }
        "balanceUpdate" => {
            serde_json::from_value(value).map(|v| UserPayload::BalanceUpdate(Box::new(v)))
        }
        "outboundAccountPosition" => {
            serde_json::from_value(value).map(|v| UserPayload::OutboundAccountPosition(Box::new(v)))
        }
        "executionReport" => {
            serde_json::from_value(value).map(|v| UserPayload::ExecutionReport(Box::new(v)))
        }
        "listStatus" => serde_json::from_value(value).map(|v| UserPayload::ListStatus(Box::new(v))),
        "listenKeyExpired" => {
            serde_json::from_value(value).map(|v| UserPayload::ListenKeyExpired(Box::new(v)))
        }
        "eventStreamTerminated" => {
            return value
                .get("E")
                .and_then(Value::as_i64)
                .map(|event_time| UserPayload::EventStreamTerminated { event_time })
                .ok_or(crate::Error::Gap("Margin termination event time required"));
        }
        _ => return Ok(UserPayload::Unknown(value.into())),
    }
    .map_err(|_| crate::Error::Gap("malformed Margin user event"))
}
