// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared transport plumbing for native linear and inverse Futures products.
//! Wire payloads, endpoint catalogs and rate tables remain product-specific.

mod client;
mod config;
mod product;
pub(crate) mod sealed;
pub(crate) mod streams;
pub(crate) mod validation;
mod websocket;

pub use client::RestClient;
pub use config::{Config, Environment};
pub use product::{ApiProduct, Product};
pub use websocket::{ApiEvent, ApiEvents, ConnectionDriver, WsClient};
