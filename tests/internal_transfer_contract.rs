// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Own-wallet transfers retain native asset, symbol, history and outcome evidence.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::{Asset, Budgets, Credentials, Decimal, Error, Outcome, Symbol, wallet};
use std::{sync::Arc, time::Duration};
use support::{FixedClock, HttpFixture, deadline};

fn config() -> wallet::Config {
    wallet::Config::with_pools(&binance_client::WeightPools::new())
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
}

fn transfer() -> wallet::rest_requests::UserUniversalTransfer {
    wallet::rest_requests::UserUniversalTransfer::new()
        .type_value("ISOLATEDMARGIN_ISOLATEDMARGIN")
        .asset(Asset::new("BTC").unwrap())
        .amount(Decimal::new(123_456_789_123_456_789, 18))
        .from_symbol(Symbol::new("BTCUSDT").unwrap())
        .to_symbol(Symbol::new("BTCFDUSD").unwrap())
        .build()
        .unwrap()
}

fn history(page: i64) -> wallet::rest_requests::QueryUserUniversalTransferHistory {
    wallet::rest_requests::QueryUserUniversalTransferHistory::new()
        .type_value("ISOLATEDMARGIN_ISOLATEDMARGIN")
        .from_symbol(Symbol::new("BTCUSDT").unwrap())
        .to_symbol(Symbol::new("BTCFDUSD").unwrap())
        .start_time(1_700_000_000_000)
        .end_time(1_700_000_002_000)
        .current(page)
        .size(1)
        .build()
        .unwrap()
}

#[test]
fn isolated_routes_require_their_native_symbol_and_positive_amount() {
    let base = wallet::rest_requests::UserUniversalTransfer::new()
        .asset(Asset::new("BTC").unwrap())
        .amount(Decimal::ONE);
    assert!(
        base.clone()
            .type_value("ISOLATEDMARGIN_MARGIN")
            .build()
            .is_err()
    );
    assert!(
        base.clone()
            .type_value("MARGIN_ISOLATEDMARGIN")
            .build()
            .is_err()
    );
    assert!(
        base.clone()
            .type_value("ISOLATEDMARGIN_ISOLATEDMARGIN")
            .from_symbol(Symbol::new("BTCUSDT").unwrap())
            .build()
            .is_err()
    );
    assert!(
        wallet::rest_requests::QueryUserUniversalTransferHistory::new()
            .type_value("MARGIN_ISOLATEDMARGIN")
            .build()
            .is_err()
    );
    assert!(
        base.type_value("MAIN_MARGIN")
            .amount(Decimal::ZERO)
            .build()
            .is_err()
    );
    assert!(
        wallet::rest_requests::QueryUserUniversalTransferHistory::new()
            .type_value("MAIN_MARGIN")
            .current(0)
            .build()
            .is_err()
    );
    assert!(
        wallet::rest_requests::QueryUserUniversalTransferHistory::new()
            .type_value("MAIN_MARGIN")
            .size(101)
            .build()
            .is_err()
    );
}

#[tokio::test]
async fn signed_transfer_and_explicit_history_pages_preserve_native_values() {
    let mut transfer_fixture =
        HttpFixture::new(200, "", r#"{"tranId":9007199254740993}"#, None, false).await;
    let mut first_page = HttpFixture::new(
        200,
        "",
        r#"{"total":2,"rows":[{"asset":"BTC","amount":"0.123456789123456789","type":"ISOLATEDMARGIN_ISOLATEDMARGIN","status":"CONFIRMED","tranId":9007199254740993,"timestamp":1700000001000}]}"#,
        None,
        false,
    )
    .await;
    let mut second_page = HttpFixture::new(
        200,
        "",
        r#"{"total":2,"rows":[{"asset":"BTC","amount":"0.000000000000000001","type":"ISOLATEDMARGIN_ISOLATEDMARGIN","status":"FUTURE_NATIVE_STATUS","tranId":9007199254740994,"timestamp":1700000002000}]}"#,
        None,
        false,
    )
    .await;
    let client =
        wallet::RestClient::new(config().rest_url(&transfer_fixture.url).unwrap()).unwrap();
    let receipt = client
        .user_universal_transfer(&transfer(), deadline())
        .await
        .unwrap();
    assert_eq!(receipt.data.tran_id, 9_007_199_254_740_993);
    let first_client =
        wallet::RestClient::new(config().rest_url(&first_page.url).unwrap()).unwrap();
    let first = first_client
        .query_user_universal_transfer_history(&history(1), deadline())
        .await
        .unwrap();
    let second_client =
        wallet::RestClient::new(config().rest_url(&second_page.url).unwrap()).unwrap();
    let second = second_client
        .query_user_universal_transfer_history(&history(2), deadline())
        .await
        .unwrap();
    assert_eq!(first.data.total, Some(2));
    assert_eq!(second.data.total, Some(2));
    let first_row = &first.data.rows.as_ref().unwrap()[0];
    assert_eq!(first_row.asset.as_str(), "BTC");
    assert_eq!(first_row.amount, Decimal::new(123_456_789_123_456_789, 18));
    assert_eq!(first_row.tran_id, Some(9_007_199_254_740_993));
    assert_eq!(first_row.status.as_deref(), Some("CONFIRMED"));
    let second_row = &second.data.rows.as_ref().unwrap()[0];
    assert_eq!(second_row.amount, Decimal::new(1, 18));
    assert_eq!(second_row.tran_id, Some(9_007_199_254_740_994));
    assert_eq!(second_row.status.as_deref(), Some("FUTURE_NATIVE_STATUS"));

    let sent = transfer_fixture.requests.recv().await.unwrap();
    assert!(sent.starts_with("POST /sapi/v1/asset/transfer"));
    assert!(
        sent.contains("X-MBX-APIKEY: synthetic-key")
            || sent.contains("x-mbx-apikey: synthetic-key")
    );
    for native in [
        "type=ISOLATEDMARGIN_ISOLATEDMARGIN",
        "asset=BTC",
        "amount=0.123456789123456789",
        "fromSymbol=BTCUSDT",
        "toSymbol=BTCFDUSD",
        "signature=",
    ] {
        assert!(sent.contains(native), "missing {native}");
    }
    assert!(!sent.contains("clientTranId="));
    for (fixture, page) in [(&mut first_page, 1), (&mut second_page, 2)] {
        let sent = fixture.requests.recv().await.unwrap();
        assert!(sent.starts_with("GET /sapi/v1/asset/transfer"));
        assert!(sent.contains(&format!("current={page}")));
        for native in [
            "type=ISOLATEDMARGIN_ISOLATEDMARGIN",
            "startTime=1700000000000",
            "endTime=1700000002000",
            "size=1",
            "fromSymbol=BTCUSDT",
            "toSymbol=BTCFDUSD",
            "signature=",
        ] {
            assert!(sent.contains(native), "missing {native}");
        }
    }
    transfer_fixture.finish().await;
    first_page.finish().await;
    second_page.finish().await;
}

#[tokio::test]
async fn missing_receipt_and_malformed_history_cannot_confirm_transfer() {
    let missing_receipt = HttpFixture::new(200, "", "{}", None, false).await;
    let client = wallet::RestClient::new(config().rest_url(&missing_receipt.url).unwrap()).unwrap();
    let error = client
        .user_universal_transfer(&transfer(), deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    assert_eq!(missing_receipt.connections_accepted(), 1);
    missing_receipt.finish().await;

    let absent_history = HttpFixture::new(200, "", "{}", None, false).await;
    let client = wallet::RestClient::new(config().rest_url(&absent_history.url).unwrap()).unwrap();
    let result = client
        .query_user_universal_transfer_history(&history(1), deadline())
        .await
        .unwrap();
    assert_eq!(result.data.total, None);
    assert!(result.data.rows.is_none());
    absent_history.finish().await;

    let incomplete_row = HttpFixture::new(
        200,
        "",
        r#"{"total":1,"rows":[{"asset":"BTC","amount":"1"}]}"#,
        None,
        false,
    )
    .await;
    let client = wallet::RestClient::new(config().rest_url(&incomplete_row.url).unwrap()).unwrap();
    let result = client
        .query_user_universal_transfer_history(&history(1), deadline())
        .await
        .unwrap();
    let row = &result.data.rows.as_ref().unwrap()[0];
    assert!(row.status.is_none());
    assert!(row.tran_id.is_none());
    assert!(row.timestamp.is_none());
    incomplete_row.finish().await;

    let malformed_history = HttpFixture::new(
        200,
        "",
        r#"{"total":1,"rows":[{"asset":"BTC","amount":"NaN"}]}"#,
        None,
        false,
    )
    .await;
    let client =
        wallet::RestClient::new(config().rest_url(&malformed_history.url).unwrap()).unwrap();
    let error = client
        .query_user_universal_transfer_history(&history(1), deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::ReadFailed));
    malformed_history.finish().await;
}

#[tokio::test]
async fn transfer_refusals_and_uncertain_answers_keep_single_attempt_and_no_caller_id() {
    for (status, body, length, outcome) in [
        (
            200,
            r#"{"tranId":9007199254740993}"#,
            Some(100),
            Outcome::Unknown,
        ),
        (
            400,
            r#"{"code":-1102,"msg":"missing parameter"}"#,
            None,
            Outcome::Rejected,
        ),
        (
            400,
            r#"{"code":-999999,"msg":"future code"}"#,
            None,
            Outcome::Unknown,
        ),
        (
            503,
            r#"{"code":-1102,"msg":"failure"}"#,
            None,
            Outcome::Unknown,
        ),
    ] {
        let fixture = HttpFixture::new(status, "", body, length, false).await;
        let client = wallet::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client
            .user_universal_transfer(&transfer(), deadline())
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(outcome), "status {status}");
        if let Error::Transport {
            client_order_ids, ..
        } = error
        {
            assert!(client_order_ids.is_empty());
        }
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn timed_out_transfer_keeps_unknown_outcome_without_replay() {
    let mut fixture = HttpFixture::new(200, "", "{", Some(100), true).await;
    let client = wallet::RestClient::new(
        config()
            .rest_url(&fixture.url)
            .unwrap()
            .timeout(Duration::from_millis(200))
            .unwrap(),
    )
    .unwrap();
    let pending = tokio::spawn(async move {
        client
            .user_universal_transfer(&transfer(), deadline())
            .await
    });
    assert!(
        fixture
            .requests
            .recv()
            .await
            .unwrap()
            .starts_with("POST /sapi/v1/asset/transfer")
    );
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(1)).await;
    let error = pending.await.unwrap().unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    tokio::time::resume();
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn transfer_uid_and_history_ip_headers_bind_only_their_own_scopes() {
    let budgets = Budgets::sapi().unwrap();
    let transfer_fixture = HttpFixture::new(
        200,
        "X-SAPI-USED-UID-WEIGHT-1M: 179500\r\n",
        r#"{"tranId":123}"#,
        None,
        false,
    )
    .await;
    let history_fixture = HttpFixture::new(
        200,
        "X-SAPI-USED-IP-WEIGHT-1M: 11999\r\n",
        r#"{"total":0,"rows":[]}"#,
        None,
        false,
    )
    .await;
    let transfer_client = wallet::RestClient::new(
        config()
            .budgets(budgets.clone())
            .rest_url(&transfer_fixture.url)
            .unwrap(),
    )
    .unwrap();
    let history_client = wallet::RestClient::new(
        config()
            .budgets(budgets)
            .rest_url(&history_fixture.url)
            .unwrap(),
    )
    .unwrap();
    transfer_client
        .user_universal_transfer(&transfer(), deadline())
        .await
        .unwrap();
    transfer_client
        .user_universal_transfer(&transfer(), deadline())
        .await
        .unwrap();
    let transfer_error = transfer_client
        .user_universal_transfer(&transfer(), deadline())
        .await
        .unwrap_err();
    assert!(matches!(transfer_error, Error::Admission { .. }));
    assert_eq!(transfer_error.outcome(), Some(Outcome::NotSent));
    history_client
        .query_user_universal_transfer_history(&history(1), deadline())
        .await
        .unwrap();
    history_client
        .query_user_universal_transfer_history(&history(2), deadline())
        .await
        .unwrap();
    let history_error = history_client
        .query_user_universal_transfer_history(&history(3), deadline())
        .await
        .unwrap_err();
    assert!(matches!(history_error, Error::Admission { .. }));
    assert_eq!(history_error.outcome(), Some(Outcome::NotSent));
    assert_eq!(transfer_fixture.connections_accepted(), 2);
    assert_eq!(history_fixture.connections_accepted(), 2);
    transfer_fixture.finish().await;
    history_fixture.finish().await;
}
