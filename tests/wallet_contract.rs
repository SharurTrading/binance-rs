// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Wallet keeps asset/network evidence and uses endpoint-specific SAPI scopes.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;
use binance_client::{Asset, Credentials, Decimal, Error, Outcome, wallet};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};
fn config() -> wallet::Config {
    wallet::Config::new()
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
}
#[test]
fn asset_evidence_is_required_and_exact() {
    let row: wallet::rest_models::UserAssetResponseItem = serde_json::from_str(r#"{"asset":"BTC","free":"0.123456789123456789","locked":"0.1","freeze":"0","withdrawing":"0","ipoable":"0"}"#).unwrap();
    assert_eq!(row.asset.as_str(), "BTC");
    assert_eq!(
        row.free,
        Decimal::from_str_exact("0.123456789123456789").unwrap()
    );
    assert!(
        serde_json::from_str::<wallet::rest_models::UserAssetResponseItem>(
            r#"{"free":"0","locked":"0"}"#
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<wallet::rest_models::UserAssetResponseItem>(
            r#"{"asset":"BTC","free":"NaN","locked":"0"}"#
        )
        .is_err()
    );
}
#[test]
fn withdrawal_requires_caller_id_and_redacts_destination() {
    use wallet::rest_requests::Withdraw;
    let request = Withdraw::new()
        .coin(Asset::new("BTC").unwrap())
        .address(binance_client::SensitiveString::new(
            "synthetic-destination",
        ))
        .amount(Decimal::ONE);
    assert!(request.clone().build().is_err());
    let request = request
        .withdraw_order_id(wallet::WithdrawalId::new("caller-withdrawal").unwrap())
        .build()
        .unwrap();
    assert!(!format!("{request:?}").contains("synthetic-destination"));
}
#[tokio::test]
async fn endpoint_weight_headers_survive_truncated_withdrawal_without_retry() {
    let fixture = HttpFixture::new(
        200,
        "X-SAPI-USED-UID-WEIGHT-1M: 180000\r\n",
        r#"{"id":"venue-id"}"#,
        Some(100),
        false,
    )
    .await;
    let client = wallet::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = wallet::rest_requests::Withdraw::new()
        .coin(Asset::new("BTC").unwrap())
        .address(binance_client::SensitiveString::new(
            "synthetic-destination",
        ))
        .amount(Decimal::ONE)
        .withdraw_order_id(wallet::WithdrawalId::new("caller-withdrawal").unwrap())
        .build()
        .unwrap();
    let error = client.withdraw(&request, deadline()).await.unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        meta: Some(meta),
        client_order_ids,
        ..
    } = error
    else {
        panic!("transport evidence")
    };
    assert_eq!(meta.rates.counters["x-sapi-used-uid-weight-1m"], 180_000);
    assert_eq!(client_order_ids["withdrawOrderId"], "caller-withdrawal");
    assert!(matches!(
        client.clone().withdraw(&request, deadline()).await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(fixture.attempts(), 1);
    fixture.finish().await;
}
#[tokio::test]
async fn definitive_codes_match_the_pinned_error_code_snapshot() {
    // Machine-checks schema/wallet-error-codes.json: every pinned definitive
    // code must classify as a definitive refusal below 500, every pinned
    // never-definitive code (retryable, unknown-execution, rate, and the
    // retired -1002) must stay ambiguous, and every 5xx stays ambiguous.
    let snapshot = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/schema/wallet-error-codes.json"
    ))
    .unwrap();
    let snapshot: serde_json::Value = serde_json::from_str(&snapshot).unwrap();
    let request = wallet::rest_requests::Withdraw::new()
        .coin(Asset::new("BTC").unwrap())
        .address(binance_client::SensitiveString::new(
            "synthetic-destination",
        ))
        .amount(Decimal::ONE)
        .withdraw_order_id(wallet::WithdrawalId::new("caller-withdrawal").unwrap())
        .build()
        .unwrap();
    for (field, expected) in [
        ("definitive", Outcome::Rejected),
        ("never_definitive", Outcome::Unknown),
    ] {
        for entry in snapshot[field].as_array().unwrap() {
            let code = entry["code"].as_i64().unwrap();
            let fixture =
                HttpFixture::new(400, "", &format!("{{\"code\":{code}}}"), None, false).await;
            let client = wallet::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
            let error = client.withdraw(&request, deadline()).await.unwrap_err();
            assert_eq!(error.outcome(), Some(expected), "code {code}");
            assert_eq!(fixture.attempts(), 1);
            fixture.finish().await;
        }
    }
    for entry in snapshot["definitive"].as_array().unwrap() {
        let code = entry["code"].as_i64().unwrap();
        let fixture = HttpFixture::new(503, "", &format!("{{\"code\":{code}}}"), None, false).await;
        let client = wallet::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client.withdraw(&request, deadline()).await.unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown), "5xx code {code}");
        fixture.finish().await;
    }
}
#[tokio::test]
async fn read_post_is_read_failed_and_different_endpoint_has_own_budget() {
    let mut fixture = HttpFixture::new(
        503,
        "X-SAPI-USED-IP-WEIGHT-1M: 12000\r\n",
        r#"{"code":-1000}"#,
        None,
        false,
    )
    .await;
    let client = wallet::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let read = wallet::rest_requests::UserAsset::new();
    assert_eq!(
        client
            .user_asset(&read, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::ReadFailed)
    );
    assert!(matches!(
        client.clone().user_asset(&read, deadline()).await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(
        client
            .account_info(&wallet::rest_requests::AccountInfo::new(), deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::ReadFailed)
    );
    assert_eq!(fixture.attempts(), 2);
    assert!(
        fixture
            .requests
            .recv()
            .await
            .unwrap()
            .starts_with("POST /sapi/v3/asset/getUserAsset")
    );
    fixture.finish().await;
}

#[tokio::test]
async fn wallet_quote_asset_is_explicit_request_provenance() {
    assert!(
        wallet::rest_requests::QueryUserWalletBalance::new()
            .build()
            .is_err()
    );
    let fixture = HttpFixture::new(
        200,
        "",
        r#"[{"walletName":"Spot","activate":true,"balance":"1.125"}]"#,
        None,
        false,
    )
    .await;
    let client = wallet::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let response = client
        .query_user_wallet_balance(
            &wallet::rest_requests::QueryUserWalletBalance::new()
                .quote_asset(Asset::new("ETH").unwrap())
                .build()
                .unwrap(),
            deadline(),
        )
        .await
        .unwrap();
    assert_eq!(response.data.quote_asset.as_str(), "ETH");
    assert_eq!(response.data.wallets[0].balance, Decimal::new(1125, 3));
    fixture.finish().await;
}
#[test]
fn isolated_transfer_requires_the_native_symbol_and_dust_assets_are_comma_encoded() {
    let request = wallet::rest_requests::UserUniversalTransfer::new()
        .type_value("ISOLATEDMARGIN_MARGIN")
        .asset(Asset::new("BTC").unwrap())
        .amount(Decimal::ONE);
    assert!(request.clone().build().is_err());
    assert!(
        request
            .from_symbol(binance_client::Symbol::new("BTCUSDT").unwrap())
            .build()
            .is_ok()
    );
    let assets =
        wallet::DustAssets::new(vec![Asset::new("BTC").unwrap(), Asset::new("ETH").unwrap()])
            .unwrap();
    let request = wallet::rest_requests::DustTransfer::new()
        .asset(assets)
        .build()
        .unwrap();
    assert_eq!(serde_json::to_value(request).unwrap()["asset"], "BTC,ETH");
}

#[test]
fn asset_details_are_keyed_by_arbitrary_asset_and_integer_fees_are_decimal() {
    let details:wallet::rest_models::AssetDetailResponse=serde_json::from_str(r#"{"SYNTHETIC":{"minWithdrawAmount":"0.00000001","withdrawFee":2,"depositStatus":true,"withdrawStatus":false}}"#).unwrap();
    assert_eq!(
        details[&Asset::new("SYNTHETIC").unwrap()].withdraw_fee,
        Some(Decimal::new(2, 0))
    );
}

#[tokio::test]
async fn dust_receipt_retains_caller_selected_target_asset() {
    let fixture=HttpFixture::new(200,"",r#"{"totalTransfered":"0.1","totalServiceCharge":"0.01","transferResult":[{"tranId":5,"fromAsset":"BTC","amount":"0.001","transferedAmount":"0.1","serviceChargeAmount":"0.01","operateTime":1700000001000}]}"#,None,false).await;
    let client = wallet::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = wallet::rest_requests::DustConvert::new()
        .asset(Asset::new("BTC").unwrap())
        .target_asset(Asset::new("ETH").unwrap())
        .client_id(binance_client::RequestId::new("caller-dust").unwrap())
        .build()
        .unwrap();
    let response = client.dust_convert(&request, deadline()).await.unwrap();
    assert_eq!(response.data.target_asset.as_str(), "ETH");
    assert_eq!(
        response.data.receipt.transfer_result.as_ref().unwrap()[0]
            .from_asset
            .as_str(),
        "BTC"
    );
    fixture.finish().await;
}

#[test]
fn capital_and_dividend_history_ranges_match_documented_boundaries() {
    let ninety_days = 7_776_000_000;
    assert!(
        wallet::rest_requests::DepositHistory::new()
            .start_time(0)
            .end_time(ninety_days)
            .build()
            .is_err()
    );
    assert!(
        wallet::rest_requests::DepositHistory::new()
            .start_time(0)
            .end_time(ninety_days - 1)
            .build()
            .is_ok()
    );
    assert!(
        wallet::rest_requests::WithdrawHistory::new()
            .withdraw_order_id(wallet::WithdrawalId::new("caller-history").unwrap())
            .start_time(0)
            .end_time(604_800_000)
            .build()
            .is_err()
    );
    assert!(
        wallet::rest_requests::AssetDividendRecord::new()
            .start_time(0)
            .end_time(15_552_000_000)
            .build()
            .is_ok()
    );
    assert!(
        wallet::rest_requests::AssetDividendRecord::new()
            .start_time(0)
            .end_time(15_552_000_001)
            .build()
            .is_err()
    );
    assert!(
        wallet::rest_requests::WithdrawHistory::new()
            .id_list((0..46).map(|n| n.to_string()).collect::<Vec<_>>().join(","))
            .build()
            .is_err()
    );
}

#[test]
fn native_network_identity_is_validated_without_inferring_asset() {
    assert!(wallet::Network::new("").is_err());
    assert!(wallet::Network::new("bad\nnetwork").is_err());
    let request = wallet::rest_requests::Withdraw::new()
        .network(wallet::Network::new("SYNTHETIC_NETWORK").unwrap());
    assert_eq!(
        serde_json::to_value(request).unwrap()["network"],
        "SYNTHETIC_NETWORK"
    );
}
