// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native account evidence with caller-supplied provenance retained.

/// Venue wallet balances with the caller-selected quote asset retained explicitly.
/// The asset is request provenance; it is not a fabricated venue response field.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct QuotedWalletBalance {
    /// Caller-selected asset used by Binance for each wallet's `balance`.
    pub quote_asset: crate::Asset,
    /// Native wallet records; no unlike balances are added together.
    pub wallets: super::rest_models::QueryUserWalletBalanceResponse,
}

/// A dust conversion receipt with the caller-selected target asset retained.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DustConversion {
    /// Request provenance for transferred quantities and service charges.
    pub target_asset: crate::Asset,
    /// The native receipt; this acknowledgment does not authorize replay.
    pub receipt: super::rest_models::DustConvertResponse,
}

/// Convertible dust amounts with explicit caller-selected target asset provenance.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ConvertibleDust {
    /// Target asset requested from the venue; no reporting asset is selected here.
    pub target_asset: crate::Asset,
    /// Native values, including the venue's separately named quota-asset amounts.
    pub assets: super::rest_models::DustConvertibleAssetsResponse,
}
