// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Convert SAPI: quote/limit amounts retain direction and native assets.
//! Every mutation is attempted once. Quotes carry explicit venue expiry authority;
//! acceptance checks it again immediately before sending. Caller cancellation never
//! proves venue cancellation; reconcile unknown outcomes with `order_status`.
//! No hidden runtime, background tasks, wallet selection or accounting is provided.
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
pub use identity::{AcceptanceOrderId, OrderId, QuoteId};
pub use quote::{AcceptQuote, Acceptance, Quotation};
/// Cloneable REST client sharing caller-supplied endpoint IP/UID budgets.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
}
impl RestClient {
    /// Construct a transport without network I/O or runtime ownership.
    ///
    /// # Errors
    /// Returns invalid transport configuration errors.
    pub fn new(config: Config) -> Result<Self, crate::Error> {
        Ok(Self {
            inner: crate::core::HttpClient::new(
                config.rest,
                config.credentials,
                config.clock,
                config.budgets,
                config.timeout,
                config.proxy,
            )?,
        })
    }
    /// Clear an IP ban the venue gave no timing for, on the SAPI pool this client
    /// draws on; see [`Config::release_unknown_ban`].
    ///
    /// # Errors
    /// Returns a configuration error if the pool's lock is poisoned.
    pub fn release_unknown_ban(&self) -> Result<(), crate::Error> {
        self.inner.release_unknown_ban()
    }
}
