// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared Futures wire shapes with identical provider semantics.

use crate::core::wire::{UnknownMessage, parse_decimal};
use crate::{Decimal, Error, Outcome, SensitiveString};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
use serde_json::Value;

/// An exact [price, quantity] pair, retaining zero quantities for deletion updates.
///
/// Quantities can only enter through the validated constructor or deserializer.
///
/// ```compile_fail
/// use binance_client::{Decimal, usdm::wire::PriceLevel};
/// let mut level = PriceLevel::new(Decimal::ONE, Decimal::ONE)?;
/// level.quantity = -Decimal::ONE;
/// # Ok::<(), binance_client::Error>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PriceLevel {
    /// Signed price of this level, exactly as sent; zero and negative prices are kept.
    price: Decimal,
    /// Absolute quantity at this level, not a delta.
    quantity: Decimal,
}
impl PriceLevel {
    /// Signed price exactly as stated by the venue.
    #[must_use]
    pub fn price(&self) -> Decimal {
        self.price
    }

    /// Absolute quantity; zero removes a level.
    #[must_use]
    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    /// Construct a Futures price level without rounding.
    ///
    /// The price is signed: the venue states no sign for a level price, so its
    /// sign is the consumer's to judge. The venue documents each level's quantity
    /// as the absolute quantity for a price level in both the
    /// [USDⓈ-M](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/websocket-market-streams/How-to-manage-a-local-order-book-correctly)
    /// and [COIN-M](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/websocket-market-streams/How-to-manage-a-local-order-book-correctly) documentation.
    ///
    /// # Errors
    /// Refuses a negative quantity, which an absolute level quantity cannot be.
    pub fn new(price: Decimal, quantity: Decimal) -> Result<Self, Error> {
        if quantity < Decimal::ZERO {
            return Err(Error::Validation("price level"));
        }
        Ok(Self { price, quantity })
    }
}
impl<'de> Deserialize<'de> for PriceLevel {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let [price, quantity] = <[Value; 2]>::deserialize(d)?;
        Self::new(
            parse_decimal(&price).map_err(serde::de::Error::custom)?,
            parse_decimal(&quantity).map_err(serde::de::Error::custom)?,
        )
        .map_err(serde::de::Error::custom)
    }
}
impl Serialize for PriceLevel {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        [self.price.to_string(), self.quantity.to_string()].serialize(s)
    }
}

/// A member of a batch: successes and failures remain in original input order.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum BatchResult<T> {
    /// Successful order evidence.
    Success(T),
    /// Venue refusal or uncertainty for this individual member.
    Failure(BatchFailure),
    /// Malformed or future member evidence, retained without losing other receipts.
    /// Resolve this member through venue reads before considering another submit.
    Unknown(UnknownMessage),
}
impl<'de, T: DeserializeOwned> Deserialize<'de> for BatchResult<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        if value
            .get("code")
            .and_then(Value::as_i64)
            .is_some_and(|code| code < 0)
        {
            return Ok(serde::Deserialize::deserialize(&value)
                .map_or_else(|_| Self::Unknown(value.into()), Self::Failure));
        }
        Ok(serde::Deserialize::deserialize(&value)
            .map(Self::Success)
            .or_else(|_| serde::Deserialize::deserialize(&value).map(Self::Failure))
            .unwrap_or_else(|_| Self::Unknown(value.into())))
    }
}

/// Per-member failure evidence. The message is redacted by default.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BatchFailure {
    /// Venue code for this member.
    pub code: i64,
    /// Raw provider diagnostic, available only through explicit sensitive-string access.
    pub msg: Option<SensitiveString>,
    /// Additional provider evidence, with redacted Debug output.
    #[serde(flatten)]
    pub extra: UnknownMessage,
}
impl BatchFailure {
    /// Classify only documented rejection evidence; unknown codes remain unknown.
    #[must_use]
    pub fn outcome(&self) -> Outcome {
        let value = serde_json::json!({"code":self.code});
        match crate::core::error::failure(
            "batchMember",
            true,
            400,
            &value,
            crate::RateEvidence::default(),
        ) {
            Error::Venue(v) => v.outcome,
            _ => Outcome::Unknown,
        }
    }
}
