// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Caller-driven native sockets; each generation preserves accepted source order.
use super::config::{Config, Product};
use crate::core::{
    Socket, SocketEvent,
    socket::{SocketEvents, SocketPolicy},
};
use crate::{Error, SensitiveString, StreamControl};
use serde_json::Value;
use std::{collections::BTreeMap, marker::PhantomData};

/// Sealed native stream protocol, implemented only for products with official streams.
pub trait StreamingProduct: Product {
    /// Product-local event payload.
    type Payload;
    /// Whether the product has public market subscriptions.
    const MARKET: bool;
    /// A documented maximum connection age, absent when the source states none.
    const MAX_GENERATION_AGE: Option<std::time::Duration>;
    /// Whether the market route adds `/ws` to the configured base.
    const MARKET_WS_SUFFIX: bool;
    /// Product-native subscription-list method spelling.
    const LIST_CONTROL: StreamControl;
    /// Private stream kind, absent for event-discriminated Portfolio Margin streams.
    const PRIVATE_KIND: Option<&'static str>;
    /// Append this documented suffix to an issued user-data key.
    const PRIVATE_SUFFIX: Option<&'static str>;
    /// Decode native evidence; future kinds remain explicit unknown payloads.
    ///
    /// # Errors
    /// Returns a gap for malformed known data.
    fn decode(kind: Option<&str>, value: Value) -> Result<Self::Payload, Error>;
}
/// A typed native public stream name; execution streams use `user_data` instead.
pub struct Stream<P: StreamingProduct> {
    name: String,
    kind: &'static str,
    product: PhantomData<P>,
}
impl<P: StreamingProduct> std::fmt::Debug for Stream<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream")
            .field("name", &self.name)
            .field("kind", &self.kind)
            .finish()
    }
}
impl<P: StreamingProduct> Stream<P> {
    pub(crate) fn new(name: String, kind: &'static str) -> Result<Self, Error> {
        if name.is_empty()
            || name.chars().any(char::is_whitespace)
            || name.contains(['/', '?', '&', '#', '{', '}'])
        {
            return Err(Error::Validation("native stream name"));
        }
        Ok(Self {
            name,
            kind,
            product: PhantomData,
        })
    }
    /// Exact native market stream name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
/// Source-ordered data and lifecycle evidence for one socket generation.
#[derive(Debug)]
#[non_exhaustive]
pub enum StreamEvent<P> {
    /// Established native socket generation.
    Established(u64),
    /// Native evidence; raw frames have no stream identity invented for them.
    Data {
        /// Generation that accepted the frame.
        generation: u64,
        /// Native combined-stream identity, redacted because private names can contain tokens.
        stream: Option<SensitiveString>,
        /// Product-local event.
        payload: P,
    },
    /// Actual transport or decode loss; ordinary silence and lag are not gaps.
    Gap {
        /// Generation that detected loss.
        generation: u64,
        /// Typed cause.
        error: Error,
    },
    /// A control answer received after its caller stopped waiting.
    LateControl {
        /// Original generation.
        generation: u64,
        /// Original control operation.
        control: StreamControl,
        /// Public subscription names from that operation.
        streams: Vec<String>,
        /// Exact late subscription list or refusal evidence.
        result: Result<Option<Vec<String>>, Error>,
    },
    /// Terminal boundary after the entire accepted prefix drains.
    Retired(u64),
}
/// One native market or execution connection and its single-consumer ingress.
/// No task, runtime, recovery, renewal or resubscription is started implicitly.
pub struct Streams<P: StreamingProduct> {
    socket: Socket,
    events: SocketEvents,
    kinds: BTreeMap<String, &'static str>,
    private: bool,
    product: PhantomData<P>,
}
impl<P: StreamingProduct> Streams<P> {
    /// Open the documented market socket with no subscriptions. Run the returned
    /// driver alongside `subscribe`, then close, drain `Retired` and join it.
    ///
    /// # Errors
    /// Refuses products without public streams or a failed native connection.
    pub async fn connect(config: Config<P>) -> Result<(Self, ConnectionDriver), Error> {
        if !P::MARKET {
            return Err(Error::Validation("product has only execution streams"));
        }
        let mut url = config
            .streams
            .clone()
            .ok_or(Error::Configuration("native stream route required"))?;
        if P::MARKET_WS_SUFFIX {
            url.set_path(&format!("{}/ws", url.path().trim_end_matches('/')));
        }
        Self::open(config, url, false).await
    }
    /// Open a separate execution socket using a caller-issued native listen key.
    /// The caller owns issuance and timely renewal. Reconnect establishes a new
    /// generation and never authorizes mutation replay.
    ///
    /// # Errors
    /// Refuses empty/path-bearing tokens and failed connections.
    pub async fn user_data(
        config: Config<P>,
        listen_key: SensitiveString,
    ) -> Result<(Self, ConnectionDriver), Error> {
        let suffix =
            P::PRIVATE_SUFFIX.ok_or(Error::Validation("product has no execution stream"))?;
        let key = listen_key.as_str();
        if key.is_empty()
            || key.chars().any(char::is_whitespace)
            || key.contains(['/', '?', '&', '#', '%'])
        {
            return Err(Error::Validation("native listen key"));
        }
        let mut url = config
            .streams
            .clone()
            .ok_or(Error::Configuration("native stream route required"))?;
        url.set_path(&format!(
            "{}/ws/{key}{suffix}",
            url.path().trim_end_matches('/')
        ));
        url.set_query(None);
        Self::open(config, url, true).await
    }
    async fn open(
        config: Config<P>,
        url: url::Url,
        private: bool,
    ) -> Result<(Self, ConnectionDriver), Error> {
        config.check_stream_transport()?;
        // These product pages state no numeric incoming-message ceiling. Preserve
        // the native ping/pong duty cycle without inventing another product's cap.
        let (socket, events, driver) = Socket::connect_with_policy(
            url,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            SocketPolicy {
                handshake: crate::core::Cost::default(),
                incoming_limit: usize::MAX,
                time_unit: crate::TimeUnit::Milliseconds,
                binary_decoder: None,
                api_key_header: false,
                max_generation_age: P::MAX_GENERATION_AGE,
            },
        )
        .await?;
        Ok((
            Self {
                socket,
                events,
                kinds: BTreeMap::new(),
                private,
                product: PhantomData,
            },
            ConnectionDriver { inner: driver },
        ))
    }
    /// Subscribe public names in this generation. The caller must drive the socket.
    /// Mappings remain for late/uncertain answers; only venue answers prove membership.
    ///
    /// # Errors
    /// Refuses private sockets, duplicates and typed transport/venue errors. A lost
    /// answer is uncertain; the library never replays the control request.
    pub async fn subscribe(
        &mut self,
        streams: &[Stream<P>],
        deadline: tokio::time::Instant,
    ) -> Result<(), Error> {
        if self.private {
            return Err(Error::Validation(
                "execution subscriptions cannot change membership",
            ));
        }
        let mut names = BTreeMap::new();
        for stream in streams {
            if names.insert(stream.name.clone(), stream.kind).is_some() {
                return Err(Error::Validation("duplicate native subscription"));
            }
        }
        if names.is_empty() {
            return Err(Error::Validation("empty native subscription request"));
        }
        self.kinds.extend(names.clone());
        self.socket
            .control(
                StreamControl::Subscribe,
                names.into_keys().collect(),
                deadline,
            )
            .await?;
        Ok(())
    }
    /// Unsubscribe public names; retain decoding attribution for already accepted data.
    ///
    /// # Errors
    /// Refuses execution sockets and returns typed control evidence on failure.
    pub async fn unsubscribe(
        &self,
        streams: &[Stream<P>],
        deadline: tokio::time::Instant,
    ) -> Result<(), Error> {
        if self.private || streams.is_empty() {
            return Err(Error::Validation("native unsubscribe request"));
        }
        self.socket
            .control(
                StreamControl::Unsubscribe,
                streams.iter().map(|s| s.name.clone()).collect(),
                deadline,
            )
            .await?;
        Ok(())
    }
    /// Query this generation's actual public membership.
    ///
    /// # Errors
    /// Refuses private sockets, failures and malformed list answers.
    pub async fn list_subscriptions(
        &self,
        deadline: tokio::time::Instant,
    ) -> Result<Vec<String>, Error> {
        if self.private {
            return Err(Error::Validation("execution subscription list"));
        }
        self.socket
            .control(P::LIST_CONTROL, Vec::new(), deadline)
            .await?
            .ok_or(Error::Gap("native subscription list required"))
    }
    /// Receive the next accepted native event or explicit lifecycle boundary.
    /// Malformed known data stops further ingress, then drains the accepted prefix.
    pub async fn recv(&mut self) -> Option<StreamEvent<P::Payload>> {
        Some(match self.events.recv().await? {
            SocketEvent::Established(g) => StreamEvent::Established(g),
            SocketEvent::Retired(g) => StreamEvent::Retired(g),
            SocketEvent::Gap { generation, error } => StreamEvent::Gap { generation, error },
            SocketEvent::NativeBinary { generation, .. } => {
                self.events.stop();
                StreamEvent::Gap {
                    generation,
                    error: Error::Configuration("unexpected native binary socket payload"),
                }
            }
            SocketEvent::Data {
                generation,
                mut value,
            } => {
                let (stream, kind, data) = if self.private {
                    (None, P::PRIVATE_KIND, value)
                } else if let (Some(stream), Some(_)) = (
                    value.get("stream").and_then(Value::as_str),
                    value.get("data"),
                ) {
                    (
                        Some(SensitiveString::new(stream)),
                        self.kinds.get(stream).copied(),
                        value["data"].take(),
                    )
                } else if value.get("stream").is_some() {
                    self.events.stop();
                    return Some(StreamEvent::Gap {
                        generation,
                        error: Error::Gap("malformed native combined-stream envelope"),
                    });
                } else {
                    (None, None, value)
                };
                match P::decode(kind, data) {
                    Ok(payload) => StreamEvent::Data {
                        generation,
                        stream,
                        payload,
                    },
                    Err(error) => {
                        self.events.stop();
                        StreamEvent::Gap { generation, error }
                    }
                }
            }
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
            SocketEvent::Late { generation, .. } => {
                self.events.stop();
                StreamEvent::Gap {
                    generation,
                    error: Error::Gap("unexpected API response on native event socket"),
                }
            }
        })
    }
    /// Observable backlog and accepted/drained counters; lag does not discard data.
    ///
    /// # Errors
    /// Returns a diagnostic lock failure.
    pub fn queue_stats(&self) -> Result<crate::core::socket::QueueStats, Error> {
        self.socket.stats()
    }
    /// Request generation retirement. Continue receiving and join the driver.
    ///
    /// # Errors
    /// Returns a closed-driver failure.
    pub async fn close(&self) -> Result<(), Error> {
        self.socket.close().await
    }
}
/// Caller-owned driver; run it on the caller runtime and join after retirement.
#[must_use = "the caller must run and join this native stream driver"]
pub struct ConnectionDriver {
    inner: crate::core::socket::SocketDriver,
}
impl ConnectionDriver {
    /// Drive this generation until explicit close or transport retirement.
    ///
    /// # Errors
    /// Returns lifecycle/consumer failures; transport loss also emits a gap.
    pub async fn run(self) -> Result<(), Error> {
        self.inner.run().await
    }
}
