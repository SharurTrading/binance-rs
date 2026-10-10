// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Asset, Error, Outcome,
    core::{Cost, Operation, Request, parameters, validate_parameters},
};
use serde::Serialize;
/// Native quote receipt with both caller-requested asset identities retained.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Quotation {
    /// Source asset selected by the caller.
    pub from_asset: Asset,
    /// Destination asset selected by the caller.
    pub to_asset: Asset,
    /// Caller-selected wallet combination; absent retains the documented venue default.
    pub wallet_type: Option<String>,
    /// Native quote; missing quote ID or expiry cannot authorize acceptance.
    pub receipt: super::rest_models::SendQuoteRequestResponse,
}
/// Acceptance receipt with original quoted amounts and assets for reconciliation.
/// Quoted quantities are not a fabricated fill; inspect the native order status.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Acceptance {
    /// Original quote and request provenance, distinct from execution evidence.
    pub quotation: Quotation,
    /// Native receipt, including string order ID and future status values.
    pub receipt: super::rest_models::AcceptQuoteResponse,
}
impl Acceptance {
    /// Classify only documented acceptance statuses; unknown values stay ambiguous.
    /// Accepted/processing acknowledgments do not prove completion or fills.
    #[must_use]
    pub fn outcome(&self) -> Outcome {
        status_outcome(&self.receipt.order_status)
    }
}
fn status_outcome(status: &super::enums::OrderStatus) -> Outcome {
    use super::enums::OrderStatus;
    match status {
        OrderStatus::Process | OrderStatus::AcceptSuccess | OrderStatus::Success => {
            Outcome::Accepted
        }
        OrderStatus::Fail => Outcome::Rejected,
        OrderStatus::Unknown(_) => Outcome::Unknown,
    }
}
/// Quote acceptance authority constructed from a venue receipt with explicit expiry.
/// Dispatch rechecks venue time after admission; acceptance is never automatically retried.
#[derive(Clone, Debug, Serialize)]
#[must_use]
pub struct AcceptQuote {
    #[serde(rename = "quoteId")]
    quote_id: super::QuoteId,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    recv_window: Option<i64>,
    #[serde(skip)]
    valid_until: u64,
    #[serde(skip)]
    quotation: Quotation,
}
impl AcceptQuote {
    /// Retain a quote's venue identity, expiry, exact amounts and source assets.
    ///
    /// # Errors
    /// Refuses missing quote IDs/expiry, invalid financial magnitudes or invalid time.
    pub fn new(quote: &Quotation) -> Result<Self, Error> {
        let quote_id = quote
            .receipt
            .quote_id
            .clone()
            .ok_or(Error::Validation("quote identity absent"))?;
        let valid_until = u64::try_from(
            quote
                .receipt
                .valid_timestamp
                .ok_or(Error::Validation("quote expiry absent"))?,
        )
        .map_err(|_| Error::Validation("quote expiry"))?;
        if valid_until == 0
            || quote.receipt.from_amount <= crate::Decimal::ZERO
            || quote.receipt.to_amount <= crate::Decimal::ZERO
        {
            return Err(Error::Validation("quote authority"));
        }
        Ok(Self {
            quote_id,
            recv_window: None,
            valid_until,
            quotation: quote.clone(),
        })
    }
    /// Set a millisecond receive window, independently of venue quote expiry.
    #[must_use = "receive-window changes apply to the returned builder"]
    pub fn recv_window(mut self, value: i64) -> Self {
        self.recv_window = Some(value);
        self
    }
    /// Validate receive-window input before dispatch.
    ///
    /// # Errors
    /// Refuses invalid millisecond windows.
    pub fn build(self) -> Result<Self, Error> {
        self.validate()?;
        Ok(self)
    }
    pub(crate) fn quotation(&self) -> &Quotation {
        &self.quotation
    }
}
impl Request for AcceptQuote {
    type Response = super::rest_models::AcceptQuoteResponse;
    const OP: Operation = super::rest_requests::ACCEPT_QUOTE_OPERATION;
    fn validate(&self) -> Result<(), Error> {
        validate_parameters(&parameters(self)?, &["quoteId"], &[], &[])
    }
    fn cost(&self) -> Result<Cost, Error> {
        super::rate::cost(Self::OP, &parameters(self)?)
    }
    fn validate_authority(&self, now: u64) -> Result<(), Error> {
        if now >= self.valid_until {
            return Err(Error::Expired(Self::OP.name));
        }
        Ok(())
    }
}
impl super::rest_models::PlaceLimitOrderResponse {
    /// Preserve status uncertainty rather than inferring acceptance from HTTP 200.
    ///
    /// The Convert catalog documents this `status` field as a bare string with
    /// an example value (`PROCESS`) and no enum — unlike `acceptQuote`'s
    /// `orderStatus`, whose enum is pinned in
    /// `schema/convert-error-codes.json`. A placement acknowledgment is
    /// therefore never a definitive outcome here; the caller reads the exact
    /// native `status` string.
    #[must_use]
    pub fn outcome(&self) -> Outcome {
        Outcome::Unknown
    }
}
impl super::rest_models::CancelLimitOrderResponse {
    /// Preserve status uncertainty; a cancel acknowledgment is not proven removal.
    ///
    /// The Convert catalog documents this `status` field as a bare string with
    /// an example value (`CANCELED`) and no enum, so no literal is pinned and
    /// the caller reads the exact native `status` string.
    #[must_use]
    pub fn outcome(&self) -> Outcome {
        Outcome::Unknown
    }
}
