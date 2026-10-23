// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Response evidence for one caller-owned WebSocket upgrade attempt.

use crate::core::socket::SocketPolicy;
use crate::core::{Budgets, Clock, Error, Outcome, RateEvidence, ResponseMeta, TransportFailure};
use std::{
    collections::BTreeMap,
    future::Future,
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
    time::Instant,
};
use tokio_websockets::resolver::Resolver as _;
use tokio_websockets::{ClientBuilder, Connector, Limits, MaybeTlsStream, WebSocketStream};
use zeroize::{Zeroize, Zeroizing};

pub(crate) type SocketStream = WebSocketStream<HandshakeStream>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Phase {
    #[default]
    Status,
    Headers,
    Done,
}

#[derive(Default)]
struct Head {
    line: Zeroizing<Vec<u8>>,
    status: Option<u16>,
    rates: RateEvidence,
    headers: usize,
    sent: bool,
    refusal: Option<Error>,
    phase: Phase,
}

impl Head {
    fn receive(&mut self, bytes: &[u8]) {
        for byte in bytes {
            if self.phase == Phase::Done {
                break;
            }
            self.line.push(*byte);
            if *byte != b'\n' {
                continue;
            }
            let content = &self.line[..self.line.len().saturating_sub(1)];
            let content = content.strip_suffix(b"\r").unwrap_or(content);
            match self.phase {
                Phase::Status => {
                    if !content.is_empty() {
                        self.status = status(content);
                        self.phase = Phase::Headers;
                    }
                }
                Phase::Headers => {
                    if content.is_empty() {
                        self.phase = Phase::Done;
                    } else {
                        self.headers = self.headers.saturating_add(1);
                        capture_header(&mut self.rates, content);
                        // The native upgrade codec accepts at most 25 response headers.
                        if self.headers >= 25 {
                            self.phase = Phase::Done;
                        }
                    }
                }
                Phase::Done => break,
            }
            self.line.zeroize();
        }
    }

    fn metadata(&self, time_unit: crate::TimeUnit) -> Option<ResponseMeta> {
        self.status.map(|status| ResponseMeta {
            time_unit,
            client_order_ids: BTreeMap::new(),
            status,
            operation: "connect",
            rates: self.rates.clone(),
        })
    }
}

fn status(line: &[u8]) -> Option<u16> {
    let mut parts = line.split(|byte| *byte == b' ');
    if !matches!(parts.next(), Some(b"HTTP/1.1" | b"HTTP/1.0")) {
        return None;
    }
    let code = parts.next()?;
    if code.len() != 3 || !code.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(code).ok()?.parse().ok()
}

fn capture_header(rates: &mut RateEvidence, line: &[u8]) {
    let Some(colon) = line.iter().position(|byte| *byte == b':') else {
        return;
    };
    let Ok(name) = reqwest::header::HeaderName::from_bytes(&line[..colon]) else {
        return;
    };
    let value = &line[colon.saturating_add(1)..];
    let value = value.trim_ascii();
    let counted = name.as_str().starts_with("x-mbx-used-weight-")
        || name.as_str().starts_with("x-mbx-order-count-")
        || name.as_str().starts_with("x-sapi-used-")
        || matches!(name.as_str(), "x-mbx-used-weight" | "x-mbx-order-count");
    if counted
        && let Ok(value) = std::str::from_utf8(value)
        && let Ok(value) = value.parse()
    {
        rates.counters.insert(name.as_str().to_owned(), value);
    }
    if name == reqwest::header::RETRY_AFTER
        && rates.retry_after.is_none()
        && !rates.retry_after_unusable
    {
        rates.retry_after = std::str::from_utf8(value)
            .ok()
            .and_then(|value| value.parse().ok())
            .map(Duration::from_secs);
        rates.retry_after_unusable = rates.retry_after.is_none();
    }
}

pub(crate) struct HandshakeStream<S = MaybeTlsStream<TcpStream>> {
    inner: S,
    capture: Option<Arc<Mutex<Head>>>,
    admission: WriteAdmission,
}

enum WriteAdmission {
    Pending(Box<PendingAdmission>),
    Refused,
    Sent,
}

struct PendingAdmission {
    budgets: Budgets,
    clock: Arc<dyn Clock>,
    charge: crate::core::rate::HandshakeCharge,
    deadline: Instant,
    evidence: Arc<Mutex<Head>>,
}

impl PendingAdmission {
    fn validate(&self) -> Result<(), Error> {
        if Instant::now() >= self.deadline {
            return Err(Error::Expired("connect"));
        }
        self.budgets
            .validate_handshake_charge(&self.charge, self.clock.now_millis()?)
    }
}

impl<S> HandshakeStream<S> {
    fn before_write(&mut self) -> io::Result<()> {
        let pending = match &self.admission {
            WriteAdmission::Pending(pending) => pending,
            WriteAdmission::Refused => return Err(io::Error::other("upgrade admission refused")),
            WriteAdmission::Sent => return Ok(()),
        };
        if let Err(error) = pending.validate() {
            pending
                .evidence
                .lock()
                .map_err(|_| io::Error::other("handshake evidence synchronization failed"))?
                .refusal = Some(error);
            self.admission = WriteAdmission::Refused;
            return Err(io::Error::other("upgrade admission refused"));
        }
        Ok(())
    }

    fn observe_write(&mut self, result: &Poll<io::Result<usize>>) -> io::Result<()> {
        if matches!(result, Poll::Ready(Ok(size)) if *size > 0) {
            self.admission = WriteAdmission::Sent;
            if let Some(capture) = &self.capture {
                capture
                    .lock()
                    .map_err(|_| io::Error::other("handshake evidence synchronization failed"))?
                    .sent = true;
            }
        }
        Ok(())
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for HandshakeStream<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let before = buffer.filled().len();
        let result = Pin::new(&mut self.inner).poll_read(cx, buffer);
        if matches!(result, Poll::Ready(Ok(()))) {
            let done = if let Some(capture) = &self.capture {
                let mut head = capture
                    .lock()
                    .map_err(|_| io::Error::other("handshake evidence synchronization failed"))?;
                head.receive(&buffer.filled()[before..]);
                head.phase == Phase::Done
            } else {
                false
            };
            if done {
                self.capture = None;
            }
        }
        result
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for HandshakeStream<S> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.before_write()?;
        let result = Pin::new(&mut self.inner).poll_write(cx, buffer);
        self.observe_write(&result)?;
        result
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buffers: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        self.before_write()?;
        let result = Pin::new(&mut self.inner).poll_write_vectored(cx, buffers);
        self.observe_write(&result)?;
        result
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }
}

pub(crate) async fn connect(
    url: &url::Url,
    api_key: Option<reqwest::header::HeaderValue>,
    clock: Arc<dyn Clock>,
    budgets: &Budgets,
    timeout: Duration,
    policy: &SocketPolicy,
) -> Result<SocketStream, Error> {
    let connector = match url.scheme() {
        "wss" => Connector::Rustls(tokio_rustls::TlsConnector::from(Arc::new(
            crate::core::socket::tls_config()?,
        ))),
        "ws" => Connector::Plain,
        _ => return Err(Error::Configuration("WebSocket scheme")),
    };
    let host = url
        .host_str()
        .ok_or(Error::Configuration("WebSocket host"))?
        .trim_start_matches('[')
        .trim_end_matches(']');
    let port = url
        .port_or_known_default()
        .ok_or(Error::Configuration("WebSocket port"))?;
    let mut builder = ClientBuilder::new()
        .uri(url.as_str())
        .map_err(|_| Error::Configuration("WebSocket URI"))?
        .limits(Limits::unlimited());
    if let Some(api_key) = api_key {
        builder = builder
            .add_header(
                reqwest::header::HeaderName::from_static("x-mbx-apikey"),
                api_key,
            )
            .map_err(|_| Error::Configuration("API key handshake header"))?;
    }
    let setup = async {
        let address = tokio_websockets::resolver::Gai.resolve(host, port).await?;
        let transport = TcpStream::connect(address).await?;
        connector.wrap(host, transport).await
    };
    connect_on_setup(builder, setup, clock, budgets, timeout, policy).await
}

async fn connect_on_setup(
    builder: ClientBuilder<'_>,
    setup: impl Future<Output = Result<MaybeTlsStream<TcpStream>, tokio_websockets::Error>>,
    clock: Arc<dyn Clock>,
    budgets: &Budgets,
    timeout: Duration,
    policy: &SocketPolicy,
) -> Result<SocketStream, Error> {
    let deadline = Instant::now()
        .checked_add(timeout)
        .ok_or(Error::Configuration("socket timeout range"))?;
    let capture = Arc::new(Mutex::new(Head::default()));
    let admitted_at = clock.now_millis()?;
    let charge = budgets.admit_handshake(policy.handshake, admitted_at)?;
    let admission = WriteAdmission::Pending(Box::new(PendingAdmission {
        budgets: budgets.clone(),
        clock: Arc::clone(&clock),
        charge,
        deadline,
        evidence: Arc::clone(&capture),
    }));
    let attempt = async {
        let stream = HandshakeStream {
            inner: setup.await?,
            capture: Some(Arc::clone(&capture)),
            admission,
        };
        builder.connect_on(stream).await.map(|(socket, _)| socket)
    };
    let result = tokio::time::timeout_at(deadline, attempt).await;
    let (meta, sent, refusal) = {
        let mut head = capture
            .lock()
            .map_err(|_| Error::Configuration("handshake evidence synchronization failed"))?;
        head.line.zeroize();
        head.phase = Phase::Done;
        (
            head.metadata(policy.time_unit),
            head.sent,
            head.refusal.take(),
        )
    };
    if let Some(refusal) = refusal {
        return Err(refusal);
    }
    let outcome = if sent {
        Outcome::ReadFailed
    } else {
        Outcome::NotSent
    };
    if let Some(meta) = &meta {
        crate::core::response_rates::observe(
            budgets,
            clock.as_ref(),
            meta,
            Outcome::ReadFailed,
            crate::core::response_rates::Transport::WebSocket,
        )?;
    }
    match result {
        Ok(Ok(mut socket)) => {
            socket.get_mut().capture = None;
            Ok(socket)
        }
        Ok(Err(_)) => {
            let cause = match &meta {
                Some(meta) if meta.status != 101 => TransportFailure::WebSocketHandshakeRejected,
                Some(_) => TransportFailure::WebSocketHandshakeInvalid,
                None => TransportFailure::ResponseHeadersFailed,
            };
            Err(failure(meta, cause, outcome))
        }
        Err(_) => Err(failure(
            meta,
            TransportFailure::ResponseHeadersTimeout,
            outcome,
        )),
    }
}

fn failure(meta: Option<ResponseMeta>, cause: TransportFailure, outcome: Outcome) -> Error {
    Error::Transport {
        client_order_ids: BTreeMap::new(),
        operation: "connect",
        outcome,
        failure: Some(cause),
        meta: meta.map(Box::new),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragmented_head_retains_only_safe_evidence_and_stops_before_socket_data() {
        let mut head = Head::default();
        let wire = b"HTTP/1.1 429 synthetic-secret-status\r\nRetry-After: 2\r\nX-MBX-USED-WEIGHT-1M: 17\r\nSet-Cookie: synthetic-secret-cookie\r\n\r\nRetry-After: 9999\r\nX-MBX-USED-WEIGHT-1M: 9999\r\n";
        for byte in wire {
            head.receive(std::slice::from_ref(byte));
        }
        let meta = head.metadata(crate::TimeUnit::Milliseconds).unwrap();
        assert_eq!(meta.status, 429);
        assert_eq!(meta.rates.retry_after, Some(Duration::from_secs(2)));
        assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 17);
        assert_eq!(meta.rates.counters.len(), 1);
        assert!(head.line.is_empty());
        assert!(head.phase == Phase::Done);
        assert!(!format!("{meta:?}").contains("synthetic-secret"));
    }

    #[test]
    fn fragmented_leading_empty_lines_preserve_received_status_and_rates() {
        let mut head = Head::default();
        let wire = b"\r\n\n\r\nHTTP/1.1 429 synthetic-secret\r\nRetry-After: 2\r\n\r\n";
        for byte in wire {
            head.receive(std::slice::from_ref(byte));
        }
        let meta = head.metadata(crate::TimeUnit::Milliseconds).unwrap();
        assert_eq!(meta.status, 429);
        assert_eq!(meta.rates.retry_after, Some(Duration::from_secs(2)));
        assert!(head.line.is_empty());
        assert!(!format!("{meta:?}").contains("synthetic-secret"));
    }

    #[test]
    fn unregistered_status_preserves_safe_response_evidence() {
        for (code, expected) in [("000", 0), ("099", 99), ("799", 799), ("999", 999)] {
            let mut head = Head::default();
            let wire = format!("HTTP/1.1 {code} synthetic-secret\r\nRetry-After: 2\r\n\r\n");
            head.receive(wire.as_bytes());
            let meta = head.metadata(crate::TimeUnit::Milliseconds).unwrap();
            assert_eq!(meta.status, expected);
            assert_eq!(meta.rates.retry_after, Some(Duration::from_secs(2)));
            assert!(!format!("{meta:?}").contains("synthetic-secret"));
        }
    }

    #[test]
    fn native_line_feed_termination_stops_before_socket_data() {
        let mut head = Head::default();
        head.receive(b"HTTP/1.1 418 synthetic\nX-MBX-USED-WEIGHT-1M: 17\n\nRetry-After: 9999\n");
        assert_eq!(head.status, Some(418));
        assert_eq!(head.rates.counters["x-mbx-used-weight-1m"], 17);
        assert_eq!(head.rates.retry_after, None);
        assert!(head.phase == Phase::Done);
        assert!(head.line.is_empty());
    }

    #[test]
    fn native_header_limit_stops_and_clears_ephemeral_capture() {
        let mut head = Head::default();
        head.receive(b"HTTP/1.1 429 synthetic\r\nRetry-After: 2\r\n");
        for _ in 0..24 {
            head.receive(b"x-ignored: synthetic-secret\r\n");
        }
        head.receive(b"Retry-After: 9999\r\nX-MBX-USED-WEIGHT-1M: 9999\r\n\r\n");
        assert!(head.phase == Phase::Done);
        assert_eq!(head.headers, 25);
        assert!(head.line.is_empty());
        assert_eq!(head.rates.retry_after, Some(Duration::from_secs(2)));
        assert!(head.rates.counters.is_empty());
    }

    #[test]
    fn unusable_retry_timing_retains_only_the_explicit_unknown_timing_marker() {
        let mut head = Head::default();
        head.receive(b"HTTP/1.1 429 synthetic\r\nRetry-After: synthetic-secret\r\n\r\n");
        assert_eq!(head.rates.retry_after, None);
        assert!(head.rates.retry_after_unusable);
        assert!(
            !format!("{:?}", head.metadata(crate::TimeUnit::Milliseconds))
                .contains("synthetic-secret")
        );
    }
    mod admission {
        use super::*;
        use crate::core::rate::reserve::ExitEligibility;
        use crate::{BudgetLimits, ExitReserve};
        use std::collections::VecDeque;
        use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;
        use tokio::sync::oneshot;
        use tokio_websockets::ServerBuilder;

        const ADMITTED: u64 = 659_000;

        struct ControlledClock(AtomicU64, AtomicBool);

        impl Clock for ControlledClock {
            fn now_millis(&self) -> Result<u64, Error> {
                if self.1.swap(false, Ordering::SeqCst) {
                    Err(Error::Configuration(
                        "synthetic handshake clock unavailable",
                    ))
                } else {
                    Ok(self.0.load(Ordering::SeqCst))
                }
            }
        }

        fn clock() -> Arc<ControlledClock> {
            Arc::new(ControlledClock(
                AtomicU64::new(ADMITTED),
                AtomicBool::new(false),
            ))
        }

        fn policy(recovery: bool) -> SocketPolicy {
            SocketPolicy {
                handshake: crate::core::Cost {
                    ws_weight: 2,
                    connections: 1,
                    exit: if recovery {
                        ExitEligibility::Eligible
                    } else {
                        ExitEligibility::Ordinary
                    },
                    ..Default::default()
                },
                incoming_limit: 5,
                time_unit: crate::TimeUnit::Milliseconds,
                binary_decoder: None,
                api_key_header: false,
                max_generation_age: None,
            }
        }

        struct RecordedStream {
            stream: TcpStream,
            received: Arc<Mutex<Vec<u8>>>,
        }

        impl AsyncRead for RecordedStream {
            fn poll_read(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buffer: &mut ReadBuf<'_>,
            ) -> Poll<io::Result<()>> {
                let before = buffer.filled().len();
                let result = Pin::new(&mut self.stream).poll_read(cx, buffer);
                if matches!(result, Poll::Ready(Ok(()))) {
                    self.received
                        .lock()
                        .unwrap()
                        .extend_from_slice(&buffer.filled()[before..]);
                }
                result
            }
        }

        impl AsyncWrite for RecordedStream {
            fn poll_write(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                bytes: &[u8],
            ) -> Poll<io::Result<usize>> {
                Pin::new(&mut self.stream).poll_write(cx, bytes)
            }

            fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
                Pin::new(&mut self.stream).poll_flush(cx)
            }

            fn poll_shutdown(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<io::Result<()>> {
                Pin::new(&mut self.stream).poll_shutdown(cx)
            }
        }

        async fn venue() -> (std::net::SocketAddr, tokio::task::JoinHandle<Vec<u8>>) {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let task = tokio::spawn(async move {
                let (stream, _) = listener.accept().await.unwrap();
                let received = Arc::new(Mutex::new(Vec::new()));
                let transport = RecordedStream {
                    stream,
                    received: Arc::clone(&received),
                };
                let result = ServerBuilder::new().accept(transport).await;
                drop(result);
                received.lock().unwrap().clone()
            });
            (address, task)
        }

        async fn shared_refusal(budgets: &Budgets, clock: Arc<ControlledClock>, status: u16) {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url = url::Url::parse(&format!("ws://{}", listener.local_addr().unwrap())).unwrap();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut bytes = Vec::new();
                let mut buffer = [0; 1024];
                while !bytes.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
                    let read = stream.read(&mut buffer).await.unwrap();
                    assert!(read > 0);
                    bytes.extend_from_slice(&buffer[..read]);
                }
                let retry = if status == 429 {
                    "Retry-After: 2\r\n"
                } else {
                    ""
                };
                let response = format!("HTTP/1.1 {status} fixture\r\n{retry}\r\n");
                stream.write_all(response.as_bytes()).await.unwrap();
            });
            let error = connect(
                &url,
                None,
                clock,
                budgets,
                Duration::from_secs(5),
                &policy(false),
            )
            .await
            .err()
            .unwrap();
            assert!(
                matches!(error, Error::Transport { meta: Some(meta), .. } if meta.status == status)
            );
            server.await.unwrap();
        }

        #[derive(Clone, Copy)]
        enum Change {
            TimedCooldown,
            UntimedBan,
            MinuteWindow,
            ConnectionWindow,
            BackwardClock,
            ClockFailure,
            Limit,
            Reserve,
        }

        fn check_refusal(error: &Error, change: Change) {
            assert_eq!(error.outcome(), Some(Outcome::NotSent));
            match change {
                Change::TimedCooldown => assert!(
                    matches!(error, Error::Admission { retry_after } if *retry_after == Duration::from_secs(2))
                ),
                Change::UntimedBan => assert!(matches!(error, Error::CooldownTimingUnknown)),
                Change::MinuteWindow | Change::ConnectionWindow | Change::BackwardClock => {
                    assert!(matches!(
                        error,
                        Error::NotSent {
                            operation: "connect",
                            reason: "handshake quota admission is no longer valid",
                            ..
                        }
                    ));
                }
                Change::ClockFailure => assert!(matches!(
                    error,
                    Error::Configuration("synthetic handshake clock unavailable")
                )),
                Change::Limit | Change::Reserve => {
                    assert!(matches!(error, Error::Admission { .. }));
                }
            }
        }

        async fn setup_refusal(recovery: bool, change: Change) {
            let clock = clock();
            let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
            let mut policy = policy(recovery);
            if matches!(change, Change::ConnectionWindow) {
                policy.handshake.ws_weight = 0;
                clock.0.store(899_000, Ordering::SeqCst);
            }
            let charged_at = clock.now_millis().unwrap();
            let (address, server) = venue().await;
            let (ready, accepted) = oneshot::channel();
            let (release, released) = oneshot::channel();
            let setup = async move {
                let stream = TcpStream::connect(address).await?;
                ready.send(()).unwrap();
                released.await.unwrap();
                Ok(MaybeTlsStream::Plain(stream))
            };
            let owner = budgets.clone();
            let observed = Arc::clone(&clock);
            let attempt = tokio::spawn(async move {
                let builder = ClientBuilder::new()
                    .uri(&format!("ws://{address}"))
                    .unwrap();
                connect_on_setup(
                    builder,
                    setup,
                    observed,
                    &owner,
                    Duration::from_secs(5),
                    &policy,
                )
                .await
            });
            accepted.await.unwrap();
            match change {
                Change::TimedCooldown => shared_refusal(&budgets, Arc::clone(&clock), 429).await,
                Change::UntimedBan => shared_refusal(&budgets, Arc::clone(&clock), 418).await,
                Change::MinuteWindow => clock.0.store(661_000, Ordering::SeqCst),
                Change::ConnectionWindow => clock.0.store(901_000, Ordering::SeqCst),
                Change::BackwardClock => clock.0.store(658_999, Ordering::SeqCst),
                Change::ClockFailure => clock.1.store(true, Ordering::SeqCst),
                Change::Limit => budgets
                    .adopt_stated([crate::core::rate::StatedLimit {
                        kind: Some("REQUEST_WEIGHT"),
                        interval: Some("MINUTE"),
                        interval_num: Some(1),
                        limit: Some(1),
                    }])
                    .unwrap(),
                Change::Reserve => budgets
                    .set_exit_reserve(ExitReserve::default().request_weight(6_000))
                    .unwrap(),
            }
            release.send(()).unwrap();
            let error = match attempt.await.unwrap() {
                Ok(socket) => {
                    drop(socket);
                    None
                }
                Err(error) => Some(error),
            };
            let received = server.await.unwrap();
            assert!(
                received.is_empty(),
                "an upgrade reached the venue after setup lost admission"
            );
            check_refusal(&error.unwrap(), change);
            let expected = if matches!(change, Change::ConnectionWindow) {
                0
            } else if matches!(change, Change::TimedCooldown | Change::UntimedBan) {
                4
            } else {
                2
            };
            assert_eq!(
                budgets.usage(charged_at).unwrap().request_weight.used,
                expected
            );
        }

        macro_rules! setup_change {
            ($name:ident, $change:ident) => {
                #[tokio::test]
                async fn $name() {
                    for recovery in [false, true] {
                        setup_refusal(recovery, Change::$change).await;
                    }
                }
            };
        }

        setup_change!(
            a_shared_429_during_setup_prevents_the_first_upgrade_write,
            TimedCooldown
        );
        setup_change!(
            a_shared_untimed_418_during_setup_prevents_the_first_upgrade_write,
            UntimedBan
        );
        setup_change!(
            a_charged_weight_minute_expiring_during_setup_prevents_upgrade,
            MinuteWindow
        );
        setup_change!(
            a_charged_connection_window_expiring_during_setup_prevents_upgrade,
            ConnectionWindow
        );
        setup_change!(
            a_backward_clock_during_setup_prevents_upgrade,
            BackwardClock
        );
        setup_change!(a_native_limit_reduced_during_setup_prevents_upgrade, Limit);

        setup_change!(
            a_first_write_clock_failure_preserves_the_typed_unsent_refusal,
            ClockFailure
        );

        #[tokio::test]
        async fn ordinary_reserve_growth_during_setup_prevents_upgrade() {
            setup_refusal(false, Change::Reserve).await;
        }

        #[tokio::test]
        async fn exact_fit_ordinary_and_protected_handshakes_keep_one_charge() {
            for recovery in [false, true] {
                let mut limits = BudgetLimits::spot();
                limits.weight_per_minute = 2;
                limits.connections_per_five_minutes = Some(1);
                let budgets = Budgets::new(limits).unwrap();
                if recovery {
                    budgets
                        .set_exit_reserve(ExitReserve::default().request_weight(2))
                        .unwrap();
                }
                let (address, server) = venue().await;
                let url = url::Url::parse(&format!("ws://{address}")).unwrap();
                let socket = connect(
                    &url,
                    None,
                    clock(),
                    &budgets,
                    Duration::from_secs(5),
                    &policy(recovery),
                )
                .await
                .unwrap();
                drop(socket);
                assert!(server.await.unwrap().starts_with(b"GET /"));
                assert_eq!(budgets.usage(ADMITTED).unwrap().request_weight.used, 2);
                assert!(matches!(
                    budgets.admit(
                        crate::core::Cost {
                            connections: 1,
                            ..Default::default()
                        },
                        ADMITTED
                    ),
                    Err(Error::Admission { .. })
                ));
            }
        }

        #[tokio::test]
        async fn an_elapsed_ticket_refuses_native_upgrade_before_any_wire_byte() {
            let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
            let charge = budgets
                .admit_handshake(policy(false).handshake, ADMITTED)
                .unwrap();
            let deadline = Instant::now();
            let (address, server) = venue().await;
            let evidence = Arc::new(Mutex::new(Head::default()));
            let stream = HandshakeStream {
                inner: MaybeTlsStream::Plain(TcpStream::connect(address).await.unwrap()),
                capture: Some(Arc::clone(&evidence)),
                admission: WriteAdmission::Pending(Box::new(PendingAdmission {
                    budgets: budgets.clone(),
                    clock: clock(),
                    charge,
                    deadline,
                    evidence: Arc::clone(&evidence),
                })),
            };
            let builder = ClientBuilder::new()
                .uri(&format!("ws://{address}"))
                .unwrap();
            let result = builder.connect_on(stream).await;
            drop(result);
            assert!(server.await.unwrap().is_empty());
            let head = evidence.lock().unwrap();
            let error = head.refusal.as_ref().unwrap();
            assert!(matches!(error, Error::Expired("connect")));
            assert_eq!(error.outcome(), Some(Outcome::NotSent));
            assert!(!head.sent);
            assert_eq!(budgets.usage(ADMITTED).unwrap().request_weight.used, 2);
        }

        enum Write {
            Pending,
            Zero,
            Positive(usize),
        }

        struct Writer {
            script: VecDeque<Write>,
            polls: usize,
            sent: Vec<u8>,
        }

        impl AsyncWrite for Writer {
            fn poll_write(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                bytes: &[u8],
            ) -> Poll<io::Result<usize>> {
                self.polls += 1;
                match self.script.pop_front().unwrap() {
                    Write::Pending => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    Write::Zero => Poll::Ready(Ok(0)),
                    Write::Positive(limit) => {
                        let size = limit.min(bytes.len());
                        self.sent.extend_from_slice(&bytes[..size]);
                        Poll::Ready(Ok(size))
                    }
                }
            }

            fn poll_write_vectored(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buffers: &[io::IoSlice<'_>],
            ) -> Poll<io::Result<usize>> {
                self.poll_write(cx, &buffers[0])
            }

            fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
                Poll::Ready(Ok(()))
            }

            fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
                Poll::Ready(Ok(()))
            }
        }

        fn guarded(
            script: Vec<Write>,
            deadline: Instant,
        ) -> (
            HandshakeStream<Writer>,
            Budgets,
            Arc<ControlledClock>,
            Arc<Mutex<Head>>,
        ) {
            let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
            let charge = budgets
                .admit_handshake(policy(false).handshake, ADMITTED)
                .unwrap();
            let evidence = Arc::new(Mutex::new(Head::default()));
            let clock = clock();
            let stream = HandshakeStream {
                inner: Writer {
                    script: script.into(),
                    polls: 0,
                    sent: Vec::new(),
                },
                capture: Some(Arc::clone(&evidence)),
                admission: WriteAdmission::Pending(Box::new(PendingAdmission {
                    budgets: budgets.clone(),
                    clock: clock.clone(),
                    charge,
                    deadline,
                    evidence: Arc::clone(&evidence),
                })),
            };
            (stream, budgets, clock, evidence)
        }

        #[test]
        fn each_pending_first_write_poll_rechecks_shared_admission() {
            let (mut stream, budgets, _, evidence) = guarded(
                vec![Write::Pending, Write::Pending, Write::Positive(3)],
                Instant::now() + Duration::from_secs(5),
            );
            let mut cx = Context::from_waker(std::task::Waker::noop());
            assert!(
                Pin::new(&mut stream)
                    .poll_write(&mut cx, b"GET")
                    .is_pending()
            );
            assert!(
                Pin::new(&mut stream)
                    .poll_write(&mut cx, b"GET")
                    .is_pending()
            );
            budgets.observe_ban(418, &RateEvidence::default()).unwrap();
            assert!(matches!(
                Pin::new(&mut stream).poll_write(&mut cx, b"GET"),
                Poll::Ready(Err(_))
            ));
            assert_eq!(stream.inner.polls, 2);
            assert!(stream.inner.sent.is_empty());
            let head = evidence.lock().unwrap();
            assert!(matches!(head.refusal, Some(Error::CooldownTimingUnknown)));
            assert!(!head.sent);
        }

        #[test]
        fn a_zero_byte_write_keeps_the_guard_for_the_next_vectored_poll() {
            let (mut stream, budgets, _, evidence) = guarded(
                vec![Write::Zero, Write::Positive(3)],
                Instant::now() + Duration::from_secs(5),
            );
            let mut cx = Context::from_waker(std::task::Waker::noop());
            assert!(matches!(
                Pin::new(&mut stream).poll_write(&mut cx, b"GET"),
                Poll::Ready(Ok(0))
            ));
            budgets.observe_ban(418, &RateEvidence::default()).unwrap();
            assert!(matches!(
                Pin::new(&mut stream).poll_write_vectored(&mut cx, &[io::IoSlice::new(b"GET")]),
                Poll::Ready(Err(_))
            ));
            assert_eq!(stream.inner.polls, 1);
            assert!(stream.inner.sent.is_empty());
            assert!(matches!(
                evidence.lock().unwrap().refusal,
                Some(Error::CooldownTimingUnknown)
            ));
        }

        #[test]
        fn a_positive_partial_write_commits_the_native_attempt() {
            let (mut stream, budgets, _, evidence) = guarded(
                vec![Write::Positive(1), Write::Positive(2)],
                Instant::now() + Duration::from_secs(5),
            );
            let mut cx = Context::from_waker(std::task::Waker::noop());
            assert!(matches!(
                Pin::new(&mut stream).poll_write(&mut cx, b"GET"),
                Poll::Ready(Ok(1))
            ));
            budgets.observe_ban(418, &RateEvidence::default()).unwrap();
            assert!(matches!(
                Pin::new(&mut stream).poll_write_vectored(&mut cx, &[io::IoSlice::new(b"ET")]),
                Poll::Ready(Ok(2))
            ));
            assert_eq!(stream.inner.sent, b"GET");
            let head = evidence.lock().unwrap();
            assert!(head.sent);
            assert!(head.refusal.is_none());
        }

        #[test]
        fn an_elapsed_deadline_prevents_any_transport_write() {
            let (mut stream, _, _, evidence) = guarded(vec![Write::Positive(3)], Instant::now());
            let mut cx = Context::from_waker(std::task::Waker::noop());
            assert!(matches!(
                Pin::new(&mut stream).poll_write(&mut cx, b"GET"),
                Poll::Ready(Err(_))
            ));
            assert_eq!(stream.inner.polls, 0);
            assert!(matches!(
                evidence.lock().unwrap().refusal,
                Some(Error::Expired("connect"))
            ));
        }

        #[test]
        fn an_unsupported_handshake_cost_does_not_charge_any_owner() {
            let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
            let cost = crate::core::Cost {
                orders10: 1,
                ws_weight: 2,
                ..Default::default()
            };
            let result = budgets.admit_handshake(cost, ADMITTED);
            assert_eq!(budgets.usage(ADMITTED).unwrap().request_weight.used, 0);
            assert!(
                budgets
                    .account_usage(ADMITTED)
                    .unwrap()
                    .orders
                    .iter()
                    .all(|window| window.used == 0)
            );
            assert!(matches!(
                result,
                Err(Error::Configuration("unsupported WebSocket handshake cost"))
            ));
        }
    }
}
