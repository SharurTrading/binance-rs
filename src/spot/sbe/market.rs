// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Decimal, Error, Symbol};
use serde::{Deserialize, Serialize};

/// Native stream payload; best bid/ask may be culled by the venue before delivery.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MarketEvent {
    /// Raw trades in venue order.
    Trades(Trades),
    /// Current best bid/ask, with venue auto-culling semantics.
    BestBidAsk(BestBidAsk),
    /// Top 20 levels, a finite snapshot.
    DepthSnapshot(DepthSnapshot),
    /// Incremental depth update; zero quantity removes a level.
    DepthDiff(DepthDiff),
}
/// Raw trade event with provider timestamps in microseconds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Trades {
    /// Provider event time, microseconds.
    pub event_time: i64,
    /// Matching-engine time, microseconds.
    pub transact_time: i64,
    /// Native symbol; its assets must come from exchange metadata.
    pub symbol: Symbol,
    /// Every accepted trade, in source order.
    pub trades: Vec<Trade>,
}
/// One native trade; quantity denotes the base asset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Trade {
    /// Venue trade identity.
    pub id: i64,
    /// Exact quote price.
    pub price: Decimal,
    /// Exact base quantity.
    pub qty: Decimal,
    /// Whether buyer was maker.
    pub is_buyer_maker: bool,
    /// Schema constant: always true in stream schema 1:0.
    pub is_best_match: bool,
}
/// Native best prices, preserving source update identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BestBidAsk {
    /// Provider event time, microseconds.
    pub event_time: i64,
    /// Native book update identity. Venue auto-culling means skipped IDs alone
    /// do not establish a client transport loss for this stream.
    pub book_update_id: i64,
    /// Native symbol.
    pub symbol: Symbol,
    /// Exact bid price.
    pub bid_price: Decimal,
    /// Exact bid base quantity.
    pub bid_qty: Decimal,
    /// Exact ask price.
    pub ask_price: Decimal,
    /// Exact ask base quantity.
    pub ask_qty: Decimal,
}
/// One depth price and base quantity. No rounding is performed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Level {
    /// Quote price.
    pub price: Decimal,
    /// Base quantity; zero is a deletion in a diff.
    pub qty: Decimal,
}
/// Finite partial-depth snapshot; never claims a complete book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepthSnapshot {
    /// Provider event time, microseconds.
    pub event_time: i64,
    /// Last native update identity represented by this snapshot.
    pub book_update_id: i64,
    /// Native symbol.
    pub symbol: Symbol,
    /// Returned bid levels only.
    pub bids: Vec<Level>,
    /// Returned ask levels only.
    pub asks: Vec<Level>,
}
/// Native diff IDs, suitable for the documented Spot snapshot/update bootstrap.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DepthDiff {
    /// Provider event time, microseconds.
    pub event_time: i64,
    /// First native update included.
    pub first_book_update_id: i64,
    /// Last native update included.
    pub last_book_update_id: i64,
    /// Native symbol.
    pub symbol: Symbol,
    /// Bid changes in source order.
    pub bids: Vec<Level>,
    /// Ask changes in source order.
    pub asks: Vec<Level>,
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
    header: Option<(u16, u16, u16)>,
}
impl Reader<'_> {
    fn error(&self, reason: &'static str) -> Error {
        Error::BinaryDecode {
            schema_id: self.header.map(|h| h.0),
            version: self.header.map(|h| h.1),
            template_id: self.header.map(|h| h.2),
            offset: self.offset,
            reason,
        }
    }
    fn read<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or_else(|| self.error("offset overflow"))?;
        let result = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| self.error("truncated field"))?
            .try_into()
            .map_err(|_| self.error("field width"))?;
        self.offset = end;
        Ok(result)
    }
    fn i64(&mut self) -> Result<i64, Error> {
        let value = i64::from_le_bytes(self.read()?);
        if value == i64::MIN {
            return Err(self.error("null required market integer"));
        }
        Ok(value)
    }
    fn u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.read()?))
    }
    fn exponent(&mut self) -> Result<i8, Error> {
        let value = i8::from_le_bytes(self.read()?);
        if value == i8::MIN {
            return Err(self.error("null required market exponent"));
        }
        Ok(value)
    }
    fn boolean(&mut self) -> Result<bool, Error> {
        match self.read::<1>()?[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(self.error("unknown boolean")),
        }
    }
    fn decimal(&mut self, exponent: i8) -> Result<Decimal, Error> {
        let mantissa = self.i64()?;
        if mantissa == i64::MIN {
            return Err(self.error("null required mantissa"));
        }
        if exponent == i8::MIN {
            return Err(self.error("null required market exponent"));
        }
        super::schema::exact_decimal(i128::from(mantissa), exponent)
            .ok_or_else(|| self.error("unrepresentable market decimal"))
    }
    fn symbol(&mut self) -> Result<Symbol, Error> {
        let length = usize::from(self.read::<1>()?[0]);
        let end = self
            .offset
            .checked_add(length)
            .ok_or_else(|| self.error("string overflow"))?;
        let b = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| self.error("truncated symbol"))?;
        let text = std::str::from_utf8(b).map_err(|_| self.error("symbol UTF-8"))?;
        let symbol = Symbol::new(text).map_err(|_| self.error("invalid symbol"))?;
        self.offset = end;
        Ok(symbol)
    }
    fn dimensions(&mut self, width: u16, long: bool) -> Result<usize, Error> {
        if self.u16()? != width {
            return Err(self.error("unexpected group block length"));
        }
        let count = if long {
            u32::from_le_bytes(self.read()?)
        } else {
            u32::from(self.u16()?)
        };
        let count = usize::try_from(count).map_err(|_| self.error("group count overflow"))?;
        if count > (self.bytes.len() - self.offset) / usize::from(width) {
            return Err(self.error("truncated group"));
        }
        Ok(count)
    }
    fn levels(&mut self, price: i8, qty: i8) -> Result<Vec<Level>, Error> {
        let count = self.dimensions(16, false)?;
        (0..count)
            .map(|_| {
                Ok(Level {
                    price: self.decimal(price)?,
                    qty: self.decimal(qty)?,
                })
            })
            .collect()
    }
}

/// Decode exactly one market schema 1:0 message. Header lengths, groups and UTF-8
/// are checked before access/allocation; there is no client capacity cutoff.
///
/// # Errors
/// Refuses truncation, unknown schema/version/template, malformed booleans,
/// trailing bytes, null required mantissas and decimals requiring rounding.
pub fn decode_market(bytes: &[u8]) -> Result<MarketEvent, Error> {
    let mut r = Reader {
        bytes,
        offset: 0,
        header: None,
    };
    let block = r.u16()?;
    let template = r.u16()?;
    let schema = r.u16()?;
    let version = r.u16()?;
    r.header = Some((schema, version, template));
    if schema != 1 || version != 0 {
        return Err(r.error("unsupported market schema"));
    }
    let expected = match template {
        10000 | 10002 => 18,
        10001 => 50,
        10003 => 26,
        _ => return Err(r.error("unknown market template")),
    };
    if block != expected {
        return Err(r.error("unexpected root block length"));
    }
    let event_time = r.i64()?;
    let payload = match template {
        10000 => {
            let transact_time = r.i64()?;
            let price = r.exponent()?;
            let qty = r.exponent()?;
            let count = r.dimensions(25, true)?;
            let trades = (0..count)
                .map(|_| {
                    Ok(Trade {
                        id: r.i64()?,
                        price: r.decimal(price)?,
                        qty: r.decimal(qty)?,
                        is_buyer_maker: r.boolean()?,
                        is_best_match: true,
                    })
                })
                .collect::<Result<_, Error>>()?;
            MarketEvent::Trades(Trades {
                event_time,
                transact_time,
                trades,
                symbol: r.symbol()?,
            })
        }
        10001 => {
            let book_update_id = r.i64()?;
            let price = r.exponent()?;
            let qty = r.exponent()?;
            MarketEvent::BestBidAsk(BestBidAsk {
                event_time,
                book_update_id,
                bid_price: r.decimal(price)?,
                bid_qty: r.decimal(qty)?,
                ask_price: r.decimal(price)?,
                ask_qty: r.decimal(qty)?,
                symbol: r.symbol()?,
            })
        }
        10002 => {
            let book_update_id = r.i64()?;
            let price = r.exponent()?;
            let qty = r.exponent()?;
            MarketEvent::DepthSnapshot(DepthSnapshot {
                event_time,
                book_update_id,
                bids: r.levels(price, qty)?,
                asks: r.levels(price, qty)?,
                symbol: r.symbol()?,
            })
        }
        _ => {
            let first_book_update_id = r.i64()?;
            let last_book_update_id = r.i64()?;
            if first_book_update_id > last_book_update_id {
                return Err(r.error("inverted depth update range"));
            }
            let price = r.exponent()?;
            let qty = r.exponent()?;
            MarketEvent::DepthDiff(DepthDiff {
                event_time,
                first_book_update_id,
                last_book_update_id,
                bids: r.levels(price, qty)?,
                asks: r.levels(price, qty)?,
                symbol: r.symbol()?,
            })
        }
    };
    if r.offset != bytes.len() {
        return Err(r.error("trailing market bytes"));
    }
    Ok(payload)
}

pub(super) fn decode_value(bytes: &[u8]) -> Result<serde_json::Value, Error> {
    serde_json::to_value(decode_market(bytes)?).map_err(|_| Error::Gap("SBE model serialization"))
}
