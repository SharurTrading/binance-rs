// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{
    Budgets, Clock, Cost, Credentials, Error, Operation, Outcome, RateEvidence, RequestId,
    ResponseMeta, Security,
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
        op: Operation,
        value: Value,
        meta: ResponseMeta,
    },
    Gap {
        generation: u64,
        error: Error,
    },
    Retired(u64),
}

enum Command {
    Call {
        op: Operation,
        params: BTreeMap<String, Value>,
        cost: Box<Cost>,
        id: RequestId,
        deadline: Instant,
        reply: oneshot::Sender<Result<(Value, ResponseMeta), Error>>,
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
    ping_limit: usize,
    time_unit: super::TimeUnit,
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
    pub ping_limit: usize,
    pub time_unit: super::TimeUnit,
}

impl Socket {
    pub async fn connect(
        url: url::Url,
        credentials: Option<Credentials>,
        clock: Arc<dyn Clock>,
        budgets: Budgets,
        timeout: Duration,
        api: bool,
    ) -> Result<(Self, SocketEvents, SocketDriver), Error> {
        Self::connect_with_policy(
            url,
            credentials,
            clock,
            budgets,
            timeout,
            SocketPolicy {
                handshake: Cost {
                    ws_weight: if api { 5 } else { 0 },
                    ..Cost::default()
                },
                ping_limit: if api { 5 } else { 10 },
                time_unit: super::TimeUnit::Milliseconds,
            },
        )
        .await
    }
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
        let builder = ClientBuilder::new()
            .uri(url.as_str())
            .map_err(|_| Error::Configuration("WebSocket URI"))?
            .connector(&connector)
            .limits(Limits::unlimited());
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
            ping_limit: policy.ping_limit,
            time_unit: policy.time_unit,
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
                op,
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
                    (op.validate_time)(&params, self.clock.now_millis()?)?;
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
                        op,
                        id,
                        deadline,
                        reply: Some(reply),
                    },
                );
                let outcome = tokio::select! {
                    ()=self.stop.cancelled()=>Err(Error::Closed),
                    value=tokio::time::timeout_at(deadline.min(Instant::now()+self.timeout),self.ws.send(Message::text(body)))=>{
                        value.map_err(|_|Error::Gap("WebSocket send timeout"))?.map_err(|_|Error::Gap("WebSocket send failure"))
                    }
                };
                outcome
            }
        }
    }
    fn expire(&mut self) {
        let now = Instant::now();
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
        let id = value.get("id").and_then(Value::as_str);
        if let Some(id) = id
            && let Some(mut p) = self.pending.remove(id)
        {
            let status = value
                .get("status")
                .and_then(Value::as_u64)
                .and_then(|s| u16::try_from(s).ok())
                .ok_or(Error::Gap("WebSocket response status"))?;
            let now = self
                .clock
                .now_millis()
                .map_err(|_| Error::Gap("clock unavailable after response"))?;
            let rates = ws_rates(&value, now);
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
                self.budgets.refund_weight(
                    p.weight.saturating_sub(success),
                    p.admitted_at,
                    true,
                )?;
            }
            if let Some(reply) = p.reply.take() {
                if let Err(result) = reply.send(Ok((payload.clone(), meta.clone())))
                    && result.is_ok()
                {
                    self.emit(SocketEvent::Late {
                        generation: self.generation,
                        id: p.id,
                        op: p.op,
                        value: payload,
                        meta,
                    })?;
                }
            } else {
                self.emit(SocketEvent::Late {
                    generation: self.generation,
                    id: p.id,
                    op: p.op,
                    value: payload,
                    meta,
                })?;
            }
            return Ok(());
        }
        self.emit(SocketEvent::Data {
            generation: self.generation,
            value,
        })
    }
    /// Run on the caller's runtime. This future owns the socket and every lifecycle write;
    /// it spawns no tasks. The caller joins the task if it chooses to spawn this future.
    pub async fn run(mut self) -> Result<(), Error> {
        self.emit(SocketEvent::Established(self.generation))?;
        let lifetime = self.lifetime;
        let mut failure = None;
        let mut pings: VecDeque<Instant> = VecDeque::new();
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
                        match serde_json::from_slice(message.as_payload()) {
                            Ok(value)=>self.response(value),
                            Err(_)=>Err(Error::Gap("malformed JSON event")),
                        }
                    }
                    // The codec automatically sends the exact ping payload as a pong.
                    Some(Ok(message)) if message.is_ping()=>{
                        let now=Instant::now();
                        while pings.front().is_some_and(|p|now.duration_since(*p)>=Duration::from_secs(1)) {pings.pop_front();}
                        if pings.len()>=self.ping_limit {Err(Error::Gap("documented ping/pong rate exceeded"))} else {
                            pings.push_back(now);
                            tokio::time::timeout(self.timeout,self.ws.flush()).await.map_err(|_|Error::Gap("pong timeout")).and_then(|v|v.map_err(|_|Error::Gap("pong failure")))
                        }
                    },
                    Some(Ok(message)) if message.is_close()=>{failure=Some(Error::Gap("venue closed socket"));break;},
                    Some(Ok(message)) if message.is_binary()=>Err(Error::Gap("unexpected binary event on JSON connection")),
                    Some(Ok(_))=>Ok(()),
                    Some(Err(_))=>Err(Error::Gap("WebSocket transport loss")),
                    None=>{failure=Some(Error::Gap("WebSocket EOF"));break;},
                },
            };
            if let Err(error) = step {
                failure = Some(error);
                break;
            }
        }
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
                Command::Call { op, reply, .. } => {
                    let _ = reply.send(Err(Error::Expired(op.name)));
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
        Ok(())
    }
}

fn ws_rates(v: &Value, now: u64) -> RateEvidence {
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
    e.retry_after = v
        .get("error")
        .and_then(|e| {
            e.get("retryAfter")
                .or_else(|| e.get("data").and_then(|d| d.get("retryAfter")))
        })
        .and_then(Value::as_u64)
        .map(|deadline| Duration::from_millis(deadline.saturating_sub(now)));
    e
}

impl Drop for SocketEvents {
    fn drop(&mut self) {
        self.stop.cancel();
    }
}
