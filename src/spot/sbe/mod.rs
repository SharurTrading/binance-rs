// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Spot SBE protocols retain native schemas, timestamp units and financial values.
//! Market schema 1:0 is documented in `docs/core-protocols.md`.

mod market;
pub(crate) mod schema;
mod streams;
pub use market::{
    BestBidAsk, DepthDiff, DepthSnapshot, Level, MarketEvent, Trade, Trades, decode_market,
};
pub use schema::decode_api;
pub use streams::{MarketConfig, MarketStream, MarketStreams, StreamEvent};
