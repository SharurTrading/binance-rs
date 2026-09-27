// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated constructors for every documented market stream.

use super::streams::{Route, Stream};
use crate::{Error, Symbol};

impl Stream {
    /// [aggregateTradeStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#aggregate-trade-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn aggregate_trade_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@aggTrade".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "aggregateTradeStreams")
    }

    /// [allBookTickersStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#all-book-tickers-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_book_tickers_stream() -> Result<Self, Error> {
        let name = "!bookTicker".to_owned();
        Self::new(name, Route::Market, "allBookTickersStream")
    }

    /// [allMarketLiquidationOrderStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#all-market-liquidation-order-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_market_liquidation_order_streams() -> Result<Self, Error> {
        let name = "!forceOrder@arr".to_owned();
        Self::new(name, Route::Market, "allMarketLiquidationOrderStreams")
    }

    /// [allMarketMiniTickersStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#all-market-mini-tickers-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_market_mini_tickers_stream() -> Result<Self, Error> {
        let name = "!miniTicker@arr".to_owned();
        Self::new(name, Route::Market, "allMarketMiniTickersStream")
    }

    /// [allMarketTickersStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#all-market-tickers-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_market_tickers_streams() -> Result<Self, Error> {
        let name = "!ticker@arr".to_owned();
        Self::new(name, Route::Market, "allMarketTickersStreams")
    }

    /// [continuousContractKlineCandlestickStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#continuous-contract-kline-candlestick-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn continuous_contract_kline_candlestick_streams(
        pair: &Symbol,
        contract_type: &str,
        interval: &str,
    ) -> Result<Self, Error> {
        let mut name = "{pair}_{contractType}@continuousKline_{interval}".to_owned();
        if ![
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w",
            "1M",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        if !["perpetual", "current_quarter", "next_quarter"].contains(&contract_type) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{pair}", pair.as_str().to_lowercase().as_str());
        name = name.replace("{contractType}", contract_type);
        name = name.replace("{interval}", interval);
        Self::new(
            name,
            Route::Market,
            "continuousContractKlineCandlestickStreams",
        )
    }

    /// [contractInfoStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#contract-info-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn contract_info_stream() -> Result<Self, Error> {
        let name = "!contractInfo".to_owned();
        Self::new(name, Route::Market, "contractInfoStream")
    }

    /// [diffBookDepthStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#diff-book-depth-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn diff_book_depth_streams(symbol: &Symbol, update_speed: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@depth@{updateSpeed}".to_owned();
        if !["100ms", "500ms"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "diffBookDepthStreams")
    }

    /// [indexKlineCandlestickStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#index-kline-candlestick-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn index_kline_candlestick_streams(pair: &Symbol, interval: &str) -> Result<Self, Error> {
        let mut name = "{pair}@indexPriceKline_{interval}".to_owned();
        if ![
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w",
            "1M",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{pair}", pair.as_str().to_lowercase().as_str());
        name = name.replace("{interval}", interval);
        Self::new(name, Route::Market, "indexKlineCandlestickStreams")
    }

    /// [indexPriceStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#index-price-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn index_price_stream(pair: &Symbol, update_speed: &str) -> Result<Self, Error> {
        let mut name = "{pair}@indexPrice@{updateSpeed}".to_owned();
        if !["1s"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{pair}", pair.as_str().to_lowercase().as_str());
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "indexPriceStream")
    }

    /// [individualSymbolBookTickerStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#individual-symbol-book-ticker-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn individual_symbol_book_ticker_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@bookTicker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "individualSymbolBookTickerStreams")
    }

    /// [individualSymbolMiniTickerStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#individual-symbol-mini-ticker-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn individual_symbol_mini_ticker_stream(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@miniTicker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "individualSymbolMiniTickerStream")
    }

    /// [individualSymbolTickerStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#individual-symbol-ticker-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn individual_symbol_ticker_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@ticker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "individualSymbolTickerStreams")
    }

    /// [klineCandlestickStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#kline-candlestick-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn kline_candlestick_streams(symbol: &Symbol, interval: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@kline_{interval}".to_owned();
        if ![
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w",
            "1M",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{interval}", interval);
        Self::new(name, Route::Market, "klineCandlestickStreams")
    }

    /// [marketLiquidationOrderStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#market-liquidation-order-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn market_liquidation_order_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@forceOrder".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "marketLiquidationOrderStreams")
    }

    /// [markPriceKlineCandlestickStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#mark-price-kline-candlestick-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn mark_price_kline_candlestick_streams(
        symbol: &Symbol,
        interval: &str,
    ) -> Result<Self, Error> {
        let mut name = "{symbol}@markPriceKline_{interval}".to_owned();
        if ![
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w",
            "1M",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{interval}", interval);
        Self::new(name, Route::Market, "markPriceKlineCandlestickStreams")
    }

    /// [markPriceOfAllSymbolsOfAPair](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#mark-price-of-all-symbols-of-apair).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn mark_price_of_all_symbols_of_a_pair(
        pair: &Symbol,
        update_speed: &str,
    ) -> Result<Self, Error> {
        let mut name = "{pair}@markPrice@{updateSpeed}".to_owned();
        if !["1s"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{pair}", pair.as_str().to_lowercase().as_str());
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "markPriceOfAllSymbolsOfAPair")
    }

    /// [markPriceStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#mark-price-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn mark_price_stream(symbol: &Symbol, update_speed: &str) -> Result<Self, Error> {
        let mut name = "{symbol}@markPrice@{updateSpeed}".to_owned();
        if !["1s"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "markPriceStream")
    }

    /// [partialBookDepthStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/ws-streams/market-data#partial-book-depth-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn partial_book_depth_streams(
        symbol: &Symbol,
        levels: &str,
        update_speed: &str,
    ) -> Result<Self, Error> {
        let mut name = "{symbol}@depth{levels}@{updateSpeed}".to_owned();
        if !["100ms", "500ms"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        if !["5", "10", "20"].contains(&levels) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        name = name.replace("{levels}", levels);
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "partialBookDepthStreams")
    }
}
