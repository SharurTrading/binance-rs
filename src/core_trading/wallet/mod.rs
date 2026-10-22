// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Wallet REST operations: preserve native assets, networks and account categories.
//! Every mutation is attempted once. Dropping a future never proves cancellation;
//! reconcile uncertain transfers through the matching venue history.
//! No runtime or background tasks are started. All calls take caller deadlines.
//!
//! External-wallet withdrawal submission is outside this crate's scope. Capital,
//! Travel Rule and broker withdrawal submission builders, withdrawal switches and
//! withdrawal destination discovery are deliberately absent.
//! Withdrawal history, quota/status and account evidence remain available for
//! read-only reconciliation, as do transfers within Binance. Account permissions
//! remain a separate boundary: use credentials without withdrawal permission.
//!
//! ```compile_fail
//! use binance_client::wallet::rest_requests::Withdraw;
//! ```
//! ```compile_fail
//! use binance_client::wallet::rest_requests::WithdrawTravelRule;
//! ```
//! ```compile_fail
//! use binance_client::wallet::rest_requests::BrokerWithdraw;
//! ```
//! ```compile_fail
//! use binance_client::wallet::rest_requests::EnableFastWithdrawSwitch;
//! ```
//! ```compile_fail
//! use binance_client::wallet::rest_requests::DisableFastWithdrawSwitch;
//! ```
//! ```compile_fail
//! use binance_client::wallet::rest_requests::FetchWithdrawAddressList;
//! ```
//! ```compile_fail
//! use binance_client::wallet::rest_requests::FetchAddressVerificationList;
//! ```

mod balance;
mod client;
/// Provider-native response enumerations with exact unknown-value retention.
pub mod enums;
pub mod event_payloads;
mod identity;
mod rate;
pub mod rest_models;
pub mod rest_requests;
mod validation;
mod wire;

pub use crate::core::sapi::Config;
pub use balance::{ConvertibleDust, DustConversion, QuotedWalletBalance};
pub use client::RestClient;
pub use identity::{DepositIds, DustAssets, Network, WithdrawalId};
