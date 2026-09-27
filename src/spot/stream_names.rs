// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated constructors for every documented market stream.

use super::streams::{Route, Stream};
use crate::{Error, Symbol};

impl Stream {
    /// [aggTrade](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn agg_trade(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@aggTrade".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "aggTrade")
    }

    /// [allMarketRollingWindowTicker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_market_rolling_window_ticker(window_size: &str) -> Result<Self, Error> {
        let mut name = "!ticker_{windowSize}@arr".to_owned();
        if !["1h", "4h", "1d"].contains(&window_size) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{windowSize}", window_size);
        Self::new(name, Route::Market, "allMarketRollingWindowTicker")
    }

    /// [allMiniTicker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_mini_ticker() -> Result<Self, Error> {
        let name = "!miniTicker@arr".to_owned();
        Self::new(name, Route::Market, "allMiniTicker")
    }

    /// [avgPrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn avg_price(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@avgPrice".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "avgPrice")
    }

    /// [bookTicker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn book_ticker(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@bookTicker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "bookTicker")
    }

    /// [diffBookDepth](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn diff_book_depth(symbol: &Symbol, update_speed: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@depth@{updateSpeed}".to_owned();
        if !["100ms"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "diffBookDepth")
    }

    /// [kline](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn kline(symbol: &Symbol, interval: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@kline_{interval}".to_owned();
        if ![
            "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d",
            "1w", "1M",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{interval}", interval);
        Self::new(name, Route::Market, "kline")
    }

    /// [klineOffset](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn kline_offset(symbol: &Symbol, interval: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@kline_{interval}@+08:00".to_owned();
        if ![
            "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d",
            "1w", "1M",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{interval}", interval);
        Self::new(name, Route::Market, "klineOffset")
    }

    /// [miniTicker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn mini_ticker(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@miniTicker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "miniTicker")
    }

    /// [partialBookDepth](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn partial_book_depth(
        symbol: &Symbol,
        levels: &str,
        update_speed: &str,
    ) -> Result<Self, Error> {
        let mut name = "{symbol}@depth{levels}@{updateSpeed}".to_owned();
        if !["100ms"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        if !["5", "10", "20"].contains(&levels) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{levels}", levels);
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "partialBookDepth")
    }

    /// [referencePrice](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn reference_price(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@referencePrice".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "referencePrice")
    }

    /// [rollingWindowTicker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn rolling_window_ticker(symbol: &Symbol, window_size: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@ticker_{windowSize}".to_owned();
        if !["1h", "4h", "1d"].contains(&window_size) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{windowSize}", window_size);
        Self::new(name, Route::Market, "rollingWindowTicker")
    }

    /// [ticker](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn ticker(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@ticker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "ticker")
    }

    /// [trade](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn trade(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@trade".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "trade")
    }

    /// [blockTrade](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-streams/market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn block_trade(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@blockTrade".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "blockTrade")
    }
}
