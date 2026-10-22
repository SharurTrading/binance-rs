// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Spot Trading: asset balances, order amounts, and transport semantics.
//!
//! JSON catalog coverage includes advanced order lists and partial cancel/replace.
//! Native FIX/SBE protocols have separate modules; remaining evidence is tracked in
//! [issue #11](https://github.com/SharurTrading/binance-rs/issues/11).
//!
//! All operations use caller deadlines. The library never retries a mutation.
//! Construct `Config` with an explicit demo/production environment; it draws on
//! the process's IP weight pool for that environment. WebSocket drivers run on
//! caller-owned tasks.

mod client;
mod config;
pub mod enums;
pub mod event_payloads;
mod exit_reads;
pub mod fix;
mod identity;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub mod sbe;
pub mod stream_models;
mod stream_names;
pub mod streams;
mod validation;
mod websocket;
pub mod wire;
pub mod ws_models;
pub mod ws_requests;

pub use client::RestClient;
pub use config::{Config, Environment};
pub use crate::core::socket::QueueStats;
pub use identity::ClientOrderId;
pub use streams::{Route, Stream, StreamEvent, Streams};
pub use websocket::{ApiEvent, ApiEvents, ConnectionDriver, WsClient};
