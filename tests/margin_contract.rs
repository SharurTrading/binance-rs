// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Margin retains cross/isolated assets, caller IDs and execution uncertainty.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;
use binance_client::{Asset, Credentials, Decimal, Error, Outcome, Symbol, margin};
use margin::ClientOrderId;
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};
fn config() -> margin::Config {
    margin::Config::with_pools(&binance_client::WeightPools::new())
        .unwrap()
        .budgets(binance_client::Budgets::new(binance_client::BudgetLimits::spot()).unwrap())
        .clock(Arc::new(FixedClock(1000)))
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
        .order_limits(&serde_json::from_str(r#"[{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":10000,"count":0}]"#).unwrap()).unwrap()
}
fn order() -> margin::rest_requests::MarginAccountNewOrder {
    margin::rest_requests::MarginAccountNewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::new(1, 3))
        .new_client_order_id(ClientOrderId::new("caller-margin-1").unwrap())
}
#[test]
fn order_requires_caller_identity_and_valid_native_quantity_direction() {
    assert!(
        margin::rest_requests::MarginAccountNewOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .build()
            .is_err()
    );
    assert!(order().build().is_ok());
    assert!(order().quote_order_qty(Decimal::ONE).build().is_err());
    assert!(order().quantity(Decimal::ZERO).build().is_err());
    assert!(order().type_value("LIMIT").build().is_err());
}
#[test]
fn isolated_borrow_requires_symbol_and_positive_amount() {
    let request = margin::rest_requests::MarginAccountBorrowRepay::new()
        .asset(Asset::new("BTC").unwrap())
        .is_isolated("TRUE")
        .amount(Decimal::ONE)
        .type_value("BORROW");
    assert!(request.clone().build().is_err());
    assert!(
        request
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .build()
            .is_ok()
    );
}
#[test]
fn cross_balance_requires_native_asset_and_exact_financial_evidence() {
    let body = r#"{"created":true,"borrowEnabled":true,"marginLevel":"2","totalAssetOfBtc":"1","totalLiabilityOfBtc":"0.1","totalNetAssetOfBtc":"0.9","tradeEnabled":true,"transferInEnabled":true,"transferOutEnabled":true,"accountType":"MARGIN_1","userAssets":[{"asset":"BTC","borrowed":"0.1000000000000000000000000001","free":"0.2","interest":"0.001","locked":"0.3","netAsset":"0.399"}]}"#;
    let response: margin::rest_models::QueryCrossMarginAccountDetailsResponse =
        serde_json::from_str(body).unwrap();
    assert_eq!(response.user_assets[0].asset.as_str(), "BTC");
    assert_eq!(
        response.user_assets[0].borrowed.to_string(),
        "0.1000000000000000000000000001"
    );
    assert!(
        serde_json::from_str::<margin::rest_models::QueryCrossMarginAccountDetailsResponse>(
            &body.replace("\"asset\":\"BTC\",", "")
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<margin::rest_models::QueryCrossMarginAccountDetailsResponse>(
            &body.replace("\"0.2\"", "\"NaN\"")
        )
        .is_err()
    );
}
#[tokio::test]
async fn truncated_mutation_preserves_client_identity_and_uid_quota_without_retry() {
    let fixture = HttpFixture::new(
        200,
        "X-SAPI-USED-UID-WEIGHT-1M: 180000\r\n",
        r#"{"orderId":123}"#,
        Some(100),
        false,
    )
    .await;
    let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = order().build().unwrap();
    let error = client
        .margin_account_new_order(&request, deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    let Error::Transport {
        client_order_ids,
        meta: Some(meta),
        ..
    } = error
    else {
        panic!("transport")
    };
    assert_eq!(client_order_ids["newClientOrderId"], "caller-margin-1");
    assert_eq!(meta.rates.counters["x-sapi-used-uid-weight-1m"], 180_000);
    assert!(matches!(
        client
            .clone()
            .margin_account_new_order(&request, deadline())
            .await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
#[tokio::test]
async fn documented_refusal_and_future_code_do_not_hide_uncertainty() {
    for (status, code, outcome) in [
        (400, -1022, Outcome::Rejected),
        (400, -999_999, Outcome::Unknown),
        (503, -1022, Outcome::Unknown),
    ] {
        let fixture = HttpFixture::new(
            status,
            "",
            &format!("{{\"code\":{code},\"msg\":\"private-message\"}}"),
            None,
            false,
        )
        .await;
        let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client
            .margin_account_new_order(&order().build().unwrap(), deadline())
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(outcome));
        assert!(!format!("{error:?}").contains("private-message"));
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}
#[test]
fn list_orders_require_every_caller_owned_leg_id() {
    assert!(
        margin::rest_requests::MarginAccountNewOco::new()
            .build()
            .is_err()
    );
    assert!(
        margin::rest_requests::MarginAccountNewOto::new()
            .build()
            .is_err()
    );
    assert!(
        margin::rest_requests::MarginAccountNewOtoco::new()
            .build()
            .is_err()
    );
}
#[test]
fn special_key_receipts_redact_and_zeroize_secrets() {
    let response: margin::rest_models::CreateSpecialKeyResponse = serde_json::from_str(
        r#"{"apiKey":"private-api-key","secretKey":"private-secret-key","type":"HMAC_SHA256"}"#,
    )
    .unwrap();
    assert!(!format!("{response:?}").contains("private-"));
    assert_eq!(response.api_key.as_str(), "private-api-key");
}

fn id(value: &str) -> ClientOrderId {
    ClientOrderId::new(value).unwrap()
}
fn oco() -> margin::rest_requests::MarginAccountNewOco {
    margin::rest_requests::MarginAccountNewOco::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("SELL")
        .quantity(Decimal::ONE)
        .price(Decimal::new(120, 0))
        .stop_price(Decimal::new(90, 0))
}
fn oto() -> margin::rest_requests::MarginAccountNewOto {
    margin::rest_requests::MarginAccountNewOto::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .working_type("LIMIT")
        .working_side("BUY")
        .working_price(Decimal::new(100, 0))
        .working_quantity(Decimal::ONE)
        .working_time_in_force("GTC")
        .pending_type("MARKET")
        .pending_side("BUY")
        .pending_quantity(Decimal::new(2, 0))
}
fn otoco() -> margin::rest_requests::MarginAccountNewOtoco {
    margin::rest_requests::MarginAccountNewOtoco::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .working_type("LIMIT")
        .working_side("BUY")
        .working_price(Decimal::new(100, 0))
        .working_quantity(Decimal::ONE)
        .working_time_in_force("GTC")
        .pending_side("SELL")
        .pending_quantity(Decimal::ONE)
        .pending_above_type("LIMIT_MAKER")
        .pending_above_price(Decimal::new(120, 0))
        .pending_below_type("STOP_LOSS")
        .pending_below_stop_price(Decimal::new(90, 0))
}
#[test]
fn every_placement_leg_requires_its_own_caller_identity() {
    for omitted in 0..3 {
        let mut request = oco();
        if omitted != 0 {
            request = request.list_client_order_id(id("oco-list"));
        }
        if omitted != 1 {
            request = request.limit_client_order_id(id("oco-limit"));
        }
        if omitted != 2 {
            request = request.stop_client_order_id(id("oco-stop"));
        }
        assert!(request.build().is_err(), "missing OCO leg {omitted}");
        let mut request = oto();
        if omitted != 0 {
            request = request.list_client_order_id(id("oto-list"));
        }
        if omitted != 1 {
            request = request.working_client_order_id(id("oto-working"));
        }
        if omitted != 2 {
            request = request.pending_client_order_id(id("oto-pending"));
        }
        assert!(request.build().is_err(), "missing OTO leg {omitted}");
    }
    for omitted in 0..4 {
        let mut request = otoco();
        if omitted != 0 {
            request = request.list_client_order_id(id("otoco-list"));
        }
        if omitted != 1 {
            request = request.working_client_order_id(id("otoco-working"));
        }
        if omitted != 2 {
            request = request.pending_above_client_order_id(id("otoco-above"));
        }
        if omitted != 3 {
            request = request.pending_below_client_order_id(id("otoco-below"));
        }
        assert!(request.build().is_err(), "missing OTOCO leg {omitted}");
    }
    assert!(
        oco()
            .list_client_order_id(id("oco-list"))
            .limit_client_order_id(id("oco-limit"))
            .stop_client_order_id(id("oco-stop"))
            .build()
            .is_ok()
    );
    assert!(
        oto()
            .list_client_order_id(id("oto-list"))
            .working_client_order_id(id("oto-working"))
            .pending_client_order_id(id("oto-pending"))
            .build()
            .is_ok()
    );
    assert!(
        otoco()
            .list_client_order_id(id("otoco-list"))
            .working_client_order_id(id("otoco-working"))
            .pending_above_client_order_id(id("otoco-above"))
            .pending_below_client_order_id(id("otoco-below"))
            .build()
            .is_ok()
    );
}
#[tokio::test]
async fn per_ip_leverage_cap_applies_across_accounts_and_clones() {
    let budgets = binance_client::Budgets::new(binance_client::BudgetLimits::spot()).unwrap();
    let fixture = HttpFixture::new(200, "", r#"{"success":true}"#, None, false).await;
    let first = margin::RestClient::new(
        config()
            .budgets(budgets.clone())
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let second = margin::RestClient::new(
        config()
            .budgets(budgets.for_account())
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let request = margin::rest_requests::AdjustCrossMarginMaxLeverage::new()
        .max_leverage(3)
        .build()
        .unwrap();
    assert!(
        first
            .adjust_cross_margin_max_leverage(&request, deadline())
            .await
            .unwrap()
            .data
            .success
    );
    for client in [&first, &second] {
        assert!(
            matches!(client.adjust_cross_margin_max_leverage(&request,deadline()).await,Err(Error::Admission{retry_after})if retry_after==std::time::Duration::from_secs(59))
        );
    }
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
#[test]
fn inventory_decimal_and_nullable_pending_order_evidence_are_preserved() {
    let value: margin::rest_models::QueryMarginAvailableInventoryResponse = serde_json::from_str(
        r#"{"assets":{"BTC":"0.0000000000000000000000000001","ETH":"2"},"updateTime":123}"#,
    )
    .unwrap();
    assert_eq!(
        value.assets.amounts()[&Asset::new("BTC").unwrap()],
        Decimal::new(1, 28)
    );
    assert!(
        serde_json::from_str::<margin::rest_models::QueryMarginAvailableInventoryResponse>(
            r#"{"assets":{"BTC":"NaN"},"updateTime":123}"#
        )
        .is_err()
    );
    let body = r#"[{"orderListId":1,"contingencyType":"OTO","listStatusType":"EXEC_STARTED","listOrderStatus":"EXECUTING","listClientOrderId":"caller-list","transactionTime":123,"symbol":"BTCUSDT","orders":[{"symbol":"BTCUSDT","orderId":null,"status":"PENDING_NEW","clientOrderId":"caller-pending"}]}]"#;
    let value: margin::rest_models::QueryMarginAccountsOpenOtootocoOrderListsResponse =
        serde_json::from_str(body).unwrap();
    assert_eq!(value[0].orders[0].order_id, None);
    assert_eq!(
        value[0].orders[0].client_order_id.as_str(),
        "caller-pending"
    );
}
#[test]
fn native_partial_events_preserve_margin_liabilities_and_fail_missing_evidence() {
    let payload=margin::event_payloads::user_payload(serde_json::json!({"e":"USER_LIABILITY_CHANGE","E":123,"a":"ETH","t":"BORROW","p":"0.001","i":"0.00001"})).unwrap();
    let margin::event_payloads::UserPayload::UserLiabilityChange(value) = payload else {
        panic!("liability")
    };
    assert_eq!(value.a.as_str(), "ETH");
    assert_eq!(value.p, Decimal::new(1, 3));
    assert_eq!(value.i, Decimal::new(1, 5));
    assert!(
        margin::event_payloads::user_payload(
            serde_json::json!({"e":"USER_LIABILITY_CHANGE","E":123,"a":"ETH","t":"BORROW","p":"1"})
        )
        .is_err()
    );
    let payload=margin::event_payloads::user_payload(serde_json::json!({"e":"outboundAccountPosition","E":123,"u":120,"B":[{"a":"BTC","f":"1","l":"2"}]})).unwrap();
    let margin::event_payloads::UserPayload::OutboundAccountPosition(value) = payload else {
        panic!("partial balances")
    };
    assert_eq!(value.upper_b.len(), 1);
    assert_eq!(value.upper_b[0].a.as_str(), "BTC");
    assert_eq!(value.upper_b[0].f, Decimal::ONE);
}
fn issued_token() -> margin::ListenToken {
    let receipt =
        serde_json::from_str(r#"{"token":"synthetic-margin-token","expirationTime":2000}"#)
            .unwrap();
    let result = margin::ListenToken::new(
        margin::AccountScope::Isolated(Symbol::new("BTCUSDT").unwrap()),
        receipt,
    )
    .unwrap();
    assert!(!format!("{result:?}").contains("synthetic-margin-token"));
    result
}
#[tokio::test]
async fn token_issuance_uses_api_key_only_and_retains_native_scope() {
    for (isolated, has_symbol) in [(false, false), (true, true), (false, true)] {
        let mut fixture = HttpFixture::new(
            200,
            "X-SAPI-USED-UID-WEIGHT-1M: 1\r\n",
            r#"{"token":"synthetic-issued-token","expirationTime":2000}"#,
            None,
            false,
        )
        .await;
        let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let mut request = margin::rest_requests::CreateUserListenToken::new().validity(1000);
        if has_symbol {
            request = request
                .is_isolated(isolated)
                .symbol(Symbol::new("BTCUSDT").unwrap());
        }
        let response = client
            .create_user_listen_token(&request, deadline())
            .await
            .unwrap();
        let scope = if isolated {
            margin::AccountScope::Isolated(Symbol::new("BTCUSDT").unwrap())
        } else {
            margin::AccountScope::Cross
        };
        assert_eq!(response.data.scope, scope);
        assert_eq!(response.data.receipt.expiration_time, 2000);
        assert_eq!(
            response.data.receipt.token.as_str(),
            "synthetic-issued-token"
        );
        assert!(!format!("{:?}", response.data).contains("synthetic-issued-token"));
        assert_eq!(response.meta.rates.counters["x-sapi-used-uid-weight-1m"], 1);
        let wire = fixture.requests.recv().await.unwrap();
        assert!(wire.starts_with("POST /sapi/v1/userListenToken HTTP/1.1"));
        assert!(
            wire.to_ascii_lowercase()
                .contains("x-mbx-apikey: synthetic-key")
        );
        assert!(wire.contains("validity=1000"));
        assert_eq!(wire.contains("isIsolated=true"), isolated);
        assert_eq!(wire.contains("symbol=BTCUSDT"), has_symbol);
        assert_eq!(wire.contains("isIsolated=false"), has_symbol && !isolated);
        assert!(!wire.contains("signature="));
        assert!(!wire.contains("timestamp="));
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}
#[test]
fn token_scope_and_validity_are_checked_without_fabricating_defaults() {
    let request = margin::rest_requests::CreateUserListenToken::new();
    assert!(request.clone().build().is_ok());
    assert!(request.clone().is_isolated(true).build().is_err());
    assert!(
        request
            .clone()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .build()
            .is_ok()
    );
    assert!(
        request
            .clone()
            .is_isolated(false)
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .build()
            .is_ok()
    );
    assert!(request.clone().validity(0).build().is_err());
    assert!(request.clone().validity(86_400_001).build().is_err());
    assert!(request.validity(86_400_000).build().is_ok());
}
#[tokio::test]
async fn token_issuance_without_credentials_refuses_before_network_send() {
    let fixture = HttpFixture::new(
        200,
        "",
        r#"{"token":"synthetic-issued-token","expirationTime":2000}"#,
        None,
        false,
    )
    .await;
    let client = margin::RestClient::new(
        margin::Config::with_pools(&binance_client::WeightPools::new())
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let error = client
        .create_user_listen_token(
            &margin::rest_requests::CreateUserListenToken::new(),
            deadline(),
        )
        .await
        .unwrap_err();
    assert!(matches!(error, Error::CredentialsRequired));
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}
#[tokio::test]
async fn malformed_issued_token_preserves_ambiguous_response_evidence_without_retry() {
    for body in [
        r#"{"token":"","expirationTime":2000}"#,
        r#"{"token":"synthetic-issued-token","expirationTime":-1}"#,
        r#"{"token":"synthetic-issued-token"}"#,
    ] {
        let fixture =
            HttpFixture::new(200, "X-SAPI-USED-UID-WEIGHT-1M: 1\r\n", body, None, false).await;
        let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client
            .create_user_listen_token(
                &margin::rest_requests::CreateUserListenToken::new(),
                deadline(),
            )
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown));
        assert!(!format!("{error:?}").contains("synthetic-issued-token"));
        let Error::Transport {
            meta: Some(meta), ..
        } = error
        else {
            panic!("token response decode evidence")
        };
        assert_eq!(meta.status, 200);
        assert_eq!(meta.rates.counters["x-sapi-used-uid-weight-1m"], 1);
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}
#[tokio::test]
async fn capacity_keeps_asset_and_isolated_request_provenance() {
    let fixture = HttpFixture::new(
        200,
        "",
        r#"{"amount":"1.001","borrowLimit":"2"}"#,
        None,
        false,
    )
    .await;
    let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let request = margin::rest_requests::QueryMaxBorrow::new()
        .asset(Asset::new("BTC").unwrap())
        .isolated_symbol(Symbol::new("BTCUSDT").unwrap())
        .build()
        .unwrap();
    let capacity = client
        .query_max_borrow(&request, deadline())
        .await
        .unwrap()
        .data;
    assert_eq!(capacity.asset.as_str(), "BTC");
    assert_eq!(
        capacity.isolated_symbol.as_ref().unwrap().as_str(),
        "BTCUSDT"
    );
    assert_eq!(capacity.capacity.amount, Decimal::new(1001, 3));
    fixture.finish().await;
}

mod sockets {
    use super::*;
    use binance_client::{Clock, RequestId};
    use futures_util::{SinkExt, StreamExt};
    use serde_json::{Value, json};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::{
        net::{TcpListener, TcpStream},
        sync::oneshot,
        time::Duration,
    };
    use tokio_websockets::{Limits, Message, ServerBuilder, WebSocketStream};
    async fn listener() -> (TcpListener, String) {
        let l = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("ws://{}", l.local_addr().unwrap());
        (l, url)
    }
    async fn accept(l: TcpListener) -> WebSocketStream<TcpStream> {
        ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(l.accept().await.unwrap().0)
            .await
            .unwrap()
            .1
    }
    async fn finish(mut ws: WebSocketStream<TcpStream>) {
        while let Some(m) = ws.next().await {
            if m.unwrap().is_close() {
                ws.flush().await.unwrap();
                break;
            }
        }
    }
    struct ExpiringClock(AtomicUsize);
    impl Clock for ExpiringClock {
        fn now_millis(&self) -> Result<u64, Error> {
            Ok(if self.0.fetch_add(1, Ordering::SeqCst) < 2 {
                1000
            } else {
                2000
            })
        }
    }
    #[tokio::test]
    async fn token_expiring_during_admission_cannot_send_a_late_subscription() {
        let request = margin::SubscribeToken::new(&issued_token()).unwrap();
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            assert!(ws.next().await.unwrap().unwrap().is_close());
            ws.flush().await.unwrap();
        });
        let config = margin::WsConfig::with_pools(&binance_client::WeightPools::new())
            .unwrap()
            .api_url(&url)
            .unwrap()
            .clock(Arc::new(ExpiringClock(AtomicUsize::new(0))));
        let (client, mut events, driver) = margin::WsClient::connect(config).await.unwrap();
        let driver = tokio::spawn(driver.run());
        assert!(matches!(
            events.recv().await,
            Some(margin::ApiEvent::Established(_))
        ));
        let error = client
            .subscribe_listen_token(
                &request,
                RequestId::new("expires-during-admission").unwrap(),
                deadline(),
            )
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::NotSent));
        client.close().await.unwrap();
        assert!(
            matches!(events.recv().await,Some(margin::ApiEvent::Retired(g))if g==client.generation())
        );
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
    #[tokio::test]
    async fn late_subscription_answer_keeps_source_generation_and_accepted_event_prefix() {
        let request = margin::SubscribeToken::new(&issued_token()).unwrap();
        let (listener, url) = listener().await;
        let (sent, received) = oneshot::channel();
        let (answer, permit) = oneshot::channel();
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            let frame = ws.next().await.unwrap().unwrap();
            let frame: Value = serde_json::from_slice(frame.as_payload()).unwrap();
            assert_eq!(frame["method"], "userDataStream.subscribe.listenToken");
            assert_eq!(frame["params"]["listenToken"], "synthetic-margin-token");
            sent.send(()).unwrap();
            permit.await.unwrap();
            ws.send(Message::text(json!({"id":frame["id"],"status":200,"result":{"subscriptionId":7,"expirationTime":2000},"rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"count":2}]}).to_string())).await.unwrap();
            ws.send(Message::text(json!({"subscriptionId":7,"event":{"e":"USER_LIABILITY_CHANGE","E":123,"a":"BTC","t":"REPAY","p":"1","i":"0.01"}}).to_string())).await.unwrap();
            ws.send(Message::text(
                json!({"subscriptionId":7,"event":{"e":"eventStreamTerminated","E":124}})
                    .to_string(),
            ))
            .await
            .unwrap();
            finish(ws).await;
        });
        let config = margin::WsConfig::with_pools(&binance_client::WeightPools::new())
            .unwrap()
            .api_url(&url)
            .unwrap()
            .clock(Arc::new(FixedClock(1000)));
        let (client, mut events, driver) = margin::WsClient::connect(config).await.unwrap();
        let generation = client.generation();
        let driver = tokio::spawn(driver.run());
        assert!(
            matches!(events.recv().await,Some(margin::ApiEvent::Established(g))if g==generation)
        );
        let sender = client.clone();
        let call = tokio::spawn(async move {
            sender
                .subscribe_listen_token(
                    &request,
                    RequestId::new("late-margin").unwrap(),
                    tokio::time::Instant::now() + Duration::from_secs(1),
                )
                .await
        });
        received.await.unwrap();
        // Pause only after real I/O confirms acceptance, then resume before any
        // awaited I/O or task completion can advance the 24-hour generation timer.
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(1)).await;
        tokio::time::resume();
        assert_eq!(
            call.await.unwrap().unwrap_err().outcome(),
            Some(Outcome::Unknown)
        );
        answer.send(()).unwrap();
        let event = events.recv().await.unwrap();
        let margin::ApiEvent::Late {
            generation: g,
            id,
            result,
        } = event
        else {
            panic!("late answer: {event:?}")
        };
        assert_eq!(g, generation);
        assert_eq!(id.as_str(), "late-margin");
        let response = result.unwrap();
        assert_eq!(response.data.subscription_id, 7);
        assert_eq!(response.meta.status, 200);
        assert_eq!(
            response.meta.operation,
            "userDataStreamSubscribeListenToken"
        );
        assert_eq!(response.meta.rates.counters["x-mbx-used-weight-1m"], 2);
        assert!(
            matches!(events.recv().await,Some(margin::ApiEvent::UserData{generation:g,subscription_id:7,payload:margin::event_payloads::UserPayload::UserLiabilityChange(_),})if g==generation)
        );
        assert!(
            matches!(events.recv().await,Some(margin::ApiEvent::UserData{generation:g,subscription_id:7,payload:margin::event_payloads::UserPayload::EventStreamTerminated{event_time:124},})if g==generation)
        );
        client.close().await.unwrap();
        assert!(matches!(events.recv().await,Some(margin::ApiEvent::Retired(g))if g==generation));
        assert!(events.recv().await.is_none());
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
    #[tokio::test]
    async fn risk_socket_drains_accepted_prefix_before_malformed_gap_and_retirement() {
        let (listener, url) = listener().await;
        let server = tokio::spawn(async move {
            let mut ws = accept(listener).await;
            for (event_time, principal) in [(123, "1"), (124, "2")] {
                ws.send(Message::text(json!({"e":"USER_LIABILITY_CHANGE","E":event_time,"a":"ETH","t":"BORROW","p":principal,"i":"0"}).to_string())).await.unwrap();
            }
            ws.send(Message::text(json!({"e":"USER_LIABILITY_CHANGE","E":125,"a":"ETH","t":"BORROW","p":"NaN","i":"0"}).to_string())).await.unwrap();
            finish(ws).await;
        });
        let config = margin::WsConfig::with_pools(&binance_client::WeightPools::new())
            .unwrap()
            .risk_url(&url)
            .unwrap()
            .clock(Arc::new(FixedClock(1000)));
        let (mut risk, driver) = margin::RiskStream::connect(
            config,
            &binance_client::SensitiveString::new("synthetic-risk-key"),
        )
        .await
        .unwrap();
        let generation = risk.generation();
        let driver = tokio::spawn(driver.run());
        assert!(
            matches!(risk.recv().await,Some(margin::RiskEvent::Established(g))if g==generation)
        );
        for (event_time, principal) in [(123, Decimal::ONE), (124, Decimal::new(2, 0))] {
            let event = risk.recv().await.unwrap();
            let margin::RiskEvent::Data {
                generation: g,
                payload: margin::event_payloads::UserPayload::UserLiabilityChange(value),
            } = event
            else {
                panic!("risk liability")
            };
            assert_eq!(g, generation);
            assert_eq!(value.upper_e, event_time);
            assert_eq!(value.p, principal);
            assert_eq!(value.a.as_str(), "ETH");
        }
        assert!(
            matches!(risk.recv().await,Some(margin::RiskEvent::Gap{generation:g,error:Error::Gap("malformed Margin user event"),})if g==generation)
        );
        assert!(matches!(risk.recv().await,Some(margin::RiskEvent::Retired(g))if g==generation));
        assert!(risk.recv().await.is_none());
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
}
#[test]
fn special_key_rsa_has_explicit_absent_secret_and_future_commission_is_unavailable() {
    let rsa: margin::rest_models::CreateSpecialKeyResponse =
        serde_json::from_str(r#"{"apiKey":"private-api-key","secretKey":null,"type":"RSA"}"#)
            .unwrap();
    assert!(rsa.secret_key.is_none());
    let fill: margin::rest_models::MarginAccountNewOrderResponseFillsItem = serde_json::from_str(
        r#"{"price":"1","qty":"2","commission":null,"commissionAsset":null,"tradeId":3}"#,
    )
    .unwrap();
    assert!(fill.commission.is_none());
    assert!(fill.commission_asset.is_none());
}
#[test]
fn retired_cross_pro_catalog_operation_is_excluded_from_active_bindings() {
    let snapshot: serde_json::Value =
        serde_json::from_str(include_str!("../schema/margin-coverage.json")).unwrap();
    assert!(
        snapshot["rest"]
            .as_array()
            .unwrap()
            .iter()
            .all(|o| o["name"] != "queryLiabilityCoinLeverageBracketInCrossMarginProMode")
    );
}
#[tokio::test]
async fn documented_empty_special_key_response_is_distinct_from_corruption_and_unknown_order_body()
{
    let request = margin::rest_requests::DeleteSpecialKey::new()
        .api_key(binance_client::SensitiveString::new(
            "synthetic-managed-key",
        ))
        .build()
        .unwrap();
    let fixture = HttpFixture::new(200, "", "", None, false).await;
    let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let response = client
        .delete_special_key(&request, deadline())
        .await
        .unwrap();
    assert_eq!(response.meta.status, 200);
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
    for (status, body) in [(200, "{"), (503, "")] {
        let fixture = HttpFixture::new(status, "", body, None, false).await;
        let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        assert_eq!(
            client
                .delete_special_key(&request, deadline())
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::Unknown)
        );
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
    let fixture = HttpFixture::new(200, "", "", None, false).await;
    let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    assert_eq!(
        client
            .margin_account_new_order(&order().build().unwrap(), deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::Unknown)
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
#[tokio::test]
async fn upcoming_open_order_list_endpoint_refuses_before_effective_utc_date() {
    let fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let request = margin::rest_requests::QueryMarginAccountsOpenOtootocoOrderLists::new();
    let early = margin::RestClient::new(
        config()
            .clock(Arc::new(FixedClock(1_791_935_999_999)))
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        early
            .query_margin_accounts_open_otootoco_order_lists(&request, deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::NotSent)
    );
    assert_eq!(fixture.connections_accepted(), 0);
    let eligible = margin::RestClient::new(
        config()
            .clock(Arc::new(FixedClock(1_791_936_000_000)))
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    assert!(
        eligible
            .query_margin_accounts_open_otootoco_order_lists(&request, deadline())
            .await
            .unwrap()
            .data
            .is_empty()
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
#[tokio::test]
async fn definitive_margin_codes_match_independent_product_snapshot() {
    let snapshot: serde_json::Value =
        serde_json::from_str(include_str!("../schema/margin-error-codes.json")).unwrap();
    for (field, outcome) in [
        ("definitive", Outcome::Rejected),
        ("never_definitive", Outcome::Unknown),
    ] {
        for code in snapshot[field].as_array().unwrap() {
            let fixture =
                HttpFixture::new(400, "", &format!("{{\"code\":{code}}}"), None, false).await;
            let client = margin::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
            assert_eq!(
                client
                    .margin_account_new_order(&order().build().unwrap(), deadline())
                    .await
                    .unwrap_err()
                    .outcome(),
                Some(outcome),
                "code {code}"
            );
            assert_eq!(fixture.connections_accepted(), 1);
            fixture.finish().await;
        }
    }
}

fn quota(limit: u64, count: u64) -> margin::rest_models::QueryCurrentMarginOrderCountUsageResponse {
    serde_json::from_value(serde_json::json!([{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":limit,"count":count}])).unwrap()
}
#[tokio::test]
async fn order_placements_require_native_authority_but_reads_remain_available() {
    let fixture = HttpFixture::new(
        200,
        "",
        r#"[{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":2,"count":0}]"#,
        None,
        false,
    )
    .await;
    let client = margin::RestClient::new(
        margin::Config::with_pools(&binance_client::WeightPools::new())
            .unwrap()
            .budgets(binance_client::Budgets::new(binance_client::BudgetLimits::spot()).unwrap())
            .clock(Arc::new(FixedClock(1000)))
            .credentials(Credentials::hmac("synthetic", "synthetic").unwrap())
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let error = client
        .margin_account_new_order(&order().build().unwrap(), deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(fixture.connections_accepted(), 0);
    let evidence = client
        .query_current_margin_order_count_usage(
            &margin::rest_requests::QueryCurrentMarginOrderCountUsage::new()
                .build()
                .unwrap(),
            deadline(),
        )
        .await
        .unwrap();
    assert_eq!(evidence.data[0].limit, 2);
    client.observe_order_limits(&evidence.data).unwrap();
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}
#[tokio::test]
async fn native_order_limits_share_account_owners_and_apply_remote_count_floors() {
    let fixture = HttpFixture::new(
        200,
        "X-MBX-ORDER-COUNT-10S: 2\r\n",
        r#"{"symbol":"BTCUSDT","orderId":1,"clientOrderId":"caller-margin-1","isIsolated":false,"transactTime":1000}"#,
        None,
        false,
    )
    .await;
    let budgets = binance_client::Budgets::new(binance_client::BudgetLimits::spot()).unwrap();
    let configured = config()
        .budgets(budgets.clone())
        .order_limits(&quota(2, 0))
        .unwrap()
        .rest_url(&fixture.url)
        .unwrap();
    let first = margin::RestClient::new(configured.clone()).unwrap();
    let second = margin::RestClient::new(configured.clone()).unwrap();
    first
        .margin_account_new_order(&order().build().unwrap(), deadline())
        .await
        .unwrap();
    assert!(matches!(
        second
            .margin_account_new_order(&order().build().unwrap(), deadline())
            .await,
        Err(Error::Admission { .. })
    ));
    // A malformed complete response cannot discard the accepted remote count floor.
    let malformed = serde_json::from_value(serde_json::json!([
        {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":100,"count":0},
        {"rateLimitType":"ORDERS","interval":"FORTNIGHT","intervalNum":1,"limit":100,"count":0}
    ]))
    .unwrap();
    assert!(first.observe_order_limits(&malformed).is_err());
    assert!(matches!(
        first
            .margin_account_new_order(&order().build().unwrap(), deadline())
            .await,
        Err(Error::Admission { .. })
    ));
    let different = margin::RestClient::new(configured.budgets(budgets.for_account())).unwrap();
    assert_eq!(
        different
            .margin_account_new_order(&order().build().unwrap(), deadline())
            .await
            .unwrap_err()
            .outcome(),
        Some(Outcome::NotSent)
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn order_lists_reserve_all_documented_legs_before_any_send() {
    let fixture = HttpFixture::new(200, "", "{}", None, false).await;
    let client = margin::RestClient::new(
        config()
            .order_limits(&quota(1, 0))
            .unwrap()
            .rest_url(&fixture.url)
            .unwrap(),
    )
    .unwrap();
    let request = oco()
        .list_client_order_id(id("list"))
        .limit_client_order_id(id("limit"))
        .stop_client_order_id(id("stop"))
        .build()
        .unwrap();
    assert!(matches!(
        client.margin_account_new_oco(&request, deadline()).await,
        Err(Error::Admission { .. })
    ));
    let request = oto()
        .list_client_order_id(id("list"))
        .working_client_order_id(id("working"))
        .pending_client_order_id(id("pending"))
        .build()
        .unwrap();
    assert!(matches!(
        client.margin_account_new_oto(&request, deadline()).await,
        Err(Error::Admission { .. })
    ));
    client.observe_order_limits(&quota(2, 0)).unwrap();
    let request = otoco()
        .list_client_order_id(id("list"))
        .working_client_order_id(id("working"))
        .pending_above_client_order_id(id("above"))
        .pending_below_client_order_id(id("below"))
        .build()
        .unwrap();
    assert!(matches!(
        client.margin_account_new_otoco(&request, deadline()).await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}

#[test]
fn native_margin_identities_preserve_numeric_text_and_individual_sentinel() {
    for encoded in ["123", "\"00123\""] {
        let identity: margin::OrderId = serde_json::from_str(encoded).unwrap();
        assert_eq!(identity.value(), 123);
        assert_eq!(serde_json::to_string(&identity).unwrap(), encoded);
        let request = margin::rest_requests::QueryMarginAccountsOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .order_id(identity)
            .build()
            .unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(
            value["orderId"],
            serde_json::from_str::<serde_json::Value>(encoded).unwrap()
        );
    }
    let sentinel: margin::OrderListId = serde_json::from_str("-1").unwrap();
    assert_eq!(
        sentinel.kind().unwrap(),
        margin::OrderListKind::IndividualOrder
    );
    assert_eq!(serde_json::to_string(&sentinel).unwrap(), "-1");
    assert_eq!(
        margin::OrderListId::new(0).kind().unwrap(),
        margin::OrderListKind::List
    );
    let unknown = margin::OrderListId::from_text("-0002").unwrap();
    let error = unknown.kind().unwrap_err();
    assert_eq!(error.identity(), &unknown);
    assert_eq!(serde_json::to_string(&unknown).unwrap(), "\"-0002\"");
    for malformed in ["\"\"", "\"1.2\"", "\" 1\"", "\"9223372036854775808\""] {
        assert!(serde_json::from_str::<margin::OrderId>(malformed).is_err());
    }
    assert!(margin::ClientOrderId::new("").is_err());
    assert!(margin::ClientOrderId::new("caller\nidentity").is_err());
    let lexical = margin::ClientOrderId::new("native margin/id:1").unwrap();
    assert_eq!(lexical.as_str(), "native margin/id:1");
}
