// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated constructors for every documented market stream.

use super::Symbol;
use super::streams::{Route, Stream};
use crate::Error;

impl Stream {
    /// [indexPriceStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#index-price-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn index_price_streams() -> Result<Self, Error> {
        let name = "!index@arr".to_owned();
        Self::new(name, Route::Market, "indexPriceStreams")
    }

    /// [klineCandlestickStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#kline-candlestick-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn kline_candlestick_streams(symbol: &Symbol, interval: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@kline_{interval}".to_owned();
        if ![
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d", "3d", "1w",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        if interval.is_empty() {
            return Err(Error::Validation("required stream parameter"));
        }
        name = name.replace("{interval}", interval);
        Self::new(name, Route::Market, "klineCandlestickStreams")
    }

    /// [optionMarkPrice](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#option-mark-price).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn option_mark_price(underlying: &crate::Symbol) -> Result<Self, Error> {
        let mut name = "{underlying}@optionMarkPrice".to_owned();
        name = name.replace("{underlying}", underlying.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "optionMarkPrice")
    }

    /// [newSymbolInfo](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#new-symbol-info).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn new_symbol_info() -> Result<Self, Error> {
        let name = "!optionSymbol".to_owned();
        Self::new(name, Route::Market, "newSymbolInfo")
    }

    /// [openInterest](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/market#open-interest).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn open_interest(underlying: &crate::Symbol, expiration_date: &str) -> Result<Self, Error> {
        let mut name = "{underlying}@openInterest@{expirationDate}".to_owned();
        name = name.replace("{underlying}", underlying.as_str().to_lowercase().as_str());
        if expiration_date.is_empty() {
            return Err(Error::Validation("required stream parameter"));
        }
        name = name.replace("{expirationDate}", expiration_date);
        Self::new(name, Route::Market, "openInterest")
    }

    /// [diffBookDepthStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#diff-book-depth-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn diff_book_depth_streams(symbol: &Symbol, update_speed: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@depth@{updateSpeed}".to_owned();
        if !["100ms", "500ms"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        if update_speed.is_empty() {
            return Err(Error::Validation("required stream parameter"));
        }
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Public, "diffBookDepthStreams")
    }

    /// [individualSymbolBookTickerStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#individual-symbol-book-ticker-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn individual_symbol_book_ticker_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@bookTicker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Public, "individualSymbolBookTickerStreams")
    }

    /// [partialBookDepthStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#partial-book-depth-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn partial_book_depth_streams(
        symbol: &Symbol,
        level: &str,
        update_speed: &str,
    ) -> Result<Self, Error> {
        let mut name = "{symbol}@depth{level}@{updateSpeed}".to_owned();
        if !["100ms", "500ms"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        if !["5", "10", "20"].contains(&level) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        if level.is_empty() {
            return Err(Error::Validation("required stream parameter"));
        }
        name = name.replace("{level}", level);
        if update_speed.is_empty() {
            return Err(Error::Validation("required stream parameter"));
        }
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Public, "partialBookDepthStreams")
    }

    /// [hour24Ticker](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#hour24-ticker).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn hour24_ticker(symbol: &Symbol, expiration_date: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@optionTicker{expirationDate}".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{expirationDate}", expiration_date);
        Self::new(name, Route::Public, "hour24Ticker")
    }

    /// [tradeStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams/public#trade-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn trade_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@optionTrade".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Public, "tradeStreams")
    }
}
