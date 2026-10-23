// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

struct Attempt {
    serializations: AtomicUsize,
    authority_checks: AtomicUsize,
    validated_id: Mutex<Option<String>>,
}

impl Serialize for Attempt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let snapshot = self.serializations.fetch_add(1, Ordering::SeqCst) + 1;
        serde_json::json!({"symbol":"BTCUSDT", "newClientOrderId":format!("retained:{snapshot}")})
            .serialize(serializer)
    }
}

impl Request for Attempt {
    type Response = serde_json::Value;
    const OP: Operation = <crate::usdm::rest_requests::NewOrder as Request>::OP;

    fn validate_with_parameters(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
    ) -> Result<(), Error> {
        let id = params
            .get("newClientOrderId")
            .and_then(serde_json::Value::as_str)
            .ok_or(Error::Validation("snapshot identity absent"))?;
        *self
            .validated_id
            .lock()
            .map_err(|_| Error::Configuration("snapshot state poisoned"))? = Some(id.to_owned());
        Ok(())
    }

    fn validate_authority(&self, _now: u64) -> Result<(), Error> {
        if self.authority_checks.fetch_add(1, Ordering::SeqCst) == 0 {
            Ok(())
        } else {
            Err(Error::Validation("authority revoked before sending"))
        }
    }

    fn cost_with_parameters(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
    ) -> Result<Cost, Error> {
        self.check_snapshot(params, "cost used a different parameter snapshot")?;
        Ok(Cost::default())
    }

    fn validate_authority_with_parameters(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
        now: u64,
    ) -> Result<(), Error> {
        self.check_snapshot(params, "authority used a different parameter snapshot")?;
        self.validate_authority(now)
    }
}

impl Attempt {
    fn check_snapshot(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
        reason: &'static str,
    ) -> Result<(), Error> {
        let validated = self
            .validated_id
            .lock()
            .map_err(|_| Error::Configuration("snapshot state poisoned"))?;
        if validated.as_deref()
            == params
                .get("newClientOrderId")
                .and_then(serde_json::Value::as_str)
        {
            Ok(())
        } else {
            Err(Error::Validation(reason))
        }
    }
}

struct RecordingSigner(Arc<Mutex<Option<String>>>);
impl super::super::Signer for RecordingSigner {
    fn sign(&self, payload: &[u8]) -> Result<String, Error> {
        let payload = std::str::from_utf8(payload).map_err(|_| Error::Signing)?;
        *self.0.lock().map_err(|_| Error::Signing)? = Some(payload.to_owned());
        Ok("fixture+/=".into())
    }
}

#[tokio::test]
async fn one_parameter_snapshot_survives_signing_and_the_authority_recheck() -> Result<(), Error> {
    let attempt = Attempt {
        serializations: AtomicUsize::new(0),
        authority_checks: AtomicUsize::new(0),
        validated_id: Mutex::new(None),
    };
    let signed = Arc::new(Mutex::new(None));
    let client = HttpClient::new(
        super::super::validate_url("http://127.0.0.1:9", false)?,
        Some(Credentials::external(
            "fixture-key",
            Arc::new(RecordingSigner(Arc::clone(&signed))),
        )?),
        Arc::new(super::super::SystemClock),
        Budgets::new(super::super::BudgetLimits::usdm())?,
        Duration::from_secs(1),
        None,
    )?;
    let result = client
        .execute(&attempt, Instant::now() + Duration::from_secs(1))
        .await;
    assert!(
        matches!(
            &result,
            Err(Error::Validation("authority revoked before sending"))
        ),
        "{result:?}"
    );
    assert_eq!(attempt.authority_checks.load(Ordering::SeqCst), 2);
    assert_eq!(attempt.serializations.load(Ordering::SeqCst), 1);
    let signed = signed.lock().map_err(|_| Error::Signing)?;
    assert!(signed.as_deref().is_some_and(|payload| {
        url::form_urlencoded::parse(payload.as_bytes())
            .any(|(name, value)| name == "newClientOrderId" && value == "retained:1")
    }));
    Ok(())
}

impl RecoveryRead for Attempt {}

#[tokio::test]
async fn recovery_dispatch_refuses_a_mutation_before_encoding_or_sending() -> Result<(), Error> {
    let attempt = Attempt {
        serializations: AtomicUsize::new(0),
        authority_checks: AtomicUsize::new(0),
        validated_id: Mutex::new(None),
    };
    let client = HttpClient::new(
        super::super::validate_url("http://127.0.0.1:9", false)?,
        None,
        Arc::new(super::super::SystemClock),
        Budgets::new(super::super::BudgetLimits::usdm())?,
        Duration::from_secs(1),
        None,
    )?;
    let result = client
        .execute_recovery_read(&attempt, Instant::now() + Duration::from_secs(1))
        .await;
    assert!(
        matches!(
            result,
            Err(Error::Configuration("recovery request must be read-only"))
        ),
        "{result:?}"
    );
    assert_eq!(attempt.serializations.load(Ordering::SeqCst), 0);
    assert_eq!(attempt.authority_checks.load(Ordering::SeqCst), 0);
    Ok(())
}

struct FixedWallClock;

impl Clock for FixedWallClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(1_700_000_001_000)
    }
}

struct StalledResponse {
    url: url::Url,
    received: tokio::sync::oneshot::Receiver<Vec<u8>>,
    release: tokio_util::sync::CancellationToken,
    task: tokio::task::JoinHandle<()>,
}

impl StalledResponse {
    async fn new(response_headers: Option<&'static str>) -> Self {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = url::Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let (observed, received) = tokio::sync::oneshot::channel();
        let release = tokio_util::sync::CancellationToken::new();
        let stop = release.clone();
        let task = tokio::spawn(async move {
            let (mut stream, _) = tokio::select! {
                () = stop.cancelled() => return,
                accepted = listener.accept() => accepted.unwrap(),
            };
            let mut wire = Vec::new();
            let mut buffer = [0; 4096];
            loop {
                let size = tokio::select! {
                    () = stop.cancelled() => return,
                    read = stream.read(&mut buffer) => read.unwrap(),
                };
                assert!(size > 0);
                wire.extend_from_slice(&buffer[..size]);
                if let Some(end) = wire.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = std::str::from_utf8(&wire[..end]).unwrap();
                    let content_length = headers
                        .lines()
                        .find_map(|line| {
                            line.to_ascii_lowercase()
                                .strip_prefix("content-length: ")
                                .map(|length| length.parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if wire.len() >= end + 4 + content_length {
                        break;
                    }
                }
            }
            if let Some(headers) = response_headers {
                tokio::select! {
                    () = stop.cancelled() => return,
                    written = stream.write_all(headers.as_bytes()) => written.unwrap(),
                }
            }
            observed.send(wire).unwrap();
            stop.cancelled().await;
        });
        Self {
            url,
            received,
            release,
            task,
        }
    }

    async fn finish(self) {
        self.release.cancel();
        self.task.await.unwrap();
    }
}

fn assert_request_operation<R: Request>(wire: &[u8]) {
    let request_line = std::str::from_utf8(wire).unwrap().lines().next().unwrap();
    let mut parts = request_line.split_whitespace();
    assert_eq!(parts.next(), Some(R::OP.method));
    let target = parts.next().unwrap();
    assert_eq!(target.split('?').next(), Some(R::OP.path));
    assert_eq!(parts.next(), Some("HTTP/1.1"));
    assert_eq!(parts.next(), None);
}

async fn stalled_response_error<R: Request>(
    request: R,
    response_headers: Option<&'static str>,
    timeout: Duration,
    caller_lifetime: Duration,
    observe_after: Duration,
    sample_gap: Duration,
) -> (Error, super::super::PoolUsage)
where
    R::Response: std::fmt::Debug,
{
    use futures_util::FutureExt as _;

    let mut fixture = StalledResponse::new(response_headers).await;
    let client = (|| {
        HttpClient::new(
            fixture.url.clone(),
            Some(Credentials::hmac("synthetic-api-key", "synthetic-secret")?),
            Arc::new(FixedWallClock),
            Budgets::new(super::super::BudgetLimits::usdm())?,
            timeout,
            None,
        )
    })();
    let client = match client {
        Ok(client) => client,
        Err(error) => {
            fixture.finish().await;
            panic!("HTTP fixture client setup failed: {error:?}");
        }
    };
    let start = Instant::now();
    let deadline = start + caller_lifetime;
    let observe_at = start + observe_after;
    let mut sampled_at = start;
    let response = client.send_with_clock(&request, deadline, Admission::Priced, move || {
        let now = sampled_at;
        sampled_at = start + sample_gap;
        now
    });
    tokio::pin!(response);
    let setup: Result<Vec<u8>, String> = async {
        let wire = tokio::select! {
            result = &mut response => return Err(format!("response finished before the fixture received it: {result:?}")),
            wire = &mut fixture.received => wire.map_err(|error| format!("fixture request observation failed: {error:?}"))?,
        };
        if response_headers.is_some() {
            tokio::select! {
                result = &mut response => return Err(format!("response finished before headers were observed: {result:?}")),
                ready = async {
                    loop {
                        if client.pool_usage()?.request_weight.used == 22 {
                            return Ok::<(), Error>(());
                        }
                        tokio::task::yield_now().await;
                    }
                } => ready.map_err(|error| format!("response header observation failed: {error:?}"))?,
            }
        }
        let ready_at = Instant::now();
        if ready_at >= deadline {
            return Err("fixture readiness exceeded the original caller deadline".into());
        }
        if ready_at >= observe_at - Duration::from_millis(1) {
            return Err("fixture readiness exceeded the expected response bound".into());
        }
        Ok(wire)
    }
    .await;
    let wire = match setup {
        Ok(wire) => wire,
        Err(error) => {
            fixture.finish().await;
            panic!("{error}");
        }
    };
    tokio::time::pause();
    let observation_wait = observe_at.checked_duration_since(Instant::now());
    let Some(observation_wait) = observation_wait else {
        fixture.finish().await;
        panic!("observation target passed before advancing controlled time");
    };
    // The due barrier lets the timer driver process millisecond-rounded deadlines.
    let timer_barrier = tokio::time::sleep_until(observe_at - Duration::from_millis(1));
    tokio::pin!(timer_barrier);
    if timer_barrier.as_mut().now_or_never().is_some() {
        fixture.finish().await;
        panic!("observation timer expired before advancing controlled time");
    }
    tokio::time::advance(observation_wait).await;
    let before_barrier = Instant::now();
    timer_barrier.await;
    let after_barrier = Instant::now();
    let observed = response.as_mut().now_or_never();
    let usage = client.pool_usage();
    fixture.finish().await;
    assert_eq!(
        before_barrier, observe_at,
        "observation clock drift before the timer barrier"
    );
    assert_eq!(
        after_barrier, observe_at,
        "observation clock drift after the timer barrier"
    );
    assert_request_operation::<R>(&wire);
    let usage = usage.unwrap();
    let error = observed
        .expect("the response must finish at the earlier absolute bound")
        .unwrap_err();
    (error, usage)
}

#[tokio::test]
async fn caller_deadline_between_clock_samples_bounds_response_headers() {
    let (error, _) = stalled_response_error(
        crate::usdm::rest_requests::CheckServerTime::new(),
        None,
        Duration::from_mins(1),
        Duration::from_secs(5),
        Duration::from_millis(5_001),
        Duration::from_secs(3),
    )
    .await;
    assert!(matches!(
        error,
        Error::Transport {
            operation: "checkServerTime",
            outcome: Outcome::ReadFailed,
            failure: Some(TransportFailure::ResponseHeadersTimeout),
            meta: None,
            ..
        }
    ));
}

fn timeout_order() -> crate::usdm::rest_requests::NewOrder {
    crate::usdm::rest_requests::NewOrder::new()
        .symbol(crate::Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(crate::Decimal::new(1, 3))
        .new_client_order_id(crate::ClientOrderId::new("fixture/deadline:1").unwrap())
}

const STALLED_BODY: &str = "HTTP/1.1 200 fixture\r\nConnection: close\r\nContent-Length: 100\r\nX-MBX-USED-WEIGHT-1M: 22\r\nRetry-After: 2\r\n\r\n{";

fn assert_body_timeout_evidence(error: Error, usage: super::super::PoolUsage) {
    let Error::Transport {
        client_order_ids,
        operation: "newOrder",
        outcome: Outcome::Unknown,
        failure: Some(TransportFailure::ResponseBodyTimeout),
        meta: Some(meta),
    } = error
    else {
        panic!("body timeout evidence was lost: {error:?}");
    };
    assert_eq!(client_order_ids["newClientOrderId"], "fixture/deadline:1");
    assert_eq!(
        meta.client_order_ids["newClientOrderId"],
        "fixture/deadline:1"
    );
    assert_eq!(meta.operation, "newOrder");
    assert_eq!(meta.status, 200);
    assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 22);
    assert_eq!(meta.rates.retry_after, Some(Duration::from_secs(2)));
    assert_eq!(usage.request_weight.used, 22);
}

#[tokio::test]
async fn caller_deadline_between_clock_samples_bounds_response_body_and_retains_evidence() {
    let (error, usage) = stalled_response_error(
        timeout_order(),
        Some(STALLED_BODY),
        Duration::from_mins(1),
        Duration::from_secs(5),
        Duration::from_millis(5_001),
        Duration::from_secs(3),
    )
    .await;
    assert_body_timeout_evidence(error, usage);
}

#[tokio::test]
async fn shorter_configured_timeout_bounds_response_headers() {
    let (error, _) = stalled_response_error(
        crate::usdm::rest_requests::CheckServerTime::new(),
        None,
        Duration::from_secs(2),
        Duration::from_secs(10),
        Duration::from_millis(2_001),
        Duration::ZERO,
    )
    .await;
    assert!(matches!(
        error,
        Error::Transport {
            operation: "checkServerTime",
            outcome: Outcome::ReadFailed,
            failure: Some(TransportFailure::ResponseHeadersTimeout),
            meta: None,
            ..
        }
    ));
}

#[tokio::test]
async fn shorter_configured_timeout_bounds_response_body_and_retains_evidence() {
    let (error, usage) = stalled_response_error(
        timeout_order(),
        Some(STALLED_BODY),
        Duration::from_secs(2),
        Duration::from_secs(10),
        Duration::from_millis(2_001),
        Duration::ZERO,
    )
    .await;
    assert_body_timeout_evidence(error, usage);
}
