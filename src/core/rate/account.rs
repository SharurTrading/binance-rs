// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The caller's name for one venue account, used to share its account scope.
//!
//! Binance counts orders per account, not per client, so every client of one
//! account and pool must count against one account owner. The caller names the
//! account; the SDK compares the name and never interprets it.

use std::{cmp::Ordering, sync::Arc};
use zeroize::Zeroizing;

/// An opaque, caller-chosen key naming one venue account.
///
/// Clients drawn from one [`WeightPools`](crate::WeightPools) registry with equal keys
/// share the account scope of each venue pool they draw on: order counters,
/// quote and download quotas, SAPI UID weight and Margin's native order windows. The key is any bytes or text the caller chooses (an
/// account or sub-account identifier, a label); the SDK only compares it. A caller
/// may derive it from an identifier it treats as sensitive, so it is redacted from
/// `Debug` and zeroized when its last copy drops. A registry keeps a copy for as
/// long as the registry lives.
///
/// ```
/// use binance_client::core_trading::{coinm, usdm};
/// use binance_client::{AccountKey, Error};
///
/// let account = AccountKey::new("main-account");
/// let um = usdm::Config::new_for_account(usdm::Environment::Production, &account)?;
/// let cm = coinm::Config::new_for_account(coinm::Environment::Production, &account)?;
/// assert_eq!(um.pool_key(), cm.pool_key());
/// assert_eq!(format!("{account:?}"), "AccountKey([REDACTED])");
/// # Ok::<(), Error>(())
/// ```
#[derive(Clone)]
pub struct AccountKey(Arc<Zeroizing<Vec<u8>>>);

impl AccountKey {
    /// Name an account by `value`. Equal bytes name the same account.
    #[must_use]
    pub fn new(value: impl AsRef<[u8]>) -> Self {
        Self(Arc::new(Zeroizing::new(value.as_ref().to_vec())))
    }

    fn bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl std::fmt::Debug for AccountKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccountKey([REDACTED])")
    }
}

impl PartialEq for AccountKey {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}

impl Eq for AccountKey {}

impl PartialOrd for AccountKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AccountKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes().cmp(other.bytes())
    }
}

impl std::hash::Hash for AccountKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes().hash(state);
    }
}
