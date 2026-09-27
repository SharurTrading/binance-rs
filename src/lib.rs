// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Independent Binance inner clients with provider-native product modules.
//!
//! [`spot`], [`usdm`], and [`coinm`] implement distinct trading products.
//! [`wallet`] retains native assets, networks, account categories and SAPI quotas.
//! [`convert`] retains native quote/limit amounts and venue expiry authority.
//! The caller owns the Tokio runtime,
//! credentials, order IDs, trading policy, and recovery of ambiguous mutations.
//! Infrastructure is shared without conflating product account or contract models.

pub mod coinm;
pub mod convert;
/// Read-only demo metadata example; callers own the runtime.
///
/// ```no_run
/// use binance_client::Error;
/// use binance_client::usdm::{Config, Environment, RestClient, rest_requests::CheckServerTime};
/// use std::time::Duration;
/// use tokio::time::Instant;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), Error> {
///     let client = RestClient::new(Config::new(Environment::Demo)?)?;
///     let response = client
///         .check_server_time(
///             &CheckServerTime::new(),
///             Instant::now() + Duration::from_secs(10),
///         )
///         .await?;
///     println!("Server time: {:?}", response.data.server_time);
///     Ok(())
/// }
/// ```
mod core;
pub mod spot;
pub mod usdm;
pub mod wallet;

pub use core::{
    Asset, ClientOrderId, Clock, Credentials, Error, OperationLeg, Outcome, PartialOperation,
    RateEvidence, RequestId, Response, ResponseMeta, SensitiveString, Signer, Symbol, SystemClock,
    TimeUnit, VenueFailure,
};
pub use core::{BudgetLimits, Budgets};
pub use rust_decimal::Decimal;
