// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit account-recovery and startup-metadata reads using reserved capacity.
//!
//! Native request costs, deadlines and venue cooldowns still apply. Ordinary
//! methods retain ordinary admission; no mutation is eligible through this API.

use super::rest_models::{
    AccountInformationResponse, AccountTradeListResponse, CheckServerTimeResponse,
    CurrentAllOpenOrdersResponse, ExchangeInformationResponse, FuturesAccountBalanceResponse,
    GetCurrentPositionModeResponse, OpenAlgoOrdersResponse, PositionInformationResponse,
};
use super::rest_requests::{
    AccountInformation, AccountTradeList, CheckServerTime, CurrentAllOpenOrders,
    ExchangeInformation, FuturesAccountBalance, GetCurrentPositionMode, OpenAlgoOrders,
    PositionInformation,
};
use crate::{Error, Response};
use tokio::time::Instant;

impl crate::core::RecoveryRead for ExchangeInformation {}

impl crate::core::RecoveryRead for FuturesAccountBalance {}

impl crate::core::RecoveryRead for GetCurrentPositionMode {}

impl crate::core::RecoveryRead for AccountInformation {}

impl crate::core::RecoveryRead for CurrentAllOpenOrders {}

impl crate::core::RecoveryRead for OpenAlgoOrders {}

impl crate::core::RecoveryRead for PositionInformation {}

impl crate::core::RecoveryRead for AccountTradeList {}

impl crate::core::RecoveryRead for CheckServerTime {}

impl super::RestClient {
    /// Read native COIN-M positions for bounded recovery, using reserved capacity.
    ///
    /// Uses the signed [`Self::position_information`] endpoint and its native cost.
    /// The caller owns the recovery schedule, shared reserve policy and deadline.
    /// Sends at most once without waiting for capacity or retrying.
    ///
    /// # Errors
    /// As [`Self::position_information`], including validation and transport failures,
    /// native capacity or cooldown refusal, and expiry before sending.
    pub async fn position_information_for_recovery(
        &self,
        request: &PositionInformation,
        deadline: Instant,
    ) -> Result<Response<PositionInformationResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The native metadata read for bounded account recovery, using reserved capacity.
    /// Adopts stated venue limits exactly as [`Self::exchange_information`] does.
    ///
    /// Sent at most once with native costs and cooldowns. The caller owns the
    /// recovery schedule; ordinary catalog refresh keeps ordinary admission.
    ///
    /// # Errors
    /// As [`Self::exchange_information`], including invalid stated limits, plus native
    /// capacity/cooldown refusal or an expired deadline before sending.
    pub async fn exchange_information_for_recovery(
        &self,
        request: &ExchangeInformation,
        deadline: Instant,
    ) -> Result<Response<ExchangeInformationResponse>, Error> {
        let response = self.inner.execute_recovery_read(request, deadline).await?;
        super::rate::adopt_stated_limits(&self.inner, response)
    }

    /// The same native read as [`Self::futures_account_balance`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::futures_account_balance`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn futures_account_balance_for_recovery(
        &self,
        request: &FuturesAccountBalance,
        deadline: Instant,
    ) -> Result<Response<FuturesAccountBalanceResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::get_current_position_mode`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::get_current_position_mode`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn get_current_position_mode_for_recovery(
        &self,
        request: &GetCurrentPositionMode,
        deadline: Instant,
    ) -> Result<Response<GetCurrentPositionModeResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::account_information`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::account_information`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn account_information_for_recovery(
        &self,
        request: &AccountInformation,
        deadline: Instant,
    ) -> Result<Response<AccountInformationResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::current_all_open_orders`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::current_all_open_orders`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn current_all_open_orders_for_recovery(
        &self,
        request: &CurrentAllOpenOrders,
        deadline: Instant,
    ) -> Result<Response<CurrentAllOpenOrdersResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::open_algo_orders`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::open_algo_orders`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn open_algo_orders_for_recovery(
        &self,
        request: &OpenAlgoOrders,
        deadline: Instant,
    ) -> Result<Response<OpenAlgoOrdersResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::account_trade_list`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::account_trade_list`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn account_trade_list_for_recovery(
        &self,
        request: &AccountTradeList,
        deadline: Instant,
    ) -> Result<Response<AccountTradeListResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::check_server_time`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::check_server_time`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn check_server_time_for_recovery(
        &self,
        request: &CheckServerTime,
        deadline: Instant,
    ) -> Result<Response<CheckServerTimeResponse>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }
}
