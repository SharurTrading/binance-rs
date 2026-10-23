// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! One caller-owned position-mode exchange, guarded outside TLS before HTTP writes.

use crate::core::http::{HttpClient, header_rates, transport_after_response};
use crate::core::{
    Budgets, Clock, Cost, Error, Outcome, PositionModeControl, Request, Response, ResponseMeta,
    TransportFailure, request,
};
use http_body_util::BodyExt as _;
use hyper_util::rt::TokioIo;
use std::{
    collections::BTreeMap,
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
    time::Instant,
};

#[derive(Default)]
struct Evidence {
    attempted: bool,
    refusal: Option<Error>,
}

struct Guard {
    budgets: Budgets,
    clock: Arc<dyn Clock>,
    charge: crate::core::rate::HandshakeCharge,
    deadline: Instant,
    operation: &'static str,
}

impl Guard {
    fn validate(&self) -> Result<(), Error> {
        if Instant::now() >= self.deadline {
            return Err(Error::Expired(self.operation));
        }
        self.budgets
            .validate_handshake_charge(&self.charge, self.clock.now_millis()?)
    }
}

struct Guarded<S> {
    inner: S,
    guard: Option<Guard>,
    evidence: Arc<Mutex<Evidence>>,
}

impl<S> Guarded<S> {
    fn before_write(&mut self) -> io::Result<()> {
        let Some(guard) = &self.guard else {
            return Ok(());
        };
        let mut evidence = self
            .evidence
            .lock()
            .map_err(|_| io::Error::other("HTTP admission synchronization failed"))?;
        if evidence.refusal.is_some() {
            return Err(io::Error::other("HTTP admission refused"));
        }
        if let Err(error) = guard.validate() {
            evidence.refusal = Some(error);
            return Err(io::Error::other("HTTP admission refused"));
        }
        Ok(())
    }

    fn after_write(&mut self, result: &Poll<io::Result<usize>>) -> io::Result<()> {
        if matches!(result, Poll::Ready(Ok(size)) if *size > 0)
            || matches!(result, Poll::Ready(Err(_)))
        {
            // A TLS writer can emit ciphertext and then return an error.
            self.evidence
                .lock()
                .map_err(|_| io::Error::other("HTTP admission synchronization failed"))?
                .attempted = true;
            self.guard = None;
        }
        Ok(())
    }
}

impl<S: AsyncRead + Unpin> AsyncRead for Guarded<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buffer)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for Guarded<S> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.before_write()?;
        let result = Pin::new(&mut self.inner).poll_write(cx, buffer);
        self.after_write(&result)?;
        result
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buffers: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        self.before_write()?;
        let result = Pin::new(&mut self.inner).poll_write_vectored(cx, buffers);
        self.after_write(&result)?;
        result
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

trait Transport: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Transport for T {}
type OwnedTransport = Box<dyn Transport>;

impl HttpClient {
    pub(crate) async fn execute_position_mode_control<R: PositionModeControl>(
        &self,
        request: &R,
        deadline: Instant,
    ) -> Result<Response<R::Response>, Error> {
        self.send_mode_control(request, deadline, |url| self.mode_transport(url))
            .await
    }

    async fn send_mode_control<R, F, Fut>(
        &self,
        request: &R,
        deadline: Instant,
        setup: F,
    ) -> Result<Response<R::Response>, Error>
    where
        R: PositionModeControl,
        F: FnOnce(url::Url) -> Fut,
        Fut: std::future::Future<Output = Result<OwnedTransport, Error>>,
    {
        if self.explicit_proxy {
            return Err(Error::Configuration(
                "position-mode recovery transport does not support an explicit proxy",
            ));
        }
        let mut params = request::parameters(request)?;
        request.validate_with_parameters(&params)?;
        let op = R::OP;
        let wire = self.prepare(op, request, &mut params, deadline)?;
        let cost = Cost {
            exit: crate::core::rate::reserve::ExitEligibility::Eligible,
            ..request.cost_with_parameters(&params)?
        };
        let now = self.clock.now_millis()?;
        let charge = self.budgets.admit_precharged(cost, now, op.name)?;
        let attempt_deadline = Instant::now()
            .checked_add(self.timeout)
            .ok_or(Error::Configuration("HTTP timeout range"))?
            .min(deadline);
        let evidence = Arc::new(Mutex::new(Evidence::default()));
        let guard = Guard {
            budgets: self.budgets.clone(),
            clock: Arc::clone(&self.clock),
            charge,
            deadline: attempt_deadline,
            operation: op.name,
        };
        let attempt = async {
            let transport = tokio::time::timeout_at(attempt_deadline, setup(wire.url().clone()))
                .await
                .map_err(|_| {
                    mode_failure(
                        op.name,
                        Outcome::NotSent,
                        TransportFailure::ResponseHeadersTimeout,
                    )
                })??;
            if Instant::now() >= attempt_deadline {
                return Err(Error::Expired(op.name));
            }
            self.mode_exchange::<R>(
                wire,
                Guarded {
                    inner: transport,
                    guard: Some(guard),
                    evidence: Arc::clone(&evidence),
                },
                cost,
                attempt_deadline,
            )
            .await
        };
        let result = attempt.await;
        let mut evidence = evidence
            .lock()
            .map_err(|_| Error::Configuration("HTTP admission synchronization failed"))?;
        if let Some(refusal) = evidence.refusal.take() {
            return Err(refusal);
        }
        let outcome = if evidence.attempted {
            Outcome::Unknown
        } else {
            Outcome::NotSent
        };
        match result {
            Ok(response) => Ok(response),
            Err(Error::Transport {
                meta: None,
                failure: Some(failure),
                ..
            }) => Err(mode_failure(op.name, outcome, failure)),
            Err(error) => Err(error),
        }
    }

    async fn mode_transport(&self, url: url::Url) -> Result<OwnedTransport, Error> {
        let host = url
            .host_str()
            .ok_or(Error::Configuration("HTTP host"))?
            .trim_start_matches('[')
            .trim_end_matches(']');
        let port = url
            .port_or_known_default()
            .ok_or(Error::Configuration("HTTP port"))?;
        let stream = TcpStream::connect((host, port)).await.map_err(|_| {
            mode_failure(
                "changePositionMode",
                Outcome::NotSent,
                TransportFailure::ResponseHeadersFailed,
            )
        })?;
        match url.scheme() {
            "http" => Ok(Box::new(stream)),
            "https" => {
                let name = rustls::pki_types::ServerName::try_from(host.to_owned())
                    .map_err(|_| Error::Configuration("TLS server name"))?;
                let stream = tokio_rustls::TlsConnector::from(Arc::clone(&self.tls))
                    .connect(name, stream)
                    .await
                    .map_err(|_| {
                        mode_failure(
                            "changePositionMode",
                            Outcome::NotSent,
                            TransportFailure::ResponseHeadersFailed,
                        )
                    })?;
                Ok(Box::new(stream))
            }
            _ => Err(Error::Configuration("HTTP scheme")),
        }
    }

    async fn mode_exchange<R: Request>(
        &self,
        mut wire: reqwest::Request,
        stream: Guarded<OwnedTransport>,
        cost: Cost,
        deadline: Instant,
    ) -> Result<Response<R::Response>, Error> {
        let op = R::OP;
        let authority = &wire.url()[url::Position::BeforeHost..url::Position::AfterPort];
        let target = &wire.url()[url::Position::BeforePath..url::Position::AfterQuery];
        let mut request = hyper::Request::builder()
            .method(wire.method().clone())
            .uri(target)
            .header(hyper::header::HOST, authority)
            .body(
                wire.body_mut()
                    .take()
                    .ok_or(Error::Configuration("HTTP form body"))?,
            )
            .map_err(|_| Error::Configuration("HTTP request"))?;
        request.headers_mut().extend(wire.headers().clone());
        let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|_| {
                mode_failure(
                    op.name,
                    Outcome::NotSent,
                    TransportFailure::ResponseHeadersFailed,
                )
            })?;
        let exchange = async {
            let response = tokio::time::timeout_at(deadline, sender.send_request(request))
                .await
                .map_err(|_| {
                    mode_failure(
                        op.name,
                        Outcome::Unknown,
                        TransportFailure::ResponseHeadersTimeout,
                    )
                })?
                .map_err(|_| {
                    mode_failure(
                        op.name,
                        Outcome::Unknown,
                        TransportFailure::ResponseHeadersFailed,
                    )
                })?;
            let status = response.status().as_u16();
            let binary_content = response
                .headers()
                .get(hyper::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| v.split(';').next() == Some("application/sbe"));
            let meta = ResponseMeta {
                time_unit: self.time_unit,
                client_order_ids: BTreeMap::new(),
                status,
                operation: op.name,
                rates: header_rates(response.headers()),
            };
            let observed_at = crate::core::response_rates::observe(
                &self.budgets,
                self.clock.as_ref(),
                &meta,
                Outcome::Unknown,
                crate::core::response_rates::Transport::Http(cost),
            )?;
            let body = tokio::time::timeout_at(deadline, response.into_body().collect())
                .await
                .map_err(|_| {
                    transport_after_response(
                        &meta,
                        Outcome::Unknown,
                        TransportFailure::ResponseBodyTimeout,
                    )
                })?
                .map_err(|_| {
                    transport_after_response(
                        &meta,
                        Outcome::Unknown,
                        TransportFailure::ResponseBodyFailed,
                    )
                })?
                .to_bytes();
            let value = self.decode_received::<R>(&body, status, binary_content, &meta)?;
            self.decode_response(op, status, value, meta, observed_at)
        };
        tokio::pin!(exchange, connection);
        tokio::select! {
            biased;
            result = &mut exchange => result,
            _ = &mut connection => exchange.await,
        }
    }
}

fn mode_failure(operation: &'static str, outcome: Outcome, failure: TransportFailure) -> Error {
    Error::Transport {
        client_order_ids: BTreeMap::new(),
        operation,
        outcome,
        failure: Some(failure),
        meta: None,
    }
}

#[cfg(test)]
#[path = "http_mode_tests.rs"]
mod tests;
