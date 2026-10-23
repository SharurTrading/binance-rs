// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit account-recovery and startup-metadata reads using reserved capacity.
//!
//! Native request costs, deadlines and venue cooldowns still apply. Ordinary
//! methods retain ordinary admission; no mutation is eligible through this API.

use super::rest_models::{
    AccountInformationV3Response, AccountTradeListResponse, CheckServerTimeResponse,
    CurrentAllAlgoOpenOrdersResponse, CurrentAllOpenOrdersResponse, ExchangeInformationResponse,
    FuturesAccountBalanceV3Response, FuturesAccountConfigurationResponse,
    GetCurrentPositionModeResponse, PositionInformationV3Response, QueryOrderResponse,
};
use super::rest_requests::{
    AccountInformationV3, AccountTradeList, CheckServerTime, CurrentAllAlgoOpenOrders,
    CurrentAllOpenOrders, ExchangeInformation, FuturesAccountBalanceV3,
    FuturesAccountConfiguration, GetCurrentPositionMode, PositionInformationV3, QueryOrder,
};
use crate::{Error, Response};
use tokio::time::Instant;

impl crate::core::RecoveryRead for ExchangeInformation {}

impl crate::core::RecoveryRead for FuturesAccountBalanceV3 {}

impl crate::core::RecoveryRead for FuturesAccountConfiguration {}

impl crate::core::RecoveryRead for GetCurrentPositionMode {}

impl crate::core::RecoveryRead for AccountInformationV3 {}

impl crate::core::RecoveryRead for PositionInformationV3 {}

impl crate::core::RecoveryRead for CurrentAllOpenOrders {}

impl crate::core::RecoveryRead for CurrentAllAlgoOpenOrders {}

impl crate::core::RecoveryRead for AccountTradeList {}

impl crate::core::RecoveryRead for QueryOrder {}

impl crate::core::RecoveryRead for CheckServerTime {}

impl super::RestClient {
    /// Read one native order's status for bounded recovery, using reserved capacity.
    ///
    /// The same native request and response as [`Self::query_order`], sent at most
    /// once without waiting for capacity or retrying. The caller owns the recovery
    /// schedule, shared reserve policy and original absolute deadline. Dropping the
    /// future does not roll back a sent read or refund its native cost.
    ///
    /// # Errors
    /// As [`Self::query_order`], including native validation and transport failures.
    /// Before sending: [`Error::Admission`] when the whole native window is spent
    /// or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`] for a
    /// ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn query_order_for_recovery(
        &self,
        request: &QueryOrder,
        deadline: Instant,
    ) -> Result<Response<QueryOrderResponse>, Error> {
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

    /// The same native read as [`Self::futures_account_balance_v3`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::futures_account_balance_v3`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn futures_account_balance_v3_for_recovery(
        &self,
        request: &FuturesAccountBalanceV3,
        deadline: Instant,
    ) -> Result<Response<FuturesAccountBalanceV3Response>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// Read native account configuration for bounded recovery, using reserved capacity.
    ///
    /// Sent at most once with native costs and cooldowns. The caller owns the
    /// recovery schedule and shared reserve policy; ordinary requests remain ordinary.
    ///
    /// # Errors
    /// As [`Self::futures_account_configuration`], including native validation and
    /// transport failures, native capacity or cooldown refusal, and an expired deadline.
    pub async fn futures_account_configuration_for_recovery(
        &self,
        request: &FuturesAccountConfiguration,
        deadline: Instant,
    ) -> Result<Response<FuturesAccountConfigurationResponse>, Error> {
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

    /// The same native read as [`Self::account_information_v3`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::account_information_v3`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn account_information_v3_for_recovery(
        &self,
        request: &AccountInformationV3,
        deadline: Instant,
    ) -> Result<Response<AccountInformationV3Response>, Error> {
        self.inner.execute_recovery_read(request, deadline).await
    }

    /// The same native read as [`Self::position_information_v3`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::position_information_v3`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn position_information_v3_for_recovery(
        &self,
        request: &PositionInformationV3,
        deadline: Instant,
    ) -> Result<Response<PositionInformationV3Response>, Error> {
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

    /// The same native read as [`Self::current_all_algo_open_orders`], admitted from reserved capacity
    /// for bounded account recovery.
    ///
    /// Sent at most once, without waiting for capacity or retrying. Dropping the
    /// future does not roll back a sent read or refund its charged native costs.
    /// The caller owns recovery scheduling and the shared reserve policy.
    ///
    /// # Errors
    /// As [`Self::current_all_algo_open_orders`], including native request validation and transport
    /// failures. Before sending: [`Error::Admission`] when the whole native window
    /// is spent or a timed venue cooldown is active; [`Error::CooldownTimingUnknown`]
    /// for a ban with no timing; [`Error::Expired`] when `deadline` has passed.
    pub async fn current_all_algo_open_orders_for_recovery(
        &self,
        request: &CurrentAllAlgoOpenOrders,
        deadline: Instant,
    ) -> Result<Response<CurrentAllAlgoOpenOrdersResponse>, Error> {
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
