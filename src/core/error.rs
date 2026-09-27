// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use std::{collections::BTreeMap, time::Duration};

/// What the available evidence proves about an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Outcome {
    /// Refused before the wire; the venue did not receive this attempt.
    NotSent,
    /// A documented definitive venue rejection.
    Rejected,
    /// A mutation may have executed; query venue truth before taking further action.
    Unknown,
    /// A read failed; no mutation was requested.
    ReadFailed,
}

/// Venue rate evidence retained on successes and failures.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RateEvidence {
    /// Safe rate/counter header names and values, including unknown future intervals.
    pub counters: BTreeMap<String, u64>,
    /// HTTP Retry-After seconds, or WebSocket retry deadline converted to a duration.
    pub retry_after: Option<Duration>,
}

/// Transport response metadata independent of product payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ResponseMeta {
    /// Caller-supplied order identities keyed by parameter paths, including batch indices.
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

/// Safe, typed evidence for an unsuccessful venue response.
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
    /// Classification of the operation's execution outcome.
    pub outcome: Outcome,
    /// Rate evidence, even when the payload could not be read or decoded.
    pub rates: RateEvidence,
}

/// Client errors never retain a request URL, secret, or raw sensitive body.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
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
    /// An unsent command expired.
    #[error("deadline expired before sending {0}")]
    Expired(&'static str),
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
        /// Metadata received before failure, including header counters.
        meta: Option<Box<ResponseMeta>>,
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
            Self::Gap(_) | Self::Task | Self::Closed => None,
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
    failure_classified(
        op.name,
        op.mutation,
        status,
        value,
        rates,
        (op.definitive)(status, value),
    )
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
        outcome,
        rates,
    }))
}
