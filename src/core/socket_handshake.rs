// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Response evidence for one caller-owned WebSocket upgrade attempt.

use crate::core::socket::SocketPolicy;
use crate::core::{Budgets, Clock, Error, Outcome, RateEvidence, ResponseMeta, TransportFailure};
use std::{
    collections::BTreeMap,
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
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

pub(crate) struct HandshakeStream {
    inner: MaybeTlsStream<TcpStream>,
    capture: Option<Arc<Mutex<Head>>>,
}

impl HandshakeStream {
    fn observe_write(&self, result: &Poll<io::Result<usize>>) -> io::Result<()> {
        if matches!(result, Poll::Ready(Ok(size)) if *size > 0)
            && let Some(capture) = &self.capture
        {
            capture
                .lock()
                .map_err(|_| io::Error::other("handshake evidence synchronization failed"))?
                .sent = true;
        }
        Ok(())
    }
}

impl AsyncRead for HandshakeStream {
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

impl AsyncWrite for HandshakeStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<io::Result<usize>> {
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
    clock: &dyn Clock,
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
    let capture = Arc::new(Mutex::new(Head::default()));
    let admitted_at = clock.now_millis()?;
    budgets.admit(policy.handshake, admitted_at)?;
    let attempt = async {
        let address = tokio_websockets::resolver::Gai.resolve(host, port).await?;
        let transport = TcpStream::connect(address).await?;
        let transport = connector.wrap(host, transport).await?;
        let stream = HandshakeStream {
            inner: transport,
            capture: Some(Arc::clone(&capture)),
        };
        builder.connect_on(stream).await.map(|(socket, _)| socket)
    };
    let result = tokio::time::timeout(timeout, attempt).await;
    let (meta, sent) = {
        let mut head = capture
            .lock()
            .map_err(|_| Error::Configuration("handshake evidence synchronization failed"))?;
        head.line.zeroize();
        head.phase = Phase::Done;
        (head.metadata(policy.time_unit), head.sent)
    };
    let outcome = if sent {
        Outcome::ReadFailed
    } else {
        Outcome::NotSent
    };
    if let Some(meta) = &meta {
        crate::core::response_rates::observe(
            budgets,
            clock,
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
}
