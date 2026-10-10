// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Routed market and execution streams with explicit generation boundaries.

use super::{
    Config, ConnectionDriver,
    event_payloads::{MarketPayload, UserPayload, market_payload, user_payload},
};
use crate::core::control::{self, Membership};
use crate::core::socket::{QueueStats, SocketEvents};
use crate::core::{Socket, SocketEvent};
use crate::{Error, SensitiveString};
use tokio::time::Instant;

/// Binance's distinct market-data and execution endpoint routes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Route {
    /// High-frequency public depth/book data.
    Public,
    /// Regular market data.
    Market,
    /// Account and order execution events.
    Private,
}
impl Route {
    fn path(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Market => "market",
            Self::Private => "private",
        }
    }
}

/// A validated documented stream name and its required route.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stream {
    name: String,
    route: Route,
    kind: &'static str,
}
impl Stream {
    pub(crate) fn new(name: String, route: Route, kind: &'static str) -> Result<Self, Error> {
        if name.is_empty()
            || name.chars().any(char::is_whitespace)
            || name.contains(['/', '?', '&', '#', '{', '}'])
        {
            return Err(Error::Validation("stream name"));
        }
        Ok(Self { name, route, kind })
    }
    /// Exact wire stream name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The required routed endpoint.
    #[must_use]
    pub fn route(&self) -> Route {
        self.route
    }
}

fn pairs(streams: &[Stream]) -> Vec<(&str, &'static str)> {
    streams.iter().map(|s| (s.name.as_str(), s.kind)).collect()
}

fn names(streams: &[Stream]) -> Vec<&str> {
    streams.iter().map(|s| s.name.as_str()).collect()
}

/// Lossless, single-consumer ingress for one routed socket generation.
pub struct Streams {
    socket: Socket,
    events: SocketEvents,
    membership: Membership,
    route: Route,
    private: bool,
}
impl Streams {
    /// Connect one routed market socket without spawning a task.
    ///
    /// With an empty `streams` slice, the client omits the initial stream query
    /// and initializes empty membership. [`Streams::subscribe`] can subsequently
    /// send Binance's documented live subscription control messages. A socket
    /// stays on its route for life.
    ///
    /// # Errors
    /// Refuses the private route (use [`Streams::user_data`]), a stream of another
    /// route, duplicate names, or more than Binance's documented 1024 subscriptions
    /// per socket. Open distinct sockets for distinct routes.
    pub async fn connect(
        config: Config,
        route: Route,
        streams: &[Stream],
    ) -> Result<(Self, ConnectionDriver), Error> {
        if route == Route::Private {
            return Err(Error::Validation("private route carries user data only"));
        }
        if streams.iter().any(|s| s.route != route) {
            return Err(Error::Validation("stream belongs to another route"));
        }
        let membership = Membership::connect(&pairs(streams))?;
        let mut url = config.streams.clone();
        url.set_path(&format!("/{}/stream", route.path()));
        if !streams.is_empty() {
            url.query_pairs_mut()
                .append_pair("streams", &names(streams).join("/"));
        }
        let (socket, events, driver) = Socket::connect_with_policy(
            url,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            crate::core::socket::SocketPolicy {
                handshake: crate::core::Cost::default(),
                // Pongs and stream control messages share the connection's
                // documented budget: "WebSocket connections have a limit of 10
                // incoming messages per second" (USDⓈ-M WebSocket Market
                // Streams, Connect; verified 2026-10-10).
                incoming_limit: 10,
                time_unit: crate::core::TimeUnit::Milliseconds,
                binary_decoder: None,
                api_key_header: false,
            },
        )
        .await?;
        Ok((
            Self {
                socket,
                events,
                membership,
                route,
                private: false,
            },
            ConnectionDriver { inner: driver },
        ))
    }
    /// Connect one user-data listen key through the private route.
    /// The caller renews the listen key through the documented REST/WS operation;
    /// renewals do not imply execution continuity, and expiration is an explicit event.
    ///
    /// # Errors
    /// Refuses an empty listen key or failed connection.
    pub async fn user_data(
        config: Config,
        listen_key: &SensitiveString,
    ) -> Result<(Self, ConnectionDriver), Error> {
        if listen_key.as_str().is_empty() {
            return Err(Error::Validation("empty listen key"));
        }
        let mut url = config.streams.clone();
        url.set_path("/private/ws/");
        url.path_segments_mut()
            .map_err(|()| Error::Configuration("private stream URL"))?
            .pop_if_empty()
            .push(listen_key.as_str());
        let (socket, events, driver) = Socket::connect_with_policy(
            url,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            crate::core::socket::SocketPolicy {
                handshake: crate::core::Cost::default(),
                // Same documented raw-stream connection duty cycle as the
                // market route: 10 venue-incoming messages per second bound the
                // pong stream this driver emits (USDⓈ-M WebSocket Market
                // Streams, Connect; verified 2026-10-03).
                incoming_limit: 10,
                time_unit: crate::core::TimeUnit::Milliseconds,
                binary_decoder: None,
                api_key_header: false,
            },
        )
        .await?;
        Ok((
            Self {
                socket,
                events,
                membership: Membership::default(),
                route: Route::Private,
                private: true,
            },
            ConnectionDriver { inner: driver },
        ))
    }
    /// Next source-ordered data/lifecycle event. Backlog never discards a fact.
    /// Malformed known execution data emits a gap and retires the transport.
    pub async fn recv(&mut self) -> Option<StreamEvent> {
        Some(match self.events.recv().await? {
            SocketEvent::Established(g) => StreamEvent::Established(g),
            SocketEvent::Retired(g) => StreamEvent::Retired(g),
            SocketEvent::Gap { generation, error } => StreamEvent::Gap { generation, error },
            SocketEvent::Data { generation, value } => {
                let decoded = if self.private {
                    user_payload(value).map(StreamPayload::User)
                } else {
                    let name = value.get("stream").and_then(serde_json::Value::as_str);
                    let kind = name.and_then(|name| self.membership.kind(name));
                    match (name, kind, value.get("data")) {
                        (Some(name), Some(kind), Some(data)) => market_payload(kind, data.clone())
                            .map(|payload| StreamPayload::Market {
                                stream: name.to_owned(),
                                payload,
                            }),
                        // A stream name this build did not subscribe or model is
                        // retained evidence on a live socket, not a continuity
                        // break; only a malformed envelope of a known kind gaps.
                        (Some(name), None, Some(data)) => Ok(StreamPayload::Market {
                            stream: name.to_owned(),
                            payload: MarketPayload::Unknown(data.clone().into()),
                        }),
                        _ => Err(Error::Gap("unexpected combined stream envelope")),
                    }
                };
                match decoded {
                    Ok(payload) => StreamEvent::Data {
                        generation,
                        payload,
                    },
                    Err(error) => {
                        self.events.stop();
                        StreamEvent::Gap { generation, error }
                    }
                }
            }
            SocketEvent::Late { generation, .. } => StreamEvent::Gap {
                generation,
                error: Error::Gap("unexpected market request response"),
            },
            SocketEvent::ControlLate {
                generation,
                control,
                streams,
                result,
            } => StreamEvent::LateControl {
                generation,
                control,
                streams,
                result,
            },
        })
    }
    /// Add `streams` to this open socket with one `SUBSCRIBE` control message.
    ///
    /// Data for the added streams arrives in this generation, decoded like the
    /// streams named at connect. Ingress keeps accumulating while the call waits.
    /// The message is charged with pongs against the connection's documented
    /// incoming-message ceiling, whose last slot stays reserved for a pong.
    /// Nothing is resubscribed after a reconnect: a new
    /// generation carries only the streams its own `connect` and calls name.
    ///
    /// # Errors
    /// Unsent: an empty or duplicated set, a user-data socket, a stream of another route,
    /// more than 1024 streams on the socket, an expired deadline, or
    /// [`Error::Admission`] when the incoming-message ceiling has no slot a
    /// control may take; its last slot stays reserved for a pong.
    /// [`Error::ControlRefused`] carries the venue's code when it refuses. A lost
    /// answer is [`crate::Outcome::Unknown`]; ask [`Streams::list_subscriptions`].
    pub async fn subscribe(&mut self, streams: &[Stream], deadline: Instant) -> Result<(), Error> {
        self.market_socket()?;
        if streams.iter().any(|s| s.route != self.route) {
            return Err(Error::Validation("stream belongs to another route"));
        }
        control::subscribe(
            &self.socket,
            &mut self.membership,
            &pairs(streams),
            deadline,
        )
        .await
    }

    /// Remove `streams` from this open socket with one `UNSUBSCRIBE` control message.
    ///
    /// The generation stays open. Frames the venue sent before it applied the
    /// change are still delivered and decoded.
    ///
    /// # Errors
    /// Unsent: an empty or duplicated set, a stream this socket never subscribed,
    /// a user-data socket, an expired deadline, or [`Error::Admission`].
    /// [`Error::ControlRefused`] carries the venue's code when it refuses. A lost
    /// answer is [`crate::Outcome::Unknown`] and the streams stay counted.
    pub async fn unsubscribe(
        &mut self,
        streams: &[Stream],
        deadline: Instant,
    ) -> Result<(), Error> {
        self.market_socket()?;
        control::unsubscribe(
            &self.socket,
            &mut self.membership,
            &names(streams),
            deadline,
        )
        .await
    }

    /// The venue's own list of this socket's subscriptions, from `LIST_SUBSCRIPTIONS`.
    ///
    /// # Errors
    /// Unsent: a user-data socket, an expired deadline, or [`Error::Admission`].
    /// [`Error::ControlRefused`] carries the venue's code when it refuses; a lost
    /// answer is [`crate::Outcome::ReadFailed`].
    pub async fn list_subscriptions(&self, deadline: Instant) -> Result<Vec<String>, Error> {
        self.market_socket()?;
        control::list(&self.socket, deadline).await
    }

    /// Request retirement, drain through `Retired`, and join the caller-owned driver.
    ///
    /// # Errors
    /// Returns `Closed` if the driver already retired.
    pub async fn close(&self) -> Result<(), Error> {
        self.socket.close().await
    }
    fn market_socket(&self) -> Result<(), Error> {
        if self.private {
            return Err(Error::Validation(
                "user-data socket has no stream membership",
            ));
        }
        Ok(())
    }

    /// Immutable source generation.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.socket.generation()
    }
    /// Ingress backlog diagnostics.
    ///
    /// # Errors
    /// Returns a synchronization error if the metrics owner failed.
    pub fn queue_stats(&self) -> Result<QueueStats, Error> {
        self.socket.stats()
    }
}

/// Typed data inside a stream event.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum StreamPayload {
    /// A market stream retains its exact subscribed stream identity.
    Market {
        /// Subscription name, including interval/update speed where applicable.
        stream: String,
        /// Provider-native market payload.
        payload: MarketPayload,
    },
    /// Account, position, order and execution evidence, without deriving portfolio policy.
    User(UserPayload),
}

/// Source-ordered ingress and explicit lifecycle boundaries.
#[derive(Debug)]
#[non_exhaustive]
pub enum StreamEvent {
    /// Socket established; consumers still establish their required state.
    Established(u64),
    /// Accepted provider fact.
    Data {
        /// Original socket generation.
        generation: u64,
        /// Typed provider payload.
        payload: StreamPayload,
    },
    /// Actual transport or malformed-data gap, distinct from ordinary lag.
    Gap {
        /// Original socket generation.
        generation: u64,
        /// Typed cause.
        error: Error,
    },
    /// A control answer that arrived after its caller stopped waiting.
    LateControl {
        /// Original socket generation.
        generation: u64,
        /// The control message this answers.
        control: crate::StreamControl,
        /// Stream names the message carried; empty for a list.
        streams: Vec<String>,
        /// `None` for a confirmed `SUBSCRIBE` or `UNSUBSCRIBE`, the venue's list
        /// for `LIST_SUBSCRIPTIONS`, or the venue's refusal.
        result: Result<Option<Vec<String>>, Error>,
    },
    /// Terminal boundary after the generation's accepted prefix.
    Retired(u64),
}
