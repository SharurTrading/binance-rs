// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{
    Config,
    event_payloads::{ApiPayload, SessionStatus, UnknownMessage, api_payload},
};
use crate::core::socket::{QueueStats, SocketDriver, SocketEvents};
use crate::core::{Request, Socket, SocketEvent};
use crate::{Error, RequestId, Response};
use tokio::time::Instant;

/// Caller-owned socket driver. This library creates no hidden tasks or runtime.
///
/// Run `run()` alongside the handle's requests/events, and join it after closing.
#[must_use = "the caller must run and join this driver"]
pub struct ConnectionDriver {
    pub(crate) inner: SocketDriver,
}
impl ConnectionDriver {
    /// Drive the socket on the caller's runtime until its explicit retirement.
    ///
    /// # Errors
    /// Returns lifecycle/consumer errors. Network loss is also delivered as a gap event.
    pub async fn run(self) -> Result<(), Error> {
        self.inner.run().await
    }
}

/// Cloneable WebSocket API request handle, sharing its generation and venue budgets.
#[derive(Clone)]
pub struct WsClient {
    pub(crate) socket: Socket,
}
impl WsClient {
    /// Connect the API socket without spawning a task.
    ///
    /// # Errors
    /// Refuses invalid transport configuration or failed handshake/rate admission.
    pub async fn connect(config: Config) -> Result<(Self, ApiEvents, ConnectionDriver), Error> {
        let (socket, events, driver) = Socket::connect_with_policy(
            config.websocket,
            config.credentials,
            config.clock,
            config.budgets,
            config.timeout,
            crate::core::socket::SocketPolicy {
                handshake: crate::core::Cost {
                    ws_weight: 2,
                    connections: 1,
                    ..Default::default()
                },
                ping_limit: 5,
            },
        )
        .await?;
        Ok((
            Self { socket },
            ApiEvents { inner: events },
            ConnectionDriver { inner: driver },
        ))
    }
    pub(crate) async fn execute<R: Request>(
        &self,
        request: &R,
        id: RequestId,
        deadline: Instant,
    ) -> Result<Response<R::Response>, Error> {
        request.validate()?;
        let (value, meta) = self
            .socket
            .call(
                R::OP,
                crate::core::parameters(request)?,
                request.cost()?,
                id,
                deadline,
            )
            .await?;
        crate::core::http::decode(R::OP, meta.status, value, meta)
    }
    /// Close promptly, settle unsent work and uncertain sent work, and retire the socket.
    /// Continue draining events, then join the caller-owned driver task.
    ///
    /// # Errors
    /// Returns `Closed` if the generation has already retired.
    pub async fn close(&self) -> Result<(), Error> {
        self.socket.close().await
    }
    /// This handle's immutable generation.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.socket.generation()
    }
    /// Observable ingress queue lag, without discarding accepted events.
    ///
    /// # Errors
    /// Returns a metrics synchronization error if its owner failed.
    pub fn queue_stats(&self) -> Result<QueueStats, Error> {
        self.socket.stats()
    }

    /// Inspect the current provider authentication session.
    ///
    /// # Errors
    /// Returns typed admission/transport/provider errors without retrying.
    pub async fn session_status(
        &self,
        id: RequestId,
        deadline: Instant,
    ) -> Result<Response<SessionStatus>, Error> {
        self.session(
            "sessionStatus",
            "/session.status",
            false,
            false,
            id,
            deadline,
        )
        .await
    }
    /// Authenticate this connection using its configured Ed25519 credentials.
    /// Other signing types are refused before sending.
    ///
    /// # Errors
    /// Returns validation/admission/transport/provider evidence without retrying.
    pub async fn session_logon(
        &self,
        id: RequestId,
        deadline: Instant,
    ) -> Result<Response<SessionStatus>, Error> {
        self.session("sessionLogon", "/session.logon", true, true, id, deadline)
            .await
    }
    /// Forget connection authentication; this does not cancel any venue orders.
    ///
    /// # Errors
    /// Returns admission/transport/provider evidence without retrying.
    pub async fn session_logout(
        &self,
        id: RequestId,
        deadline: Instant,
    ) -> Result<Response<SessionStatus>, Error> {
        self.session(
            "sessionLogout",
            "/session.logout",
            false,
            true,
            id,
            deadline,
        )
        .await
    }
    async fn session(
        &self,
        name: &'static str,
        path: &'static str,
        signed: bool,
        mutation: bool,
        id: RequestId,
        deadline: Instant,
    ) -> Result<Response<SessionStatus>, Error> {
        let op = crate::core::Operation {
            name,
            path,
            method: "POST",
            security: if signed {
                crate::core::Security::Signed
            } else {
                crate::core::Security::Public
            },
            mutation,
            weight: 2,
            success_weight: None,
            validate_time: super::validation::validate_time,
            definitive: super::validation::definitive,
        };
        let (value, meta) = self
            .socket
            .call(
                op,
                std::collections::BTreeMap::new(),
                crate::core::Cost {
                    ws_weight: 2,
                    ..Default::default()
                },
                id,
                deadline,
            )
            .await?;
        crate::core::http::decode(op, meta.status, value, meta)
    }
}

/// API lifecycle and late-response evidence, in source order.
#[derive(Debug)]
#[non_exhaustive]
pub enum ApiEvent {
    /// Connection established; order/account continuity still belongs to the consumer.
    Established(u64),
    /// A response arriving after timeout or caller cancellation, with original attribution.
    LateResponse {
        /// Original socket generation.
        generation: u64,
        /// Original caller request ID.
        id: RequestId,
        /// Exact response evidence.
        result: Result<Response<ApiPayload>, Error>,
    },
    /// A current Spot subscription event, kept separate from market delivery.
    UserData {
        /// Source socket generation.
        generation: u64,
        /// Provider subscription identifier.
        subscription_id: i64,
        /// Typed execution/account payload; balance events are partial updates.
        payload: super::event_payloads::UserPayload,
    },
    /// Provider shutdown notice; caller initiates a new generation explicitly.
    ServerShutdown {
        /// Source generation.
        generation: u64,
        /// Exact shutdown notice, retained without a continuity assertion.
        payload: UnknownMessage,
    },
    /// Uncorrelated or future API event, explicitly preserved.
    Unknown {
        /// Socket generation.
        generation: u64,
        /// Redacted provider payload.
        payload: UnknownMessage,
    },
    /// A real transport or malformed-data gap.
    Gap {
        /// Socket generation.
        generation: u64,
        /// Typed failure evidence.
        error: Error,
    },
    /// Terminal boundary after all preceding accepted events; no automatic reconnect/replay.
    Retired(u64),
}

/// Single consumer for API lifecycle/late-response evidence.
pub struct ApiEvents {
    inner: SocketEvents,
}
impl ApiEvents {
    /// Drain the next event. Keep draining after close to receive the accepted prefix.
    pub async fn recv(&mut self) -> Option<ApiEvent> {
        Some(match self.inner.recv().await? {
            SocketEvent::Established(g) => ApiEvent::Established(g),
            SocketEvent::Retired(g) => ApiEvent::Retired(g),
            SocketEvent::Gap { generation, error } => ApiEvent::Gap { generation, error },
            SocketEvent::Data { generation, value } => {
                let notice = value.get("event").unwrap_or(&value);
                if notice.get("e").and_then(serde_json::Value::as_str) == Some("serverShutdown") {
                    if notice
                        .get("E")
                        .and_then(serde_json::Value::as_i64)
                        .is_none()
                    {
                        self.inner.stop();
                        ApiEvent::Gap {
                            generation,
                            error: Error::Gap("malformed shutdown notice"),
                        }
                    } else {
                        ApiEvent::ServerShutdown {
                            generation,
                            payload: value.into(),
                        }
                    }
                } else if let Some(event) = value.get("event") {
                    let decoded = value
                        .get("subscriptionId")
                        .and_then(serde_json::Value::as_i64)
                        .filter(|id| *id >= 0)
                        .ok_or(Error::Gap("Spot subscription identity"))
                        .and_then(|id| {
                            super::event_payloads::user_payload(event.clone())
                                .map(|payload| (id, payload))
                        });
                    match decoded {
                        Ok((subscription_id, payload)) => ApiEvent::UserData {
                            generation,
                            subscription_id,
                            payload,
                        },
                        Err(error) => {
                            self.inner.stop();
                            ApiEvent::Gap { generation, error }
                        }
                    }
                } else {
                    ApiEvent::Unknown {
                        generation,
                        payload: value.into(),
                    }
                }
            }
            SocketEvent::Late {
                generation,
                id,
                op,
                value,
                meta,
            } => {
                let result = if !(200..300).contains(&meta.status)
                    || value
                        .get("code")
                        .and_then(serde_json::Value::as_i64)
                        .is_some_and(|c| c < 0)
                {
                    Err(crate::core::error::failure_for(
                        op,
                        meta.status,
                        &value,
                        meta.rates.clone(),
                    )
                    .with_order_ids(meta.client_order_ids.clone()))
                } else {
                    api_payload(op.name, value)
                        .map_err(|_| Error::Transport {
                            client_order_ids: meta.client_order_ids.clone(),
                            operation: op.name,
                            outcome: if op.mutation {
                                crate::Outcome::Unknown
                            } else {
                                crate::Outcome::ReadFailed
                            },
                            meta: Some(Box::new(meta.clone())),
                        })
                        .map(|data| Response { data, meta })
                };
                ApiEvent::LateResponse {
                    generation,
                    id,
                    result,
                }
            }
        })
    }
}
