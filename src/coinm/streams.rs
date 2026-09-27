// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Routed market and execution streams with explicit generation boundaries.

use super::{
    Config, ConnectionDriver,
    event_payloads::{MarketPayload, UserPayload, market_payload, user_payload},
};
use crate::core::socket::{QueueStats, SocketEvents};
use crate::core::{Socket, SocketEvent};
use crate::{Error, SensitiveString};
use std::collections::BTreeMap;

/// Binance's distinct market-data and execution endpoint routes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Route {
    /// Regular market data.
    Market,
    /// Account and order execution events.
    Private,
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

/// Lossless, single-consumer ingress for one routed socket generation.
pub struct Streams {
    socket: Socket,
    events: SocketEvents,
    kinds: BTreeMap<String, &'static str>,
    private: bool,
}
impl Streams {
    /// Connect a set of market subscriptions without spawning a task.
    ///
    /// # Errors
    /// Refuses mixed routes, duplicate names, or more than Binance's documented
    /// 1024 subscriptions per socket. Open distinct sockets for distinct routes.
    pub async fn connect(
        config: Config,
        streams: &[Stream],
    ) -> Result<(Self, ConnectionDriver), Error> {
        let route = streams
            .first()
            .ok_or(Error::Validation("empty subscription set"))?
            .route;
        if streams.len() > 1024 {
            return Err(Error::Validation("documented 1024 streams per socket"));
        }
        let mut kinds = BTreeMap::new();
        for s in streams {
            if s.route != route || kinds.insert(s.name.clone(), s.kind).is_some() {
                return Err(Error::Validation("mixed routes or duplicate streams"));
            }
        }
        let mut url = config.streams.clone();
        url.set_path("/stream");
        url.query_pairs_mut().append_pair(
            "streams",
            &streams
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>()
                .join("/"),
        );
        let (socket, events, driver) = Socket::connect(
            url,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            false,
        )
        .await?;
        Ok((
            Self {
                socket,
                events,
                kinds,
                private: false,
            },
            ConnectionDriver { inner: driver },
        ))
    }
    /// Connect one user-data listen key through the documented raw stream path.
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
        url.set_path("/ws/");
        url.path_segments_mut()
            .map_err(|()| Error::Configuration("private stream URL"))?
            .pop_if_empty()
            .push(listen_key.as_str());
        let (socket, events, driver) = Socket::connect(
            url,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            false,
        )
        .await?;
        Ok((
            Self {
                socket,
                events,
                kinds: BTreeMap::new(),
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
                    let kind = name.and_then(|name| self.kinds.get(name)).copied();
                    match (name, kind, value.get("data")) {
                        (Some(name), Some(kind), Some(data)) => market_payload(kind, data.clone())
                            .map(|payload| StreamPayload::Market {
                                stream: name.to_owned(),
                                payload,
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
        })
    }
    /// Request retirement, drain through `Retired`, and join the caller-owned driver.
    ///
    /// # Errors
    /// Returns `Closed` if the driver already retired.
    pub async fn close(&self) -> Result<(), Error> {
        self.socket.close().await
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
    /// Terminal boundary after the generation's accepted prefix.
    Retired(u64),
}
