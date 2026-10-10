// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Caller-driven Margin risk sockets and recommended listen-token subscriptions.
use super::{
    AccountScope, ListenToken,
    event_payloads::{UnknownMessage, UserPayload, user_payload},
};
use crate::core::socket::{QueueStats, SocketDriver, SocketEvents, SocketPolicy};
use crate::core::{Cost, Operation, PoolEnvironment, Security, Socket, SocketEvent, VenuePool};
use crate::{
    Budgets, Clock, Error, RequestId, Response, SensitiveString, StreamControl, SystemClock,
    WeightPools,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

/// Explicit production Margin socket endpoints and caller-owned connection budgets.
/// The API weight/connection owner defaults to the process Spot production pool.
/// SAPI REST has independent endpoint scopes; explicit `budgets` selects another owner.
/// Margin uses the [same API route](https://developers.binance.com/en/docs/products/margin-trading/listen-token-data-stream);
/// its [API limits](https://developers.binance.com/legacy-docs/binance-spot-api-docs/websocket-api/rate-limits)
/// accumulate weight per IP across all connections and limit connections per IP.
#[derive(Clone)]
pub struct WsConfig {
    api: url::Url,
    risk: url::Url,
    clock: Arc<dyn Clock>,
    budgets: Budgets,
    timeout: Duration,
}
impl WsConfig {
    /// Configure documented production API/risk routes without starting tasks.
    ///
    /// # Errors
    /// Returns invalid endpoint or venue-budget configuration errors.
    pub fn new() -> Result<Self, Error> {
        Self::with_pools(WeightPools::process())
    }
    /// Configure production routes with an explicit Spot production IP registry.
    /// Use an independent registry for synthetic fixtures; clients sharing this registry
    /// share API weight and connection admission with other Spot clients drawn from it.
    ///
    /// # Errors
    /// Returns invalid endpoint or venue-budget configuration errors.
    pub fn with_pools(pools: &WeightPools) -> Result<Self, Error> {
        Ok(Self {
            api: crate::core::validate_url("wss://ws-api.binance.com:443/ws-api/v3", true)?,
            risk: crate::core::validate_url("wss://margin-stream.binance.com", true)?,
            clock: Arc::new(SystemClock),
            budgets: pools.draw(VenuePool::Spot, PoolEnvironment::Production)?,
            timeout: Duration::from_secs(10),
        })
    }
    /// Use a caller's signing/expiry clock.
    #[must_use]
    pub fn clock(mut self, value: Arc<dyn Clock>) -> Self {
        self.clock = value;
        self
    }
    /// Select an explicit API weight/connection owner. Margin SAPI endpoint scopes
    /// remain independent of Spot aggregate weight, including under a common owner.
    #[must_use]
    pub fn budgets(mut self, value: Budgets) -> Self {
        self.budgets = value;
        self
    }
    /// Override the WebSocket API endpoint, permitting insecure exact loopback fixtures.
    ///
    /// # Errors
    /// Refuses insecure remote or credential-bearing URLs.
    pub fn api_url(mut self, value: &str) -> Result<Self, Error> {
        self.api = crate::core::validate_url(value, true)?;
        Ok(self)
    }
    /// Override the risk endpoint, permitting insecure exact loopback fixtures.
    ///
    /// # Errors
    /// Refuses insecure remote or credential-bearing URLs.
    pub fn risk_url(mut self, value: &str) -> Result<Self, Error> {
        self.risk = crate::core::validate_url(value, true)?;
        Ok(self)
    }
    /// Set the connect/transport timeout.
    ///
    /// # Errors
    /// Refuses zero or unrepresentable durations.
    pub fn timeout(mut self, value: Duration) -> Result<Self, Error> {
        if value.is_zero() || tokio::time::Instant::now().checked_add(value).is_none() {
            return Err(Error::Configuration("Margin socket timeout"));
        }
        self.timeout = value;
        Ok(self)
    }
}
/// Caller-owned driver; run on the caller's runtime and join after draining retirement.
#[must_use = "the caller must run and join the Margin socket driver"]
pub struct ConnectionDriver {
    inner: SocketDriver,
}
impl ConnectionDriver {
    /// Drive the generation until closed; transport gaps are also delivered in source order.
    ///
    /// # Errors
    /// Returns lifecycle/consumer failures.
    pub async fn run(self) -> Result<(), Error> {
        self.inner.run().await
    }
}
/// Caller-authorized subscription using an issued token's mode and expiry evidence.
#[derive(Clone, Debug, Serialize)]
pub struct SubscribeToken {
    #[serde(rename = "listenToken")]
    token: SensitiveString,
    #[serde(skip)]
    expiration_time: u64,
    #[serde(skip)]
    scope: AccountScope,
}
impl SubscribeToken {
    /// Preserve token expiry and request account mode. Dispatch rechecks expiry before send.
    ///
    /// # Errors
    /// Refuses an empty token or unrepresentable venue expiry.
    pub fn new(token: &ListenToken) -> Result<Self, Error> {
        if token.receipt.token.as_str().is_empty() {
            return Err(Error::Validation("empty Margin listen token"));
        }
        let expiration_time = u64::try_from(token.receipt.expiration_time)
            .map_err(|_| Error::Validation("Margin token expiry"))?;
        Ok(Self {
            token: token.receipt.token.clone(),
            expiration_time,
            scope: token.scope.clone(),
        })
    }
    /// Caller-selected source account scope, retained from token issuance.
    #[must_use]
    pub fn scope(&self) -> &AccountScope {
        &self.scope
    }
}
/// Native token subscription acknowledgment; unknown future fields retained.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Subscription {
    /// Venue subscription identity, scoped to this socket generation.
    #[serde(rename = "subscriptionId")]
    pub subscription_id: i64,
    /// Venue expiration value retained exactly. Documentation examples have inconsistent
    /// units; this field never overrides the REST token's millisecond authority.
    #[serde(rename = "expirationTime")]
    pub expiration_time: i64,
    /// Future native evidence with redacted diagnostics.
    #[serde(flatten)]
    pub extra: UnknownMessage,
}
const SUBSCRIBE: Operation = Operation {
    name: "userDataStreamSubscribeListenToken",
    path: "/userDataStream.subscribe.listenToken",
    method: "POST",
    security: Security::Public,
    mutation: true,
    weight: 2,
    requests_per_second: None,
    requests_per_minute: None,
    success_weight: None,
    partial: None,
    definitive: super::validation::definitive,
    validate_time: super::validation::validate_time,
};
fn decode_subscription(
    value: serde_json::Value,
    meta: crate::ResponseMeta,
) -> Result<Response<Subscription>, Error> {
    let result: Response<Subscription> =
        crate::core::http::decode(SUBSCRIBE, meta.status, value, meta)?;
    if result.data.subscription_id < 0 || result.data.expiration_time < 0 {
        return Err(Error::Transport {
            client_order_ids: result.meta.client_order_ids.clone(),
            operation: SUBSCRIBE.name,
            outcome: crate::Outcome::Unknown,
            meta: Some(Box::new(result.meta)),
        });
    }
    Ok(result)
}

/// Cloneable handle for recommended Margin listen-token subscriptions.
/// No authenticated session is needed. Pending late replies retain attribution.
#[derive(Clone)]
pub struct WsClient {
    socket: Socket,
}
impl WsClient {
    /// Connect without spawning tasks. Run and join the returned driver explicitly.
    ///
    /// # Errors
    /// Returns connection admission/transport errors before accepting the socket.
    pub async fn connect(config: WsConfig) -> Result<(Self, ApiEvents, ConnectionDriver), Error> {
        let (socket, inner, driver) = Socket::connect_with_policy(
            config.api,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            SocketPolicy {
                handshake: Cost {
                    connections: 1,
                    ws_weight: 2,
                    ..Cost::default()
                },
                incoming_limit: 5,
                time_unit: crate::TimeUnit::Milliseconds,
                binary_decoder: None,
                api_key_header: false,
            },
        )
        .await?;
        Ok((
            Self { socket },
            ApiEvents { inner },
            ConnectionDriver { inner: driver },
        ))
    }
    /// Subscribe or renew using the issued token, once. Expiry is rechecked before send.
    /// Caller cancellation never authorizes replay; late answers remain socket events.
    ///
    /// # Errors
    /// Returns expired authority/admission before send or typed ambiguous/provider evidence.
    pub async fn subscribe_listen_token(
        &self,
        request: &SubscribeToken,
        id: RequestId,
        deadline: tokio::time::Instant,
    ) -> Result<Response<Subscription>, Error> {
        let (value, meta) = self
            .socket
            .call(
                SUBSCRIBE,
                crate::core::parameters(request)?,
                Cost {
                    ws_weight: 2,
                    authority_expiry: Some(request.expiration_time),
                    ..Cost::default()
                },
                id,
                deadline,
            )
            .await?;
        decode_subscription(value, meta)
    }

    /// Request shutdown promptly; drain events through retirement, then join the driver.
    ///
    /// # Errors
    /// Returns `Closed` for an already retired socket.
    pub async fn close(&self) -> Result<(), Error> {
        self.socket.close().await
    }
    /// Immutable source generation.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.socket.generation()
    }
    /// Observable ingress depth, oldest-item age and progress.
    ///
    /// # Errors
    /// Returns a synchronization error for failed metrics ownership.
    pub fn queue_stats(&self) -> Result<QueueStats, Error> {
        self.socket.stats()
    }
}
/// Source-ordered API subscription evidence and explicit socket generation boundaries.
#[derive(Debug)]
#[non_exhaustive]
pub enum ApiEvent {
    /// Unexpected transport control evidence with an explicit typed invariant failure.
    UnexpectedControl(UnexpectedControl),
    /// Socket established; this does not establish account state continuity.
    Established(u64),
    /// Native Margin partial update or execution evidence.
    UserData {
        /// Source socket generation.
        generation: u64,
        /// Native subscription ID; do not reuse it across generations.
        subscription_id: i64,
        /// Native event with asset identity and snapshot/update distinction.
        payload: UserPayload,
    },
    /// Late subscription reply retains request identity and safe rate metadata.
    Late {
        /// Source socket generation.
        generation: u64,
        /// Original caller correlation ID.
        id: RequestId,
        /// Native typed subscription answer or typed provider failure.
        result: Result<Response<Subscription>, Error>,
    },
    /// Future API evidence, retained without a continuity claim.
    Unknown {
        /// Source socket generation.
        generation: u64,
        /// Exact provider payload, redacted in diagnostics.
        payload: UnknownMessage,
    },
    /// Actual transport loss or malformed known event.
    Gap {
        /// Source socket generation.
        generation: u64,
        /// Typed cause, never log-only.
        error: Error,
    },
    /// Accepted generation prefix drained before this terminal boundary.
    Retired(u64),
}
/// Accepted stream-control evidence on a Margin socket that exposes no control API.
/// Preserve the complete answer and report a typed failure; retire after the accepted prefix.
#[non_exhaustive]
pub struct UnexpectedControl {
    /// Socket generation that accepted this answer.
    pub generation: u64,
    /// Native control kind retained without reinterpretation.
    pub control: StreamControl,
    /// Original caller stream identities.
    pub streams: Vec<String>,
    /// Venue/control outcome retained even when it failed.
    pub result: Result<Option<Vec<String>>, Error>,
    /// Explicit local boundary failure; this is never a successful Margin operation.
    pub error: Error,
}
impl std::fmt::Debug for UnexpectedControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnexpectedControl")
            .field("generation", &self.generation)
            .field("control", &self.control)
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}
/// Single consumer preserving all accepted Margin user events in source order.
pub struct ApiEvents {
    inner: SocketEvents,
}
impl ApiEvents {
    /// Receive the next event; after shutdown drain the accepted prefix through retirement.
    pub async fn recv(&mut self) -> Option<ApiEvent> {
        Some(match self.inner.recv().await? {
            SocketEvent::ControlLate {
                generation,
                control,
                streams,
                result,
            } => {
                self.inner.stop();
                ApiEvent::UnexpectedControl(UnexpectedControl {
                    generation,
                    control,
                    streams,
                    result,
                    error: Error::Gap("unexpected Margin API stream control answer"),
                })
            }
            SocketEvent::Established(g) => ApiEvent::Established(g),
            SocketEvent::Retired(g) => ApiEvent::Retired(g),
            SocketEvent::Gap { generation, error } => ApiEvent::Gap { generation, error },
            SocketEvent::Late {
                generation,
                id,
                op,
                value,
                meta,
            } => {
                if op.name == SUBSCRIBE.name {
                    ApiEvent::Late {
                        generation,
                        id,
                        result: decode_subscription(value, meta),
                    }
                } else {
                    ApiEvent::Gap {
                        generation,
                        error: Error::Gap("unknown Margin pending operation"),
                    }
                }
            }
            SocketEvent::Data { generation, value } => {
                if let Some(event) = value.get("event") {
                    let decoded = value
                        .get("subscriptionId")
                        .and_then(serde_json::Value::as_i64)
                        .filter(|n| *n >= 0)
                        .ok_or(Error::Gap("Margin subscription identity required"))
                        .and_then(|subscription_id| {
                            user_payload(event.clone()).map(|payload| (subscription_id, payload))
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
        })
    }
}
/// Dedicated cross-margin risk socket. It never claims isolated margin coverage.
pub struct RiskStream {
    socket: Socket,
    events: SocketEvents,
}
impl RiskStream {
    /// Connect a cross-margin risk key without spawning tasks or refreshing it silently.
    /// Caller renews the REST key, drains retirement and joins the driver on shutdown.
    ///
    /// # Errors
    /// Refuses malformed keys or typed connection admission/transport errors.
    pub async fn connect(
        config: WsConfig,
        key: &SensitiveString,
    ) -> Result<(Self, ConnectionDriver), Error> {
        if key.as_str().is_empty() || key.as_str().chars().any(char::is_control) {
            return Err(Error::Validation("Margin risk listen key"));
        }
        let mut url = config.risk;
        url.path_segments_mut()
            .map_err(|()| Error::Configuration("Margin risk URI path"))?
            .clear()
            .push("ws")
            .push(key.as_str());
        url.set_query(None);
        let (socket, events, driver) = Socket::connect_with_policy(
            url,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            SocketPolicy {
                handshake: Cost {
                    connections: 1,
                    ..Cost::default()
                },
                incoming_limit: 5,
                time_unit: crate::TimeUnit::Milliseconds,
                binary_decoder: None,
                api_key_header: false,
            },
        )
        .await?;
        Ok((Self { socket, events }, ConnectionDriver { inner: driver }))
    }
    /// Receive every accepted risk event in source order; malformed known records gap.
    pub async fn recv(&mut self) -> Option<RiskEvent> {
        Some(match self.events.recv().await? {
            SocketEvent::ControlLate {
                generation,
                control,
                streams,
                result,
            } => {
                self.events.stop();
                RiskEvent::UnexpectedControl(UnexpectedControl {
                    generation,
                    control,
                    streams,
                    result,
                    error: Error::Gap("unexpected Margin risk stream control answer"),
                })
            }
            SocketEvent::Established(g) => RiskEvent::Established(g),
            SocketEvent::Retired(g) => RiskEvent::Retired(g),
            SocketEvent::Gap { generation, error } => RiskEvent::Gap { generation, error },
            SocketEvent::Data { generation, value } => match user_payload(value) {
                Ok(payload) => RiskEvent::Data {
                    generation,
                    payload,
                },
                Err(error) => {
                    self.events.stop();
                    RiskEvent::Gap { generation, error }
                }
            },
            SocketEvent::Late { generation, .. } => {
                self.events.stop();
                RiskEvent::Gap {
                    generation,
                    error: Error::Gap("unexpected Margin risk request answer"),
                }
            }
        })
    }
    /// Request retirement; drain events and join the caller-owned driver.
    ///
    /// # Errors
    /// Returns `Closed` for an already retired stream.
    pub async fn close(&self) -> Result<(), Error> {
        self.socket.close().await
    }
    /// Source generation, unchanged for every accepted event on this socket.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.socket.generation()
    }
    /// Inspect ingress backlog without declaring a continuity gap.
    ///
    /// # Errors
    /// Returns synchronization errors from the metrics owner.
    pub fn queue_stats(&self) -> Result<QueueStats, Error> {
        self.socket.stats()
    }
}
/// Risk-stream source ordering and lifecycle boundaries.
#[derive(Debug)]
#[non_exhaustive]
pub enum RiskEvent {
    /// Unexpected transport control evidence with an explicit typed invariant failure.
    UnexpectedControl(UnexpectedControl),
    /// Risk socket established, without a complete account-state claim.
    Established(u64),
    /// Partial cross-margin risk fact with asset and principal/interest evidence.
    Data {
        /// Source generation.
        generation: u64,
        /// Native Margin risk payload.
        payload: UserPayload,
    },
    /// Malformed known record or actual transport loss.
    Gap {
        /// Source generation.
        generation: u64,
        /// Typed cause.
        error: Error,
    },
    /// Terminal boundary after the accepted prefix drains.
    Retired(u64),
}
