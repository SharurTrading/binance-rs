// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Binance Options EAPI and routed market/private streams.
//!
//! Native option contracts, underlying identities, settlement assets and Greeks
//! remain distinct from Futures and Spot models. All timestamps use milliseconds.
//! Each money-moving request is attempted once; uncertain outcomes require venue
//! reads. Streams expose generation boundaries and caller-owned driver teardown.
//! Private balance/position events update only the listed assets and contracts.

mod client;
mod config;
pub mod event_payloads;
mod identity;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub mod stream_models;
mod stream_names;
pub mod streams;
mod validation;
pub mod wire;

pub use client::RestClient;
pub use config::{Config, Environment};
pub use crate::core::socket::QueueStats;
pub use identity::{
    BlockOrderMatchingKey, BlockTradeSettlementKey, ClientOrderId, OrderId, RecordId, Symbol,
    TradeId,
};
pub use rate::budget_limits;
pub use streams::{ConnectionDriver, Route, Stream, StreamEvent, Streams};
