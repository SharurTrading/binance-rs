// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated constructors for every documented market stream.

use super::streams::{Route, Stream};
use crate::{Error, Symbol};

impl Stream {
    /// [aggregateTradeStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#aggregate-trade-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn aggregate_trade_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@aggTrade".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "aggregateTradeStreams")
    }

    /// [allMarketLiquidationOrderStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#all-market-liquidation-order-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_market_liquidation_order_streams() -> Result<Self, Error> {
        let name = "!forceOrder@arr".to_owned();
        Self::new(name, Route::Market, "allMarketLiquidationOrderStreams")
    }

    /// [allMarketMiniTickersStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#all-market-mini-tickers-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_market_mini_tickers_stream() -> Result<Self, Error> {
        let name = "!miniTicker@arr".to_owned();
        Self::new(name, Route::Market, "allMarketMiniTickersStream")
    }

    /// [allMarketTickersStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#all-market-tickers-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_market_tickers_streams() -> Result<Self, Error> {
        let name = "!ticker@arr".to_owned();
        Self::new(name, Route::Market, "allMarketTickersStreams")
    }

    /// [compositeIndexSymbolInformationStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#composite-index-symbol-information-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn composite_index_symbol_information_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@compositeIndex".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(
            name,
            Route::Market,
            "compositeIndexSymbolInformationStreams",
        )
    }

    /// [continuousContractKlineCandlestickStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#continuous-contract-kline-candlestick-streams).
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
            "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d",
            "1w", "1M",
        ]
        .contains(&interval)
        {
            return Err(Error::Validation("stream parameter"));
        }
        if ![
            "perpetual",
            "current_quarter",
            "next_quarter",
            "tradifi_perpetual",
        ]
        .contains(&contract_type)
        {
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

    /// [contractInfoStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#contract-info-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn contract_info_stream() -> Result<Self, Error> {
        let name = "!contractInfo".to_owned();
        Self::new(name, Route::Market, "contractInfoStream")
    }

    /// [individualSymbolMiniTickerStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#individual-symbol-mini-ticker-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn individual_symbol_mini_ticker_stream(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@miniTicker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "individualSymbolMiniTickerStream")
    }

    /// [individualSymbolTickerStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#individual-symbol-ticker-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn individual_symbol_ticker_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@ticker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "individualSymbolTickerStreams")
    }

    /// [klineCandlestickStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#kline-candlestick-streams).
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

    /// [liquidationOrderStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#liquidation-order-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn liquidation_order_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@forceOrder".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Market, "liquidationOrderStreams")
    }

    /// [markPriceStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#mark-price-stream).
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

    /// [markPriceStreamForAllMarket](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#mark-price-stream-for-all-market).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn mark_price_stream_for_all_market(update_speed: &str) -> Result<Self, Error> {
        let mut name = "!markPrice@arr@{updateSpeed}".to_owned();
        if !["1s"].contains(&update_speed) {
            return Err(Error::Validation("stream parameter"));
        }
        name = name.replace("{updateSpeed}", update_speed);
        Self::new(name, Route::Market, "markPriceStreamForAllMarket")
    }

    /// [assetIndex](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#asset-index).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn asset_index() -> Result<Self, Error> {
        let name = "!assetIndex@arr".to_owned();
        Self::new(name, Route::Market, "assetIndex")
    }

    /// [tradingSessionStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/market#trading-session-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn trading_session_stream() -> Result<Self, Error> {
        let name = "tradingSession".to_owned();
        Self::new(name, Route::Market, "tradingSessionStream")
    }

    /// [allBookTickersStream](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/public#all-book-tickers-stream).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn all_book_tickers_stream() -> Result<Self, Error> {
        let name = "!bookTicker".to_owned();
        Self::new(name, Route::Public, "allBookTickersStream")
    }

    /// [diffBookDepthStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/public#diff-book-depth-streams).
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
        Self::new(name, Route::Public, "diffBookDepthStreams")
    }

    /// [individualSymbolBookTickerStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/public#individual-symbol-book-ticker-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn individual_symbol_book_ticker_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@bookTicker".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Public, "individualSymbolBookTickerStreams")
    }

    /// [partialBookDepthStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/public#partial-book-depth-streams).
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
        Self::new(name, Route::Public, "partialBookDepthStreams")
    }

    /// [rpiDiffBookDepthStreams](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/ws-streams/public#rpi-diff-book-depth-streams).
    ///
    /// # Errors
    /// Refuses an invalid documented stream parameter.
    pub fn rpi_diff_book_depth_streams(symbol: &Symbol) -> Result<Self, Error> {
        let mut name = "{symbol}@rpiDepth@500ms".to_owned();
        name = name.replace("{symbol}", symbol.as_str().to_lowercase().as_str());
        Self::new(name, Route::Public, "rpiDiffBookDepthStreams")
    }
}
