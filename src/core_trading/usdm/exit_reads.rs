// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The reads an exit makes before it can cancel or close, admitted from the exit reserve.
//!
//! Each method sends the same request as its ordinary counterpart and is admitted as
//! cancellations and reducing orders are: up to the venue's whole limit, the exit
//! reserve included. Only these explicit calls draw on the reserve; the ordinary
//! methods never do, whatever the caller intends.

use super::rest_models::{
    CurrentAllAlgoOpenOrdersResponse, CurrentAllOpenOrdersResponse, PositionInformationV3Response,
};
use super::rest_requests::{CurrentAllAlgoOpenOrders, CurrentAllOpenOrders, PositionInformationV3};
use crate::{Error, Response};
use tokio::time::Instant;

impl crate::core::ExitRead for CurrentAllOpenOrders {}

impl crate::core::ExitRead for CurrentAllAlgoOpenOrders {}

impl crate::core::ExitRead for PositionInformationV3 {}

impl super::RestClient {
    /// [currentAllOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-open-orders)
    /// for an exit, admitted from the exit reserve.
    ///
    /// The same read as [`Self::current_all_open_orders`], for an exit that must name
    /// the regular orders it cancels. It is sent at most once and never waits for
    /// capacity.
    ///
    /// # Errors
    /// As [`Self::current_all_open_orders`].
    /// Nothing is sent when the request is refused before sending: [`Error::Admission`]
    /// when the venue's whole limit, reserve included, is spent in the current window
    /// or a venue `Retry-After` cooldown is active, with `retry_after` saying when to
    /// try again; [`Error::CooldownTimingUnknown`] while a ban the venue gave no
    /// timing for stands; and [`Error::Expired`] once `deadline` has passed.
    pub async fn current_all_open_orders_for_exit(
        &self,
        request: &CurrentAllOpenOrders,
        deadline: Instant,
    ) -> Result<Response<CurrentAllOpenOrdersResponse>, Error> {
        self.inner.execute_exit_read(request, deadline).await
    }

    /// [currentAllAlgoOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#current-all-algo-open-orders)
    /// for an exit, admitted from the exit reserve.
    ///
    /// The same read as [`Self::current_all_algo_open_orders`], for an exit that must
    /// name the conditional orders it cancels. It is sent at most once and never
    /// waits for capacity.
    ///
    /// # Errors
    /// As [`Self::current_all_algo_open_orders`].
    /// Nothing is sent when the request is refused before sending: [`Error::Admission`]
    /// when the venue's whole limit, reserve included, is spent in the current window
    /// or a venue `Retry-After` cooldown is active, with `retry_after` saying when to
    /// try again; [`Error::CooldownTimingUnknown`] while a ban the venue gave no
    /// timing for stands; and [`Error::Expired`] once `deadline` has passed.
    pub async fn current_all_algo_open_orders_for_exit(
        &self,
        request: &CurrentAllAlgoOpenOrders,
        deadline: Instant,
    ) -> Result<Response<CurrentAllAlgoOpenOrdersResponse>, Error> {
        self.inner.execute_exit_read(request, deadline).await
    }

    /// [positionInformationV3](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#position-information-v3)
    /// for an exit, admitted from the exit reserve.
    ///
    /// The same read as [`Self::position_information_v3`], for an exit that must
    /// learn the positions it closes. It is sent at most once and never waits for
    /// capacity.
    ///
    /// # Errors
    /// As [`Self::position_information_v3`].
    /// Nothing is sent when the request is refused before sending: [`Error::Admission`]
    /// when the venue's whole limit, reserve included, is spent in the current window
    /// or a venue `Retry-After` cooldown is active, with `retry_after` saying when to
    /// try again; [`Error::CooldownTimingUnknown`] while a ban the venue gave no
    /// timing for stands; and [`Error::Expired`] once `deadline` has passed.
    pub async fn position_information_v3_for_exit(
        &self,
        request: &PositionInformationV3,
        deadline: Instant,
    ) -> Result<Response<PositionInformationV3Response>, Error> {
        self.inner.execute_exit_read(request, deadline).await
    }
}
