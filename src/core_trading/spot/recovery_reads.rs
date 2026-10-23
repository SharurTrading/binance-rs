// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit account-recovery and startup-metadata reads using reserved capacity.
//!
//! Native request costs, deadlines and venue cooldowns still apply. Ordinary
//! methods retain ordinary admission; no mutation is eligible through this API.

use super::rest_models::{
    ExchangeInfoResponse, GetAccountResponse, GetOpenOrdersResponse, MyTradesResponse, TimeResponse,
};
use super::rest_requests::{ExchangeInfo, GetAccount, GetOpenOrders, MyTrades, Time};
use crate::{Error, Response};
use tokio::time::Instant;

impl crate::core::RecoveryRead for ExchangeInfo {}

impl crate::core::RecoveryRead for GetAccount {}

impl crate::core::RecoveryRead for GetOpenOrders {}

impl crate::core::RecoveryRead for MyTrades {}

impl crate::core::RecoveryRead for Time {}

impl super::RestClient {
    /// The native metadata read for bounded account recovery, using reserved capacity.
    /// Adopts stated venue limits exactly as [`Self::exchange_info`] does.
    ///
    /// Sent at most once with native costs and cooldowns. The caller owns the
    /// recovery schedule; ordinary catalog refresh keeps ordinary admission.
    ///
    /// # Errors
    /// As [`Self::exchange_info`], including invalid stated limits, plus native
    /// capacity/cooldown refusal or an expired deadline before sending.
    pub async fn exchange_info_for_recovery(
        &self,
        request: &ExchangeInfo,
        deadline: Instant,
    ) -> Result<Response<ExchangeInfoResponse>, Error> {
        let response = self.inner.execute_recovery_read(request, deadline).await?;
        super::rate::adopt_stated_limits(&self.inner, response)
    }

    /// The same native read as [`Self::get_account`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::get_account`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn get_account_for_recovery(
        &self,
        request: &GetAccount,
        deadline: Instant,
    ) -> Result<Response<GetAccountResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::get_open_orders`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::get_open_orders`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn get_open_orders_for_recovery(
        &self,
        request: &GetOpenOrders,
        deadline: Instant,
    ) -> Result<Response<GetOpenOrdersResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::my_trades`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::my_trades`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn my_trades_for_recovery(
        &self,
        request: &MyTrades,
        deadline: Instant,
    ) -> Result<Response<MyTradesResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::time`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::time`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn time_for_recovery(
        &self,
        request: &Time,
        deadline: Instant,
    ) -> Result<Response<TimeResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }
}
