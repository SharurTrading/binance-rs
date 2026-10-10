// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact wire primitives and batch outcomes.

use crate::{Decimal, Error, Outcome, SensitiveString};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
use serde_json::Value;

pub(crate) fn parse_decimal(value: &Value) -> Result<Decimal, &'static str> {
    let text = match value {
        Value::String(value) => value.clone(),
        Value::Number(value) => value.to_string(),
        _ => return Err("decimal must be a string or exact JSON number"),
    };
    Decimal::from_str_exact(&text)
        .or_else(|_| {
            let (base, _) = text
                .split_once(['e', 'E'])
                .ok_or(rust_decimal::Error::ErrorString(
                    "scientific exponent missing".into(),
                ))?;
            Decimal::from_str_exact(base)?;
            Decimal::from_scientific(&text)
        })
        .map_err(|_| "invalid or unrepresentable decimal")
}

pub(crate) fn decimal_rows<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Vec<Decimal>>, D::Error> {
    Vec::<Vec<Value>>::deserialize(d)?
        .into_iter()
        .map(|row| {
            row.iter()
                .map(|value| parse_decimal(value).map_err(serde::de::Error::custom))
                .collect()
        })
        .collect()
}

pub(crate) fn decimal<'de, D: Deserializer<'de>>(d: D) -> Result<Decimal, D::Error> {
    parse_decimal(&Value::deserialize(d)?).map_err(serde::de::Error::custom)
}

pub(crate) fn decimal_option<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Decimal>, D::Error> {
    Option::<Value>::deserialize(d)?
        .map(|v| parse_decimal(&v).map_err(serde::de::Error::custom))
        .transpose()
}

// Only fields with an explicitly documented unavailable sentinel use this parser.
pub(crate) fn decimal_option_empty<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Decimal>, D::Error> {
    match Option::<Value>::deserialize(d)? {
        None => Ok(None),
        Some(Value::String(value)) if value.is_empty() => Ok(None),
        Some(value) => parse_decimal(&value)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

pub(crate) fn decimal_option_null_string<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<Decimal>, D::Error> {
    match Option::<Value>::deserialize(d)? {
        None => Ok(None),
        Some(Value::String(value)) if value == "null" => Ok(None),
        Some(value) => parse_decimal(&value)
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

/// An exact [price, quantity] pair, retaining zero quantities for deletion updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PriceLevel {
    /// Signed price of this level, exactly as sent; zero and negative prices are kept.
    pub price: Decimal,
    /// Absolute quantity at this level, not a delta.
    pub quantity: Decimal,
}
impl PriceLevel {
    /// Construct a Futures price level without rounding.
    ///
    /// The price is signed: the venue states no sign for a level price, so its
    /// sign is the consumer's to judge. The venue documents each level's quantity
    /// as [the absolute quantity for a price level](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/websocket-market-streams/How-to-manage-a-local-order-book-correctly).
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

/// A Futures REST candlestick's documented twelve columns.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Kline {
    /// UTC opening timestamp in milliseconds.
    pub open_time: i64,
    /// Opening price.
    pub open: Decimal,
    /// Highest price.
    pub high: Decimal,
    /// Lowest price.
    pub low: Decimal,
    /// Closing price.
    pub close: Decimal,
    /// Base-asset volume.
    pub volume: Decimal,
    /// UTC closing timestamp in milliseconds.
    pub close_time: i64,
    /// Quote-asset volume.
    pub quote_volume: Decimal,
    /// Trade count.
    pub trades: i64,
    /// Taker buy base-asset volume.
    pub taker_buy_volume: Decimal,
    /// Taker buy quote-asset volume.
    pub taker_buy_quote_volume: Decimal,
    /// Provider's reserved final column, not interpreted as financial truth.
    pub reserved: Value,
}
impl Serialize for Kline {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut tuple = s.serialize_tuple(12)?;
        tuple.serialize_element(&self.open_time)?;
        tuple.serialize_element(&self.open.to_string())?;
        tuple.serialize_element(&self.high.to_string())?;
        tuple.serialize_element(&self.low.to_string())?;
        tuple.serialize_element(&self.close.to_string())?;
        tuple.serialize_element(&self.volume.to_string())?;
        tuple.serialize_element(&self.close_time)?;
        tuple.serialize_element(&self.quote_volume.to_string())?;
        tuple.serialize_element(&self.trades)?;
        tuple.serialize_element(&self.taker_buy_volume.to_string())?;
        tuple.serialize_element(&self.taker_buy_quote_volume.to_string())?;
        tuple.serialize_element(&self.reserved)?;
        tuple.end()
    }
}
impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = Vec::<Value>::deserialize(d)?;
        if v.len() != 12 {
            return Err(serde::de::Error::custom("expected twelve kline columns"));
        }
        let integer = |i| {
            v.get(i)
                .and_then(Value::as_i64)
                .ok_or_else(|| serde::de::Error::custom("integer kline column"))
        };
        let dec = |i| {
            v.get(i)
                .ok_or_else(|| serde::de::Error::custom("decimal kline column"))
                .and_then(|v| parse_decimal(v).map_err(serde::de::Error::custom))
        };
        Ok(Self {
            open_time: integer(0)?,
            open: dec(1)?,
            high: dec(2)?,
            low: dec(3)?,
            close: dec(4)?,
            volume: dec(5)?,
            close_time: integer(6)?,
            quote_volume: dec(7)?,
            trades: integer(8)?,
            taker_buy_volume: dec(9)?,
            taker_buy_quote_volume: dec(10)?,
            reserved: v
                .get(11)
                .cloned()
                .ok_or_else(|| serde::de::Error::custom("reserved kline column"))?,
        })
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
    Unknown(super::event_payloads::UnknownMessage),
}
impl<'de, T: DeserializeOwned> Deserialize<'de> for BatchResult<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;
        if value
            .get("code")
            .and_then(Value::as_i64)
            .is_some_and(|code| code < 0)
        {
            return Ok(serde_json::from_value(value.clone())
                .map_or_else(|_| Self::Unknown(value.into()), Self::Failure));
        }
        Ok(serde_json::from_value(value.clone())
            .map(Self::Success)
            .or_else(|_| serde_json::from_value(value.clone()).map(Self::Failure))
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
    pub extra: super::event_payloads::UnknownMessage,
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
