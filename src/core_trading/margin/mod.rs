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
mod account;
mod client;
mod config;
/// Provider-native response enumerations with exact unknown-value retention.
pub mod enums;
pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
mod validation;
mod wire;
pub use account::{AccountScope, AssetInventory, BorrowCapacity, ListenToken, TransferCapacity};
pub use client::RestClient;
pub use config::Config;
pub mod stream_models;
mod websocket;
pub use websocket::{
    ApiEvent, ApiEvents, ConnectionDriver, RiskEvent, RiskStream, SubscribeToken, Subscription,
    UnexpectedControl, WsClient, WsConfig,
};
