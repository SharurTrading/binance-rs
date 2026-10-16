// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::core::VenueMessage;
use std::{collections::BTreeMap, time::Duration};

/// What the available evidence proves about an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Outcome {
    /// Refused before the wire; the venue did not receive this attempt.
    NotSent,
    /// A documented definitive venue rejection.
    Rejected,
    /// A venue acknowledgment accepted a leg; this does not prove any fill.
    Accepted,
    /// A documented multi-leg operation accepted some work and refused other work.
    Partial,
    /// A mutation may have executed; query venue truth before taking further action.
    Unknown,
    /// A read failed; no mutation was requested.
    ReadFailed,
}

/// Safe classification of a transport or response failure.
///
/// The category does not reveal a raw dependency error, response body, or URL.
/// Header and body categories describe HTTP attempts. Response model decoding is
/// shared with WebSocket replies. A mutation timeout still has an unknown outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransportFailure {
    /// The request timed out before complete response headers arrived.
    ResponseHeadersTimeout,
    /// The request failed before complete response headers arrived.
    ResponseHeadersFailed,
    /// Received rate evidence could not be applied safely.
    RateEvidenceFailed,
    /// Complete response headers arrived, but the body timed out.
    ResponseBodyTimeout,
    /// Complete response headers arrived, but the body could not be read.
    ResponseBodyFailed,
    /// The response body was not valid JSON.
    JsonDecodeFailed,
    /// The response body could not be decoded as the negotiated binary format.
    BinaryDecodeFailed,
    /// The response used an unexpected content type for negotiated binary data.
    UnexpectedContentType,
    /// A decoded response could not be converted to its operation's response type.
    ResponseDecodeFailed,
}

/// Venue rate evidence retained on successes and failures.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RateEvidence {
    /// Safe rate/counter header names and values, including unknown future intervals.
    pub counters: BTreeMap<String, u64>,
    /// A positive `REQUEST_WEIGHT` per-minute limit the reply stated in its
    /// `rateLimits` alongside the counters, for the window they report; absent
    /// when it states none or its limit evidence is unusable, leaving the
    /// owner's existing limit standing.
    pub stated_weight_per_minute: Option<u64>,
    /// A counted rate-limit statement was malformed. No limit from that
    /// statement was adopted. A WebSocket response still retains its venue
    /// outcome and counter evidence; its driver also emits a continuity gap.
    /// Account order queries fail with retained response metadata.
    pub stated_limits_unusable: bool,
    /// HTTP Retry-After seconds, or WebSocket retry deadline converted to a duration.
    pub retry_after: Option<Duration>,
    /// The venue sent retry timing this client cannot use, so no expiry is invented
    /// and every send sharing the owner is refused until the caller releases it with
    /// `release_unknown_ban`.
    pub retry_after_unusable: bool,
}

/// Transport response metadata independent of product payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ResponseMeta {
    /// Units selected for the provider timestamps in this response.
    pub time_unit: super::TimeUnit,
    /// Caller-supplied reconciliation identities keyed by parameter paths, including
    /// client, native order, quote and withdrawal IDs; numbers retain exact decimal text.
    pub client_order_ids: BTreeMap<String, String>,
    /// HTTP-equivalent status.
    pub status: u16,
    /// The operation name; never a signed URI.
    pub operation: &'static str,
    /// Provider rate evidence.
    pub rates: RateEvidence,
}

/// A decoded product-native response together with its rate evidence.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Response<T> {
    /// Product-native response data. Batch responses preserve each member's outcome.
    pub data: T,
    /// Evidence from this attempt.
    pub meta: ResponseMeta,
}

/// Typed evidence for an unsuccessful venue response, with venue text redacted.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct VenueFailure {
    /// Caller-supplied order identities retained for reconciliation.
    pub client_order_ids: BTreeMap<String, String>,
    /// HTTP-equivalent status.
    pub status: u16,
    /// Operation identity, excluding parameters and secrets.
    pub operation: &'static str,
    /// Venue error code, absent if the error envelope itself was malformed.
    pub code: Option<i64>,
    /// Exact native string code, including leading zeros, when the product uses
    /// a string envelope. Redacted because future values may contain sensitive text.
    pub text_code: Option<super::SensitiveString>,
    /// Native unsuccessful/unknown submission status, retained without inventing
    /// acceptance from HTTP success. This is not an order lifecycle status.
    pub service_status: Option<super::SensitiveString>,
    /// Exact service success flag when the product supplies one.
    pub service_success: Option<bool>,
    /// Exact venue explanation when `msg` or `message` is a string. Explicit access
    /// is required because the venue may echo sensitive request data in this field.
    pub message: Option<VenueMessage>,
    /// Classification of the operation's execution outcome.
    pub outcome: Outcome,
    /// Rate evidence, even when the payload could not be read or decoded.
    pub rates: RateEvidence,
    /// Safe, independent leg evidence from a documented partial-operation response.
    /// Financial/account payloads are not stored in errors; query venue state for fills.
    pub partial: Option<Box<PartialOperation>>,
}

/// Safe evidence for each leg of a multi-operation request, keyed by wire leg name.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PartialOperation {
    /// Every venue-reported leg remains, including unknown outcomes. Empty when
    /// the product reports only an aggregate partial outcome without leg identities.
    pub legs: BTreeMap<String, OperationLeg>,
    /// Strongest documented overall execution evidence.
    pub outcome: Outcome,
}

/// Identifiers and execution evidence for one operation leg; never a raw body.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct OperationLeg {
    /// Strongest outcome supported by the documented result and venue code.
    pub outcome: Outcome,
    /// Venue error code, including unknown future codes.
    pub code: Option<i64>,
    /// Venue order identity when an acknowledgment supplies it.
    pub order_id: Option<i64>,
    /// Caller order identity returned by the venue.
    pub client_order_id: Option<String>,
}

/// Client errors never retain the transport request URL or raw response body.
/// Venue explanation fields may echo sensitive URLs and are redacted when formatted.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Malformed FIX evidence, excluding raw sensitive fields.
    #[error("FIX decode failure: {reason} at byte {offset}")]
    FixDecode {
        /// Native tag when known.
        tag: Option<u32>,
        /// Source byte offset.
        offset: usize,
        /// Static safe diagnostic.
        reason: &'static str,
    },
    /// Invalid binary protocol evidence; no raw sensitive payload is retained.
    #[error("binary decode failure: {reason} at byte {offset}")]
    BinaryDecode {
        /// Schema ID if its header could be read.
        schema_id: Option<u16>,
        /// Schema version if its header could be read.
        version: Option<u16>,
        /// Native template ID if its header could be read.
        template_id: Option<u16>,
        /// Byte offset of the refused field.
        offset: usize,
        /// Static safe diagnostic.
        reason: &'static str,
    },
    /// Invalid input, rejected before network admission.
    #[error("invalid input: {0}")]
    Validation(&'static str),
    /// Invalid client configuration.
    #[error("invalid configuration: {0}")]
    Configuration(&'static str),
    /// Signing failed before network admission.
    #[error("signing failed")]
    Signing,
    /// An operation requiring credentials was refused before sending.
    #[error("credentials required")]
    CredentialsRequired,
    /// A budget or venue cooldown refused admission.
    #[error("rate admission refused; retry after {retry_after:?}")]
    Admission {
        /// Earliest estimated delay for a new attempt; no automatic retry is performed.
        retry_after: Duration,
    },
    /// Venue retry timing was unusable, or an IP ban supplied no timing. No expiry is invented;
    /// all clients sharing this owner refuse sends until the caller decides the ban
    /// has ended and calls `release_unknown_ban` on a configuration or client of the
    /// pool ([`crate::Budgets::release_unknown_ban`] for an explicit owner). The release
    /// clears only this refusal; a cooldown with a known `Retry-After` still holds.
    #[error("venue IP cooldown timing is unknown")]
    CooldownTimingUnknown,
    /// An unsent command expired.
    #[error("deadline expired before sending {0}")]
    Expired(&'static str),
    /// An unsent command was refused before any byte reached the socket. The
    /// reason is the caller's own lifecycle, not a venue deadline or verdict.
    #[error("{operation} was not sent: {reason}")]
    NotSent {
        /// Caller-supplied reconciliation identities of the refused command.
        client_order_ids: BTreeMap<String, String>,
        /// Operation identity that never reached the venue.
        operation: &'static str,
        /// Static safe diagnostic.
        reason: &'static str,
    },
    /// Venue response, including ambiguous mutations.
    #[error("venue response: {0:?}")]
    Venue(Box<VenueFailure>),
    /// Transport, response-body, or payload decoding failed.
    #[error("{operation} failed with {outcome:?}")]
    Transport {
        /// Caller-supplied order identities retained after uncertain sends.
        client_order_ids: BTreeMap<String, String>,
        /// Operation identity.
        operation: &'static str,
        /// The strongest outcome supported by the evidence.
        outcome: Outcome,
        /// Safe HTTP failure category, if this error came from an HTTP attempt.
        /// WebSocket transport errors carry `None`; the shared response decoder
        /// can also report `ResponseDecodeFailed` for a WebSocket reply.
        failure: Option<TransportFailure>,
        /// Metadata received before failure, including header counters.
        meta: Option<Box<ResponseMeta>>,
    },
    /// The venue refused a market-stream control message; nothing in it took effect.
    #[error("{operation} refused by the venue with code {code}")]
    ControlRefused {
        /// Control method, such as `SUBSCRIBE`.
        operation: &'static str,
        /// Venue error code, including unknown future codes.
        code: i64,
        /// The venue's own explanation, when it sent one.
        message: Option<VenueMessage>,
    },
    /// Correlation ID reused in a socket generation.
    #[error("duplicate WebSocket request ID")]
    DuplicateRequestId,
    /// Socket lifecycle has ended.
    #[error("socket is closed")]
    Closed,
    /// Malformed stream data or disproven depth continuity.
    #[error("stream continuity lost: {0}")]
    Gap(&'static str),
    /// A managed task could not be joined.
    #[error("socket task failed")]
    Task,
}

impl Error {
    pub(crate) fn with_order_ids(mut self, ids: BTreeMap<String, String>) -> Self {
        match &mut self {
            Self::Venue(v) => v.client_order_ids = ids,
            Self::Transport {
                client_order_ids, ..
            }
            | Self::NotSent {
                client_order_ids, ..
            } => *client_order_ids = ids,
            _ => (),
        }
        self
    }
    /// The execution outcome, where this error belongs to an operation.
    #[must_use]
    pub fn outcome(&self) -> Option<Outcome> {
        match self {
            Self::Venue(e) => Some(e.outcome),
            Self::Transport { outcome, .. } => Some(*outcome),
            Self::ControlRefused { .. } => Some(Outcome::Rejected),
            Self::Gap(_)
            | Self::BinaryDecode { .. }
            | Self::FixDecode { .. }
            | Self::Task
            | Self::Closed => None,
            _ => Some(Outcome::NotSent),
        }
    }
}

pub(crate) fn failure(
    operation: &'static str,
    mutation: bool,
    status: u16,
    value: &serde_json::Value,
    rates: RateEvidence,
) -> Error {
    failure_classified(
        operation,
        mutation,
        status,
        value,
        rates,
        futures_definitive(status, value),
    )
}

pub(crate) fn futures_definitive(status: u16, value: &serde_json::Value) -> bool {
    let code = value.get("code").and_then(serde_json::Value::as_i64);
    let msg = value
        .get("msg")
        .or_else(|| value.get("message"))
        .and_then(serde_json::Value::as_str);
    // Only documented execution evidence is definitive. Unknown future codes stay unknown.
    code.is_some_and(|c| {
        matches!(
            c,
            -1008
                | -1013
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
                | -1108
                | -1109
                | -1110
                | -1111
                | -1112
                | -1114
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
                | -1130
                | -1136
                | -2010
                | -2011
                | -2014
                | -2015
                | -2022
                | -2025
                | -2027
                | -2028
        )
    }) || (status == 503
        && matches!(
            msg,
            // Pinned in schema/futures-general-info.json against the documented
            // 503 variants of both Futures general-info pages: each message
            // below is documented as "a failure API operation" (order certainly
            // not accepted, resend allowed). The sibling variants "Unknown
            // error, please check your request or try again later." and the
            // 5XX "Request occur unknown error." are documented
            // execution-unknown and must never match here.
            Some(
                "Service Unavailable."
                    | "Internal error; unable to process your request. Please try again."
            )
        ))
}

pub(crate) fn failure_for(
    op: super::Operation,
    status: u16,
    value: &serde_json::Value,
    rates: RateEvidence,
) -> Error {
    let mut error = failure_classified(
        op.name,
        op.mutation,
        status,
        value,
        rates,
        (op.definitive)(status, value),
    );
    if let Some(parse) = op.partial
        && let Some(partial) = parse(status, value)
        && let Error::Venue(venue) = &mut error
    {
        venue.outcome = partial.outcome;
        venue.partial = Some(Box::new(partial));
    }
    error
}
fn failure_classified(
    operation: &'static str,
    mutation: bool,
    status: u16,
    value: &serde_json::Value,
    rates: RateEvidence,
    definitive: bool,
) -> Error {
    let code = value.get("code").and_then(serde_json::Value::as_i64);
    let message = value
        .get("msg")
        .or_else(|| value.get("message"))
        .and_then(serde_json::Value::as_str)
        .map(VenueMessage::new);
    let outcome = if !mutation {
        Outcome::ReadFailed
    } else if definitive {
        Outcome::Rejected
    } else {
        Outcome::Unknown
    };
    Error::Venue(Box::new(VenueFailure {
        client_order_ids: BTreeMap::new(),
        status,
        operation,
        code,
        text_code: value
            .get("code")
            .and_then(serde_json::Value::as_str)
            .map(super::SensitiveString::new),
        service_status: value
            .get("status")
            .and_then(serde_json::Value::as_str)
            .map(super::SensitiveString::new),
        service_success: value.get("success").and_then(serde_json::Value::as_bool),
        message,
        outcome,
        rates,
        partial: None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn venue_message_uses_existing_msg_precedence_and_retains_only_strings() {
        for (value, expected) in [
            (
                json!({"code":-1021,"msg":"first","message":"second"}),
                Some("first"),
            ),
            (json!({"code":-1021,"message":"fallback"}), Some("fallback")),
            (json!({"code":-1021,"msg":""}), Some("")),
            (json!({"code":-1021}), None),
            (json!({"code":-1021,"msg":null}), None),
            (json!({"code":-1021,"msg":7,"message":"ignored"}), None),
        ] {
            let Error::Venue(failure) =
                failure("new_order", true, 400, &value, RateEvidence::default())
            else {
                panic!("missing venue failure")
            };
            assert_eq!(
                failure
                    .message
                    .as_ref()
                    .map(VenueMessage::expose_sensitive_text),
                expected
            );
            assert_eq!(failure.code, Some(-1021));
            assert_eq!(failure.outcome, Outcome::Rejected);
        }
    }
}
