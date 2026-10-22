// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Convert SAPI: quote/limit amounts retain direction and native assets.
//! Every mutation is attempted once. Quotes carry explicit venue expiry authority;
//! acceptance checks it again immediately before sending. Caller cancellation never
//! proves venue cancellation; reconcile unknown outcomes with `order_status`.
//! No hidden runtime, background tasks, wallet selection or accounting is provided.
mod client;
/// Provider-native response enumerations with exact unknown-value retention.
pub mod enums;
pub mod event_payloads;
mod identity;
mod quote;
mod rate;
pub mod rest_models;
pub mod rest_requests;
mod validation;
mod wire;
pub use crate::core::sapi::Config;
pub use client::RestClient;
pub use identity::{AcceptanceOrderId, OrderId, QuoteId};
pub use quote::{AcceptQuote, Acceptance, Quotation};
