// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit native position-mode control using reserved capacity.

use crate::core_trading::usdm::{
    rest_models::ChangePositionModeResponse, rest_requests::ChangePositionMode,
};
use crate::{Error, Response};
use tokio::time::Instant;

impl crate::core::PositionModeControl for ChangePositionMode {}

impl crate::core_trading::usdm::RestClient {
    /// Change or restore the native shared USD-M/COIN-M position mode using reserved capacity.
    ///
    /// This is the same signed mutation as [`Self::change_position_mode`]. Binance
    /// applies this mode to both futures markets. The caller owns authorization,
    /// fresh evidence covering both markets, exclusive account ownership and recovery
    /// scheduling. Set `deadline` to the earlier of the request deadline and the
    /// validity of that authority. The SDK does not establish those business proofs.
    ///
    /// Sends at most once, on a caller-owned connection without pooling or retries.
    /// Revalidates the original native quota charge, cooldown and deadline before
    /// the first positive application write; an expired authority cannot start a
    /// late mutation after DNS, TCP or TLS setup. Cancellation closes the owned
    /// connection; it neither undoes a sent mutation nor refunds native costs.
    ///
    /// # Errors
    /// As [`Self::change_position_mode`], preserving native mutation uncertainty
    /// after an attempted write. Native capacity/cooldown refusal and expired
    /// authority prevent sending. An explicit proxy returns [`Error::Configuration`]
    /// before admission because this owned transport cannot honor that proxy.
    pub async fn change_position_mode_for_recovery(
        &self,
        request: &ChangePositionMode,
        deadline: Instant,
    ) -> Result<Response<ChangePositionModeResponse>, Error> {
        self.inner
            .execute_position_mode_control(request, deadline)
            .await
    }
}
