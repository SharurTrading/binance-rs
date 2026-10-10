// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{
    Budgets, Clock, Cost, Credentials, Error, Operation, Outcome, RateEvidence, RequestId,
    ResponseMeta, Security, StreamControl,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
    time::Instant,
};
use tokio_util::sync::CancellationToken;
use tokio_websockets::{
    ClientBuilder, Connector, Limits, MaybeTlsStream, Message, WebSocketStream,
};

static GENERATION: AtomicU64 = AtomicU64::new(1);
pub(crate) type BinaryDecoder = fn(&[u8]) -> Result<Value, Error>;

/// Observable ingress lag; no capacity-driven dropping or disconnection.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct QueueStats {
    /// Accepted events waiting for consumer acknowledgment.
    pub depth: usize,
    /// Age of the oldest waiting event.
    pub oldest_age: Duration,
    /// Total events accepted in this generation.
    pub accepted: u64,
    /// Total events acknowledged by the consumer.
    pub drained: u64,
}

// Producer owns accepted/back insertion; the sole consumer owns drained/front removal.
// This diagnostic queue is independent of the driver's single-writer lifecycle state.
#[derive(Default)]
struct QueueMetrics {
    arrivals: VecDeque<Instant>,
    accepted: u64,
    drained: u64,
}
impl QueueMetrics {
    fn snapshot(&self) -> QueueStats {
        QueueStats {
            depth: self.arrivals.len(),
            oldest_age: self
                .arrivals
                .front()
                .map_or(Duration::ZERO, Instant::elapsed),
            accepted: self.accepted,
            drained: self.drained,
        }
    }
}

pub(crate) enum SocketEvent {
    Established(u64),
    Data {
        generation: u64,
        value: Value,
    },
    Late {
        generation: u64,
        id: RequestId,
        // Boxed so this rare-attribution variant does not size every queued
        // event after the operation facts grew to carry pinned rate evidence.
        op: Box<Operation>,
        value: Value,
        meta: ResponseMeta,
    },
    Gap {
        generation: u64,
        error: Error,
    },
    /// A stream control answer that arrived after its caller stopped waiting.
    ControlLate {
        generation: u64,
        control: StreamControl,
        streams: Vec<String>,
        result: Result<Option<Vec<String>>, Error>,
    },
    Retired(u64),
}

type ControlReply = oneshot::Sender<Result<Option<Vec<String>>, Error>>;

enum Command {
    Call {
        op: Box<Operation>,
        params: BTreeMap<String, Value>,
        cost: Box<Cost>,
        id: RequestId,
        deadline: Instant,
        reply: oneshot::Sender<Result<(Value, ResponseMeta), Error>>,
    },
    Control {
        control: StreamControl,
        streams: Vec<String>,
        deadline: Instant,
        reply: ControlReply,
    },
    Close(oneshot::Sender<()>),
}

#[derive(Clone)]
pub(crate) struct Socket {
    tx: mpsc::UnboundedSender<Command>,
    stop: CancellationToken,
    stats: Arc<Mutex<QueueMetrics>>,
    generation: u64,
}

pub(crate) struct SocketEvents {
    // The event consumer also owns connection lifetime when request handles are dropped.
    _commands: mpsc::UnboundedSender<Command>,
    rx: mpsc::UnboundedReceiver<SocketEvent>,
    stats: Arc<Mutex<QueueMetrics>>,
    stop: CancellationToken,
}
impl SocketEvents {
    pub async fn recv(&mut self) -> Option<SocketEvent> {
        let value = self.rx.recv().await;
        // A diagnostic synchronization failure remains visible through queue_stats;
        // it must not replace or discard an accepted venue event.
        if value.is_some()
            && let Ok(mut metrics) = self.stats.lock()
        {
            metrics.arrivals.pop_front();
            metrics.drained = metrics.drained.saturating_add(1);
        }
        value
    }
    pub fn stop(&self) {
        self.stop.cancel();
    }
}

struct Pending {
    admitted_at: u64,
    weight: u64,
    client_order_ids: BTreeMap<String, String>,
    op: Operation,
    id: RequestId,
    deadline: Instant,
    reply: Option<oneshot::Sender<Result<(Value, ResponseMeta), Error>>>,
}

struct PendingControl {
    control: StreamControl,
    streams: Vec<String>,
    deadline: Instant,
    reply: Option<ControlReply>,
}

fn control_lost(control: StreamControl) -> Error {
    Error::Transport {
        client_order_ids: BTreeMap::new(),
        operation: control.method(),
        outcome: control.lost_outcome(),
        meta: None,
    }
}

/// Which duty charges the connection's sliding incoming-message window.
#[derive(Clone, Copy)]
enum IncomingCharge {
    /// The automatic answer to a venue ping; may claim the reserved last slot.
    Pong,
    /// An outgoing stream control; refused before it can take that slot.
    Control,
}

pub(crate) struct SocketDriver {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    commands: mpsc::UnboundedReceiver<Command>,
    events: mpsc::UnboundedSender<SocketEvent>,
    credentials: Option<Credentials>,
    clock: Arc<dyn Clock>,
    budgets: Budgets,
    timeout: Duration,
    stop: CancellationToken,
    stats: Arc<Mutex<QueueMetrics>>,
    generation: u64,
    lifetime: Instant,
    incoming_limit: usize,
    // Venue-incoming messages (pongs and control messages) sent in the last second.
    incoming: VecDeque<Instant>,
    next_control: u64,
    controls: BTreeMap<u64, PendingControl>,
    time_unit: super::TimeUnit,
    binary_decoder: Option<BinaryDecoder>,
    binary_timestamps: bool,
    require_binary_success: bool,
    pending: BTreeMap<String, Pending>,
    // IDs are evidence until generation retirement, including completed and timed-out calls.
    used_ids: BTreeSet<String>,
}

pub(crate) fn tls_config() -> Result<rustls::ClientConfig, Error> {
    let roots = webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
        .collect::<rustls::RootCertStore>();
    Ok(rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::aws_lc_rs::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .map_err(|_| Error::Configuration("TLS versions"))?
    .with_root_certificates(roots)
    .with_no_client_auth())
}

/// Product-sourced connection/control admission, independent of wire models.
pub(crate) struct SocketPolicy {
    pub handshake: Cost,
    /// Venue-incoming message ceiling per sliding second, charged by every pong
    /// and every stream control message. Every call site supplies this from its
    /// route's documented connection duty cycle; no default arm derives it, so an
    /// uncited limit cannot be hidden here. The ceiling's last slot is reserved
    /// for a pong, so a stream control is refused one charge early.
    pub incoming_limit: usize,
    pub time_unit: super::TimeUnit,
    pub binary_decoder: Option<BinaryDecoder>,
    pub api_key_header: bool,
}

impl Socket {
    pub async fn connect_with_policy(
        url: url::Url,
        credentials: Option<Credentials>,
        clock: Arc<dyn Clock>,
        budgets: Budgets,
        timeout: Duration,
        policy: SocketPolicy,
    ) -> Result<(Self, SocketEvents, SocketDriver), Error> {
        let generation = GENERATION
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| Error::Configuration("socket generation overflow"))?;
        let connector =
            Connector::Rustls(tokio_rustls::TlsConnector::from(Arc::new(tls_config()?)));
        // No library frame/queue ceiling: real socket loss becomes an explicit generation gap.
        let mut builder = ClientBuilder::new()
            .uri(url.as_str())
            .map_err(|_| Error::Configuration("WebSocket URI"))?
            .connector(&connector)
            .limits(Limits::unlimited());
        if policy.api_key_header {
            let credentials = credentials.as_ref().ok_or(Error::CredentialsRequired)?;
            if !credentials.is_ed25519() {
                return Err(Error::Validation("SBE streams require Ed25519 credentials"));
            }
            builder = builder
                .add_header(
                    "X-MBX-APIKEY"
                        .parse()
                        .map_err(|_| Error::Configuration("API key header name"))?,
                    credentials.header()?,
                )
                .map_err(|_| Error::Configuration("API key handshake header"))?;
        }
        budgets.admit(policy.handshake, clock.now_millis()?)?;
        let (ws, _) = tokio::time::timeout(timeout, builder.connect())
            .await
            .map_err(|_| Error::Transport {
                client_order_ids: BTreeMap::new(),
                operation: "connect",
                outcome: Outcome::NotSent,
                meta: None,
            })?
            .map_err(|_| Error::Transport {
                client_order_ids: BTreeMap::new(),
                operation: "connect",
                outcome: Outcome::NotSent,
                meta: None,
            })?;
        let (tx, commands) = mpsc::unbounded_channel();
        let (events, rx) = mpsc::unbounded_channel();
        let stop = CancellationToken::new();
        let stats = Arc::new(Mutex::new(QueueMetrics::default()));
        let socket = Self {
            tx: tx.clone(),
            stop: stop.clone(),
            stats: stats.clone(),
            generation,
        };
        let receiver = SocketEvents {
            _commands: tx,
            rx,
            stats: stats.clone(),
            stop: stop.clone(),
        };
        let driver = SocketDriver {
            ws,
            commands,
            events,
            credentials,
            clock,
            budgets,
            timeout,
            stop,
            stats,
            generation,
            lifetime: Instant::now() + Duration::from_hours(24),
            incoming_limit: policy.incoming_limit,
            incoming: VecDeque::new(),
            next_control: 1,
            controls: BTreeMap::new(),
            time_unit: policy.time_unit,
            binary_decoder: policy.binary_decoder,
            binary_timestamps: policy.binary_decoder.is_some(),
            require_binary_success: policy.binary_decoder.is_some() && !policy.api_key_header,
            pending: BTreeMap::new(),
            used_ids: BTreeSet::new(),
        };
        Ok((socket, receiver, driver))
    }
    pub async fn call(
        &self,
        op: Operation,
        params: BTreeMap<String, Value>,
        cost: Cost,
        id: RequestId,
        deadline: Instant,
    ) -> Result<(Value, ResponseMeta), Error> {
        let client_order_ids = super::request::order_ids(&params);
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Command::Call {
                op: Box::new(op),
                params,
                cost: Box::new(cost),
                id,
                deadline,
                reply,
            })
            .map_err(|_| Error::Transport {
                client_order_ids: client_order_ids.clone(),
                operation: op.name,
                outcome: Outcome::NotSent,
                meta: None,
            })?;
        rx.await.map_err(|_| Error::Transport {
            client_order_ids: client_order_ids.clone(),
            operation: op.name,
            outcome: if op.mutation {
                Outcome::Unknown
            } else {
                Outcome::ReadFailed
            },
            meta: None,
        })?
    }
    /// Send one stream control message and wait for its correlated answer.
    pub async fn control(
        &self,
        control: StreamControl,
        streams: Vec<String>,
        deadline: Instant,
    ) -> Result<Option<Vec<String>>, Error> {
        let (reply, rx) = oneshot::channel();
        self.tx
            .send(Command::Control {
                control,
                streams,
                deadline,
                reply,
            })
            .map_err(|_| Error::Transport {
                client_order_ids: BTreeMap::new(),
                operation: control.method(),
                outcome: Outcome::NotSent,
                meta: None,
            })?;
        rx.await.map_err(|_| control_lost(control))?
    }
    pub async fn close(&self) -> Result<(), Error> {
        let (tx, rx) = oneshot::channel();
        // Cancellation bypasses unlimited ordinary command backlog.
        self.stop.cancel();
        self.tx
            .send(Command::Close(tx))
            .map_err(|_| Error::Closed)?;
        rx.await.map_err(|_| Error::Closed)
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn stats(&self) -> Result<QueueStats, Error> {
        self.stats
            .lock()
            .map(|s| s.snapshot())
            .map_err(|_| Error::Configuration("metrics poisoned"))
    }
}

impl SocketDriver {
    fn binary_response(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let decode = self
            .binary_decoder
            .ok_or(Error::Gap("unexpected binary event on JSON connection"))?;
        self.response(decode(bytes)?)
    }
    fn emit(&mut self, event: SocketEvent) -> Result<(), Error> {
        let mut metrics = self
            .stats
            .lock()
            .map_err(|_| Error::Configuration("metrics poisoned"))?;
        let accepted = metrics
            .accepted
            .checked_add(1)
            .ok_or(Error::Configuration("metrics overflow"))?;
        self.events.send(event).map_err(|_| Error::Closed)?;
        metrics.arrivals.push_back(Instant::now());
        metrics.accepted = accepted;
        Ok(())
    }
    async fn command(&mut self, command: Command) -> Result<(), Error> {
        match command {
            Command::Close(reply) => {
                let _ = reply.send(());
                Ok(())
            }
            Command::Control {
                control,
                streams,
                deadline,
                reply,
            } => self.send_control(control, streams, deadline, reply).await,
            Command::Call {
                op,
                mut params,
                cost,
                id,
                deadline,
                reply,
            } => {
                if !self.used_ids.insert(id.as_str().to_owned()) {
                    let _ = reply.send(Err(Error::DuplicateRequestId));
                    return Ok(());
                }
                if deadline <= Instant::now() || reply.is_closed() {
                    let _ = reply.send(Err(Error::Expired(op.name)));
                    return Ok(());
                }
                let client_order_ids = super::request::order_ids(&params);
                let prepare = (|| {
                    let now = self.clock.now_millis()?;
                    cost.validate_authority(op.name, now)?;
                    (op.validate_time)(&params, now)?;
                    if op.security != Security::Public {
                        let credentials = self
                            .credentials
                            .as_ref()
                            .ok_or(Error::CredentialsRequired)?;
                        if op.name == "sessionLogon" && !credentials.is_ed25519() {
                            return Err(Error::Validation("session.logon requires Ed25519"));
                        }
                        params.insert("apiKey".into(), credentials.api_key().into());
                        if op.security == Security::Signed {
                            params.insert(
                                "timestamp".into(),
                                self.time_unit.timestamp(self.clock.as_ref())?.into(),
                            );
                            // WS signing sorts params; unlike HTTP the payload is NOT percent encoded.
                            let payload = params
                                .iter()
                                .map(|(k, v)| Ok(format!("{k}={}", super::request::text(v)?)))
                                .collect::<Result<Vec<_>, Error>>()?
                                .join("&");
                            params.insert("signature".into(), credentials.sign(&payload)?.into());
                        }
                    }
                    let body=serde_json::to_string(&serde_json::json!({"id":id.as_str(),"method":op.path.trim_start_matches('/'),"params":params}))
                        .map_err(|_|Error::Validation("WebSocket encoding"))?;
                    self.budgets.admit(*cost, now)?;
                    let authority_time = self.clock.now_millis()?;
                    (op.validate_time)(&params, authority_time)?;
                    cost.validate_authority(op.name, authority_time)?;
                    if deadline <= Instant::now() {
                        return Err(Error::Expired(op.name));
                    }
                    Ok((body, now))
                })();
                let (body, admitted_at) = match prepare {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        let _ = reply.send(Err(error));
                        return Ok(());
                    }
                };
                self.pending.insert(
                    id.as_str().to_owned(),
                    Pending {
                        admitted_at,
                        weight: cost.ws_weight,
                        client_order_ids,
                        op: *op,
                        id,
                        deadline,
                        reply: Some(reply),
                    },
                );
                let outcome = tokio::select! {
                    ()=self.stop.cancelled()=>Err(Error::Closed),
                    value=tokio::time::timeout_at(deadline.min(Instant::now().checked_add(self.timeout).ok_or(Error::Configuration("socket timeout range"))?),self.ws.send(Message::text(body)))=>{
                        value.map_err(|_|Error::Gap("WebSocket send timeout"))?.map_err(|_|Error::Gap("WebSocket send failure"))
                    }
                };
                outcome
            }
        }
    }
    /// Charge one venue-incoming message to the connection's sliding second, or
    /// return how long until the oldest charge leaves the window. The window's
    /// last slot stays reserved for a pong, so a stream control is refused one
    /// charge early while a pong may still claim that slot.
    fn charge_incoming(&mut self, now: Instant, charge: IncomingCharge) -> Result<(), Duration> {
        let window = Duration::from_secs(1);
        while self
            .incoming
            .front()
            .is_some_and(|sent| now.duration_since(*sent) >= window)
        {
            self.incoming.pop_front();
        }
        let ceiling = match charge {
            IncomingCharge::Pong => self.incoming_limit,
            IncomingCharge::Control => self.incoming_limit.saturating_sub(1),
        };
        if self.incoming.len() >= ceiling {
            let oldest = self.incoming.front().copied().unwrap_or(now);
            return Err(window.saturating_sub(now.duration_since(oldest)));
        }
        self.incoming.push_back(now);
        Ok(())
    }
    async fn send_control(
        &mut self,
        control: StreamControl,
        streams: Vec<String>,
        deadline: Instant,
        reply: ControlReply,
    ) -> Result<(), Error> {
        if deadline <= Instant::now() || reply.is_closed() {
            let _ = reply.send(Err(Error::Expired(control.method())));
            return Ok(());
        }
        // Spot accepts a 64-bit signed integer id and the futures venues an
        // unsigned one, so ids stay within both.
        let id = self.next_control;
        let Some(next) = id
            .checked_add(1)
            .filter(|next| i64::try_from(*next).is_ok())
        else {
            let _ = reply.send(Err(Error::NotSent {
                client_order_ids: BTreeMap::new(),
                operation: control.method(),
                reason: "stream control ids exhausted for this generation",
            }));
            return Ok(());
        };
        let body = match control {
            StreamControl::ListSubscriptions => {
                serde_json::json!({"method": control.method(), "id": id})
            }
            StreamControl::Subscribe | StreamControl::Unsubscribe => {
                serde_json::json!({"method": control.method(), "params": streams, "id": id})
            }
        }
        .to_string();
        if let Err(retry_after) = self.charge_incoming(Instant::now(), IncomingCharge::Control) {
            let _ = reply.send(Err(Error::Admission { retry_after }));
            return Ok(());
        }
        self.next_control = next;
        self.controls.insert(
            id,
            PendingControl {
                control,
                streams,
                deadline,
                reply: Some(reply),
            },
        );
        tokio::select! {
            ()=self.stop.cancelled()=>Err(Error::Closed),
            value=tokio::time::timeout_at(deadline.min(Instant::now().checked_add(self.timeout).ok_or(Error::Configuration("socket timeout range"))?),self.ws.send(Message::text(body)))=>{
                value.map_err(|_|Error::Gap("WebSocket send timeout"))?.map_err(|_|Error::Gap("WebSocket send failure"))
            }
        }
    }
    /// Deliver one correlated control answer, or surface it as a late answer.
    fn settle_control(&mut self, id: u64, value: &Value) -> Result<(), Error> {
        let Some(mut pending) = self.controls.remove(&id) else {
            return Ok(());
        };
        let (result, malformed) = match pending.control.answer(value) {
            Some(result) => (result, false),
            None => (Err(control_lost(pending.control)), true),
        };
        let unanswered = match pending.reply.take() {
            Some(reply) => reply.send(result).err(),
            None => Some(result),
        };
        if let Some(result) = unanswered {
            self.emit(SocketEvent::ControlLate {
                generation: self.generation,
                control: pending.control,
                streams: pending.streams,
                result,
            })?;
        }
        if malformed {
            return Err(Error::Gap("malformed stream control answer"));
        }
        Ok(())
    }
    fn expire(&mut self) {
        let now = Instant::now();
        for p in self.controls.values_mut() {
            if p.deadline <= now
                && let Some(reply) = p.reply.take()
            {
                let _ = reply.send(Err(control_lost(p.control)));
            }
        }
        for p in self.pending.values_mut() {
            if p.deadline <= now
                && let Some(reply) = p.reply.take()
            {
                let _ = reply.send(Err(Error::Transport {
                    client_order_ids: p.client_order_ids.clone(),
                    operation: p.op.name,
                    outcome: if p.op.mutation {
                        Outcome::Unknown
                    } else {
                        Outcome::ReadFailed
                    },
                    meta: None,
                }));
            }
        }
    }
    fn response(&mut self, value: Value) -> Result<(), Error> {
        if value.get("status").and_then(Value::as_u64) == Some(401)
            && value
                .get("error")
                .and_then(|e| e.get("code"))
                .and_then(Value::as_i64)
                == Some(-2015)
            && value.get("id").is_none_or(Value::is_null)
        {
            return Err(Error::Gap("WebSocket authentication revoked"));
        }
        if let Some(id) = value.get("id").and_then(Value::as_u64)
            && self.controls.contains_key(&id)
        {
            return self.settle_control(id, &value);
        }
        let id = value.get("id").and_then(Value::as_str).map(str::to_owned);
        if let Some(id) = id
            && let Some(mut p) = self.pending.remove(&id)
        {
            // A correlated answer is accepted ingress. If its evidence cannot be
            // classified, the venue's frame is still delivered in source order before
            // the failure propagates, so no answer and no correlation id is lost.
            if let Err(error) = self.settle(&mut p, &value) {
                self.emit(SocketEvent::Data {
                    generation: self.generation,
                    value: value.clone(),
                })?;
                return Err(error);
            }
            return Ok(());
        }
        self.emit(SocketEvent::Data {
            generation: self.generation,
            value,
        })
    }
    /// Hand one correlated answer to its caller, or surface it as a late answer.
    fn settle(&mut self, p: &mut Pending, value: &Value) -> Result<(), Error> {
        let status = value
            .get("status")
            .and_then(Value::as_u64)
            .and_then(|s| u16::try_from(s).ok())
            .ok_or(Error::Gap("WebSocket response status"))?;
        let now = self
            .clock
            .now_millis()
            .map_err(|_| Error::Gap("clock unavailable after response"))?;
        let rates = ws_rates(value, now, self.binary_timestamps);
        self.budgets.observe_ban(status, &rates)?;
        self.budgets.observe(&rates, now, true)?;
        let meta = ResponseMeta {
            time_unit: self.time_unit,
            client_order_ids: p.client_order_ids.clone(),
            status,
            operation: p.op.name,
            rates,
        };
        let payload = value
            .get("result")
            .or_else(|| value.get("error"))
            .cloned()
            .ok_or(Error::Gap("WebSocket response payload"))?;
        if (200..300).contains(&status)
            && payload
                .get("code")
                .and_then(Value::as_i64)
                .is_none_or(|c| c >= 0)
            && let Some(success) = p.op.success_weight
        {
            self.budgets
                .refund_weight(p.weight.saturating_sub(success), p.admitted_at, true)?;
        }
        if let Some(reply) = p.reply.take() {
            if let Err(result) = reply.send(Ok((payload.clone(), meta.clone())))
                && result.is_ok()
            {
                self.emit(SocketEvent::Late {
                    generation: self.generation,
                    id: p.id.clone(),
                    op: Box::new(p.op),
                    value: payload,
                    meta,
                })?;
            }
        } else {
            self.emit(SocketEvent::Late {
                generation: self.generation,
                id: p.id.clone(),
                op: Box::new(p.op),
                value: payload,
                meta,
            })?;
        }
        Ok(())
    }
    /// Run on the caller's runtime. This future owns the socket and every lifecycle write;
    /// it spawns no tasks. The caller joins the task if it chooses to spawn this future.
    pub async fn run(mut self) -> Result<(), Error> {
        self.emit(SocketEvent::Established(self.generation))?;
        let lifetime = self.lifetime;
        let mut failure = None;
        let mut refusal = None;
        loop {
            if self.stop.is_cancelled() {
                break;
            }
            self.expire();
            let wake = self
                .pending
                .values()
                .filter(|p| p.reply.is_some())
                .map(|p| p.deadline)
                .chain(
                    self.controls
                        .values()
                        .filter(|p| p.reply.is_some())
                        .map(|p| p.deadline),
                )
                .min()
                .unwrap_or(lifetime)
                .min(lifetime);
            let step = tokio::select! {
                ()=self.stop.cancelled()=>break,
                ()=tokio::time::sleep_until(wake)=>{
                    if Instant::now()>=lifetime {failure=Some(Error::Gap("documented 24-hour generation lifetime"));break;}
                    continue;
                }
                command=self.commands.recv()=>match command {
                    Some(command)=>self.command(command).await,
                    None=>break,
                },
                frame=self.ws.next()=>match frame {
                    Some(Ok(message)) if message.is_text()=>{
                        match serde_json::from_slice::<Value>(message.as_payload()) {
                            Ok(value) if self.require_binary_success && value.get("status").and_then(Value::as_u64).is_none_or(|status|status<400)=>Err(Error::Gap("SBE API successful JSON fallback")),
                            Ok(value)=>self.response(value),
                            Err(_)=>Err(Error::Gap("malformed JSON event")),
                        }
                    }
                    // The codec automatically sends the exact ping payload as a pong.
                    // Pongs share the venue's incoming-message ceiling with control
                    // messages, which are admitted one slot short of it so this
                    // reserved slot is free for the pong a ping demands.
                    Some(Ok(message)) if message.is_ping()=>{
                        if self.charge_incoming(Instant::now(), IncomingCharge::Pong).is_err() {Err(Error::Gap("documented ping/pong rate exceeded"))} else {
                            tokio::time::timeout(self.timeout,self.ws.flush()).await.map_err(|_|Error::Gap("pong timeout")).and_then(|v|v.map_err(|_|Error::Gap("pong failure")))
                        }
                    },
                    Some(Ok(message)) if message.is_close()=>{failure=Some(Error::Gap("venue closed socket"));break;},
                    Some(Ok(message)) if message.is_binary()=>self.binary_response(message.as_payload()),
                    Some(Ok(_))=>Ok(()),
                    Some(Err(_))=>Err(Error::Gap("WebSocket transport loss")),
                    None=>{failure=Some(Error::Gap("WebSocket EOF"));break;},
                },
            };
            if let Err(error) = step {
                // Only a proven continuity failure is reported as a gap: a sequence gap,
                // a malformed record, or transport loss. An owner-requested stop is not
                // lost continuity, and a local failure is returned to the caller
                // instead of being relabelled as one.
                match error {
                    Error::Gap(_) | Error::BinaryDecode { .. } | Error::FixDecode { .. } => {
                        failure = Some(error);
                    }
                    Error::Closed => {}
                    other => refusal = Some(other),
                }
                break;
            }
        }
        self.retire(failure, refusal).await
    }
    /// Retire one generation after its accepted prefix drained: settle every sent
    /// request, refuse every unsent one, then expose the boundary exactly once.
    async fn retire(
        &mut self,
        failure: Option<Error>,
        refusal: Option<Error>,
    ) -> Result<(), Error> {
        // Prefix was emitted in source order. Every sent pending request is now uncertain.
        for (_, mut p) in std::mem::take(&mut self.pending) {
            if let Some(reply) = p.reply.take() {
                let _ = reply.send(Err(Error::Transport {
                    client_order_ids: p.client_order_ids.clone(),
                    operation: p.op.name,
                    outcome: if p.op.mutation {
                        Outcome::Unknown
                    } else {
                        Outcome::ReadFailed
                    },
                    meta: None,
                }));
            }
        }
        for (_, mut p) in std::mem::take(&mut self.controls) {
            if let Some(reply) = p.reply.take() {
                let _ = reply.send(Err(control_lost(p.control)));
            }
        }
        if !matches!(
            failure,
            Some(Error::Gap("documented ping/pong rate exceeded"))
        ) {
            let _ = tokio::time::timeout(self.timeout, self.ws.close()).await;
        }
        // Commands admitted to the inbox but never sent get explicit unsent outcomes.
        self.commands.close();
        let mut close_replies = Vec::new();
        while let Some(command) = self.commands.recv().await {
            match command {
                Command::Call {
                    op, params, reply, ..
                } => {
                    // The command never reached the socket, so the caller's own
                    // deadline did not expire. The reason is this retirement.
                    let _ = reply.send(Err(Error::NotSent {
                        client_order_ids: super::request::order_ids(&params),
                        operation: op.name,
                        reason: "socket generation retired before send",
                    }));
                }
                Command::Control { control, reply, .. } => {
                    let _ = reply.send(Err(Error::NotSent {
                        client_order_ids: BTreeMap::new(),
                        operation: control.method(),
                        reason: "socket generation retired before send",
                    }));
                }
                Command::Close(reply) => close_replies.push(reply),
            }
        }
        if let Some(error) = failure {
            self.emit(SocketEvent::Gap {
                generation: self.generation,
                error,
            })?;
        }
        self.emit(SocketEvent::Retired(self.generation))?;
        for reply in close_replies {
            let _ = reply.send(());
        }
        // A local failure is the caller's answer; only continuity failures are gaps.
        if let Some(error) = refusal {
            return Err(error);
        }
        Ok(())
    }
}

fn ws_rates(v: &Value, now: u64, binary_timestamps: bool) -> RateEvidence {
    let mut e = RateEvidence::default();
    if let Some(limits) = v.get("rateLimits").and_then(Value::as_array) {
        for limit in limits {
            let Some(count) = limit.get("count").and_then(Value::as_u64) else {
                continue;
            };
            let prefix = match limit.get("rateLimitType").and_then(Value::as_str) {
                Some("REQUEST_WEIGHT") => "x-mbx-used-weight",
                Some("ORDERS") => "x-mbx-order-count",
                _ => continue,
            };
            let letter = match limit.get("interval").and_then(Value::as_str) {
                Some("SECOND") => "s",
                Some("MINUTE") => "m",
                Some("HOUR") => "h",
                Some("DAY") => "d",
                _ => continue,
            };
            if let Some(number) = limit.get("intervalNum").and_then(Value::as_u64) {
                e.counters
                    .insert(format!("{prefix}-{number}{letter}"), count);
            }
        }
    }
    let deadline = v.get("error").and_then(|e| {
        e.get("retryAfter")
            .or_else(|| e.get("data").and_then(|d| d.get("retryAfter")))
    });
    if let Some(deadline) = deadline {
        // The SBE ErrorResponse schema explicitly makes retryAfter optional;
        // its null sentinel projects to JSON null and means absent timing.
        if binary_timestamps && deadline.is_null() {
            return e;
        }
        let Some(deadline) = deadline.as_u64() else {
            e.retry_after_unusable = true;
            return e;
        };
        // A venue deadline at or before the local clock proves nothing about how long
        // to wait. Zero would silently read as "no cooldown", so the timing is reported
        // unusable instead: the refusal stands and no expiry is invented.
        let remaining = if binary_timestamps {
            now.checked_mul(1000)
                .and_then(|now| deadline.checked_sub(now))
        } else {
            deadline.checked_sub(now)
        };
        match remaining {
            Some(micros) if binary_timestamps && micros > 0 => {
                e.retry_after = Some(Duration::from_micros(micros));
            }
            Some(millis) if !binary_timestamps && millis > 0 => {
                e.retry_after = Some(Duration::from_millis(millis));
            }
            _ => e.retry_after_unusable = true,
        }
    }
    e
}

impl Drop for SocketEvents {
    fn drop(&mut self) {
        self.stop.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_sbe_retry_timing_is_absent_while_malformed_json_is_unusable() {
        let error = serde_json::json!({"error":{"code":-1100,"retryAfter":null}});
        let binary = ws_rates(&error, 1_700_000_000_000, true);
        assert_eq!(binary.retry_after, None);
        assert!(!binary.retry_after_unusable);
        assert!(ws_rates(&error, 1_700_000_000_000, false).retry_after_unusable);
    }
}
