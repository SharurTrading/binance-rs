// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Local HTTP fault injection: outcome evidence, signing, and pre-wire refusal.

#[cfg(test)]
mod support;
use binance_client::usdm::{
    RestClient,
    rest_models::PlaceMultipleOrdersBatchOrdersInputItem,
    rest_requests::{CancelOrder, CheckServerTime, NewOrder, PlaceMultipleOrders},
    wire::BatchResult,
};
use binance_client::{
    BudgetLimits, Budgets, ClientOrderId, Decimal, Error, Outcome, Signer, Symbol,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
#[cfg(test)]
mod tests {
    use super::*;
    use support::{HttpFixture, config, deadline};

    fn order() -> NewOrder {
        NewOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::new(1, 3))
            .new_client_order_id(ClientOrderId::new("fixture/order:1").unwrap())
    }
    fn cancel() -> CancelOrder {
        CancelOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .orig_client_order_id(ClientOrderId::new("fixture/order:1").unwrap())
    }

    #[tokio::test]
    async fn signed_mutation_has_exact_wire_bytes_and_no_secret_in_errors() {
        let mut fixture=HttpFixture::new(200,"","{\"orderId\":7,\"clientOrderId\":\"fixture/order:1\",\"executedQty\":\"0.001000000000000000000001\"}",None,false).await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let response = client.new_order(&order(), deadline()).await.unwrap();
        assert_eq!(response.data.order_id, 7);
        assert_eq!(
            response.data.executed_qty.unwrap(),
            Decimal::from_str_exact("0.001000000000000000000001").unwrap()
        );
        let request = fixture.requests.recv().await.unwrap();
        assert!(request.starts_with("POST /fapi/v1/order HTTP/1.1"));
        assert!(
            request
                .to_lowercase()
                .contains("x-mbx-apikey: synthetic-api-key")
        );
        let body = request.split("\r\n\r\n").nth(1).unwrap();
        let (payload, signature) = body.rsplit_once("&signature=").unwrap();
        assert_eq!(
            payload,
            "newClientOrderId=fixture%2Forder%3A1&quantity=0.001&side=BUY&symbol=BTCUSDT&timestamp=1700000001000&type=MARKET"
        );
        let key = aws_lc_rs::hmac::Key::new(aws_lc_rs::hmac::HMAC_SHA256, b"synthetic-secret");
        let expected = aws_lc_rs::hmac::sign(&key, payload.as_bytes())
            .as_ref()
            .iter()
            .fold(String::new(), |mut text, b| {
                use std::fmt::Write as _;
                write!(&mut text, "{b:02x}").unwrap();
                text
            });
        assert_eq!(signature, expected);
        assert_eq!(fixture.attempts(), 1);
        fixture.finish().await;
    }

    #[tokio::test]
    async fn response_body_truncation_is_unknown_once_and_retains_headers() {
        let fixture = HttpFixture::new(
            200,
            "X-MBX-USED-WEIGHT-1M: 22\r\nRetry-After: 2\r\n",
            "{\"orderId\":",
            Some(100),
            false,
        )
        .await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client.new_order(&order(), deadline()).await.unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown));
        if let Error::Transport {
            meta: Some(meta), ..
        } = error
        {
            assert_eq!(meta.status, 200);
            assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 22);
            assert_eq!(meta.rates.retry_after, Some(Duration::from_secs(2)));
        } else {
            panic!("header evidence lost");
        }
        assert!(matches!(
            client.new_order(&order(), deadline()).await,
            Err(Error::Admission { .. })
        ));
        assert_eq!(fixture.attempts(), 1);
        fixture.finish().await;
    }

    #[tokio::test]
    async fn stalled_body_obeys_transport_timeout_and_never_retries() {
        let fixture = HttpFixture::new(200, "", "{", Some(100), true).await;
        let client = RestClient::new(
            config()
                .rest_url(&fixture.url)
                .unwrap()
                .timeout(Duration::from_millis(200))
                .unwrap(),
        )
        .unwrap();
        let result = tokio::time::timeout(
            Duration::from_secs(2),
            client.new_order(&order(), deadline()),
        )
        .await
        .expect("body must honor per-attempt timeout");
        assert_eq!(result.unwrap_err().outcome(), Some(Outcome::Unknown));
        assert_eq!(fixture.attempts(), 1);
        fixture.finish().await;
    }

    #[tokio::test]
    async fn sequential_calls_reuse_one_keep_alive_connection() {
        let mut fixture = HttpFixture::keep_alive(200, "", "{\"orderId\":7}", None).await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        client.new_order(&order(), deadline()).await.unwrap();
        client.cancel_order(&cancel(), deadline()).await.unwrap();
        let first = fixture.requests.recv().await.unwrap();
        let second = fixture.requests.recv().await.unwrap();
        assert!(first.starts_with("POST /fapi/v1/order"));
        assert!(second.starts_with("DELETE /fapi/v1/order"));
        assert_eq!(
            fixture.attempts(),
            1,
            "calls must share the pooled connection"
        );
        fixture.finish().await;
    }
    #[tokio::test]
    async fn lost_acknowledgment_on_reused_connection_is_visible_and_never_resent() {
        // Drift pin for the pooled-transport safety argument: hyper may only
        // resend a request that never started on the wire. A request the
        // venue received (server read it fully, then closed without
        // responding) must surface as a visible unknown outcome exactly once,
        // with no second wire attempt on a fresh connection.
        let mut fixture = HttpFixture::keep_alive(200, "", "{\"orderId\":7}", Some(2)).await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        client.new_order(&order(), deadline()).await.unwrap();
        let error = client
            .cancel_order(&cancel(), deadline())
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(Outcome::Unknown));
        let first = fixture.requests.recv().await.unwrap();
        let second = fixture.requests.recv().await.unwrap();
        assert!(first.starts_with("POST /fapi/v1/order"));
        assert!(second.starts_with("DELETE /fapi/v1/order"));
        assert_eq!(
            fixture.attempts(),
            1,
            "a lost acknowledgment must not redial"
        );
        assert!(
            fixture.requests.try_recv().is_err(),
            "no resend may follow a lost acknowledgment"
        );
        fixture.finish().await;
    }

    #[tokio::test]
    async fn futures_503_classification_matches_the_pinned_general_info_snapshot() {
        // Machine-checks schema/futures-general-info.json: every documented 503
        // variant must classify exactly as its pinned documented outcome, so
        // neither an unevidenced message clause nor snapshot drift passes CI.
        let snapshot = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/futures-general-info.json"
        ))
        .unwrap();
        let snapshot: serde_json::Value = serde_json::from_str(&snapshot).unwrap();
        let variants = snapshot["http_return_codes"]["503_variants"]
            .as_array()
            .unwrap();
        assert!(variants.len() >= 4);
        for variant in variants {
            let message = variant["msg"].as_str().unwrap();
            let expected = match variant["documented_outcome"].as_str().unwrap() {
                "failure" => Outcome::Rejected,
                "execution-unknown" => Outcome::Unknown,
                other => panic!("unknown pinned outcome {other}"),
            };
            let body = serde_json::json!({"code": variant.get("code").cloned().unwrap_or(serde_json::json!(-1000)), "msg": message}).to_string();
            let fixture = HttpFixture::new(503, "", &body, None, false).await;
            let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
            let error = client.new_order(&order(), deadline()).await.unwrap_err();
            assert_eq!(error.outcome(), Some(expected), "variant {message}");
            assert_eq!(fixture.attempts(), 1);
            fixture.finish().await;
        }
        // A 503 body without a documented definitive message or code stays
        // execution-unknown for a mutation, including the documented 5XX
        // retry-later message and a bare unclassified code.
        for body in [
            "{\"code\":-1000,\"msg\":\"Request occur unknown error.\"}",
            "{\"code\":-1000}",
        ] {
            let fixture = HttpFixture::new(503, "", body, None, false).await;
            let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
            let error = client.new_order(&order(), deadline()).await.unwrap_err();
            assert_eq!(error.outcome(), Some(Outcome::Unknown));
            assert_eq!(fixture.attempts(), 1);
            fixture.finish().await;
        }
    }

    #[tokio::test]
    async fn rejection_and_unknown_codes_remain_distinct_and_redacted() {
        for (status, body, outcome) in [
            (
                503,
                "{\"code\":-1000,\"msg\":\"Unknown error, please check your request or try again later.\"}",
                Outcome::Unknown,
            ),
            (503, "{\"msg\":\"Service Unavailable.\"}", Outcome::Rejected),
            (
                400,
                "{\"code\":-1022,\"msg\":\"synthetic-api-key synthetic-secret\"}",
                Outcome::Rejected,
            ),
            (
                400,
                "{\"code\":-1135,\"msg\":\"future unknown code\"}",
                Outcome::Unknown,
            ),
        ] {
            let fixture = HttpFixture::new(status, "", body, None, false).await;
            let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
            let error = client.new_order(&order(), deadline()).await.unwrap_err();
            assert_eq!(error.outcome(), Some(outcome));
            let debug = format!("{error:?}");
            assert!(!debug.contains("synthetic-api-key"));
            assert!(!debug.contains("synthetic-secret"));
            assert_eq!(fixture.attempts(), 1);
            fixture.finish().await;
        }
    }

    #[tokio::test]
    async fn clones_and_accounts_share_the_documented_ip_budget() {
        let fixture = HttpFixture::new(
            200,
            "X-MBX-USED-WEIGHT-1M: 2\r\n",
            "{\"serverTime\":1700000001000}",
            None,
            false,
        )
        .await;
        let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(2)).unwrap();
        let first = RestClient::new(
            config()
                .rest_url(&fixture.url)
                .unwrap()
                .budgets(budgets.clone()),
        )
        .unwrap();
        let second = RestClient::new(
            config()
                .rest_url(&fixture.url)
                .unwrap()
                .budgets(budgets.for_account()),
        )
        .unwrap();
        first
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap();
        assert!(matches!(
            first
                .clone()
                .check_server_time(&CheckServerTime::new(), deadline())
                .await,
            Err(Error::Admission { .. })
        ));
        assert!(matches!(
            second
                .check_server_time(&CheckServerTime::new(), deadline())
                .await,
            Err(Error::Admission { .. })
        ));
        assert_eq!(fixture.attempts(), 1);
        fixture.finish().await;
    }

    #[tokio::test]
    async fn invalid_and_expired_requests_never_reach_the_wire() {
        let fixture = HttpFixture::new(200, "", "{}", None, false).await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        assert_eq!(
            client
                .new_order(&NewOrder::new(), deadline())
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::NotSent)
        );
        assert_eq!(
            client
                .new_order(&order(), tokio::time::Instant::now())
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::NotSent)
        );
        assert_eq!(fixture.attempts(), 0);
        fixture.finish().await;
    }

    #[tokio::test]
    async fn mixed_batch_preserves_order_and_per_member_outcome() {
        let fixture=HttpFixture::new(200,"","[{\"orderId\":7},{\"code\":-2010,\"msg\":\"Order rejected\"},{\"code\":-9999,\"msg\":\"future code\"}]",None,false).await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let batch = (0..3)
            .map(|i| {
                PlaceMultipleOrdersBatchOrdersInputItem::new()
                    .symbol(Symbol::new("BTCUSDT").unwrap())
                    .side("BUY")
                    .type_value("MARKET")
                    .quantity(Decimal::ONE)
                    .new_client_order_id(ClientOrderId::new(format!("fixture-{i}")).unwrap())
            })
            .collect();
        let response = client
            .place_multiple_orders(&PlaceMultipleOrders::new().batch_orders(batch), deadline())
            .await
            .unwrap();
        assert_eq!(response.data.len(), 3);
        assert!(matches!(&response.data[0],BatchResult::Success(v) if v.order_id==7));
        assert!(
            matches!(&response.data[1],BatchResult::Failure(v) if v.outcome()==Outcome::Rejected)
        );
        assert!(
            matches!(&response.data[2],BatchResult::Failure(v) if v.outcome()==Outcome::Unknown)
        );
        fixture.finish().await;
    }

    struct ExternalSigner(Arc<Mutex<Vec<Vec<u8>>>>);
    impl Signer for ExternalSigner {
        fn sign(&self, bytes: &[u8]) -> Result<String, Error> {
            self.0.lock().unwrap().push(bytes.to_vec());
            Ok("fixture+/=".into())
        }
    }
    #[tokio::test]
    async fn external_signature_is_encoded_once_and_delete_body_is_preserved() {
        let mut fixture = HttpFixture::new(200, "", "{\"orderId\":7}", None, false).await;
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let credentials = binance_client::Credentials::external(
            "synthetic",
            Arc::new(ExternalSigner(payloads.clone())),
        )
        .unwrap();
        let client = RestClient::new(
            config()
                .credentials(credentials)
                .rest_url(&fixture.url)
                .unwrap(),
        )
        .unwrap();
        client.cancel_order(&cancel(), deadline()).await.unwrap();
        let request = fixture.requests.recv().await.unwrap();
        assert!(request.starts_with("DELETE /fapi/v1/order"));
        assert!(request.ends_with("&signature=fixture%2B%2F%3D"));
        assert_eq!(
            payloads.lock().unwrap()[0],
            b"origClientOrderId=fixture%2Forder%3A1&symbol=BTCUSDT&timestamp=1700000001000"
        );
        fixture.finish().await;
    }

    #[tokio::test]
    async fn gtd_is_checked_against_injected_venue_time_before_wire() {
        let fixture = HttpFixture::new(200, "", "{\"orderId\":7}", None, false).await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let request = order()
            .type_value("LIMIT")
            .price(Decimal::from(100))
            .time_in_force("GTD")
            .good_till_date(1_700_000_601_000);
        assert_eq!(
            client
                .new_order(&request, deadline())
                .await
                .unwrap_err()
                .outcome(),
            Some(Outcome::NotSent)
        );
        assert_eq!(fixture.attempts(), 0);
        client
            .new_order(&request.good_till_date(1_700_000_602_000), deadline())
            .await
            .unwrap();
        assert_eq!(fixture.attempts(), 1);
        fixture.finish().await;
    }

    #[tokio::test]
    async fn test_order_empty_ack_is_valid_and_does_not_claim_an_executed_order() {
        let fixture = HttpFixture::new(200, "", "{}", None, false).await;
        let client = RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let request = binance_client::usdm::rest_requests::TestOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(ClientOrderId::new("test-only").unwrap());
        assert!(
            client
                .test_order(&request, deadline())
                .await
                .unwrap()
                .data
                .order_id
                .is_none()
        );
        fixture.finish().await;
    }
}
