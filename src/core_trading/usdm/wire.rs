// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact wire primitives and batch outcomes.

use crate::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub(crate) use crate::core::wire::{
    decimal, decimal_option, decimal_option_empty, decimal_option_null_string, decimal_rows,
    parse_decimal,
};

pub use super::super::futures_wire::{BatchFailure, BatchResult, PriceLevel};

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
