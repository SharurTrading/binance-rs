// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact financial wire values for SAPI.
use crate::Decimal;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub(crate) use crate::core::wire::{
    decimal, decimal_option, decimal_option_empty as decimal_empty_option,
    decimal_option_null_string, parse_decimal,
};

/// An exact native depth price/quantity pair; zero quantity remains a deletion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct PriceLevel {
    /// Exact price, retaining its reported sign.
    pub price: Decimal,
    /// Exact quantity, retaining zero removal evidence.
    pub quantity: Decimal,
}
impl serde::Serialize for PriceLevel {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&[self.price.to_string(), self.quantity.to_string()], s)
    }
}
impl<'de> Deserialize<'de> for PriceLevel {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let [p, q] = <[Value; 2]>::deserialize(d)?;
        Ok(Self {
            price: parse_decimal(&p).map_err(serde::de::Error::custom)?,
            quantity: parse_decimal(&q).map_err(serde::de::Error::custom)?,
        })
    }
}
/// Alpha's documented twelve-column candlestick with exact financial values.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Kline {
    /// Opening UTC epoch milliseconds.
    pub open_time: i64,
    /// Opening price.
    pub open: Decimal,
    /// Highest price.
    pub high: Decimal,
    /// Lowest price.
    pub low: Decimal,
    /// Closing price.
    pub close: Decimal,
    /// Base asset volume.
    pub volume: Decimal,
    /// Closing UTC epoch milliseconds.
    pub close_time: i64,
    /// Quote asset volume.
    pub quote_volume: Decimal,
    /// Native trade count.
    pub trades: i64,
    /// Taker buy base volume.
    pub taker_buy_base_volume: Decimal,
    /// Taker buy quote volume.
    pub taker_buy_quote_volume: Decimal,
    /// Reserved final venue column, kept exactly.
    pub reserved: Value,
}
impl serde::Serialize for Kline {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut row = s.serialize_tuple(12)?;
        row.serialize_element(&self.open_time)?;
        for value in [self.open, self.high, self.low, self.close, self.volume] {
            row.serialize_element(&value.to_string())?;
        }
        row.serialize_element(&self.close_time)?;
        row.serialize_element(&self.quote_volume.to_string())?;
        row.serialize_element(&self.trades)?;
        row.serialize_element(&self.taker_buy_base_volume.to_string())?;
        row.serialize_element(&self.taker_buy_quote_volume.to_string())?;
        row.serialize_element(&self.reserved)?;
        row.end()
    }
}
impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        fn integer(v: &Value) -> Result<i64, &'static str> {
            match v {
                Value::String(v) => v.parse().map_err(|_| "invalid kline integer"),
                Value::Number(v) => v.as_i64().ok_or("invalid kline integer"),
                _ => Err("invalid kline integer"),
            }
        }
        let [
            open_time,
            open,
            high,
            low,
            close,
            volume,
            close_time,
            quote_volume,
            trades,
            buy,
            quote,
            reserved,
        ] = <[Value; 12]>::deserialize(d)?;
        let decimal = |v: &Value| parse_decimal(v).map_err(serde::de::Error::custom);
        Ok(Self {
            open_time: integer(&open_time).map_err(serde::de::Error::custom)?,
            open: decimal(&open)?,
            high: decimal(&high)?,
            low: decimal(&low)?,
            close: decimal(&close)?,
            volume: decimal(&volume)?,
            close_time: integer(&close_time).map_err(serde::de::Error::custom)?,
            quote_volume: decimal(&quote_volume)?,
            trades: integer(&trades).map_err(serde::de::Error::custom)?,
            taker_buy_base_volume: decimal(&buy)?,
            taker_buy_quote_volume: decimal(&quote)?,
            reserved,
        })
    }
}
