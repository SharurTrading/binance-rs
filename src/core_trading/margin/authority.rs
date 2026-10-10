// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;

/// Current venue trading-symbol count authority for all-symbol Margin open-order queries.
///
/// Binance counts this query as one request per symbol currently trading on the exchange.
/// Obtain the exact count from a current authoritative venue listing, such as unfiltered
/// `exchangeInfo` with native `TRADING` status. Do not count only the account's orders,
/// selected symbols, or Margin-enabled pairs. The caller supplies when that evidence
/// expires; the library neither selects a freshness interval nor performs hidden reads.
/// This authority is local admission evidence and is never serialized to Binance.
///
/// Source: <https://developers.binance.com/legacy-docs/margin_trading/trade/Query-Margin-Account-Open-Orders>.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TradingSymbolCount {
    count: u64,
    weight: u64,
    expires_at_millis: u64,
}
impl TradingSymbolCount {
    /// Retain current venue count and its caller-supplied expiry in Unix milliseconds.
    /// Dispatch refuses stale authority before admission and immediately before sending.
    /// The documented ten-weight request is multiplied by the native request count.
    ///
    /// # Errors
    /// Refuses zero count/expiry and multiplication overflow. A representable charge
    /// larger than the venue window remains an admission refusal, not an arbitrary cap.
    pub fn new(count: u64, expires_at_millis: u64) -> Result<Self, Error> {
        if count == 0 || expires_at_millis == 0 {
            return Err(Error::Validation("Margin trading-symbol count authority"));
        }
        let weight = count.checked_mul(10).ok_or(Error::Validation(
            "Margin trading-symbol request weight overflow",
        ))?;
        Ok(Self {
            count,
            weight,
            expires_at_millis,
        })
    }
    /// Number of symbols currently trading in the caller's authoritative venue listing.
    #[must_use]
    pub fn count(self) -> u64 {
        self.count
    }
    /// Caller-selected authority expiry, rechecked against the configured clock.
    #[must_use]
    pub fn expires_at_millis(self) -> u64 {
        self.expires_at_millis
    }
    pub(crate) fn weight(self) -> u64 {
        self.weight
    }
}
