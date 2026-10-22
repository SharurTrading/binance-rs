// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Margin SAPI preserves cross/isolated account mode, native assets and borrow liabilities.
//! Caller-owned order IDs are mandatory on every placement leg. Mutations have one
//! attempt; cancellation or timeout does not prove venue cancellation. Resolve
//! uncertain orders and transactions through the corresponding venue reads.
//! Clients share explicit IP/account budgets and never start a runtime or task.
//! Wallet, Convert and Margin use the same SAPI [`Config`]. Before placing orders,
//! install native account quota evidence through [`RestClient::observe_order_limits`].

mod account;
mod authority;
mod client;
mod config;
/// Provider-native response enumerations with exact unknown-value retention.
pub mod enums;
pub mod event_payloads;
mod identity;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub mod stream_models;
mod validation;
mod websocket;
mod wire;

pub use account::{AccountScope, AssetInventory, BorrowCapacity, ListenToken, TransferCapacity};
pub use authority::TradingSymbolCount;
pub use client::RestClient;
pub use config::Config;
pub use identity::{
    ClientOrderId, OrderId, OrderListId, OrderListKind, OrderListKindError, PreventedMatchId,
    RecordId, TradeId, TransactionId,
};
pub use websocket::{
    ApiEvent, ApiEvents, ConnectionDriver, RiskEvent, RiskStream, SubscribeToken, Subscription,
    UnexpectedControl, WsClient, WsConfig,
};
