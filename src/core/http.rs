// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{
    Budgets, Clock, Cost, Credentials, Error, ExitRead, Operation, Outcome, RateEvidence,
    ReadStreamControl, RecoveryRead, Request, Response, ResponseMeta, Security, TransportFailure,
    request,
};
use serde::de::DeserializeOwned;
use std::{borrow::Borrow, collections::BTreeMap, sync::Arc, time::Duration};
use tokio::time::Instant;
type BinaryDecoder = fn(&[u8]) -> Result<serde_json::Value, Error>;

#[derive(Clone)]
pub(crate) struct HttpClient {
    time_unit: super::TimeUnit,
    client: reqwest::Client,
    base: url::Url,
    credentials: Option<Credentials>,
    clock: Arc<dyn Clock>,
    budgets: Budgets,
    timeout: Duration,
    binary: Option<(&'static str, BinaryDecoder)>,
}
impl HttpClient {
    pub fn new(
        base: url::Url,
        credentials: Option<Credentials>,
        clock: Arc<dyn Clock>,
        budgets: Budgets,
        timeout: Duration,
        proxy: Option<reqwest::Proxy>,
    ) -> Result<Self, Error> {
        let mut builder = reqwest::Client::builder()
            .tls_backend_preconfigured(super::socket::tls_config()?)
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            // Pooled keep-alive safety, named for the code that enforces it.
            // hyper-util's resend loop (legacy client; `retry_canceled_requests`
            // defaults to true, gated on `connection_reused`) re-sends only a
            // request that hyper returned unstarted: never handed to the
            // connection's encoder, so not even a partial write reached the
            // venue. Venue-visible invariant: a mutation is delivered to the
            // venue at most once; a failure before the first byte reaches the
            // socket may be re-delivered transparently, and the venue cannot
            // observe the first attempt. A request the venue received fails
            // visibly. http_contract pins both directions against drift.
            .http1_only()
            // Restates reqwest's default retention window, kept explicit where
            // the previous policy overrode its sibling; venue-side idle closes
            // are governed by the semantics above, not by this timeout. The
            // per-host idle ceiling is a resource bound for sequential
            // reconcile reads, not a correctness limit.
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(4)
            .timeout(timeout)
            .connection_verbose(false);
        if let Some(proxy) = proxy {
            builder = builder.proxy(proxy);
        }
        let client = builder
            .build()
            .map_err(|_| Error::Configuration("HTTP client"))?;
        Ok(Self {
            time_unit: super::TimeUnit::Milliseconds,
            client,
            base,
            credentials,
            clock,
            budgets,
            timeout,
            binary: None,
        })
    }
    /// Adopt the limits a venue reply states for this client's IP pool.
    pub(crate) fn adopt_stated_limits<'a>(
        &self,
        stated: impl IntoIterator<Item = super::StatedLimit<'a>>,
    ) -> Result<(), Error> {
        self.budgets.adopt_stated(stated)
    }
    /// The IP windows of this client's pool, read at the client's clock.
    pub(crate) fn pool_usage(&self) -> Result<super::PoolUsage, Error> {
        self.budgets.usage(self.clock.now_millis()?)
    }
    /// The account order windows at this client's clock.
    pub(crate) fn account_usage(&self) -> Result<super::AccountUsage, Error> {
        self.budgets.account_usage(self.clock.now_millis()?)
    }
    /// Clear the pool's unknown-timing ban; see [`super::Budgets::release_unknown_ban`].
    pub(crate) fn release_unknown_ban(&self) -> Result<(), Error> {
        self.budgets.release_unknown_ban()
    }
    /// The venue pool this client draws on; `None` for an explicit owner.
    pub(crate) fn pool_key(&self) -> Option<super::PoolKey> {
        self.budgets.pool_key()
    }
    pub(crate) fn time_unit(mut self, unit: super::TimeUnit) -> Self {
        self.time_unit = unit;
        self
    }
    pub(crate) fn binary_responses(mut self, schema: &'static str, decode: BinaryDecoder) -> Self {
        self.binary = Some((schema, decode));
        self
    }
    fn decode_body(
        &self,
        body: &[u8],
        status: u16,
        binary_content: bool,
    ) -> Result<serde_json::Value, TransportFailure> {
        if let Some((_, decode)) = self.binary {
            if binary_content {
                decode(body).map_err(|_| TransportFailure::BinaryDecodeFailed)
            } else if !(200..300).contains(&status) {
                serde_json::from_slice(body).map_err(|_| TransportFailure::JsonDecodeFailed)
            } else {
                Err(TransportFailure::UnexpectedContentType)
            }
        } else {
            serde_json::from_slice(body).map_err(|_| TransportFailure::JsonDecodeFailed)
        }
    }
    fn prepare<R: Request>(
        &self,
        op: Operation,
        request: &R,
        params: &mut BTreeMap<String, serde_json::Value>,
        deadline: Instant,
    ) -> Result<reqwest::Request, Error> {
        let timestamp = self.clock.now_millis()?;
        (op.validate_time)(params, timestamp)?;
        request.validate_authority_with_parameters(params, timestamp)?;
        if Instant::now() >= deadline {
            return Err(Error::Expired(op.name));
        }
        let credentials = if op.security == Security::Public {
            None
        } else {
            Some(
                self.credentials
                    .as_ref()
                    .ok_or(Error::CredentialsRequired)?,
            )
        };
        if op.security == Security::Signed {
            params.insert(
                "timestamp".into(),
                self.time_unit.timestamp(self.clock.as_ref())?.into(),
            );
        }
        let mut url = self.base.clone();
        url.set_path(op.path);
        let query = request::encode(params)?;
        // Append to the exact buffer the signer saw; signature must be last.
        let query = if op.security == Security::Signed {
            let signature = credentials
                .ok_or(Error::CredentialsRequired)?
                .sign(&query)?;
            let mut form = url::form_urlencoded::Serializer::new(query);
            form.append_pair("signature", &signature);
            form.finish()
        } else {
            query
        };
        let method = reqwest::Method::from_bytes(op.method.as_bytes())
            .map_err(|_| Error::Configuration("HTTP method"))?;
        let mut builder = self.client.request(method, url);
        if let Some((schema, _)) = self.binary {
            builder = builder
                .header("Accept", "application/sbe")
                .header("X-MBX-SBE", schema);
        }
        if self.time_unit == super::TimeUnit::Microseconds {
            builder = builder.header("X-MBX-TIME-UNIT", "MICROSECOND");
        }
        if let Some(credentials) = credentials {
            builder = builder.header("X-MBX-APIKEY", credentials.header()?);
        }
        let mut wire = builder
            .build()
            .map_err(|_| Error::Configuration("HTTP request"))?;
        if op.method == "GET" {
            wire.url_mut().set_query(Some(&query));
        } else {
            wire.headers_mut().insert(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
            *wire.body_mut() = Some(query.into());
        }
        Ok(wire)
    }
    pub async fn execute<R: Request>(
        &self,
        request: &R,
        deadline: Instant,
    ) -> Result<Response<R::Response>, Error> {
        self.send(request, deadline, Admission::Priced).await
    }

    /// Send an exit's read, admitted from the exit reserve.
    pub async fn execute_exit_read<R: ExitRead>(
        &self,
        request: &R,
        deadline: Instant,
    ) -> Result<Response<R::Response>, Error> {
        self.send(request, deadline, Admission::ExitRead).await
    }

    /// Send a bounded account-recovery read, admitted from reserved capacity.
    pub async fn execute_recovery_read<R: RecoveryRead>(
        &self,
        request: &R,
        deadline: Instant,
    ) -> Result<Response<R::Response>, Error> {
        if R::OP.mutation {
            return Err(Error::Configuration("recovery request must be read-only"));
        }
        self.send(request, deadline, Admission::RecoveryRead).await
    }

    /// Send an explicitly selected account-event stream lifecycle control.
    pub async fn execute_read_stream_control<R: ReadStreamControl>(
        &self,
        request: &R,
        deadline: Instant,
    ) -> Result<Response<R::Response>, Error> {
        self.send(request, deadline, Admission::ReadStreamControl)
            .await
    }

    async fn send<R: Request>(
        &self,
        request: &R,
        deadline: Instant,
        admission: Admission,
    ) -> Result<Response<R::Response>, Error> {
        let mut params = request::parameters(request)?;
        request.validate_with_parameters(&params)?;
        let client_order_ids = request::order_ids(&params);
        let op = R::OP;
        let wire = self.prepare(op, request, &mut params, deadline)?;
        let now = self.clock.now_millis()?;
        let cost = admission.applied(request.cost_with_parameters(&params)?);
        self.budgets.admit(cost, now)?;
        let authority_time = self.clock.now_millis()?;
        (op.validate_time)(&params, authority_time)?;
        request.validate_authority_with_parameters(&params, authority_time)?;
        if Instant::now() >= deadline {
            return Err(Error::Expired(op.name));
        }
        let outcome = if op.mutation {
            Outcome::Unknown
        } else {
            Outcome::ReadFailed
        };
        let timeout = self
            .timeout
            .min(deadline.saturating_duration_since(Instant::now()));
        let attempt_deadline = Instant::now() + timeout;
        let response = tokio::time::timeout_at(attempt_deadline, self.client.execute(wire))
            .await
            .map_err(|_| Error::Transport {
                client_order_ids: client_order_ids.clone(),
                operation: op.name,
                outcome,
                failure: Some(TransportFailure::ResponseHeadersTimeout),
                meta: None,
            })?
            .map_err(|error| Error::Transport {
                client_order_ids: client_order_ids.clone(),
                operation: op.name,
                outcome,
                failure: Some(if error.is_timeout() {
                    TransportFailure::ResponseHeadersTimeout
                } else {
                    TransportFailure::ResponseHeadersFailed
                }),
                meta: None,
            })?;
        let status = response.status().as_u16();
        let binary_content = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.split(';').next() == Some("application/sbe"));
        let rates = header_rates(response.headers());
        let meta = ResponseMeta {
            time_unit: self.time_unit,
            client_order_ids: client_order_ids.clone(),
            status,
            operation: op.name,
            rates,
        };
        let observed_at = crate::core::response_rates::observe(
            &self.budgets,
            self.clock.as_ref(),
            &meta,
            outcome,
            crate::core::response_rates::Transport::Http(cost),
        )?;
        let body = tokio::time::timeout_at(attempt_deadline, response.bytes())
            .await
            .map_err(|_| {
                transport_after_response(&meta, outcome, TransportFailure::ResponseBodyTimeout)
            })?
            .map_err(|error| {
                let failure = if error.is_timeout() {
                    TransportFailure::ResponseBodyTimeout
                } else {
                    TransportFailure::ResponseBodyFailed
                };
                transport_after_response(&meta, outcome, failure)
            })?;
        let value = self.decode_received::<R>(&body, status, binary_content, &meta)?;
        if (200..300).contains(&status)
            && value
                .get("code")
                .and_then(serde_json::Value::as_i64)
                .is_none_or(|c| c >= 0)
            && let Some(success) = op.success_weight
        {
            self.budgets
                .refund_weight(cost.weight.saturating_sub(success), now, false)
                .map_err(|_| {
                    transport_after_response(&meta, outcome, TransportFailure::RateEvidenceFailed)
                })?;
        }
        self.decode_response(op, status, value, meta, observed_at)
    }

    /// Decode only documented empty bodies and product-local service envelopes.
    fn decode_received<R: Request>(
        &self,
        body: &[u8],
        status: u16,
        binary_content: bool,
        meta: &ResponseMeta,
    ) -> Result<serde_json::Value, Error> {
        let op = R::OP;
        let outcome = if op.mutation {
            Outcome::Unknown
        } else {
            Outcome::ReadFailed
        };
        let decoded = if R::EMPTY_RESPONSE && (200..300).contains(&status) && body.is_empty() {
            // An explicitly documented no-data receipt, not recovery from a decode
            // failure. Nonempty malformed bodies and every error still fail normally.
            Ok(if R::EMPTY_RESPONSE_NULL {
                serde_json::Value::Null
            } else {
                serde_json::json!({})
            })
        } else {
            self.decode_body(body, status, binary_content)
        };
        let value = decoded.map_err(|failure| transport_after_response(meta, outcome, failure))?;
        if R::response_failed(&value) {
            return Err(
                super::error::failure_for(op, status, &value, meta.rates.clone())
                    .with_order_ids(meta.client_order_ids.clone()),
            );
        }
        Ok(value)
    }

    /// Account query evidence is validated before changing the owner's limits/counts.
    fn decode_response<T: DeserializeOwned>(
        &self,
        op: Operation,
        status: u16,
        value: serde_json::Value,
        meta: ResponseMeta,
        observed_at: u64,
    ) -> Result<Response<T>, Error> {
        if op.path == "/papi/v1/rateLimit/order" && (200..300).contains(&status) {
            let mut response: Response<T> = decode(op, status, &value, meta)?;
            if self.budgets.adopt_portfolio_order_limits(&value).is_err() {
                response.meta.rates.stated_limits_unusable = true;
                return Err(transport_after_response(
                    &response.meta,
                    Outcome::ReadFailed,
                    TransportFailure::RateEvidenceFailed,
                ));
            }
            return Ok(response);
        }
        if op.path == "/api/v3/rateLimit/order" && (200..300).contains(&status) {
            let mut response: Response<T> = decode(op, status, &value, meta)?;
            let mut query_rates = RateEvidence::default();
            let adoption =
                super::socket::order_query_rates(&value, &mut query_rates).and_then(|()| {
                    self.budgets
                        .adopt_response_limits(None, Some(&value), false)
                });
            if adoption.is_err() {
                response.meta.rates.stated_limits_unusable = true;
                return Err(transport_after_response(
                    &response.meta,
                    Outcome::ReadFailed,
                    TransportFailure::RateEvidenceFailed,
                ));
            }
            for (name, count) in query_rates.counters {
                let current = response.meta.rates.counters.entry(name).or_default();
                *current = (*current).max(count);
            }
            self.observe_query_rates(&response.meta, observed_at)?;
            return Ok(response);
        }
        decode(op, status, value, meta)
    }

    pub(crate) fn observe_query_rates(
        &self,
        meta: &ResponseMeta,
        observed_at: u64,
    ) -> Result<(), Error> {
        self.budgets
            .observe(&meta.rates, observed_at, false)
            .map_err(|_| {
                transport_after_response(
                    meta,
                    Outcome::ReadFailed,
                    TransportFailure::RateEvidenceFailed,
                )
            })
    }
}

/// How one attempt is admitted against the exit reserve.
#[derive(Clone, Copy)]
enum Admission {
    /// As the request's own cost classifies it.
    Priced,
    /// From the exit reserve: a read an exit makes, chosen by its caller.
    ExitRead,
    /// From reserved capacity: an explicitly selected account-recovery read.
    RecoveryRead,
    /// From reserved capacity: a sealed nonfinancial stream lifecycle control.
    ReadStreamControl,
}

impl Admission {
    fn applied(self, cost: Cost) -> Cost {
        match self {
            Self::Priced => cost,
            Self::ExitRead | Self::RecoveryRead | Self::ReadStreamControl => Cost {
                exit: super::rate::reserve::ExitEligibility::Eligible,
                ..cost
            },
        }
    }
}

/// Preserve received response evidence when a decoded limit statement is unusable.
pub(crate) fn with_stated_limit_evidence<T>(
    mut response: Response<T>,
    adoption: Result<(), Error>,
) -> Result<Response<T>, Error> {
    adoption.map_err(|_| {
        response.meta.rates.stated_limits_unusable = true;
        transport_after_response(
            &response.meta,
            Outcome::ReadFailed,
            TransportFailure::RateEvidenceFailed,
        )
    })?;
    Ok(response)
}

pub(crate) fn transport_after_response(
    meta: &ResponseMeta,
    outcome: Outcome,
    failure: TransportFailure,
) -> Error {
    Error::Transport {
        client_order_ids: meta.client_order_ids.clone(),
        operation: meta.operation,
        outcome,
        failure: Some(failure),
        meta: Some(Box::new(meta.clone())),
    }
}

pub(crate) fn decode<'de, T: DeserializeOwned>(
    op: Operation,
    status: u16,
    value: impl Borrow<serde_json::Value> + serde::Deserializer<'de, Error = serde_json::Error>,
    meta: ResponseMeta,
) -> Result<Response<T>, Error> {
    if !(200..300).contains(&status)
        || value
            .borrow()
            .get("code")
            .and_then(serde_json::Value::as_i64)
            .is_some_and(|v| v < 0)
    {
        return Err(
            super::error::failure_for(op, status, value.borrow(), meta.rates)
                .with_order_ids(meta.client_order_ids),
        );
    }
    let data = T::deserialize(value).map_err(|_| {
        let outcome = if op.mutation {
            Outcome::Unknown
        } else {
            Outcome::ReadFailed
        };
        transport_after_response(&meta, outcome, TransportFailure::ResponseDecodeFailed)
    })?;
    Ok(Response { data, meta })
}

fn header_rates(headers: &reqwest::header::HeaderMap) -> RateEvidence {
    let mut evidence = RateEvidence::default();
    for (name, value) in headers {
        let name = name.as_str();
        // Venue counters are retained even when no budget window consumes them.
        // An interval-less counter is evidence, not a licence to assume an interval.
        let counted = name.starts_with("x-mbx-used-weight-")
            || name.starts_with("x-mbx-order-count-")
            || name.starts_with("x-sapi-used-")
            || name == "x-mbx-used-weight"
            || name == "x-mbx-order-count";
        if counted
            && let Ok(value) = value.to_str()
            && let Ok(value) = value.parse()
        {
            evidence.counters.insert(name.to_owned(), value);
        }
    }
    if let Some(value) = headers.get("retry-after") {
        evidence.retry_after = value
            .to_str()
            .ok()
            .and_then(|v| v.parse().ok())
            .map(Duration::from_secs);
        evidence.retry_after_unusable = evidence.retry_after.is_none();
    }
    evidence
}

#[cfg(test)]
#[path = "http_tests.rs"]
mod tests;
