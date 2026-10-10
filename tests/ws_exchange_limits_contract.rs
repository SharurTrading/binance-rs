// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Spot exchangeInfo result limits govern the same IP pool as REST exchangeInfo.
//! Source (checked 2026-10-11):
//! <https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general>.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]

#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;

use binance_client::{Clock, Error, LimitSource, Outcome, RequestId, WeightPools, spot};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_websockets::{Limits, Message, ServerBuilder};

const START: u64 = 1_700_000_100_000;

struct FixedClock;
impl Clock for FixedClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(START)
    }
}

async fn exchange_server(results: Vec<Value>) -> (String, JoinHandle<usize>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(stream)
            .await
            .unwrap()
            .1;
        let mut received = 0;
        for result in results {
            let request: Value =
                serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
            assert_eq!(request["method"], "exchangeInfo");
            received += 1;
            peer.send(Message::text(
                json!({"id":request["id"],"status":200,"result":result}).to_string(),
            ))
            .await
            .unwrap();
        }
        while let Some(message) = peer.next().await {
            let message = message.unwrap();
            if message.is_close() {
                peer.flush().await.unwrap();
                break;
            }
            assert!(!message.is_text(), "refused request reached the wire");
        }
        received
    });
    (url, server)
}

fn config(pools: &WeightPools) -> spot::Config {
    spot::Config::with_pools(spot::Environment::Demo, pools)
        .unwrap()
        .clock(Arc::new(FixedClock))
}

#[tokio::test]
async fn websocket_exchange_info_adopts_weight_and_raw_limits_for_other_clients() {
    let pools = WeightPools::new();
    let observer = config(&pools);
    assert_eq!(observer.pool_usage().unwrap().request_weight.limit, 6000);
    let (url, server) = exchange_server(vec![json!({"rateLimits":[
        {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":22},
        {"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":1},
        {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":0}
    ]})])
    .await;
    let (client, mut events, driver) =
        spot::WsClient::connect(config(&pools).websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    client
        .exchange_info(
            &spot::ws_requests::ExchangeInfo::new(),
            RequestId::new("limits").unwrap(),
            support::deadline(),
        )
        .await
        .unwrap();
    let usage = observer.pool_usage().unwrap();
    assert_eq!(usage.request_weight.limit, 22);
    assert_eq!(usage.request_weight.source, LimitSource::Stated);
    assert_eq!(usage.request_weight.used, 22); // Handshake 2 + exchangeInfo 20.
    let raw = usage.raw_requests.unwrap();
    assert_eq!(raw.limit, 1);
    assert_eq!(raw.source, LimitSource::Stated);
    assert_eq!(raw.used, 0);

    let venue = support::HttpFixture::new(200, "", "{\"serverTime\":1}", None, false).await;
    let rest = spot::RestClient::new(observer.rest_url(&venue.url).unwrap()).unwrap();
    let error = rest
        .time(&spot::rest_requests::Time::new(), support::deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert!(
        matches!(error, Error::Admission { retry_after } if retry_after == Duration::from_mins(1))
    );
    assert_eq!(venue.connections_accepted(), 0);
    assert!(matches!(
        client
            .time(
                &spot::ws_requests::Time::new(),
                RequestId::new("refused").unwrap(),
                support::deadline()
            )
            .await,
        Err(Error::Admission { .. })
    ));
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
    assert_eq!(server.await.unwrap(), 1);
    venue.finish().await;
}

#[tokio::test]
async fn malformed_websocket_counted_limits_leave_prior_limits_and_charges_intact() {
    let mut malformed = Vec::new();
    for kind in ["REQUEST_WEIGHT", "RAW_REQUESTS"] {
        for limit in [Some(json!(0)), Some(json!(-1)), Some(Value::Null), None] {
            let mut item = json!({"rateLimitType":kind,"interval":"MINUTE","intervalNum":if kind == "RAW_REQUESTS" {5} else {1}});
            if let Some(limit) = limit {
                item["limit"] = limit;
            }
            malformed.push(item);
        }
    }
    for invalid in malformed {
        let pools = WeightPools::new();
        let observer = config(&pools);
        let (url, server) = exchange_server(vec![
            json!({"rateLimits":[
                {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1000},
                {"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":100}
            ]}),
            json!({"rateLimits":[
                {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1},
                {"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":1},
                invalid
            ]}),
        ])
        .await;
        let (client, mut events, driver) =
            spot::WsClient::connect(config(&pools).websocket_url(&url).unwrap())
                .await
                .unwrap();
        let driver = tokio::spawn(driver.run());
        client
            .exchange_info(
                &spot::ws_requests::ExchangeInfo::new(),
                RequestId::new("valid").unwrap(),
                support::deadline(),
            )
            .await
            .unwrap();
        let before = observer.pool_usage().unwrap();
        let error = client
            .exchange_info(
                &spot::ws_requests::ExchangeInfo::new(),
                RequestId::new("invalid").unwrap(),
                support::deadline(),
            )
            .await
            .unwrap_err();
        assert!(
            matches!(
                error,
                Error::Gap("stated rate limit without a positive limit")
            ),
            "{error:?}"
        );
        let after = observer.pool_usage().unwrap();
        assert_eq!(after.request_weight.limit, before.request_weight.limit);
        assert_eq!(after.request_weight.source, before.request_weight.source);
        assert_eq!(after.request_weight.used, before.request_weight.used + 20);
        assert_eq!(after.raw_requests, before.raw_requests);
        assert_eq!(after.key, before.key);
        client.close().await.unwrap();
        while events.recv().await.is_some() {}
        driver.await.unwrap().unwrap();
        assert_eq!(server.await.unwrap(), 2);
    }
}

#[tokio::test]
async fn websocket_raw_limit_refuses_the_next_rest_request_after_its_capacity_is_used() {
    let pools = WeightPools::new();
    let observer = config(&pools);
    let (url, server) = exchange_server(vec![json!({"rateLimits":[
        {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1000},
        {"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":1}
    ]})])
    .await;
    let (client, mut events, driver) =
        spot::WsClient::connect(config(&pools).websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    client
        .exchange_info(
            &spot::ws_requests::ExchangeInfo::new(),
            RequestId::new("raw").unwrap(),
            support::deadline(),
        )
        .await
        .unwrap();
    let venue = support::HttpFixture::new(200, "", "{\"serverTime\":1}", None, false).await;
    let rest = spot::RestClient::new(observer.rest_url(&venue.url).unwrap()).unwrap();
    rest.time(&spot::rest_requests::Time::new(), support::deadline())
        .await
        .unwrap();
    let before = rest.pool_usage().unwrap();
    assert_eq!(before.raw_requests.unwrap().used, 1);
    let error = rest
        .time(&spot::rest_requests::Time::new(), support::deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert!(
        matches!(error, Error::Admission { retry_after } if retry_after == Duration::from_mins(5))
    );
    assert_eq!(rest.pool_usage().unwrap(), before);
    assert_eq!(venue.connections_accepted(), 1);
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
    assert_eq!(server.await.unwrap(), 1);
    venue.finish().await;
}
