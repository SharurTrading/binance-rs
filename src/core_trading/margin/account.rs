// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Margin account evidence: borrow/transfer capacity, inventory and listen tokens.

/// Native borrowing capacity paired with request asset and margin mode provenance.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct BorrowCapacity {
    /// Asset whose amount the venue returned.
    pub asset: crate::Asset,
    /// Isolated symbol when requested; absence means cross margin.
    pub isolated_symbol: Option<crate::Symbol>,
    /// Native capacity, without inferred asset or accounting calculations.
    pub capacity: super::rest_models::QueryMaxBorrowResponse,
}
/// Native transfer capacity paired with request asset and margin mode provenance.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct TransferCapacity {
    /// Asset whose amount the venue returned.
    pub asset: crate::Asset,
    /// Isolated symbol when requested; absence means cross margin.
    pub isolated_symbol: Option<crate::Symbol>,
    /// Native available transfer amount.
    pub available: super::rest_models::QueryMaxTransferOutAmountResponse,
}
/// Available inventory keyed by validated native asset identities.
/// Financial values parse without a float and the entire map fails on malformed data.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct AssetInventory(std::collections::BTreeMap<crate::Asset, crate::Decimal>);
impl AssetInventory {
    /// Borrow native inventory amounts; no assets are combined or inferred.
    #[must_use]
    pub fn amounts(&self) -> &std::collections::BTreeMap<crate::Asset, crate::Decimal> {
        &self.0
    }
}
impl<'de> serde::Deserialize<'de> for AssetInventory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let values= <std::collections::BTreeMap<String,serde_json::Value> as serde::Deserialize>::deserialize(deserializer)?;
        let mut inventory = std::collections::BTreeMap::new();
        for (asset, value) in values {
            let asset = crate::Asset::new(asset).map_err(serde::de::Error::custom)?;
            let amount = super::wire::parse_decimal(&value).map_err(serde::de::Error::custom)?;
            inventory.insert(asset, amount);
        }
        Ok(Self(inventory))
    }
}
/// Native margin account scope, preserved from token request provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AccountScope {
    /// Cross-margin source account.
    Cross,
    /// Isolated margin for this provider symbol; assets never inferred from its spelling.
    Isolated(crate::Symbol),
}
/// Issued token paired with source account provenance; secrets redact and zeroize.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ListenToken {
    /// Caller-selected account scope used for token issuance.
    pub scope: AccountScope,
    /// Venue token and expiry in milliseconds, retained without guessed defaults.
    pub receipt: super::rest_models::CreateUserListenTokenResponse,
}

impl ListenToken {
    /// Accept externally issued native token evidence with its explicit source account.
    ///
    /// # Errors
    /// Refuses empty tokens and negative expiry; dispatch rechecks expiry using the clock.
    pub fn new(
        scope: AccountScope,
        receipt: super::rest_models::CreateUserListenTokenResponse,
    ) -> Result<Self, crate::Error> {
        if receipt.token.as_str().is_empty() || receipt.expiration_time < 0 {
            return Err(crate::Error::Validation("Margin token evidence"));
        }
        Ok(Self { scope, receipt })
    }
}
