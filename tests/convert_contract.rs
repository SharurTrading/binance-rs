// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Convert retains amount direction, quote expiry, venue IDs and execution uncertainty.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;
use binance_client::{Asset, Clock, Credentials, Decimal, Error, Outcome, convert};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use support::{FixedClock, HttpFixture, deadline};
fn config() -> convert::Config {
    convert::Config::with_pools(&binance_client::WeightPools::new())
        .unwrap()
        .clock(Arc::new(FixedClock(1000)))
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
}
fn quote_request() -> convert::rest_requests::SendQuoteRequest {
    convert::rest_requests::SendQuoteRequest::new()
        .from_asset(Asset::new("BTC").unwrap())
        .to_asset(Asset::new("ETH").unwrap())
        .from_amount(Decimal::new(1, 3))
        .build()
        .unwrap()
}
async fn quote() -> convert::Quotation {
    let fixture=HttpFixture::new(200,"",r#"{"quoteId":"venue-quote","ratio":"15.125","inverseRatio":"0.0661157","validTimestamp":2000,"fromAmount":"0.001","toAmount":"0.015125"}"#,None,false).await;
    let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let result = client
        .send_quote_request(&quote_request(), deadline())
        .await
        .unwrap()
        .data;
    fixture.finish().await;
    result
}
#[test]
fn amounts_are_mutually_exclusive_and_assets_are_distinct() {
    let request = quote_request();
    assert!(request.clone().to_amount(Decimal::ONE).build().is_err());
    assert!(
        request
            .to_asset(Asset::new("BTC").unwrap())
            .build()
            .is_err()
    );
    let limit = convert::rest_requests::PlaceLimitOrder::new()
        .base_asset(Asset::new("BTC").unwrap())
        .quote_asset(Asset::new("ETH").unwrap())
        .limit_price(Decimal::ONE)
        .side("SELL")
        .expired_type("1_D");
    assert!(limit.clone().build().is_err());
    assert!(
        limit
            .base_amount(Decimal::ONE)
            .quote_amount(Decimal::ONE)
            .build()
            .is_err()
    );
}
#[tokio::test]
async fn quote_keeps_asset_provenance_and_exact_amounts() {
    let quote = quote().await;
    assert_eq!(quote.from_asset.as_str(), "BTC");
    assert_eq!(quote.to_asset.as_str(), "ETH");
    assert_eq!(quote.receipt.to_amount, Decimal::new(15125, 6));
    assert!(convert::AcceptQuote::new(&quote).is_ok());
}
struct ExpiringClock(AtomicUsize);
impl Clock for ExpiringClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(if self.0.fetch_add(1, Ordering::SeqCst) < 3 {
            1000
        } else {
            2000
        })
    }
}
#[tokio::test]
async fn authority_expiring_during_admission_prevents_send() {
    let request = convert::AcceptQuote::new(&quote().await).unwrap();
    let fixture = HttpFixture::new(200, "", "{}", None, false).await;
    let client = convert::RestClient::new(
        config()
            .clock(Arc::new(ExpiringClock(AtomicUsize::new(0))))
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let error = client.accept_quote(&request, deadline()).await.unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}
#[tokio::test]
async fn truncated_acceptance_retains_quote_id_and_never_retries() {
    let request = convert::AcceptQuote::new(&quote().await).unwrap();
    let fixture = HttpFixture::new(
        200,
        "X-SAPI-USED-UID-WEIGHT-1M: 180000\r\n",
        r#"{"orderId":"123","orderStatus":"SUCCESS"}"#,
        Some(200),
        false,
    )
    .await;
    let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let error = client.accept_quote(&request, deadline()).await.unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        client_order_ids,
        meta: Some(meta),
        ..
    } = error
    else {
        panic!("transport")
    };
    assert_eq!(client_order_ids["quoteId"], "venue-quote");
    assert_eq!(meta.rates.counters["x-sapi-used-uid-weight-1m"], 180_000);
    assert!(matches!(
        client.clone().accept_quote(&request, deadline()).await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
#[tokio::test]
async fn unknown_acceptance_status_is_preserved_without_claiming_execution() {
    let request = convert::AcceptQuote::new(&quote().await).unwrap();
    let fixture = HttpFixture::new(
        200,
        "",
        r#"{"orderId":"123","createTime":1000,"orderStatus":"FUTURE_STATUS"}"#,
        None,
        false,
    )
    .await;
    let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let result = client
        .accept_quote(&request, deadline())
        .await
        .unwrap()
        .data;
    assert_eq!(result.receipt.order_status, "FUTURE_STATUS");
    assert_eq!(result.outcome(), Outcome::Unknown);
    assert_eq!(result.quotation.from_asset.as_str(), "BTC");
    fixture.finish().await;
}
#[test]
fn history_and_order_queries_validate_required_evidence() {
    let request = convert::rest_requests::GetConvertTradeHistory::new()
        .start_time(0)
        .end_time(2_592_000_001);
    assert!(request.build().is_err());
    assert!(convert::rest_requests::OrderStatus::new().build().is_err());
    assert!(
        serde_json::from_str::<convert::rest_models::OrderStatusResponse>(
            r#"{"orderId":123,"fromAmount":"NaN","toAmount":"1","fromAsset":"BTC","toAsset":"ETH"}"#
        )
        .is_err()
    );
}

#[test]
fn unpinned_limit_order_status_literals_stay_unknown() {
    // The Convert catalog documents placeLimitOrder/cancelLimitOrder `status`
    // as bare strings with example values and no enum (pinned in
    // schema/convert-error-codes.json), so neither literal is definitive.
    let cancellation: convert::rest_models::CancelLimitOrderResponse =
        serde_json::from_str(r#"{"orderId":123,"status":"CANCELED"}"#).unwrap();
    assert_eq!(cancellation.outcome(), Outcome::Unknown);
    assert_eq!(cancellation.status, "CANCELED");
    let placement: convert::rest_models::PlaceLimitOrderResponse =
        serde_json::from_str(r#"{"orderId":123,"status":"PROCESS"}"#).unwrap();
    assert_eq!(placement.outcome(), Outcome::Unknown);
    assert_eq!(placement.status, "PROCESS");
}
#[tokio::test]
async fn accept_quote_status_enum_is_pinned_from_the_catalog() {
    // acceptQuote.orderStatus is the one Convert status field with a documented
    // enum (pinned in schema/convert-error-codes.json); it stays classified
    // while future values stay unknown.
    for (status, expected) in [
        ("PROCESS", Outcome::Accepted),
        ("ACCEPT_SUCCESS", Outcome::Accepted),
        ("SUCCESS", Outcome::Accepted),
        ("FAIL", Outcome::Rejected),
        ("FUTURE_STATUS", Outcome::Unknown),
    ] {
        let request = convert::AcceptQuote::new(&quote().await).unwrap();
        let fixture = HttpFixture::new(
            200,
            "",
            &format!(r#"{{"orderId":"123","createTime":1000,"orderStatus":"{status}"}}"#),
            None,
            false,
        )
        .await;
        let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let result = client
            .accept_quote(&request, deadline())
            .await
            .unwrap()
            .data;
        assert_eq!(result.receipt.order_status, status);
        assert_eq!(result.outcome(), expected);
        fixture.finish().await;
    }
}
#[tokio::test]
async fn documented_rejection_unknown_codes_and_5xx_are_distinct() {
    let request = convert::AcceptQuote::new(&quote().await).unwrap();
    for (status, code, outcome) in [
        (400, -1022, Outcome::Rejected),
        (400, -999_999, Outcome::Unknown),
        (503, -1022, Outcome::Unknown),
    ] {
        let fixture = HttpFixture::new(
            status,
            "",
            &format!("{{\"code\":{code},\"msg\":\"synthetic-private-diagnostic\"}}"),
            None,
            false,
        )
        .await;
        let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client.accept_quote(&request, deadline()).await.unwrap_err();
        assert_eq!(error.outcome(), Some(outcome));
        assert!(!format!("{error:?}").contains("synthetic-private-diagnostic"));
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}
#[tokio::test]
async fn definitive_codes_match_the_pinned_error_code_snapshot() {
    // Machine-checks schema/convert-error-codes.json, including Convert's only
    // intentional divergence from Wallet: the matching-engine rejection codes
    // -2010/-2011. Every never-definitive code (retryable, unknown-execution,
    // rate, and future codes) stays ambiguous, and every 5xx too.
    let snapshot = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schema/convert-error-codes.json"
    ))
    .unwrap();
    let snapshot: serde_json::Value = serde_json::from_str(&snapshot).unwrap();
    let divergence = snapshot["divergence_from_wallet"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_i64().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(divergence, vec![-2010, -2011]);
    let request = convert::AcceptQuote::new(&quote().await).unwrap();
    for (field, expected) in [
        ("definitive", Outcome::Rejected),
        ("never_definitive", Outcome::Unknown),
    ] {
        for entry in snapshot[field].as_array().unwrap() {
            let code = entry["code"].as_i64().unwrap();
            let fixture =
                HttpFixture::new(400, "", &format!("{{\"code\":{code}}}"), None, false).await;
            let client =
                convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
            let error = client.accept_quote(&request, deadline()).await.unwrap_err();
            assert_eq!(error.outcome(), Some(expected), "code {code}");
            assert_eq!(fixture.connections_accepted(), 1);
            fixture.finish().await;
        }
    }
    for entry in snapshot["definitive"].as_array().unwrap() {
        let code = entry["code"].as_i64().unwrap();
        let fixture = HttpFixture::new(503, "", &format!("{{\"code\":{code}}}"), None, false).await;
        let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client.accept_quote(&request, deadline()).await.unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown), "5xx code {code}");
        fixture.finish().await;
    }
}

#[tokio::test]
async fn sapi_ip_ban_blocks_other_endpoints_and_account_owners() {
    let budgets = binance_client::Budgets::sapi().unwrap();
    let fixture =
        HttpFixture::new(418, "Retry-After: 30\r\n", r#"{"code":-1003}"#, None, false).await;
    let wallet = binance_client::wallet::RestClient::new(
        config()
            .budgets(budgets.clone())
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let read = binance_client::wallet::rest_requests::WithdrawHistory::new();
    assert_eq!(
        wallet
            .withdraw_history(&read, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::ReadFailed)
    );
    let other = convert::RestClient::new(
        config()
            .budgets(budgets.for_account())
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let error = other
        .list_all_convert_pairs(
            &convert::rest_requests::ListAllConvertPairs::new(),
            deadline(),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(error,Error::Admission{retry_after} if retry_after==std::time::Duration::from_secs(30))
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn uncertain_limit_cancellation_retains_native_order_identity() {
    let fixture = HttpFixture::new(
        200,
        "",
        r#"{"orderId":123,"status":"CANCELED"}"#,
        Some(100),
        false,
    )
    .await;
    let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = convert::rest_requests::CancelLimitOrder::new()
        .order_id(convert::OrderId::new(123).unwrap())
        .build()
        .unwrap();
    let error = client
        .cancel_limit_order(&request, deadline())
        .await
        .unwrap_err();
    let Error::Transport {
        client_order_ids, ..
    } = error
    else {
        panic!("transport")
    };
    assert_eq!(client_order_ids["orderId"], "123");
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn ban_without_retry_timing_does_not_fabricate_permission_to_send() {
    let budgets = binance_client::Budgets::sapi().unwrap();
    let fixture = HttpFixture::new(
        418,
        "Retry-After: malformed\r\n",
        r#"{"code":-1003}"#,
        None,
        false,
    )
    .await;
    let client =
        convert::RestClient::new(config().budgets(budgets).rest_url(&fixture.url).unwrap())
            .unwrap();
    let request = convert::rest_requests::ListAllConvertPairs::new();
    assert_eq!(
        client
            .list_all_convert_pairs(&request, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::ReadFailed)
    );
    let error = client
        .list_all_convert_pairs(&request, deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn status_query_retains_acceptance_string_order_identity() {
    let id = convert::AcceptanceOrderId::new("venue-order-000123").unwrap();
    let request = convert::rest_requests::OrderStatus::new()
        .order_id(id)
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(&request).unwrap()["orderId"],
        "venue-order-000123"
    );
    let mut fixture = HttpFixture::new(
        200,
        "",
        r#"{"orderId":123,"orderStatus":"SUCCESS","fromAsset":"BTC","fromAmount":"0.001","toAsset":"ETH","toAmount":"0.015125","ratio":"15.125","inverseRatio":"0.0661157","createTime":1000}"#,
        None,
        false,
    )
    .await;
    let client = convert::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let result = client
        .order_status(&request, deadline())
        .await
        .unwrap()
        .data;
    assert_eq!(result.order_id.value(), 123);
    assert!(
        fixture
            .requests
            .recv()
            .await
            .unwrap()
            .contains("orderId=venue-order-000123")
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
