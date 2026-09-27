// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::MarketEvent;
use crate::core::{
    Socket, SocketEvent,
    socket::{QueueStats, SocketEvents, SocketPolicy},
};
use crate::core_trading::spot::{Config, ConnectionDriver, Environment};
use crate::{Budgets, Clock, Credentials, Error, Symbol, TimeUnit};
use std::{collections::BTreeSet, sync::Arc, time::Duration};

/// Caller-owned SBE market configuration. Only Ed25519 API keys are supported.
#[derive(Clone, Debug)]
pub struct MarketConfig(Config);
impl MarketConfig {
    /// Select the documented production endpoint with caller-acquired credentials.
    /// No authentication signature is sent on this public market connection.
    ///
    /// # Errors
    /// Refuses non-Ed25519 credentials or invalid endpoint/budget configuration.
    pub fn production(credentials: Credentials) -> Result<Self, Error> {
        if !credentials.is_ed25519() {
            return Err(Error::Validation("SBE streams require Ed25519 credentials"));
        }
        Ok(Self(
            Config::new(Environment::Production)?
                .streams_url("wss://stream-sbe.binance.com:9443")?
                .credentials(credentials),
        ))
    }
    /// Override the endpoint with WSS or an exact loopback fixture host.
    ///
    /// # Errors
    /// Refuses insecure remote URLs or URL credentials.
    pub fn endpoint(mut self, endpoint: &str) -> Result<Self, Error> {
        self.0 = self.0.streams_url(endpoint)?;
        Ok(self)
    }
    /// Share the IP owner used by other Spot clients on this same source IP.
    #[must_use]
    pub fn budgets(mut self, budgets: Budgets) -> Self {
        self.0 = self.0.budgets(budgets);
        self
    }
    /// Inject the caller's clock for venue budget accounting.
    #[must_use]
    pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.0 = self.0.clock(clock);
        self
    }
    /// Bound each transport attempt; no automatic reconnect or replay occurs.
    ///
    /// # Errors
    /// Refuses a zero timeout.
    pub fn timeout(mut self, timeout: Duration) -> Result<Self, Error> {
        self.0 = self.0.timeout(timeout)?;
        Ok(self)
    }
}
/// One validated SBE market stream; symbols retain their provider spelling.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketStream(String);
impl MarketStream {
    fn new(symbol: &Symbol, suffix: &str) -> Result<Self, Error> {
        if symbol.as_str().contains('@') {
            return Err(Error::Validation("SBE symbol delimiter"));
        }
        let name = format!("{}@{suffix}", symbol.as_str().to_lowercase());
        crate::core_trading::spot::Stream::new(
            name.clone(),
            crate::core_trading::spot::Route::Market,
            "sbe",
        )?;
        Ok(Self(name))
    }
    /// Raw trades.
    ///
    /// # Errors
    /// Refuses a symbol containing stream delimiters or whitespace.
    pub fn trades(symbol: &Symbol) -> Result<Self, Error> {
        Self::new(symbol, "trade")
    }
    /// Best prices; Binance may cull outdated events before delivery.
    ///
    /// # Errors
    /// Refuses a symbol containing stream delimiters or whitespace.
    pub fn best_bid_ask(symbol: &Symbol) -> Result<Self, Error> {
        Self::new(symbol, "bestBidAsk")
    }
    /// 20ms incremental depth.
    ///
    /// # Errors
    /// Refuses a symbol containing stream delimiters or whitespace.
    pub fn depth_diff(symbol: &Symbol) -> Result<Self, Error> {
        Self::new(symbol, "depth")
    }
    /// 50ms top-20 finite snapshots.
    ///
    /// # Errors
    /// Refuses a symbol containing stream delimiters or whitespace.
    pub fn depth_snapshot(symbol: &Symbol) -> Result<Self, Error> {
        Self::new(symbol, "depth20")
    }
    /// Exact provider subscription name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0
    }
}
/// Source-ordered accepted ingress, with no fixed queue capacity.
/// Best bid/ask auto-culling happens at the venue; this client never culls events.
pub struct MarketStreams {
    socket: Socket,
    events: SocketEvents,
}
/// Native data and lifecycle boundaries from one SBE market socket.
#[derive(Debug)]
#[non_exhaustive]
pub enum StreamEvent {
    /// Transport generation established.
    Established(u64),
    /// Decoded native data, timestamped in microseconds.
    Data {
        /// Source socket generation.
        generation: u64,
        /// Native payload.
        payload: MarketEvent,
    },
    /// Documented venue maintenance notice; no continuity across a replacement is assumed.
    ServerShutdown {
        /// Source socket generation.
        generation: u64,
        /// Native event timestamp in microseconds.
        event_time: i64,
    },
    /// Malformed record or actual transport loss.
    Gap {
        /// Source socket generation.
        generation: u64,
        /// Safe failure evidence.
        error: Error,
    },
    /// Accepted prefix precedes retirement; join the driver before releasing its owner.
    Retired(u64),
}
impl MarketStreams {
    /// Establish a generation without spawning a task. Run and join the returned driver.
    ///
    /// # Errors
    /// Refuses empty/duplicate subscriptions, the documented 1024 stream limit,
    /// invalid authentication, failed connection admission or handshake.
    pub async fn connect(
        config: MarketConfig,
        streams: &[MarketStream],
    ) -> Result<(Self, ConnectionDriver), Error> {
        if streams.is_empty() || streams.len() > 1024 {
            return Err(Error::Validation("SBE subscription count"));
        }
        let names: BTreeSet<_> = streams.iter().map(MarketStream::name).collect();
        if names.len() != streams.len() {
            return Err(Error::Validation("duplicate SBE subscriptions"));
        }
        let config = config.0;
        let mut url = config.streams;
        url.set_path("/stream");
        url.set_query(None);
        url.query_pairs_mut().append_pair(
            "streams",
            &streams
                .iter()
                .map(MarketStream::name)
                .collect::<Vec<_>>()
                .join("/"),
        );
        let (socket, events, inner) = Socket::connect_with_policy(
            url,
            config.credentials,
            config.clock,
            config.budgets,
            config.timeout,
            SocketPolicy {
                handshake: crate::core::Cost {
                    connections: 1,
                    ..Default::default()
                },
                ping_limit: 5,
                time_unit: TimeUnit::Microseconds,
                binary_decoder: Some(super::market::decode_value),
                api_key_header: true,
            },
        )
        .await?;
        Ok((Self { socket, events }, ConnectionDriver { inner }))
    }
    /// Next data or boundary. Queue age never triggers dropping or reconnection.
    pub async fn recv(&mut self) -> Option<StreamEvent> {
        Some(match self.events.recv().await? {
            SocketEvent::Established(g) => StreamEvent::Established(g),
            SocketEvent::Retired(g) => StreamEvent::Retired(g),
            SocketEvent::Gap { generation, error } => StreamEvent::Gap { generation, error },
            SocketEvent::Late { generation, .. } => StreamEvent::Gap {
                generation,
                error: Error::Gap("unexpected SBE market reply"),
            },
            SocketEvent::Data { generation, value } => {
                let data = value.get("data").unwrap_or(&value);
                if data.get("e").and_then(serde_json::Value::as_str) == Some("serverShutdown") {
                    if let Some(event_time) = data.get("E").and_then(serde_json::Value::as_i64) {
                        StreamEvent::ServerShutdown {
                            generation,
                            event_time,
                        }
                    } else {
                        self.events.stop();
                        StreamEvent::Gap {
                            generation,
                            error: Error::Gap("malformed SBE shutdown notice"),
                        }
                    }
                } else {
                    if let Ok(payload) = serde_json::from_value(value) {
                        StreamEvent::Data {
                            generation,
                            payload,
                        }
                    } else {
                        self.events.stop();
                        StreamEvent::Gap {
                            generation,
                            error: Error::Gap("unexpected SBE market text"),
                        }
                    }
                }
            }
        })
    }
    /// Depth, oldest accepted item age, and consumer progress for this generation.
    ///
    /// # Errors
    /// Reports diagnostic owner poisoning without swallowing accepted events.
    pub fn queue_stats(&self) -> Result<QueueStats, Error> {
        self.socket.stats()
    }
    /// Request cancellation. Drain through `Retired`, then join the returned driver.
    ///
    /// # Errors
    /// Returns `Closed` if the owner already stopped.
    pub async fn close(&self) -> Result<(), Error> {
        self.socket.close().await
    }
}
