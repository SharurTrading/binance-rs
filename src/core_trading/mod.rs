// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Binance Core Trading products, each with its own provider-native wire models.
//!
//! Spot, linear futures, inverse futures, Wallet and Convert share transport and
//! signing infrastructure without sharing product-specific account semantics.
//! The caller owns credentials, runtime, trading policy and mutation recovery.
//! Grouping these modules does not expand their documented endpoint coverage.

pub mod coinm;
pub mod convert;
pub mod spot;
pub mod usdm;
pub mod wallet;
