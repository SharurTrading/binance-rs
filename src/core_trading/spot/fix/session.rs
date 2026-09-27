// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{
    CompId, Config, Encoding, Fields, Header, Message, Request, Role, Timestamp, Value,
    budgets::{Lease, MessageBudget},
    codec::{frame_length, wire_value},
    request,
};
use crate::{
    Error, Outcome, RequestId, SensitiveString,
    core::{
        Cost,
        socket::{QueueStats, tls_config},
    },
};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    sync::{mpsc, oneshot},
    time::Instant,
};
use tokio_util::sync::CancellationToken;
use zeroize::Zeroizing;

static GENERATION: AtomicU64 = AtomicU64::new(1);
trait Transport: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Transport for T {}
type Stream = Box<dyn Transport>;

/// One caller-owned attempt. A completed socket write proves no venue acceptance.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Attempt {
    /// Session generation; never reused on a replacement connection.
    pub generation: u64,
    /// Unique caller correlation identity within this generation.
    pub id: RequestId,
    /// Native request kind.
    pub kind: super::RequestKind,
    /// Native outbound sequence, absent for pre-send refusal.
    pub sequence: Option<u32>,
    /// Exact caller IDs, with native field paths for each list/operation leg.
    pub identities: BTreeMap<String, String>,
    /// Written mutations remain unknown until native venue evidence arrives.
    pub outcome: Outcome,
}
/// Safe failure evidence for a queued or uncertain attempt. No raw wire is retained.
#[derive(Debug, thiserror::Error)]
#[error("FIX send failed: {error}")]
#[non_exhaustive]
pub struct SendFailure {
    /// Caller identities and strongest send evidence.
    pub attempt: Attempt,
    /// Typed admission, cancellation or transport cause.
    #[source]
    pub error: Error,
}
/// Native records and explicit boundaries from one FIX session.
#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    /// Authenticated initial acknowledgment, including the native server UUID.
    Established {
        /// Source generation.
        generation: u64,
        /// Authenticated caller component; binary headers omit component IDs.
        component: CompId,
        /// Native Logon acknowledgment.
        acknowledgment: Message,
    },
    /// Every accepted native message, including control and maintenance News.
    Message {
        /// Source generation.
        generation: u64,
        /// Authenticated local component, separate from binary header omissions.
        component: CompId,
        /// Source-ordered native evidence.
        message: Message,
        /// All matching local attempts, including old/reused IDs and late receipts.
        /// An empty set can be another account session's order or unsolicited data.
        candidates: Vec<Attempt>,
    },
    /// Actual sequence, decode, heartbeat-probe or transport failure.
    Gap {
        /// Source generation.
        generation: u64,
        /// Safe typed cause.
        error: Error,
        /// Every written local attempt remains attributable through retirement.
        /// Consumers retain independent receipts; this list does not override them.
        attempts: Vec<Attempt>,
    },
    /// Last event, after the accepted prefix; join the driver's task as owner.
    Retired(u64),
}
#[derive(Default)]
struct Metrics {
    arrivals: VecDeque<Instant>,
    accepted: u64,
    drained: u64,
}
enum Command {
    Send {
        id: RequestId,
        request: Request,
        deadline: Instant,
        reply: oneshot::Sender<Result<Attempt, SendFailure>>,
    },
}
/// Cloneable dispatch handle; caller correlation IDs are never generated or rewritten.
#[derive(Clone)]
pub struct Session {
    tx: mpsc::UnboundedSender<Command>,
    stop: CancellationToken,
    generation: u64,
    timeout: Duration,
}
/// Unbounded ingress owner, independent of dispatch handles. Dropping it cancels the
/// driver; its task must still be joined by the caller.
pub struct Events {
    rx: mpsc::UnboundedReceiver<Event>,
    stats: Arc<Mutex<Metrics>>,
    stop: CancellationToken,
    _commands: mpsc::UnboundedSender<Command>,
}
impl Drop for Events {
    fn drop(&mut self) {
        self.stop.cancel();
    }
}
impl Events {
    /// Receive the next accepted record in source order. Queue age is only diagnostic.
    pub async fn recv(&mut self) -> Option<Event> {
        let event = self.rx.recv().await;
        if event.is_some()
            && let Ok(mut stats) = self.stats.lock()
        {
            stats.arrivals.pop_front();
            stats.drained = stats.drained.saturating_add(1);
        }
        event
    }
    /// Observe lag and consumer progress without changing continuity.
    ///
    /// # Errors
    /// Reports diagnostic mutex poisoning, without discarding accepted records.
    pub fn queue_stats(&self) -> Result<QueueStats, Error> {
        let stats = self
            .stats
            .lock()
            .map_err(|_| Error::Configuration("FIX queue diagnostics poisoned"))?;
        Ok(QueueStats {
            depth: stats.arrivals.len(),
            oldest_age: stats
                .arrivals
                .front()
                .map_or(Duration::ZERO, Instant::elapsed),
            accepted: stats.accepted,
            drained: stats.drained,
        })
    }
}
/// Single writer and lifecycle owner. `run` starts no child tasks; spawn it only in
/// the caller's runtime, request shutdown and join that task before releasing it.
pub struct SessionDriver {
    stream: Stream,
    config: Config,
    lease: Lease,
    commands: mpsc::UnboundedReceiver<Command>,
    events: mpsc::UnboundedSender<Event>,
    stats: Arc<Mutex<Metrics>>,
    stop: CancellationToken,
    generation: u64,
    outgoing: u32,
    incoming: u32,
    attempts: BTreeMap<String, Attempt>,
    used: BTreeSet<String>,
    messages: MessageBudget,
    input: Zeroizing<Vec<u8>>,
    last_sent: Instant,
    last_received: Instant,
    probe: Option<(String, Instant)>,
    authenticated: bool,
    clean: bool,
    subscriptions: BTreeMap<String, BTreeSet<String>>,
}
impl Session {
    /// Establish TLS/TCP without spawning a task. Run the returned driver to send
    /// Logon and consume `Established` before dispatching application requests.
    /// Connection attempts/concurrency and component ownership share the account owner.
    ///
    /// # Errors
    /// Reports account admission, TCP/TLS or connection timeout failures safely.
    pub async fn connect(config: Config) -> Result<(Self, Events, SessionDriver), Error> {
        let lease = config
            .budgets
            .connect(config.role, &config.component, config.heartbeat)?;
        let generation = GENERATION
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map_err(|_| Error::Configuration("FIX generation overflow"))?;
        let stream = tokio::time::timeout(config.timeout, connect_transport(&config))
            .await
            .map_err(|_| Error::Gap("FIX connect timeout"))??;
        Ok(Self::assemble(config, stream, lease, generation))
    }
    fn assemble(
        config: Config,
        stream: Stream,
        lease: Lease,
        generation: u64,
    ) -> (Self, Events, SessionDriver) {
        let (tx, commands) = mpsc::unbounded_channel();
        let (events_tx, rx) = mpsc::unbounded_channel();
        let stats = Arc::new(Mutex::new(Metrics::default()));
        let stop = CancellationToken::new();
        let handle = Self {
            tx: tx.clone(),
            stop: stop.clone(),
            generation,
            timeout: config.timeout,
        };
        let events = Events {
            rx,
            stats: stats.clone(),
            stop: stop.clone(),
            _commands: tx,
        };
        let messages = MessageBudget::new(config.role);
        let now = Instant::now();
        let driver = SessionDriver {
            stream,
            config,
            lease,
            commands,
            events: events_tx,
            stats,
            stop,
            generation,
            outgoing: 1,
            incoming: 1,
            attempts: BTreeMap::new(),
            used: BTreeSet::new(),
            messages,
            input: Zeroizing::new(vec![]),
            last_sent: now,
            last_received: now,
            probe: None,
            authenticated: false,
            clean: false,
            subscriptions: BTreeMap::new(),
        };
        (handle, events, driver)
    }
    /// Dispatch once. Completion means a socket write, never venue acceptance.
    /// A queued cancellation/expiration prevents sending; aborting after a write
    /// cannot cancel a venue order. No failure causes a retry or reconnect.
    /// Reusing a wire client ID is permitted; the separate local ID must be unique.
    ///
    /// # Errors
    /// Retains IDs and `NotSent` versus `Unknown`/`ReadFailed` evidence in `SendFailure`.
    pub async fn send(&self, id: RequestId, request: Request) -> Result<Attempt, SendFailure> {
        let refused = attempt(self.generation, &id, &request, None, Outcome::NotSent);
        let deadline = Instant::now()
            .checked_add(self.timeout)
            .ok_or_else(|| SendFailure {
                attempt: refused.clone(),
                error: Error::Validation("FIX deadline range"),
            })?;
        let (tx, rx) = oneshot::channel();
        if self
            .tx
            .send(Command::Send {
                id,
                request,
                deadline,
                reply: tx,
            })
            .is_err()
        {
            return Err(SendFailure {
                attempt: refused,
                error: Error::Gap("FIX driver unavailable"),
            });
        }
        rx.await.map_err(|_| SendFailure {
            attempt: Attempt {
                outcome: Outcome::Unknown,
                ..refused
            },
            error: Error::Gap("FIX driver interrupted send attribution"),
        })?
    }
    /// Source generation for this immutable session handle.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }
    /// Request clean Logout and joined teardown. This does not cancel venue orders;
    /// callers must also join the task running `SessionDriver::run`.
    pub fn shutdown(&self) {
        self.stop.cancel();
    }
}
async fn connect_transport(config: &Config) -> Result<Stream, Error> {
    let host = config
        .endpoint
        .host_str()
        .ok_or(Error::Configuration("FIX host"))?
        .trim_matches(['[', ']']);
    let port = config
        .endpoint
        .port()
        .ok_or(Error::Configuration("FIX port"))?;
    let stream = TcpStream::connect((host, port))
        .await
        .map_err(|_| Error::Gap("FIX TCP connection failed"))?;
    stream
        .set_nodelay(true)
        .map_err(|_| Error::Gap("FIX TCP configuration failed"))?;
    if config.endpoint.scheme() == "tcp" {
        return Ok(Box::new(stream));
    }
    let name = rustls::pki_types::ServerName::try_from(host.to_owned())
        .map_err(|_| Error::Configuration("FIX TLS server name"))?;
    let connector = tokio_rustls::TlsConnector::from(Arc::new(tls_config()?));
    connector
        .connect(name, stream)
        .await
        .map(|s| Box::new(s) as Stream)
        .map_err(|_| Error::Gap("FIX TLS handshake failed"))
}
fn expiry(at: Instant, duration: Duration) -> Result<Instant, Error> {
    at.checked_add(duration)
        .ok_or(Error::Configuration("FIX timer range"))
}
fn attempt(
    generation: u64,
    id: &RequestId,
    request: &Request,
    sequence: Option<u32>,
    outcome: Outcome,
) -> Attempt {
    Attempt {
        generation,
        id: id.clone(),
        kind: request.kind,
        sequence,
        identities: request.identities(),
        outcome,
    }
}
impl SessionDriver {
    fn emit(&self, event: Event) -> Result<(), Error> {
        // Hold only the diagnostic lock during the synchronous unbounded send.
        let mut metrics = self
            .stats
            .lock()
            .map_err(|_| Error::Configuration("FIX queue diagnostics poisoned"))?;
        metrics.arrivals.push_back(Instant::now());
        metrics.accepted = metrics.accepted.saturating_add(1);
        self.events
            .send(event)
            .map_err(|_| Error::Gap("FIX event owner dropped"))
    }
    fn header(&self) -> Result<Header, Error> {
        Header::request(
            self.config.component.clone(),
            self.outgoing,
            Timestamp::from_micros(
                i64::try_from(self.config.clock.now_micros()?)
                    .map_err(|_| Error::Configuration("FIX clock range"))?,
            )?,
        )
    }
    fn encode(
        &self,
        kind: &str,
        fields: &Fields,
        header: &Header,
    ) -> Result<Zeroizing<Vec<u8>>, Error> {
        match self.config.encoding {
            Encoding::Ascii | Encoding::AsciiSbe => {
                request::encode(self.config.role, kind, fields, header, self.config.window)
            }
            Encoding::Sbe => super::binary::encode(
                kind,
                fields,
                header,
                &self.config.precision,
                self.config.window,
            ),
        }
    }
    async fn write(&mut self, bytes: &[u8], cancel: bool) -> Result<(), Error> {
        let limit = self.config.timeout.min(self.config.heartbeat);
        tokio::select! {biased;
            ()=self.stop.cancelled(),if cancel=>Err(Error::Gap("FIX write canceled; outcome uncertain")),
            result=tokio::time::timeout(limit,self.stream.write_all(bytes))=>{result.map_err(|_|Error::Gap("FIX write timeout; outcome uncertain"))?.map_err(|_|Error::Gap("FIX write failed; outcome uncertain"))?;self.last_sent=Instant::now();self.outgoing=self.outgoing.wrapping_add(1);Ok(())}
        }
    }
    async fn control(&mut self, kind: &str, fields: Fields) -> Result<(), Error> {
        let header = self.header()?;
        let bytes = self.encode(kind, &fields, &header)?;
        self.messages.admit()?;
        self.write(&bytes, false).await
    }
    async fn logon(&mut self) -> Result<(), Error> {
        let header = self.header()?;
        let payload = Zeroizing::new(format!(
            "A\x01{}\x01SPOT\x011\x01{}",
            self.config.component.as_str(),
            header.sending_time.wire()?
        ));
        let signature = Zeroizing::new(self.config.credentials.sign(&payload)?);
        let mut fields = Fields::new(self.config.role)
            .with("EncryptMethod", Value::Code("0".into()))?
            .with(
                "HeartBtInt",
                Value::Integer(
                    i64::try_from(self.config.heartbeat.as_secs())
                        .map_err(|_| Error::Configuration("FIX heartbeat range"))?,
                ),
            )?
            .with(
                "RawDataLength",
                Value::Unsigned(u64::try_from(signature.len()).map_err(|_| Error::Signing)?),
            )?
            .with(
                "RawData",
                Value::Text(SensitiveString::new(signature.as_str())),
            )?
            .with("ResetSeqNumFlag", Value::Boolean(true))?
            .with(
                "Username",
                Value::Text(SensitiveString::new(self.config.credentials.api_key())),
            )?
            .with("MessageHandling", Value::Code("2".into()))?;
        if self.config.role == Role::DropCopy {
            fields = fields.with("DropCopyFlag", Value::Boolean(true))?;
        }
        if self.config.encoding == Encoding::AsciiSbe {
            fields = fields
                .with("SBESchemaID", Value::Integer(1))?
                .with("SBESchemaVersion", Value::Integer(1))?;
        }
        let bytes = self.encode("A", &fields, &header)?;
        self.messages.admit()?;
        self.write(&bytes, true).await
    }
    fn next_message(&mut self) -> Result<Option<Message>, Error> {
        let length = if self.config.encoding == Encoding::Ascii {
            frame_length(&self.input)?
        } else if self.input.len() < 6 {
            None
        } else {
            let bytes = self
                .input
                .get(..4)
                .and_then(|b| b.try_into().ok())
                .ok_or(Error::Gap("FIX SBE length prefix"))?;
            let n = usize::try_from(u32::from_le_bytes(bytes))
                .map_err(|_| Error::Gap("FIX SBE length range"))?;
            if n < 26 {
                return Err(Error::Gap("FIX SBE frame length"));
            }
            Some(n)
        };
        let Some(length) = length else {
            return Ok(None);
        };
        if length > self.input.len() {
            return Ok(None);
        }
        let message = if self.config.encoding == Encoding::Ascii {
            super::decode(self.config.role, &self.input[..length])?
        } else {
            super::decode_sbe(self.config.role, &self.input[..length])?
        };
        self.input.drain(..length);
        Ok(Some(message))
    }
    fn candidates(&self, message: &Message) -> Vec<Attempt> {
        let mut returned = BTreeSet::new();
        for field in message.body.fields() {
            if field.name.as_deref().is_some_and(|n| {
                matches!(
                    n,
                    "ClOrdID"
                        | "OrigClOrdID"
                        | "ClListID"
                        | "OrigClListID"
                        | "CancelClOrdID"
                        | "MDReqID"
                        | "ReqID"
                        | "InstrumentReqID"
                )
            }) && let Ok(value) = wire_value(&field.value)
            {
                returned.insert(value);
            }
        }
        let seq = match message.field("RefSeqNum") {
            Some(Value::Unsigned(v)) => u32::try_from(*v).ok(),
            _ => None,
        };
        self.attempts
            .values()
            .filter(|a| {
                seq.is_some_and(|s| a.sequence == Some(s))
                    || a.identities.values().any(|id| returned.contains(id))
            })
            .cloned()
            .collect()
    }
    async fn incoming(&mut self, message: Message) -> Result<bool, Error> {
        if message.header.sequence != self.incoming {
            return Err(Error::Gap("FIX inbound sequence gap"));
        }
        if self.config.encoding == Encoding::Ascii
            && (message.header.sender.as_ref().map(CompId::as_str) != Some("SPOT")
                || message.header.target.as_ref() != Some(&self.config.component))
        {
            return Err(Error::Gap("FIX session component mismatch"));
        }
        self.incoming = self.incoming.wrapping_add(1);
        self.last_received = Instant::now();
        if !self.authenticated {
            if message.kind.as_str() != "A"
                || message.field("UUID").is_none()
                || message.field("EncryptMethod") != Some(&Value::Code("0".into()))
                || message.field("HeartBtInt")
                    != Some(&Value::Integer(
                        i64::try_from(self.config.heartbeat.as_secs())
                            .map_err(|_| Error::Configuration("FIX heartbeat range"))?,
                    ))
            {
                self.emit(Event::Message {
                    generation: self.generation,
                    component: self.config.component.clone(),
                    candidates: vec![],
                    message,
                })?;
                return Err(Error::Gap("FIX Logon not acknowledged"));
            }
            self.authenticated = true;
            self.emit(Event::Established {
                generation: self.generation,
                component: self.config.component.clone(),
                acknowledgment: message,
            })?;
            return Ok(false);
        }
        let kind = message.kind.as_str().to_owned();
        let test = message.field("TestReqID").map(wire_value).transpose()?;
        let rejected_subscription = if message.kind.as_str() == "Y" {
            message.field("MDReqID").map(wire_value).transpose()?
        } else {
            None
        };
        let limits = message.field("NoLimitIndicators").cloned();
        let candidates = self.candidates(&message);
        self.emit(Event::Message {
            generation: self.generation,
            component: self.config.component.clone(),
            message,
            candidates,
        })?;
        if let Some(id) = rejected_subscription
            && self
                .attempts
                .values()
                .filter(|a| a.identities.values().any(|v| v == &id))
                .count()
                == 1
        {
            self.subscriptions.remove(&id);
        }
        if kind == "XLR" {
            self.observe_limits(limits.as_ref())?;
        }
        match kind.as_str() {
            "A" => return Err(Error::Gap("FIX duplicate Logon")),
            "1" => {
                let test = test.ok_or(Error::Gap("FIX TestRequest identity missing"))?;
                self.control(
                    "0",
                    Fields::new(self.config.role)
                        .with("TestReqID", Value::Id(super::WireId::new(test)?))?,
                )
                .await?;
            }
            "0" if self
                .probe
                .as_ref()
                .is_some_and(|(id, _)| test.as_ref() == Some(id)) =>
            {
                self.probe = None;
            }
            "5" => {
                self.control("5", Fields::new(self.config.role)).await?;
                self.clean = true;
                return Ok(true);
            }
            _ => (),
        }
        Ok(false)
    }
    fn observe_limits(&mut self, value: Option<&Value>) -> Result<(), Error> {
        let Some(Value::Group(entries)) = value else {
            return Err(Error::Gap("FIX LimitResponse indicators missing"));
        };
        for entry in entries {
            let number = |name| match entry.get(name) {
                Some(Value::Integer(v)) => {
                    u64::try_from(*v).map_err(|_| Error::Gap("FIX negative limit evidence"))
                }
                Some(Value::Unsigned(v)) => Ok(*v),
                _ => Err(Error::Gap("FIX required limit evidence missing")),
            };
            let units = match entry.get("LimitResetIntervalResolution") {
                Some(Value::Code(v)) => match v.as_str() {
                    "s" => 1000,
                    "m" => 60_000,
                    "h" => 3_600_000,
                    "d" => 86_400_000,
                    _ => return Err(Error::Gap("FIX unknown limit interval unit")),
                },
                _ => return Err(Error::Gap("FIX limit interval unit missing")),
            };
            let window = number("LimitResetInterval")?
                .checked_mul(units)
                .ok_or(Error::Gap("FIX limit interval overflow"))?;
            let count = number("LimitCount")?;
            let maximum = number("LimitMax")?;
            match entry.get("LimitType") {
                Some(Value::Code(kind)) if kind == "1" => {
                    self.config.budgets.spot.observe_order_window(
                        window,
                        count,
                        maximum,
                        self.config.clock.now_millis()?,
                    )?;
                }
                Some(Value::Code(kind)) if kind == "2" => {
                    self.messages
                        .observe(Duration::from_millis(window), count, maximum)?;
                }
                _ => return Err(Error::Gap("FIX unknown limit type")),
            }
        }
        Ok(())
    }
    async fn command(&mut self, command: Command) -> Result<(), Error> {
        let Command::Send {
            id,
            request,
            deadline,
            reply,
        } = command;
        let mut evidence = attempt(self.generation, &id, &request, None, Outcome::NotSent);
        let prepared = self.prepare(&id, &request, deadline, reply.is_closed());
        let (header, bytes) = match prepared {
            Ok(value) => value,
            Err(error) => {
                let _ = reply.send(Err(SendFailure {
                    attempt: evidence,
                    error,
                }));
                return Ok(());
            }
        };
        if reply.is_closed() || Instant::now() >= deadline || self.stop.is_cancelled() {
            let _ = reply.send(Err(SendFailure {
                attempt: evidence,
                error: Error::Gap("FIX queued request canceled or expired"),
            }));
            return Ok(());
        }
        evidence.sequence = Some(header.sequence);
        evidence.outcome = if request.kind.mutation() {
            Outcome::Unknown
        } else {
            Outcome::ReadFailed
        };
        self.reserve_subscription(&request)?;
        self.used.insert(id.as_str().into());
        self.attempts.insert(id.as_str().into(), evidence.clone());
        match self.write(&bytes, true).await {
            Ok(()) => {
                let _ = reply.send(Ok(evidence));
                Ok(())
            }
            Err(error) => {
                let _ = reply.send(Err(SendFailure {
                    attempt: evidence,
                    error,
                }));
                Err(Error::Gap("FIX generation lost during a write"))
            }
        }
    }
    fn prepare(
        &mut self,
        id: &RequestId,
        request: &Request,
        deadline: Instant,
        canceled: bool,
    ) -> Result<(Header, Zeroizing<Vec<u8>>), Error> {
        if !self.authenticated {
            return Err(Error::Validation("FIX Logon acknowledgment required"));
        }
        if canceled || self.stop.is_cancelled() || Instant::now() >= deadline {
            return Err(Error::Gap("FIX queued request canceled or expired"));
        }
        if self.used.contains(id.as_str()) {
            return Err(Error::Validation("FIX local request identity reused"));
        }
        if request.fields.role != self.config.role {
            return Err(Error::Validation("FIX request/session role mismatch"));
        }
        self.subscription_admission(request)?;
        let header = self.header()?;
        let bytes = self.encode(request.kind.code(), &request.fields, &header)?;
        let count = request.order_count()?;
        self.messages.admit()?;
        self.config.budgets.spot.admit(
            Cost {
                orders10: count,
                orders60: count,
                orders_day: count,
                ..Cost::default()
            },
            self.config.clock.now_millis()?,
        )?;
        Ok((header, bytes))
    }
    fn subscription_admission(&self, request: &Request) -> Result<(), Error> {
        if request.kind != super::RequestKind::MarketData {
            return Ok(());
        }
        let id = request
            .identities()
            .get("MDReqID")
            .cloned()
            .ok_or(Error::Validation("FIX market identity"))?;
        if request.fields.get("SubscriptionRequestType") == Some(&Value::Code("2".into())) {
            return Ok(());
        }
        if self.subscriptions.contains_key(&id) {
            return Err(Error::Validation("FIX market subscription identity reused"));
        }
        let Some(Value::Group(symbols)) = request.fields.get("NoRelatedSym") else {
            return Err(Error::Validation("FIX subscription symbols"));
        };
        let types = request
            .fields
            .get("NoMDEntryTypes")
            .map(wire_value)
            .transpose()?
            .unwrap_or_default();
        let depth = request
            .fields
            .get("MarketDepth")
            .map(wire_value)
            .transpose()?
            .unwrap_or_default();
        let streams = symbols
            .iter()
            .map(|s| {
                s.get("Symbol")
                    .map(wire_value)
                    .transpose()?
                    .map(|symbol| format!("{symbol}:{types}:{depth}"))
                    .ok_or(Error::Validation("FIX subscription symbol"))
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if self
            .subscriptions
            .values()
            .map(BTreeSet::len)
            .sum::<usize>()
            .checked_add(streams.len())
            .is_none_or(|n| n > 1000)
        {
            return Err(Error::Validation("FIX documented 1000 stream limit"));
        }
        // Unsubscribe has no documented acknowledgment contract: retain uncertain slots
        // until a definitive subscription rejection or generation retirement (#11).
        Ok(())
    }
    fn reserve_subscription(&mut self, request: &Request) -> Result<(), Error> {
        if request.kind != super::RequestKind::MarketData
            || request.fields.get("SubscriptionRequestType") != Some(&Value::Code("1".into()))
        {
            return Ok(());
        }
        let id = request
            .identities()
            .get("MDReqID")
            .cloned()
            .ok_or(Error::Validation("FIX market identity"))?;
        let Some(Value::Group(symbols)) = request.fields.get("NoRelatedSym") else {
            return Err(Error::Validation("FIX market symbols"));
        };
        let streams = symbols
            .iter()
            .map(|s| {
                s.get("Symbol")
                    .map(wire_value)
                    .transpose()?
                    .ok_or(Error::Validation("FIX market symbol"))
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        self.subscriptions.insert(id, streams);
        Ok(())
    }
    fn timer(&self) -> Result<Instant, Error> {
        if !self.authenticated {
            return expiry(self.last_sent, self.config.timeout);
        }
        let probe = match &self.probe {
            Some((_, deadline)) => *deadline,
            None => expiry(self.last_received, self.config.heartbeat)?,
        };
        Ok(expiry(self.last_sent, self.config.heartbeat)?.min(probe))
    }
    async fn tick(&mut self) -> Result<(), Error> {
        if !self.authenticated {
            return Err(Error::Gap("FIX Logon acknowledgment timeout"));
        }
        let now = Instant::now();
        if self
            .probe
            .as_ref()
            .is_some_and(|(_, deadline)| now >= *deadline)
        {
            return Err(Error::Gap("FIX heartbeat probe not answered"));
        }
        if self.probe.is_none() && now >= expiry(self.last_received, self.config.heartbeat)? {
            let id = format!("probe-{}-{}", self.generation, self.outgoing);
            self.control(
                "1",
                Fields::new(self.config.role)
                    .with("TestReqID", Value::Id(super::WireId::new(id.clone())?))?,
            )
            .await?;
            self.probe = Some((id, expiry(Instant::now(), self.config.heartbeat)?));
        } else if now >= expiry(self.last_sent, self.config.heartbeat)? {
            self.control("0", Fields::new(self.config.role)).await?;
        }
        Ok(())
    }
    async fn drive(&mut self) -> Result<(), Error> {
        self.logon().await?;
        let mut chunk = Zeroizing::new([0_u8; 8192]);
        loop {
            if let Some(message) = self.next_message()? {
                if self.incoming(message).await? {
                    return Ok(());
                }
                continue;
            }
            if self.stop.is_cancelled() {
                self.control("5", Fields::new(self.config.role)).await?;
                self.clean = true;
                return Ok(());
            }
            let timer = self.timer()?;
            tokio::select! {
                ()=self.stop.cancelled()=>{},
                ()=tokio::time::sleep_until(timer)=>self.tick().await?,
                result=self.stream.read(chunk.as_mut_slice())=>{let n=result.map_err(|_|Error::Gap("FIX TCP read failed"))?;if n==0{return Err(Error::Gap(if self.input.is_empty(){"FIX transport closed"}else{"FIX transport closed with partial frame"}));}self.input.extend_from_slice(&chunk[..n]);},
                command=self.commands.recv()=>{if let Some(command)=command{self.command(command).await?;}else{self.stop.cancel();}}
            }
        }
    }
    /// Own this generation until shutdown/transport loss, emitting its accepted prefix
    /// before Gap/Retired. Every queued request gets a typed refusal on retirement.
    /// Run in the caller's runtime and join its task; no background reconnection occurs.
    ///
    /// # Errors
    /// Returns lifecycle/diagnostic delivery errors. Venue/transport failures are
    /// delivered through `Event::Gap`, with IDs retained for late-answer reconciliation.
    pub async fn run(mut self) -> Result<(), Error> {
        let failure = self.drive().await.err();
        self.commands.close();
        while let Some(Command::Send {
            id, request, reply, ..
        }) = self.commands.recv().await
        {
            let _ = reply.send(Err(SendFailure {
                attempt: attempt(self.generation, &id, &request, None, Outcome::NotSent),
                error: Error::Gap("FIX generation retired before send"),
            }));
        }
        let shutdown = tokio::time::timeout(self.config.timeout, self.stream.shutdown()).await;
        self.lease
            .retire(self.clean && matches!(shutdown, Ok(Ok(()))))?;
        if let Some(error) = failure {
            self.emit(Event::Gap {
                generation: self.generation,
                error,
                attempts: self.attempts.values().cloned().collect(),
            })?;
        }
        self.emit(Event::Retired(self.generation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BudgetLimits, Budgets, Credentials, Signer};
    struct Synthetic;
    impl Signer for Synthetic {
        fn sign(&self, _: &[u8]) -> Result<String, Error> {
            Ok("synthetic-signature".into())
        }
    }
    fn setup() -> (Session, Events, SessionDriver, tokio::io::DuplexStream) {
        let config = Config::new(
            Role::OrderEntry,
            CompId::new("CLIENT").unwrap(),
            Credentials::external_ed25519("synthetic", Arc::new(Synthetic)).unwrap(),
            super::super::AccountBudgets::new(Budgets::new(BudgetLimits::spot()).unwrap()),
        )
        .unwrap()
        .heartbeat(5)
        .unwrap();
        let lease = config
            .budgets
            .connect(config.role, &config.component, config.heartbeat)
            .unwrap();
        let (client, peer) = tokio::io::duplex(8192);
        let (session, events, driver) = Session::assemble(config, Box::new(client), lease, 1);
        (session, events, driver, peer)
    }
    fn frame(body: &str) -> Vec<u8> {
        let body = body.replace('|', "\x01");
        let mut bytes = format!("8=FIX.4.4\x019={}\x01{body}", body.len()).into_bytes();
        let sum = bytes.iter().fold(0_u8, |a, b| a.wrapping_add(*b));
        bytes.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
        bytes
    }
    async fn read(peer: &mut tokio::io::DuplexStream) -> Message {
        let mut bytes = vec![];
        loop {
            bytes.push(peer.read_u8().await.unwrap());
            if bytes.len() >= 7
                && bytes[bytes.len() - 7..].starts_with(b"10=")
                && bytes.last() == Some(&1)
            {
                break;
            }
        }
        super::super::decode(Role::OrderEntry, &bytes).unwrap()
    }
    async fn acknowledge(peer: &mut tokio::io::DuplexStream) {
        assert_eq!(read(peer).await.kind.as_str(), "A");
        peer.write_all(&frame("35=A|34=1|49=SPOT|56=CLIENT|52=20260927-01:02:03.000001|98=0|108=5|25037=synthetic-server|")).await.unwrap();
    }
    #[tokio::test(start_paused = true)]
    async fn silence_requires_a_probe_and_its_exact_echo_before_failure() {
        let (_session, mut events, driver, mut peer) = setup();
        let owner = tokio::spawn(driver.run());
        acknowledge(&mut peer).await;
        assert!(matches!(
            events.recv().await,
            Some(Event::Established { .. })
        ));
        tokio::time::advance(Duration::from_secs(5)).await;
        let probe = read(&mut peer).await;
        assert_eq!(probe.kind.as_str(), "1");
        let id = probe.field("TestReqID").map(wire_value).unwrap().unwrap();
        peer.write_all(&frame(&format!(
            "35=0|34=2|49=SPOT|56=CLIENT|52=20260927-01:02:03.000002|112={id}|"
        )))
        .await
        .unwrap();
        assert!(
            matches!(events.recv().await,Some(Event::Message{message,..}) if message.kind.as_str()=="0")
        );
        tokio::time::advance(Duration::from_secs(5)).await;
        assert_eq!(read(&mut peer).await.kind.as_str(), "1");
        tokio::time::advance(Duration::from_secs(5)).await;
        assert!(matches!(
            events.recv().await,
            Some(Event::Gap {
                error: Error::Gap("FIX heartbeat probe not answered"),
                ..
            })
        ));
        assert!(matches!(events.recv().await, Some(Event::Retired(_))));
        owner.await.unwrap().unwrap();
    }
    #[tokio::test(start_paused = true)]
    async fn expired_and_canceled_queued_requests_never_reach_the_wire() {
        let (session, mut events, mut driver, mut peer) = setup();
        let request =
            super::super::Request::builder(Role::OrderEntry, super::super::RequestKind::Limits)
                .field(
                    "ReqID",
                    Value::Id(super::super::WireId::new("expired").unwrap()),
                )
                .unwrap()
                .build()
                .unwrap();
        let command_id = RequestId::new("expired-local").unwrap();
        let copy = session.clone();
        let queued = tokio::spawn(async move { copy.send(command_id, request).await });
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(11)).await;
        // Authenticate first so the fixture specifically exercises deadline refusal.
        driver.logon().await.unwrap();
        acknowledge(&mut peer).await;
        let mut chunk = [0_u8; 1024];
        let n = driver.stream.read(&mut chunk).await.unwrap();
        driver.input.extend_from_slice(&chunk[..n]);
        let message = driver.next_message().unwrap().unwrap();
        driver.incoming(message).await.unwrap();
        assert!(matches!(
            events.recv().await,
            Some(Event::Established { .. })
        ));
        let command = driver.commands.recv().await.unwrap();
        driver.command(command).await.unwrap();
        let error = queued.await.unwrap().unwrap_err();
        assert_eq!(error.attempt.outcome, Outcome::NotSent);
        assert!(error.attempt.sequence.is_none());
        let (tx, rx) = oneshot::channel();
        drop(rx);
        let request =
            super::super::Request::builder(Role::OrderEntry, super::super::RequestKind::Limits)
                .field(
                    "ReqID",
                    Value::Id(super::super::WireId::new("canceled").unwrap()),
                )
                .unwrap()
                .build()
                .unwrap();
        driver
            .command(Command::Send {
                id: RequestId::new("canceled-local").unwrap(),
                request,
                deadline: Instant::now() + Duration::from_secs(10),
                reply: tx,
            })
            .await
            .unwrap();
        assert!(driver.attempts.is_empty());
        session.shutdown();
        driver
            .control("5", Fields::new(Role::OrderEntry))
            .await
            .unwrap();
        driver.stream.shutdown().await.unwrap();
        driver.lease.retire(true).unwrap();
        assert_eq!(read(&mut peer).await.kind.as_str(), "5");
    }
}
