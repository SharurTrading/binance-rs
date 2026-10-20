// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

#![doc = include_str!("../README.md")]

pub mod advanced_trading;
pub mod core_trading;
pub use core_trading::{coinm, convert, margin, options, spot, usdm, wallet};
mod core;

pub use core::{
    AccountKey, AccountUsage, BudgetLimits, Budgets, ExitReserve, LimitSource, PoolEnvironment,
    PoolKey, PoolUsage, VenuePool, WeightPools, WindowUsage,
};
pub use core::{
    Asset, ClientOrderId, Clock, Credentials, Error, OperationLeg, Outcome, PartialOperation,
    RateEvidence, RequestId, Response, ResponseMeta, SensitiveString, Signer, StreamControl,
    Symbol, SystemClock, TimeUnit, TransportFailure, VenueFailure, VenueMessage,
};
pub use rust_decimal::Decimal;
