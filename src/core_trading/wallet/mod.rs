// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Wallet REST operations: preserve native assets, networks and account categories.
//! Every mutation is attempted once. Dropping a future never proves cancellation;
//! reconcile uncertain withdrawals/transfers through the matching venue history.
//! No runtime or background tasks are started. All calls take caller deadlines.
/// Provider-native response enumerations with exact unknown-value retention.
pub mod enums;
pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
mod validation;
mod wire;
pub use crate::core::sapi::Config;
mod identity;
pub use identity::{DustAssets, Network, WithdrawalId};

/// REST client whose clones share endpoint budgets and transport configuration.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
}
impl RestClient {
    /// Create a client without network I/O or a hidden runtime.
    ///
    /// # Errors
    /// Returns configuration errors from HTTP construction.
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

/// Venue wallet balances with the caller-selected quote asset retained explicitly.
/// The asset is request provenance; it is not a fabricated venue response field.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct QuotedWalletBalance {
    /// Caller-selected asset used by Binance for each wallet's `balance`.
    pub quote_asset: crate::Asset,
    /// Native wallet records; no unlike balances are added together.
    pub wallets: rest_models::QueryUserWalletBalanceResponse,
}

/// A dust conversion receipt with the caller-selected target asset retained.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DustConversion {
    /// Request provenance for transferred quantities and service charges.
    pub target_asset: crate::Asset,
    /// The native receipt; this acknowledgment does not authorize replay.
    pub receipt: rest_models::DustConvertResponse,
}
/// Convertible dust amounts with explicit caller-selected target asset provenance.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ConvertibleDust {
    /// Target asset requested from the venue; no reporting asset is selected here.
    pub target_asset: crate::Asset,
    /// Native values, including the venue's separately named quota-asset amounts.
    pub assets: rest_models::DustConvertibleAssetsResponse,
}
