// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicit account-event subscription controls using reserved native capacity.
//!
//! Native mutation attribution, signing, costs and socket ownership are unchanged.

use super::ws_models::{
    UserDataStreamSubscribeSignatureResponse, UserDataStreamUnsubscribeResponse,
};
use super::ws_requests::{UserDataStreamSubscribeSignature, UserDataStreamUnsubscribe};
use crate::{Error, RequestId, Response};
use tokio::time::Instant;

impl crate::core::ReadStreamControl for UserDataStreamSubscribeSignature {}

impl crate::core::ReadStreamControl for UserDataStreamUnsubscribe {}

impl super::WsClient {
    /// The native [`Self::user_data_stream_subscribe_signature`] control using reserved capacity
    /// for an account-event feed during bounded recovery.
    ///
    /// Changes subscription state only. Sent at most once on this generation;
    /// the caller owns its driver and drains and joins it after close. Dropping
    /// the future does not undo a sent control or discard late-answer evidence.
    ///
    /// # Errors
    /// As [`Self::user_data_stream_subscribe_signature`], including native mutation uncertainty,
    /// full native capacity, cooldown and deadline refusal. No retry is added.
    pub async fn user_data_stream_subscribe_signature_for_recovery(
        &self,
        request: &UserDataStreamSubscribeSignature,
        id: RequestId,
        deadline: Instant,
    ) -> Result<Response<UserDataStreamSubscribeSignatureResponse>, Error> {
        self.execute_read_stream_control(request, id, deadline)
            .await
    }

    /// The native [`Self::user_data_stream_unsubscribe`] control using reserved capacity
    /// for an account-event feed during bounded recovery.
    ///
    /// Changes subscription state only. Sent at most once on this generation;
    /// the caller owns its driver and drains and joins it after close. Dropping
    /// the future does not undo a sent control or discard late-answer evidence.
    ///
    /// # Errors
    /// As [`Self::user_data_stream_unsubscribe`], including native mutation uncertainty,
    /// full native capacity, cooldown and deadline refusal. No retry is added.
    pub async fn user_data_stream_unsubscribe_for_recovery(
        &self,
        request: &UserDataStreamUnsubscribe,
        id: RequestId,
        deadline: Instant,
    ) -> Result<Response<UserDataStreamUnsubscribeResponse>, Error> {
        self.execute_read_stream_control(request, id, deadline)
            .await
    }
}
