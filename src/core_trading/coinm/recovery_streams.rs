// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit account-event stream controls admitted from reserved capacity.
//!
//! Controls change only the native listen-key lifecycle. Mutation evidence,
//! native costs, deadlines and cooldowns are preserved; the caller owns renewal.

use super::rest_models::{
    CloseUserDataStreamResponse, KeepaliveUserDataStreamResponse, StartUserDataStreamResponse,
};
use super::rest_requests::{CloseUserDataStream, KeepaliveUserDataStream, StartUserDataStream};
use crate::{Error, Response};
use tokio::time::Instant;

impl crate::core::ReadStreamControl for StartUserDataStream {}

impl crate::core::ReadStreamControl for KeepaliveUserDataStream {}

impl crate::core::ReadStreamControl for CloseUserDataStream {}

impl super::RestClient {
    /// The native [`Self::start_user_data_stream`] control, using reserved capacity
    /// to maintain an account-event feed during bounded recovery.
    ///
    /// Changes only the listen-key lifecycle, never orders or balances. Sends
    /// at most once; cancellation does not undo or refund an attempted control.
    /// The caller owns renewal, cleanup and any uncertain outcome.
    ///
    /// # Errors
    /// As [`Self::start_user_data_stream`], preserving native mutation uncertainty.
    /// Full native capacity, venue cooldowns and expired deadlines still refuse
    /// before sending; no admission wait or automatic retry is added.
    pub async fn start_user_data_stream_for_recovery(
        &self,
        request: &StartUserDataStream,
        deadline: Instant,
    ) -> Result<Response<StartUserDataStreamResponse>, Error> {
        self.inner
            .execute_read_stream_control(request, deadline)
            .await
    }

    /// The native [`Self::keepalive_user_data_stream`] control, using reserved capacity
    /// to maintain an account-event feed during bounded recovery.
    ///
    /// Changes only the listen-key lifecycle, never orders or balances. Sends
    /// at most once; cancellation does not undo or refund an attempted control.
    /// The caller owns renewal, cleanup and any uncertain outcome.
    ///
    /// # Errors
    /// As [`Self::keepalive_user_data_stream`], preserving native mutation uncertainty.
    /// Full native capacity, venue cooldowns and expired deadlines still refuse
    /// before sending; no admission wait or automatic retry is added.
    pub async fn keepalive_user_data_stream_for_recovery(
        &self,
        request: &KeepaliveUserDataStream,
        deadline: Instant,
    ) -> Result<Response<KeepaliveUserDataStreamResponse>, Error> {
        self.inner
            .execute_read_stream_control(request, deadline)
            .await
    }

    /// The native [`Self::close_user_data_stream`] control, using reserved capacity
    /// to maintain an account-event feed during bounded recovery.
    ///
    /// Changes only the listen-key lifecycle, never orders or balances. Sends
    /// at most once; cancellation does not undo or refund an attempted control.
    /// The caller owns renewal, cleanup and any uncertain outcome.
    ///
    /// # Errors
    /// As [`Self::close_user_data_stream`], preserving native mutation uncertainty.
    /// Full native capacity, venue cooldowns and expired deadlines still refuse
    /// before sending; no admission wait or automatic retry is added.
    pub async fn close_user_data_stream_for_recovery(
        &self,
        request: &CloseUserDataStream,
        deadline: Instant,
    ) -> Result<Response<CloseUserDataStreamResponse>, Error> {
        self.inner
            .execute_read_stream_control(request, deadline)
            .await
    }
}
