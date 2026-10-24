// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use crate::core_trading::usdm::rest_requests::{ChangePositionMode, CheckServerTime};
use crate::{BudgetLimits, Credentials, ExitReserve};
use std::{
    collections::VecDeque,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const NOW: u64 = 659_000;
const OPERATION: &str = "changePositionMode";

struct ControlledClock(AtomicU64, AtomicBool);
impl Clock for ControlledClock {
    fn now_millis(&self) -> Result<u64, Error> {
        if self.1.swap(false, Ordering::SeqCst) {
            Err(Error::Configuration("synthetic mode clock unavailable"))
        } else {
            Ok(self.0.load(Ordering::SeqCst))
        }
    }
}

fn clock() -> Arc<ControlledClock> {
    Arc::new(ControlledClock(AtomicU64::new(NOW), AtomicBool::new(false)))
}

fn owner(url: url::Url, clock: Arc<ControlledClock>, budgets: Budgets) -> HttpClient {
    HttpClient::new(
        url,
        Some(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
        clock,
        budgets,
        Duration::from_secs(5),
        None,
    )
    .unwrap()
}

async fn listener() -> (TcpListener, url::Url) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = url::Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
    (listener, url)
}

async fn serve(listener: TcpListener, status: u16, headers: &str, body: &str) -> Vec<u8> {
    let (mut stream, _) = listener.accept().await.unwrap();
    let mut wire = Vec::new();
    let mut buffer = [0; 4096];
    while !wire.windows(4).any(|part| part == b"\r\n\r\n") {
        let size = stream.read(&mut buffer).await.unwrap();
        if size == 0 {
            return wire;
        }
        wire.extend_from_slice(&buffer[..size]);
    }
    let response = format!(
        "HTTP/1.1 {status} fixture\r\nConnection: close\r\nContent-Length: {}\r\n{headers}\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).await.unwrap();
    wire
}

async fn native_cooldown(budgets: Budgets, clock: Arc<ControlledClock>, status: u16) {
    let (listener, url) = listener().await;
    let client = owner(url, clock, budgets);
    let headers = if status == 429 {
        "Retry-After: 2\r\n"
    } else {
        ""
    };
    let request = CheckServerTime::new();
    let (result, wire) = tokio::join!(
        client.execute_recovery_read(&request, Instant::now() + Duration::from_secs(5)),
        serve(listener, status, headers, "{}")
    );
    assert!(matches!(result, Err(Error::Venue(ref error)) if error.status == status));
    assert!(wire.starts_with(b"GET /fapi/v1/time"));
}

#[derive(Clone, Copy, Debug)]
enum Change {
    TimedCooldown,
    UntimedBan,
    Window,
    Backwards,
    Clock,
    Deadline,
    Limit,
}

async fn held_setup(change: Change) {
    let clock = clock();
    let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(10_000))
        .unwrap();
    let (listener, url) = listener().await;
    let client = owner(url.clone(), Arc::clone(&clock), budgets.clone());
    let request = ChangePositionMode::new().dual_side_position("true");
    let deadline = Instant::now() + Duration::from_secs(5);
    let setup = |_| async {
        let stream = TcpStream::connect(url.socket_addrs(|| None).unwrap()[0])
            .await
            .unwrap();
        match change {
            Change::TimedCooldown => {
                native_cooldown(budgets.clone(), Arc::clone(&clock), 429).await;
            }
            Change::UntimedBan => native_cooldown(budgets.clone(), Arc::clone(&clock), 418).await,
            Change::Window => clock.0.store(661_000, Ordering::SeqCst),
            Change::Backwards => clock.0.store(658_999, Ordering::SeqCst),
            Change::Clock => clock.1.store(true, Ordering::SeqCst),
            Change::Deadline => {
                tokio::time::pause();
                tokio::time::advance(Duration::from_secs(5)).await;
            }
            Change::Limit => budgets
                .adopt_stated([crate::core::StatedLimit {
                    kind: Some("REQUEST_WEIGHT"),
                    interval: Some("MINUTE"),
                    interval_num: Some(1),
                    limit: Some(1),
                }])
                .unwrap(),
        }
        Ok(Box::new(stream) as OwnedTransport)
    };
    // A second real charge makes a reduced native limit smaller than inclusive usage.
    if matches!(change, Change::Limit) {
        budgets
            .admit(
                Cost {
                    weight: 1,
                    exit: crate::core::rate::reserve::ExitEligibility::Eligible,
                    ..Default::default()
                },
                NOW,
            )
            .unwrap();
    }
    let (result, wire) = tokio::join!(
        client.send_mode_control(&request, deadline, setup),
        serve(listener, 200, "", r#"{"code":200}"#)
    );
    assert!(wire.is_empty(), "HTTP mode bytes escaped after {change:?}");
    let error = result.unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    match change {
        Change::TimedCooldown => assert!(
            matches!(error, Error::Admission { retry_after } if retry_after == Duration::from_secs(2))
        ),
        Change::UntimedBan => assert!(matches!(error, Error::CooldownTimingUnknown)),
        Change::Window | Change::Backwards => assert!(matches!(
            error,
            Error::NotSent {
                operation: OPERATION,
                ..
            }
        )),
        Change::Clock => assert!(matches!(
            error,
            Error::Configuration("synthetic mode clock unavailable")
        )),
        Change::Deadline => assert!(matches!(error, Error::Expired(OPERATION))),
        Change::Limit => assert!(matches!(error, Error::Admission { .. })),
    }
    if !matches!(change, Change::Window) {
        assert_eq!(
            budgets.usage(NOW).unwrap().request_weight.used,
            if matches!(
                change,
                Change::TimedCooldown | Change::UntimedBan | Change::Limit
            ) {
                2
            } else {
                1
            }
        );
    }
}

#[tokio::test]
async fn setup_refuses_an_elapsed_authority_before_any_http_byte() {
    held_setup(Change::Deadline).await;
}

#[tokio::test]
async fn setup_refuses_a_shared_native_429_before_any_http_byte() {
    held_setup(Change::TimedCooldown).await;
}

#[tokio::test]
async fn setup_refuses_a_shared_native_418_before_any_http_byte() {
    held_setup(Change::UntimedBan).await;
}

#[tokio::test]
async fn setup_refuses_a_crossed_native_window_before_any_http_byte() {
    held_setup(Change::Window).await;
}

#[tokio::test]
async fn setup_refuses_a_backwards_clock_before_any_http_byte() {
    held_setup(Change::Backwards).await;
}

#[tokio::test]
async fn setup_preserves_a_first_write_clock_failure_as_unsent() {
    held_setup(Change::Clock).await;
}

#[tokio::test]
async fn setup_refuses_a_reduced_native_limit_without_recharging() {
    held_setup(Change::Limit).await;
}

#[derive(Clone, Copy)]
enum Write {
    Pending,
    Zero,
    Positive(usize),
    FailAfterByte,
}
struct Writer {
    steps: VecDeque<Write>,
    polls: usize,
    bytes: Vec<u8>,
}
impl AsyncWrite for Writer {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.polls += 1;
        match self.steps.pop_front().unwrap() {
            Write::Pending => Poll::Pending,
            Write::Zero => Poll::Ready(Ok(0)),
            Write::Positive(size) => {
                let size = size.min(bytes.len());
                self.bytes.extend_from_slice(&bytes[..size]);
                Poll::Ready(Ok(size))
            }
            Write::FailAfterByte => {
                self.bytes.extend_from_slice(&bytes[..1]);
                Poll::Ready(Err(io::Error::other("synthetic TLS partial write failure")))
            }
        }
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

fn guarded(steps: Vec<Write>) -> (Guarded<Writer>, Budgets, Arc<Mutex<Evidence>>) {
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(1)).unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(10_000))
        .unwrap();
    let cost = Cost {
        weight: 1,
        exit: crate::core::rate::reserve::ExitEligibility::Eligible,
        ..Default::default()
    };
    let charge = budgets.admit_precharged(cost, NOW, OPERATION).unwrap();
    let evidence = Arc::new(Mutex::new(Evidence::default()));
    let guard = Guard {
        budgets: budgets.clone(),
        clock: clock(),
        charge,
        deadline: Instant::now() + Duration::from_secs(5),
        operation: OPERATION,
    };
    (
        Guarded {
            inner: Writer {
                steps: steps.into(),
                polls: 0,
                bytes: Vec::new(),
            },
            guard: Some(guard),
            evidence: Arc::clone(&evidence),
        },
        budgets,
        evidence,
    )
}

#[test]
fn pending_and_zero_keep_first_write_authority_until_a_positive_vectored_write() {
    let (mut stream, budgets, evidence) =
        guarded(vec![Write::Pending, Write::Zero, Write::Positive(1)]);
    let mut cx = Context::from_waker(std::task::Waker::noop());
    assert!(
        Pin::new(&mut stream)
            .poll_write(&mut cx, b"POST")
            .is_pending()
    );
    assert!(matches!(
        Pin::new(&mut stream).poll_write_vectored(&mut cx, &[io::IoSlice::new(b"POST")]),
        Poll::Ready(Ok(0))
    ));
    budgets
        .observe_ban(418, &crate::RateEvidence::default())
        .unwrap();
    assert!(matches!(
        Pin::new(&mut stream).poll_write(&mut cx, b"POST"),
        Poll::Ready(Err(_))
    ));
    assert_eq!(stream.inner.polls, 2);
    assert_eq!(stream.inner.bytes, [] as [u8; 0]);
    assert!(!evidence.lock().unwrap().attempted);
    assert!(matches!(
        evidence.lock().unwrap().refusal,
        Some(Error::CooldownTimingUnknown)
    ));
}

#[test]
fn a_partial_positive_write_commits_one_exact_fit_charge() {
    let (mut stream, budgets, evidence) = guarded(vec![Write::Positive(1), Write::Positive(3)]);
    let mut cx = Context::from_waker(std::task::Waker::noop());
    assert!(matches!(
        Pin::new(&mut stream).poll_write(&mut cx, b"POST"),
        Poll::Ready(Ok(1))
    ));
    budgets
        .observe_ban(418, &crate::RateEvidence::default())
        .unwrap();
    assert!(matches!(
        Pin::new(&mut stream).poll_write_vectored(&mut cx, &[io::IoSlice::new(b"OST")]),
        Poll::Ready(Ok(3))
    ));
    assert_eq!(stream.inner.bytes, b"POST");
    assert!(evidence.lock().unwrap().attempted);
    assert!(evidence.lock().unwrap().refusal.is_none());
    assert_eq!(budgets.usage(NOW).unwrap().request_weight.used, 1);
}

#[test]
fn an_inner_write_error_after_a_partial_tls_emission_is_attempted() {
    let (mut stream, budgets, evidence) = guarded(vec![Write::FailAfterByte]);
    let mut cx = Context::from_waker(std::task::Waker::noop());
    assert!(matches!(
        Pin::new(&mut stream).poll_write(&mut cx, b"POST"),
        Poll::Ready(Err(_))
    ));
    assert_eq!(stream.inner.bytes, b"P");
    assert!(evidence.lock().unwrap().attempted);
    assert!(evidence.lock().unwrap().refusal.is_none());
    assert_eq!(budgets.usage(NOW).unwrap().request_weight.used, 1);
}

impl AsyncRead for Writer {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Poll::Pending
    }
}

#[tokio::test]
async fn a_partial_emission_then_inner_error_returns_unknown_without_replay() {
    let (_, url) = listener().await;
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(1)).unwrap();
    let client = owner(url, clock(), budgets.clone());
    let result = client
        .send_mode_control(
            &ChangePositionMode::new().dual_side_position("true"),
            Instant::now() + Duration::from_secs(5),
            |_| async {
                Ok(Box::new(Writer {
                    steps: vec![Write::FailAfterByte].into(),
                    polls: 0,
                    bytes: Vec::new(),
                }) as OwnedTransport)
            },
        )
        .await;
    let error = result.unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    assert!(matches!(
        error,
        Error::Transport {
            failure: Some(TransportFailure::ResponseHeadersFailed),
            meta: None,
            ..
        }
    ));
    assert_eq!(budgets.usage(NOW).unwrap().request_weight.used, 1);
}

#[test]
fn an_expired_guard_refuses_before_invoking_the_plaintext_writer() {
    let (mut stream, _, evidence) = guarded(vec![Write::Positive(4)]);
    stream.guard.as_mut().unwrap().deadline = Instant::now();
    let mut cx = Context::from_waker(std::task::Waker::noop());
    assert!(matches!(
        Pin::new(&mut stream).poll_write(&mut cx, b"POST"),
        Poll::Ready(Err(_))
    ));
    assert_eq!(stream.inner.polls, 0);
    assert_eq!(stream.inner.bytes, [] as [u8; 0]);
    assert!(!evidence.lock().unwrap().attempted);
    assert!(matches!(
        evidence.lock().unwrap().refusal,
        Some(Error::Expired(OPERATION))
    ));
}

#[tokio::test]
async fn a_receipt_clock_failure_preserves_native_headers_and_holds_the_pool() {
    for status in [200, 429, 418] {
        let clock = clock();
        let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
        let (listener, url) = listener().await;
        let client = owner(url, Arc::clone(&clock), budgets.clone());
        let request = ChangePositionMode::new().dual_side_position("true");
        let server = async {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut wire = Vec::new();
            let mut buffer = [0; 4096];
            while !wire.windows(4).any(|part| part == b"\r\n\r\n") {
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(read > 0);
                wire.extend_from_slice(&buffer[..read]);
            }
            clock.1.store(true, Ordering::SeqCst);
            let retry = if status == 429 {
                "Retry-After: 2\r\n"
            } else {
                ""
            };
            let response = format!(
                "HTTP/1.1 {status} fixture\r\nConnection: close\r\nContent-Length: 12\r\nX-MBX-USED-WEIGHT-1M: 17\r\n{retry}\r\n{{\"code\":200}}"
            );
            stream.write_all(response.as_bytes()).await.unwrap();
            wire
        };
        let (result, wire) = tokio::join!(
            client.execute_position_mode_control(&request, Instant::now() + Duration::from_secs(5)),
            server
        );
        assert!(wire.starts_with(b"POST /fapi/v1/positionSide/dual"));
        let error = result.unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown));
        let Error::Transport {
            failure: Some(TransportFailure::RateEvidenceFailed),
            meta: Some(meta),
            ..
        } = error
        else {
            panic!("receipt clock failure lost native evidence")
        };
        assert_eq!(meta.status, status);
        assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 17);
        assert_eq!(
            meta.rates.retry_after,
            (status == 429).then_some(Duration::from_secs(2))
        );
        assert!(!meta.rates.retry_after_unusable);
        let error = client
            .execute_position_mode_control(&request, Instant::now() + Duration::from_secs(5))
            .await
            .unwrap_err();
        assert!(matches!(error, Error::CooldownTimingUnknown));
        assert_eq!(error.outcome(), Some(Outcome::NotSent));
        assert_eq!(budgets.usage(NOW).unwrap().request_weight.used, 1);
    }
}
