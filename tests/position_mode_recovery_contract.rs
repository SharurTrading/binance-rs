// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! The finite mode control keeps native costs, shared cooldowns and truthful outcomes.

#[cfg(test)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::{Error, ExitReserve, Outcome, TransportFailure, usdm};
use std::time::Duration;
use support::{HttpFixture, KeepAliveFault, config, deadline};
use tokio::time::Instant;

fn request(value: bool) -> usdm::rest_requests::ChangePositionMode {
    usdm::rest_requests::ChangePositionMode::new().dual_side_position(value.to_string())
}

#[tokio::test]
async fn the_finite_mode_control_changes_and_restores_with_one_native_unit_each() {
    let mut fixture =
        HttpFixture::new(200, "", r#"{"code":200,"msg":"success"}"#, None, false).await;
    let owner = config().rest_url(&fixture.url).unwrap();
    owner
        .set_exit_reserve(ExitReserve::default().request_weight(10_000))
        .unwrap();
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let error = client
        .change_position_mode(&request(true), deadline())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Admission { .. }));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    for value in [true, false] {
        let response = client
            .change_position_mode_for_recovery(&request(value), deadline())
            .await
            .unwrap();
        assert_eq!(response.data.code, 200);
        assert_eq!(response.meta.status, 200);
        assert_eq!(response.meta.operation, "changePositionMode");
        let wire = fixture.requests.recv().await.unwrap();
        assert!(wire.starts_with("POST /fapi/v1/positionSide/dual HTTP/1.1\r\n"));
        let (headers, body) = wire.split_once("\r\n\r\n").unwrap();
        assert!(headers.contains("x-mbx-apikey: synthetic-api-key"));
        let form: std::collections::BTreeMap<_, _> = url::form_urlencoded::parse(body.as_bytes())
            .into_owned()
            .collect();
        assert_eq!(form.get("dualSidePosition"), Some(&value.to_string()));
        assert_eq!(
            form.get("timestamp").map(String::as_str),
            Some("1700000001000")
        );
        assert_eq!(form.get("signature").unwrap().len(), 64);
    }
    assert_eq!(observer.pool_usage().unwrap().request_weight.limit, 2_400);
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 2);
    assert_eq!(fixture.connections_accepted(), 2);
    assert!(matches!(
        client
            .check_server_time(&usdm::rest_requests::CheckServerTime::new(), deadline())
            .await,
        Err(Error::Admission { .. })
    ));
    fixture.finish().await;
}

#[tokio::test]
async fn an_explicit_proxy_is_refused_before_any_charge_or_connection() {
    let fixture = HttpFixture::new(200, "", r#"{"code":200}"#, None, false).await;
    let owner = config()
        .rest_url(&fixture.url)
        .unwrap()
        .http_proxy(reqwest::Proxy::all(&fixture.url).unwrap());
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let error = client
        .change_position_mode_for_recovery(&request(true), deadline())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        Error::Configuration("position-mode recovery transport does not support an explicit proxy")
    ));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}

#[tokio::test]
async fn expired_authority_is_refused_before_any_charge_or_connection() {
    let fixture = HttpFixture::new(200, "", r#"{"code":200}"#, None, false).await;
    let owner = config().rest_url(&fixture.url).unwrap();
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    let error = client
        .change_position_mode_for_recovery(&request(true), Instant::now())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Expired("changePositionMode")));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}

#[tokio::test]
async fn shared_mode_native_order_and_position_refusals_are_definitive_only_for_mode() {
    for protected in [false, true] {
        for (code, outcome) in [
            (-4067, Outcome::Rejected),
            (-4068, Outcome::Rejected),
            (-999_999, Outcome::Unknown),
        ] {
            let body = format!(r#"{{"code":{code},"msg":"synthetic native refusal"}}"#);
            let fixture =
                HttpFixture::new(400, "X-MBX-USED-WEIGHT-1M: 17\r\n", &body, None, false).await;
            let client = usdm::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
            let error = if protected {
                client
                    .change_position_mode_for_recovery(&request(true), deadline())
                    .await
                    .unwrap_err()
            } else {
                client
                    .change_position_mode(&request(true), deadline())
                    .await
                    .unwrap_err()
            };
            let Error::Venue(error) = error else {
                panic!("native rejection evidence lost")
            };
            assert_eq!(error.status, 400);
            assert_eq!(error.code, Some(code));
            assert_eq!(error.operation, "changePositionMode");
            assert_eq!(error.outcome, outcome);
            assert_eq!(error.rates.counters["x-mbx-used-weight-1m"], 17);
            assert_eq!(fixture.connections_accepted(), 1);
            fixture.finish().await;
        }
    }
}

#[tokio::test]
async fn the_mode_control_refuses_a_spent_native_window_without_another_connection() {
    let fixture = HttpFixture::new(200, "", r#"{"code":200}"#, None, false).await;
    let budgets =
        binance_client::Budgets::new(binance_client::BudgetLimits::usdm().weight_per_minute(1))
            .unwrap();
    budgets
        .set_exit_reserve(ExitReserve::default().request_weight(10_000))
        .unwrap();
    let owner = config().rest_url(&fixture.url).unwrap().budgets(budgets);
    let observer = owner.clone();
    let client = usdm::RestClient::new(owner).unwrap();
    assert_eq!(
        client
            .change_position_mode_for_recovery(&request(true), deadline())
            .await
            .unwrap()
            .data
            .code,
        200
    );
    let error = client
        .change_position_mode_for_recovery(&request(false), deadline())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Admission { .. }));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 1);
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn a_truncated_rate_response_retains_headers_and_blocks_the_next_attempt() {
    for status in [429, 418] {
        let retry = if status == 429 {
            "Retry-After: 2\r\n"
        } else {
            ""
        };
        let headers =
            format!("{retry}X-MBX-USED-WEIGHT-1M: 17\r\nSet-Cookie: synthetic-secret\r\n");
        let fixture = HttpFixture::new(status, &headers, "{", Some(999), false).await;
        let client = usdm::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client
            .change_position_mode_for_recovery(&request(true), deadline())
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown));
        assert!(!format!("{error:?}").contains("synthetic-secret"));
        let Error::Transport {
            failure: Some(TransportFailure::ResponseBodyFailed),
            meta: Some(meta),
            ..
        } = error
        else {
            panic!("received body failure evidence lost")
        };
        assert_eq!(meta.status, status);
        assert_eq!(meta.operation, "changePositionMode");
        assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 17);
        assert_eq!(
            meta.rates.retry_after,
            (status == 429).then_some(Duration::from_secs(2))
        );
        let next = client
            .change_position_mode_for_recovery(&request(false), deadline())
            .await
            .unwrap_err();
        assert_eq!(next.outcome(), Some(Outcome::NotSent));
        if status == 429 {
            assert!(
                matches!(next, Error::Admission { retry_after } if retry_after == Duration::from_secs(2))
            );
        } else {
            assert!(matches!(next, Error::CooldownTimingUnknown));
        }
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn a_lost_mode_acknowledgement_is_unknown_without_replay() {
    let mut fixture = HttpFixture::keep_alive(
        200,
        "",
        r#"{"code":200}"#,
        Some(KeepAliveFault::LostAcknowledgement(1)),
    )
    .await;
    let client = usdm::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let error = client
        .change_position_mode_for_recovery(&request(true), deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    assert!(matches!(
        error,
        Error::Transport {
            failure: Some(TransportFailure::ResponseHeadersFailed),
            meta: None,
            ..
        }
    ));
    assert!(
        fixture
            .requests
            .recv()
            .await
            .unwrap()
            .starts_with("POST /fapi/v1/positionSide/dual")
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
