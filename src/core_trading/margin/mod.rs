// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Margin SAPI preserves cross/isolated account mode, native assets and borrow liabilities.
//! Caller-owned order IDs are mandatory on every placement leg. Mutations have one
//! attempt; cancellation or timeout does not prove venue cancellation. Resolve
//! uncertain orders and transactions through the corresponding venue reads.
//! Clients share explicit IP/account budgets and never start a runtime or task.
mod identity;
pub use identity::{
    ClientOrderId, OrderId, OrderListId, OrderListKind, OrderListKindError, PreventedMatchId,
    RecordId, TradeId, TransactionId,
};
mod authority;
pub use authority::TradingSymbolCount;
mod config;
/// Provider-native response enumerations with exact unknown-value retention.
pub mod enums;
pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
mod validation;
mod wire;
pub use config::Config;
/// Cloneable REST client sharing caller-owned SAPI IP/account budgets.
#[derive(Clone)]
pub struct RestClient {
    pub(crate) inner: crate::core::HttpClient,
    budgets: crate::Budgets,
    clock: std::sync::Arc<dyn crate::Clock>,
}
impl RestClient {
    /// Construct a transport without network I/O or runtime ownership.
    ///
    /// # Errors
    /// Returns typed transport configuration errors.
    pub fn new(config: Config) -> Result<Self, crate::Error> {
        let config = config.inner;
        Ok(Self {
            budgets: config.budgets.clone(),
            clock: config.clock.clone(),
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
    /// Install fresh native order count/limit evidence on this shared account owner.
    /// Validate all records before mutation; clone clients immediately share authority.
    ///
    /// # Errors
    /// Refuses missing/unknown or malformed quota evidence, overflow and clock failures.
    pub fn observe_order_limits(
        &self,
        response: &rest_models::QueryCurrentMarginOrderCountUsageResponse,
    ) -> Result<(), crate::Error> {
        let windows = rate::order_windows(response)?;
        self.budgets
            .observe_sapi_order_windows(&windows, self.clock.now_millis()?)
    }
}
/// Native borrowing capacity paired with request asset and margin mode provenance.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct BorrowCapacity {
    /// Asset whose amount the venue returned.
    pub asset: crate::Asset,
    /// Isolated symbol when requested; absence means cross margin.
    pub isolated_symbol: Option<crate::Symbol>,
    /// Native capacity, without inferred asset or accounting calculations.
    pub capacity: rest_models::QueryMaxBorrowResponse,
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
    pub available: rest_models::QueryMaxTransferOutAmountResponse,
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
            let amount = wire::parse_decimal(&value).map_err(serde::de::Error::custom)?;
            inventory.insert(asset, amount);
        }
        Ok(Self(inventory))
    }
}
pub mod stream_models;
mod websocket;
pub use websocket::{
    ApiEvent, ApiEvents, ConnectionDriver, RiskEvent, RiskStream, SubscribeToken, Subscription,
    UnexpectedControl, WsClient, WsConfig,
};
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
    pub receipt: rest_models::CreateUserListenTokenResponse,
}

impl ListenToken {
    /// Accept externally issued native token evidence with its explicit source account.
    ///
    /// # Errors
    /// Refuses empty tokens and negative expiry; dispatch rechecks expiry using the clock.
    pub fn new(
        scope: AccountScope,
        receipt: rest_models::CreateUserListenTokenResponse,
    ) -> Result<Self, crate::Error> {
        if receipt.token.as_str().is_empty() || receipt.expiration_time < 0 {
            return Err(crate::Error::Validation("Margin token evidence"));
        }
        Ok(Self { scope, receipt })
    }
}
