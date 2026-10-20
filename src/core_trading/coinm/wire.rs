// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact wire primitives and batch outcomes.

use crate::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

pub(crate) use crate::core::wire::{
    decimal, decimal_empty, decimal_option, decimal_option_empty, decimal_option_null_string,
    decimal_rows, parse_decimal,
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
    /// Contract volume.
    pub contract_volume: Decimal,
    /// UTC closing timestamp in milliseconds.
    pub close_time: i64,
    /// Base-asset volume.
    pub base_volume: Decimal,
    /// Trade count.
    pub trades: i64,
    /// Taker buy contract volume.
    pub taker_buy_contract_volume: Decimal,
    /// Taker buy base-asset volume.
    pub taker_buy_base_volume: Decimal,
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
        tuple.serialize_element(&self.contract_volume.to_string())?;
        tuple.serialize_element(&self.close_time)?;
        tuple.serialize_element(&self.base_volume.to_string())?;
        tuple.serialize_element(&self.trades)?;
        tuple.serialize_element(&self.taker_buy_contract_volume.to_string())?;
        tuple.serialize_element(&self.taker_buy_base_volume.to_string())?;
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
            contract_volume: dec(5)?,
            close_time: integer(6)?,
            base_volume: dec(7)?,
            trades: integer(8)?,
            taker_buy_contract_volume: dec(9)?,
            taker_buy_base_volume: dec(10)?,
            reserved: v
                .get(11)
                .cloned()
                .ok_or_else(|| serde::de::Error::custom("reserved kline column"))?,
        })
    }
}

/// Price-only index, mark, or premium candlestick; ignored columns are not volumes.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PriceKline {
    /// Opening timestamp in milliseconds.
    pub open_time: i64,
    /// Opening price or premium.
    pub open: Decimal,
    /// Highest price or premium.
    pub high: Decimal,
    /// Lowest price or premium.
    pub low: Decimal,
    /// Closing price or premium.
    pub close: Decimal,
    /// Closing timestamp in milliseconds.
    pub close_time: i64,
    /// Venue's ignored columns at indices 5, 7, 8, 9, 10, and 11, retained exactly.
    pub ignored_columns: [Value; 6],
}
impl Serialize for PriceKline {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let mut tuple = s.serialize_tuple(12)?;
        tuple.serialize_element(&self.open_time)?;
        for price in [self.open, self.high, self.low, self.close] {
            tuple.serialize_element(&price.to_string())?;
        }
        tuple.serialize_element(&self.ignored_columns[0])?;
        tuple.serialize_element(&self.close_time)?;
        for value in &self.ignored_columns[1..] {
            tuple.serialize_element(value)?;
        }
        tuple.end()
    }
}
impl<'de> Deserialize<'de> for PriceKline {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let values = <[Value; 12]>::deserialize(d)?;
        let integer = |i: usize| {
            values[i]
                .as_i64()
                .ok_or_else(|| serde::de::Error::custom("integer price-kline timestamp"))
        };
        let price = |i: usize| parse_decimal(&values[i]).map_err(serde::de::Error::custom);
        Ok(Self {
            open_time: integer(0)?,
            open: price(1)?,
            high: price(2)?,
            low: price(3)?,
            close: price(4)?,
            close_time: integer(6)?,
            ignored_columns: [
                values[5].clone(),
                values[7].clone(),
                values[8].clone(),
                values[9].clone(),
                values[10].clone(),
                values[11].clone(),
            ],
        })
    }
}
