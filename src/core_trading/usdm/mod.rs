// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! USDⓈ-M Futures: linear contract, account, and transport semantics.
//!
//! All operations use caller deadlines. The library never retries a mutation.
//! Construct `Config` with an explicit demo/production environment; it draws on
//! the process's IP weight pool for that environment. WebSocket drivers run on
//! caller-owned tasks.

mod config;
pub mod enums;
pub mod event_payloads;
mod exit_reads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub mod stream_models;
mod stream_names;
pub mod streams;
mod validation;
mod websocket;
pub mod wire;
pub mod ws_models;
pub mod ws_requests;

pub use crate::core::socket::QueueStats;
pub use config::{Config, Environment, Protocol};
pub use streams::{Route, Stream, StreamEvent, Streams};
pub use websocket::{ApiEvent, ApiEvents, ConnectionDriver, WsClient};

/// Cloneable REST client sharing native transport pools and venue budgets.
pub type RestClient = super::futures::RestClient<Protocol>;
