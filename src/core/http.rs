// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{
    Budgets, Clock, Credentials, Error, Operation, Outcome, RateEvidence, Request, Response,
    ResponseMeta, Security, request,
};
use serde::de::DeserializeOwned;
use std::{sync::Arc, time::Duration};
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
    ) -> Result<serde_json::Value, Error> {
        if let Some((_, decode)) = self.binary {
            if binary_content {
                decode(body)
            } else if !(200..300).contains(&status) {
                serde_json::from_slice(body)
                    .map_err(|_| Error::Gap("malformed SBE negotiation error"))
            } else {
                Err(Error::Gap("unexpected SBE response content type"))
            }
        } else {
            serde_json::from_slice(body).map_err(|_| Error::Gap("malformed JSON response"))
        }
    }
    fn prepare<R: Request>(
        &self,
        op: Operation,
        request: &R,
        deadline: Instant,
    ) -> Result<reqwest::Request, Error> {
        if op.security == Security::Unresolved {
            return Err(Error::Configuration(
                "endpoint authentication contract unresolved",
            ));
        }
        let mut params = request::parameters(request)?;
        let timestamp = self.clock.now_millis()?;
        (op.validate_time)(&params, timestamp)?;
        request.validate_authority(timestamp)?;
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
            let payload = request::encode(&params)?;
            let signature = credentials
                .ok_or(Error::CredentialsRequired)?
                .sign(&payload)?;
            params.insert("signature".into(), signature.into());
        }
        let mut url = self.base.clone();
        url.set_path(op.path);
        let query = request::encode(&params)?;
        // Signature is required last, regardless of alphabetical parameter ordering.
        let query = if let Some(signature) = params.get("signature").cloned() {
            params.remove("signature");
            let mut query = request::encode(&params)?;
            let mut form = url::form_urlencoded::Serializer::new(String::new());
            form.append_pair("signature", signature.as_str().ok_or(Error::Signing)?);
            query.push('&');
            query.push_str(&form.finish());
            query
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
        request.validate()?;
        let client_order_ids = request::order_ids(&request::parameters(request)?);
        let op = R::OP;
        let wire = self.prepare(op, request, deadline)?;
        let now = self.clock.now_millis()?;
        let cost = request.cost()?;
        self.budgets.admit(cost, now)?;
        let authority_time = self.clock.now_millis()?;
        (op.validate_time)(&request::parameters(request)?, authority_time)?;
        request.validate_authority(authority_time)?;
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
                meta: None,
            })?
            .map_err(|_| Error::Transport {
                client_order_ids: client_order_ids.clone(),
                operation: op.name,
                outcome,
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
        // A clock failure after complete response headers must not cost the venue's
        // answer, so the earlier reading of this same attempt is reused. It is a real
        // local reading, never an invented epoch, and it only ever ages a counter
        // bucket by the elapsed request time.
        let observed_at = self.clock.now_millis().unwrap_or(now);
        self.budgets
            .observe_cost(cost, &meta.rates, observed_at, status)
            .map_err(|_| Error::Transport {
                client_order_ids: client_order_ids.clone(),
                operation: op.name,
                outcome,
                meta: Some(Box::new(meta.clone())),
            })?;
        let body = tokio::time::timeout_at(attempt_deadline, response.bytes())
            .await
            .map_err(|_| Error::Transport {
                client_order_ids: client_order_ids.clone(),
                operation: op.name,
                outcome,
                meta: Some(Box::new(meta.clone())),
            })?
            .map_err(|_| Error::Transport {
                client_order_ids: client_order_ids.clone(),
                operation: op.name,
                outcome,
                meta: Some(Box::new(meta.clone())),
            })?;
        let decoded = if R::EMPTY_RESPONSE && (200..300).contains(&status) && body.is_empty() {
            // An explicitly documented no-data receipt, not recovery from a decode
            // failure. Nonempty malformed bodies and every error still fail normally.
            Ok(serde_json::json!({}))
        } else {
            self.decode_body(&body, status, binary_content)
        };
        let value = decoded.map_err(|_| Error::Transport {
            client_order_ids: client_order_ids.clone(),
            operation: op.name,
            outcome,
            meta: Some(Box::new(meta.clone())),
        })?;
        if (200..300).contains(&status)
            && value
                .get("code")
                .and_then(serde_json::Value::as_i64)
                .is_none_or(|c| c >= 0)
            && let Some(success) = op.success_weight
        {
            self.budgets
                .refund_weight(cost.weight.saturating_sub(success), now, false)?;
        }
        decode(op, status, value, meta)
    }
}

pub(crate) fn decode<T: DeserializeOwned>(
    op: Operation,
    status: u16,
    value: serde_json::Value,
    meta: ResponseMeta,
) -> Result<Response<T>, Error> {
    if !(200..300).contains(&status)
        || value
            .get("code")
            .and_then(serde_json::Value::as_i64)
            .is_some_and(|v| v < 0)
    {
        return Err(super::error::failure_for(op, status, &value, meta.rates)
            .with_order_ids(meta.client_order_ids));
    }
    let data = serde_json::from_value(value).map_err(|_| Error::Transport {
        client_order_ids: meta.client_order_ids.clone(),
        operation: op.name,
        outcome: if op.mutation {
            Outcome::Unknown
        } else {
            Outcome::ReadFailed
        },
        meta: Some(Box::new(meta.clone())),
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
