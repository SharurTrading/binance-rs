// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The read an exit makes before it can cancel, admitted from the exit reserve.
//!
//! The method sends the same request as its ordinary counterpart and is admitted as
//! cancellations are: up to the venue's whole limit, the exit reserve included. Only
//! this explicit call draws on the reserve; the ordinary method never does, whatever
//! the caller intends.

use super::rest_models::GetOpenOrdersResponse;
use super::rest_requests::GetOpenOrders;
use crate::{Error, Response};
use tokio::time::Instant;

impl crate::core::ExitRead for GetOpenOrders {}

impl super::RestClient {
    /// [getOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account)
    /// for an exit, admitted from the exit reserve.
    ///
    /// The same read as [`Self::get_open_orders`], for an exit that must name the
    /// orders it cancels. It is sent at most once and never waits for capacity.
    ///
    /// # Errors
    /// As [`Self::get_open_orders`].
    /// Nothing is sent when the request is refused before sending: [`Error::Admission`]
    /// when the venue's whole limit, reserve included, is spent in the current window
    /// or a venue `Retry-After` cooldown is active, with `retry_after` saying when to
    /// try again; [`Error::CooldownTimingUnknown`] while a ban the venue gave no
    /// timing for stands; and [`Error::Expired`] once `deadline` has passed.
    pub async fn get_open_orders_for_exit(
        &self,
        request: &GetOpenOrders,
        deadline: Instant,
    ) -> Result<Response<GetOpenOrdersResponse>, Error> {
        self.inner.execute_exit_read(request, deadline).await
    }
}
