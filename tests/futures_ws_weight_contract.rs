// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! USDⓈ-M WebSocket API weight has its own counter in the futures pool; COIN-M's
//! WebSocket API weight shares the REST counter.
//!
//! Sources (checked 2026-10-11):
//! - <https://developers.binance.info/docs/derivatives/usds-margined-futures/websocket-api-general-info>:
//!   the WebSocket API IP weight limit is not shared with the REST API IP weight
//!   limit; REST single/batch order place/modify/cancel requests also count against
//!   the WebSocket API limit.
//! - <https://developers.binance.info/docs/derivatives/coin-margined-futures/websocket-api-general-info>:
//!   COIN-M WebSocket API limits are shared with REST API.
//! - <https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice>,
//!   A.3: USDⓈ-M and COIN-M REST share one `X-MBX-USED-WEIGHT-1M` counter.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]

#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;

use binance_client::{
    ClientOrderId, Clock, Credentials, Error, Outcome, RequestId, Symbol, WeightPools, coinm, usdm,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use support::{HttpFixture, deadline};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_websockets::{Limits, Message, ServerBuilder};

const MINUTE_START: u64 = 1_700_000_040_000;

struct ManualClock(AtomicU64);

impl Clock for ManualClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}

fn clock() -> Arc<ManualClock> {
    Arc::new(ManualClock(AtomicU64::new(MINUTE_START)))
}

fn credentials() -> Credentials {
    Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()
}

fn usdm_config(pools: &WeightPools, clock: &Arc<ManualClock>) -> usdm::Config {
    usdm::Config::with_pools(usdm::Environment::Production, pools)
        .unwrap()
        .clock(clock.clone())
        .credentials(credentials())
}

fn coinm_config(pools: &WeightPools, clock: &Arc<ManualClock>) -> coinm::Config {
    coinm::Config::with_pools(coinm::Environment::Production, pools)
        .unwrap()
        .clock(clock.clone())
        .credentials(credentials())
}

/// One venue WebSocket API session answering each request in turn with `result`
/// and, when given, the venue's `REQUEST_WEIGHT` minute count. A request admission
/// refused must never reach it; the task returns the methods it answered.
async fn venue(replies: Vec<(Value, Option<u64>)>) -> (String, JoinHandle<Vec<String>>) {
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
        let mut methods = Vec::new();
        for (result, count) in replies {
            let request: Value =
                serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
            methods.push(request["method"].as_str().unwrap().to_owned());
            let mut reply = json!({"id": request["id"], "status": 200, "result": result});
            if let Some(count) = count {
                reply["rateLimits"] = json!([{"rateLimitType": "REQUEST_WEIGHT",
                    "interval": "MINUTE", "intervalNum": 1, "limit": 2400, "count": count}]);
            }
            peer.send(Message::text(reply.to_string())).await.unwrap();
        }
        while let Some(message) = peer.next().await {
            let message = message.unwrap();
            if message.is_close() {
                peer.flush().await.unwrap();
                break;
            }
            assert!(!message.is_text(), "refused request reached the wire");
        }
        methods
    });
    (url, server)
}

fn price() -> usdm::ws_requests::SymbolPriceTicker {
    usdm::ws_requests::SymbolPriceTicker::new().symbol(Symbol::new("BTCUSDT").unwrap())
}

fn price_reply() -> Value {
    json!({"symbol": "BTCUSDT", "price": "1", "time": 1})
}

async fn usdm_price(client: &usdm::WsClient, id: &str) -> Result<(), Error> {
    client
        .symbol_price_ticker(&price(), RequestId::new(id).unwrap(), deadline())
        .await
        .map(drop)
}

fn assert_refused(error: &Error) {
    assert_eq!(error.outcome(), Some(Outcome::NotSent), "{error:?}");
    assert!(
        matches!(error, Error::Admission { retry_after } if *retry_after == Duration::from_mins(1)),
        "{error:?}"
    );
}

/// Connects a USDⓈ-M WebSocket API client and runs its driver.
async fn usdm_socket(
    config: usdm::Config,
    url: &str,
) -> (
    usdm::WsClient,
    usdm::ApiEvents,
    JoinHandle<Result<(), Error>>,
) {
    let (client, events, driver) = usdm::WsClient::connect(config.websocket_url(url).unwrap())
        .await
        .unwrap();
    (client, events, tokio::spawn(driver.run()))
}

async fn retire_usdm(
    client: usdm::WsClient,
    mut events: usdm::ApiEvents,
    driver: JoinHandle<Result<(), Error>>,
) {
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
}

#[tokio::test]
async fn usdm_websocket_request_leaves_rest_weight_untouched() {
    let pools = WeightPools::new();
    let clock = clock();
    let observer = usdm_config(&pools, &clock);
    let (url, server) = venue(vec![(price_reply(), None)]).await;
    let (client, events, driver) = usdm_socket(usdm_config(&pools, &clock), &url).await;

    // Handshake 5 and a symbol price ticker 1, all on the WebSocket API counter.
    usdm_price(&client, "price").await.unwrap();

    assert_eq!(observer.pool_usage().unwrap().request_weight.used, 0);
    retire_usdm(client, events, driver).await;
    assert_eq!(server.await.unwrap(), ["ticker.price"]);
}

#[tokio::test]
async fn coinm_websocket_request_charges_the_shared_rest_weight() {
    let pools = WeightPools::new();
    let clock = clock();
    let usdm_observer = usdm_config(&pools, &clock);
    let (url, server) = venue(vec![(json!({}), None)]).await;
    let (client, mut events, driver) =
        coinm::WsClient::connect(coinm_config(&pools, &clock).websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());

    client
        .account_information(
            &coinm::ws_requests::AccountInformation::new(),
            RequestId::new("account").unwrap(),
            deadline(),
        )
        .await
        .unwrap();

    // Handshake 5 and account information 5, on the REST counter both products share.
    assert_eq!(usdm_observer.pool_usage().unwrap().request_weight.used, 10);
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
    assert_eq!(server.await.unwrap(), ["account.status"]);
}

#[tokio::test]
async fn usdm_rest_order_place_and_cancel_also_charge_the_websocket_counter() {
    let pools = WeightPools::new();
    let clock = clock();
    let config = usdm_config(&pools, &clock);
    // The venue reports 2,394 of the WebSocket API's 2,400 spent.
    let (url, server) = venue(vec![(price_reply(), Some(2394))]).await;
    let (client, events, driver) = usdm_socket(config.clone(), &url).await;
    usdm_price(&client, "spent").await.unwrap();

    let batch_venue = HttpFixture::new(200, "", "[{\"orderId\":7}]", None, false).await;
    let cancel_venue = HttpFixture::new(200, "", "{\"orderId\":7}", None, false).await;
    let place = usdm::rest_requests::PlaceMultipleOrders::new().batch_orders(vec![
        usdm::rest_models::PlaceMultipleOrdersBatchOrdersInputItem::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(binance_client::Decimal::ONE)
            .new_client_order_id(ClientOrderId::new("batch-1").unwrap()),
    ]);
    let cancel = usdm::rest_requests::CancelOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .orig_client_order_id(ClientOrderId::new("batch-1").unwrap());
    // A batch place of weight 5 and a cancel of weight 1 fill the WebSocket API's
    // remaining six units as well as six REST units.
    usdm::RestClient::new(config.clone().rest_url(&batch_venue.url).unwrap())
        .unwrap()
        .place_multiple_orders(&place, deadline())
        .await
        .unwrap();
    usdm::RestClient::new(config.clone().rest_url(&cancel_venue.url).unwrap())
        .unwrap()
        .cancel_order(&cancel, deadline())
        .await
        .unwrap();

    assert_eq!(config.pool_usage().unwrap().request_weight.used, 6);
    assert_refused(&usdm_price(&client, "refused").await.unwrap_err());
    retire_usdm(client, events, driver).await;
    assert_eq!(server.await.unwrap(), ["ticker.price"]);
    batch_venue.finish().await;
    cancel_venue.finish().await;
}

#[tokio::test]
async fn usdm_websocket_rate_evidence_updates_only_the_websocket_counter() {
    let pools = WeightPools::new();
    let clock = clock();
    let config = usdm_config(&pools, &clock);
    // The venue reports the WebSocket API's whole minute weight spent.
    let (url, server) = venue(vec![(price_reply(), Some(2400))]).await;
    let (client, events, driver) = usdm_socket(config.clone(), &url).await;
    usdm_price(&client, "spent").await.unwrap();

    assert_eq!(config.pool_usage().unwrap().request_weight.used, 0);
    assert_refused(&usdm_price(&client, "refused").await.unwrap_err());
    let rest_venue = HttpFixture::new(200, "", "{\"serverTime\":1}", None, false).await;
    usdm::RestClient::new(config.rest_url(&rest_venue.url).unwrap())
        .unwrap()
        .check_server_time(&usdm::rest_requests::CheckServerTime::new(), deadline())
        .await
        .unwrap();
    assert_eq!(rest_venue.connections_accepted(), 1);
    retire_usdm(client, events, driver).await;
    assert_eq!(server.await.unwrap(), ["ticker.price"]);
    rest_venue.finish().await;
}

#[tokio::test]
async fn exhausted_usdm_rest_weight_does_not_refuse_a_websocket_request() {
    let pools = WeightPools::new();
    let clock = clock();
    let config = usdm_config(&pools, &clock);
    // The venue reports the shared REST counter's whole minute weight spent.
    let rest_venue = HttpFixture::new(
        200,
        "X-MBX-USED-WEIGHT-1M: 2400\r\n",
        "{\"serverTime\":1}",
        None,
        false,
    )
    .await;
    let rest = usdm::RestClient::new(config.clone().rest_url(&rest_venue.url).unwrap()).unwrap();
    let time = usdm::rest_requests::CheckServerTime::new();
    rest.check_server_time(&time, deadline()).await.unwrap();
    assert_refused(&rest.check_server_time(&time, deadline()).await.unwrap_err());

    let (url, server) = venue(vec![(price_reply(), None)]).await;
    let (client, events, driver) = usdm_socket(config, &url).await;
    usdm_price(&client, "admitted").await.unwrap();

    retire_usdm(client, events, driver).await;
    assert_eq!(server.await.unwrap(), ["ticker.price"]);
    assert_eq!(rest_venue.connections_accepted(), 1);
    rest_venue.finish().await;
}

#[tokio::test]
async fn coinm_websocket_weight_is_also_charged_to_the_usdm_websocket_counter() {
    // Whether COIN-M WebSocket API traffic counts against the USDⓈ-M WebSocket API
    // limit is not stated, so it is charged there as well as on the REST counter.
    let pools = WeightPools::new();
    let clock = clock();
    let usdm = usdm_config(&pools, &clock);
    let (usdm_url, usdm_server) = venue(vec![(price_reply(), Some(2390))]).await;
    let (usdm_client, usdm_events, usdm_driver) = usdm_socket(usdm.clone(), &usdm_url).await;
    usdm_price(&usdm_client, "spent").await.unwrap();

    let (coinm_url, coinm_server) = venue(vec![(json!({}), None)]).await;
    let (client, mut events, driver) = coinm::WsClient::connect(
        coinm_config(&pools, &clock)
            .websocket_url(&coinm_url)
            .unwrap(),
    )
    .await
    .unwrap();
    let driver = tokio::spawn(driver.run());
    client
        .account_information(
            &coinm::ws_requests::AccountInformation::new(),
            RequestId::new("account").unwrap(),
            deadline(),
        )
        .await
        .unwrap();

    assert_eq!(usdm.pool_usage().unwrap().request_weight.used, 10);
    assert_refused(&usdm_price(&usdm_client, "refused").await.unwrap_err());
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
    retire_usdm(usdm_client, usdm_events, usdm_driver).await;
    assert_eq!(coinm_server.await.unwrap(), ["account.status"]);
    assert_eq!(usdm_server.await.unwrap(), ["ticker.price"]);
}

#[tokio::test]
async fn coinm_rest_order_cancel_is_also_charged_to_the_usdm_websocket_counter() {
    // Whether COIN-M REST order requests count against the USDⓈ-M WebSocket API
    // limit is not stated, so they are charged there as USDⓈ-M's are.
    let pools = WeightPools::new();
    let clock = clock();
    let usdm = usdm_config(&pools, &clock);
    let (url, server) = venue(vec![(price_reply(), Some(2399))]).await;
    let (client, events, driver) = usdm_socket(usdm.clone(), &url).await;
    usdm_price(&client, "spent").await.unwrap();

    let cancel_venue = HttpFixture::new(
        200,
        "",
        "{\"orderId\":7,\"clientOrderId\":\"inverse-1\"}",
        None,
        false,
    )
    .await;
    let cancel = coinm::rest_requests::CancelOrder::new()
        .symbol(Symbol::new("BTCUSD_PERP").unwrap())
        .orig_client_order_id(ClientOrderId::new("inverse-1").unwrap());
    coinm::RestClient::new(
        coinm_config(&pools, &clock)
            .rest_url(&cancel_venue.url)
            .unwrap(),
    )
    .unwrap()
    .cancel_order(&cancel, deadline())
    .await
    .unwrap();

    assert_eq!(usdm.pool_usage().unwrap().request_weight.used, 1);
    assert_refused(&usdm_price(&client, "refused").await.unwrap_err());
    retire_usdm(client, events, driver).await;
    assert_eq!(server.await.unwrap(), ["ticker.price"]);
    cancel_venue.finish().await;
}

#[tokio::test]
async fn coinm_websocket_weight_charges_rest_weight_on_an_explicit_separate_owner() {
    let owner = binance_client::Budgets::new(binance_client::BudgetLimits::usdm()).unwrap();
    let config = coinm_config(&WeightPools::new(), &clock()).budgets(owner);
    let (url, server) = venue(vec![(json!({}), None)]).await;
    let (client, mut events, driver) =
        coinm::WsClient::connect(config.clone().websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    client
        .account_information(
            &coinm::ws_requests::AccountInformation::new(),
            RequestId::new("account").unwrap(),
            deadline(),
        )
        .await
        .unwrap();

    // Handshake 5 and account information 5 on REST weight, as COIN-M documents.
    assert_eq!(config.pool_usage().unwrap().request_weight.used, 10);
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
    assert_eq!(server.await.unwrap(), ["account.status"]);
}
