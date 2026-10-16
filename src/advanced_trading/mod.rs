// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Binance Advanced Trading products with distinct native wire contracts.
//!
//! The caller owns credentials, runtime, trading policy and uncertain-mutation
//! recovery. Institutional Loan and external wallet withdrawals are out of scope.

pub mod algo;
pub mod alpha;
pub mod config;
pub mod copy_trading;
pub mod portfolio_margin;
pub mod portfolio_margin_pro;
pub mod stocks;
mod validation;
mod wire;

pub mod streams;
