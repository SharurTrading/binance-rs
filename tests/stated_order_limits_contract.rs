// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exchange information `ORDERS` entries set the account order limits of every
//! account owner of the pool that read them.
//!
//! Spot, USDⓈ-M, COIN-M and Options `exchangeInfo` list their `ORDERS` rate
//! limiters in `rateLimits`; the order rate limit is counted against each account
//! (<https://developers.binance.info/docs/derivatives/usds-margined-futures/general-info>,
//! <https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#limits>,
//! checked 2026-10-11). The documented examples are fallbacks: Spot's pages show
//! 50 per ten seconds, and live production stated 100 on 2026-10-11. Each test
//! builds its own registry.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]

#[allow(
    dead_code,
    reason = "fixture helpers are shared across integration suites"
)]
mod support;

use binance_client::{
    AccountKey, ClientOrderId, Clock, Credentials, Decimal, Error, Outcome, RequestId, Symbol,
    WeightPools, spot, usdm,
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
use tokio::net::TcpListener;
use tokio_websockets::{Limits, Message, ServerBuilder};

/// Aligned to the ten-second, minute and five-minute windows alike.
const START: u64 = 1_700_000_100_000;
const USDM_ORDER: &str =
    r#"{"orderId":7,"clientOrderId":"stated-limits","status":"NEW","executedQty":"0"}"#;
const SPOT_ORDER: &str = r#"{"symbol":"BTCUSDT","orderId":7,"orderListId":-1,"clientOrderId":"stated-limits","transactTime":1}"#;

struct ManualClock(AtomicU64);

impl Clock for ManualClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}

fn clock() -> Arc<ManualClock> {
    Arc::new(ManualClock(AtomicU64::new(START)))
}

fn assert_refused(error: &Error, retry_after: Duration) {
    assert_eq!(error.outcome(), Some(Outcome::NotSent), "{error:?}");
    assert!(
        matches!(error, Error::Admission { retry_after: actual } if *actual == retry_after),
        "{error:?}"
    );
}

/// A USDⓈ-M exchange information reply stating `orders` as its `ORDERS` entries.
fn usdm_statement(orders: &[Value]) -> String {
    let mut limits = vec![json!(
        {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":2400}
    )];
    limits.extend_from_slice(orders);
    json!({"rateLimits": limits, "symbols": []}).to_string()
}

fn ten_seconds(limit: i64) -> Value {
    json!({"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":limit})
}

fn usdm_client(
    config: usdm::Config,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> usdm::RestClient {
    usdm::RestClient::new(
        config
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap()
}

fn usdm_fresh(pools: &WeightPools) -> usdm::Config {
    usdm::Config::with_pools(usdm::Environment::Production, pools).unwrap()
}

async fn usdm_read(reader: &usdm::RestClient) -> Result<(), Error> {
    reader
        .exchange_information(&usdm::rest_requests::ExchangeInformation::new(), deadline())
        .await
        .map(drop)
}

async fn usdm_place(client: &usdm::RestClient) -> Result<(), Error> {
    let order = usdm::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(ClientOrderId::new("stated-limits").unwrap());
    client.new_order(&order, deadline()).await.map(drop)
}

/// Places `admitted` orders, then proves the next is refused unsent until the
/// ten-second window ends.
async fn exhaust(client: &usdm::RestClient, admitted: usize) {
    for _ in 0..admitted {
        usdm_place(client).await.unwrap();
    }
    assert_refused(
        &usdm_place(client).await.unwrap_err(),
        Duration::from_secs(10),
    );
}

#[tokio::test]
async fn a_stated_ten_second_limit_below_the_fallback_refuses_the_order_past_it() {
    let pools = WeightPools::new();
    let clock = clock();
    let exchange = HttpFixture::new(200, "", &usdm_statement(&[ten_seconds(2)]), None, false).await;
    let venue = HttpFixture::keep_alive(200, "", USDM_ORDER, None).await;
    let reader = usdm_client(usdm_fresh(&pools), &exchange, &clock);
    let trader = usdm_client(usdm_fresh(&pools), &venue, &clock);

    // The documented futures fallback admits 300 orders per ten seconds.
    usdm_read(&reader).await.unwrap();
    exhaust(&trader, 2).await;
    exchange.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_stated_ten_second_limit_above_the_fallback_admits_orders_the_fallback_refuses() {
    let pools = WeightPools::new();
    let clock = clock();
    let exchange =
        HttpFixture::new(200, "", &usdm_statement(&[ten_seconds(301)]), None, false).await;
    let venue = HttpFixture::keep_alive(200, "", USDM_ORDER, None).await;
    let reader = usdm_client(usdm_fresh(&pools), &exchange, &clock);
    let trader = usdm_client(usdm_fresh(&pools), &venue, &clock);

    usdm_read(&reader).await.unwrap();
    exhaust(&trader, 301).await;
    exchange.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_stated_limit_applies_to_keyed_and_fresh_owners_drawn_before_or_after_it() {
    let pools = WeightPools::new();
    let clock = clock();
    let key = AccountKey::new("account-one");
    let exchange = HttpFixture::new(200, "", &usdm_statement(&[ten_seconds(2)]), None, false).await;
    // One connection per order: four clients share this fixture.
    let venue = HttpFixture::new(200, "", USDM_ORDER, None, false).await;
    let reader = usdm_client(usdm_fresh(&pools), &exchange, &clock);
    let keyed_before = usdm_client(
        usdm::Config::with_pools_for_account(usdm::Environment::Production, &pools, &key).unwrap(),
        &venue,
        &clock,
    );
    let fresh_before = usdm_client(usdm_fresh(&pools), &venue, &clock);

    usdm_read(&reader).await.unwrap();
    let keyed_after = usdm_client(
        usdm::Config::with_pools_for_account(
            usdm::Environment::Production,
            &pools,
            &AccountKey::new("account-two"),
        )
        .unwrap(),
        &venue,
        &clock,
    );
    let fresh_after = usdm_client(usdm_fresh(&pools), &venue, &clock);
    // Each owner counts its own account against the venue's per-account figure.
    for client in [&keyed_before, &fresh_before, &keyed_after, &fresh_after] {
        exhaust(client, 2).await;
    }
    assert_eq!(venue.connections_accepted(), 8);
    exchange.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn a_stated_order_limit_without_a_positive_limit_is_a_gap_and_keeps_the_previous_limit() {
    for invalid in [json!(0), json!(-1), Value::Null] {
        let pools = WeightPools::new();
        let clock = clock();
        let valid =
            HttpFixture::new(200, "", &usdm_statement(&[ten_seconds(2)]), None, false).await;
        let mut entry = json!({"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10});
        if !invalid.is_null() {
            entry["limit"] = invalid.clone();
        }
        let malformed = HttpFixture::new(
            200,
            "",
            &usdm_statement(&[ten_seconds(5), entry]),
            None,
            false,
        )
        .await;
        let venue = HttpFixture::keep_alive(200, "", USDM_ORDER, None).await;
        let first = usdm_client(usdm_fresh(&pools), &valid, &clock);
        let second = usdm_client(usdm_fresh(&pools), &malformed, &clock);
        let trader = usdm_client(usdm_fresh(&pools), &venue, &clock);

        usdm_read(&first).await.unwrap();
        let error = usdm_read(&second).await.unwrap_err();
        assert!(
            matches!(
                error,
                Error::Gap("stated rate limit without a positive limit")
            ),
            "{invalid}: {error:?}"
        );
        exhaust(&trader, 2).await;
        valid.finish().await;
        malformed.finish().await;
        venue.finish().await;
    }
}

#[tokio::test]
async fn a_websocket_exchange_info_order_limit_governs_rest_orders_of_the_pool() {
    let pools = WeightPools::new();
    let clock = clock();
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
        let request: Value =
            serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
        assert_eq!(request["method"], "exchangeInfo");
        let result = json!({"rateLimits":[
            {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000},
            {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":1},
            {"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":200_000},
            {"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":300_000}
        ]});
        peer.send(Message::text(
            json!({"id":request["id"],"status":200,"result":result}).to_string(),
        ))
        .await
        .unwrap();
        while let Some(message) = peer.next().await {
            if message.unwrap().is_close() {
                peer.flush().await.unwrap();
                break;
            }
        }
    });
    let config = || {
        spot::Config::with_pools(spot::Environment::Production, &pools)
            .unwrap()
            .clock(clock.clone())
    };
    let (client, mut events, driver) =
        spot::WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    client
        .exchange_info(
            &spot::ws_requests::ExchangeInfo::new(),
            RequestId::new("limits").unwrap(),
            deadline(),
        )
        .await
        .unwrap();

    let venue = HttpFixture::keep_alive(200, "", SPOT_ORDER, None).await;
    let trader = spot::RestClient::new(
        config()
            .rest_url(&venue.url)
            .unwrap()
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap();
    let order = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(spot::ClientOrderId::new("stated-limits").unwrap());
    // Spot's documented fallback admits 50 orders per ten seconds.
    trader.new_order(&order, deadline()).await.unwrap();
    assert_refused(
        &trader.new_order(&order, deadline()).await.unwrap_err(),
        Duration::from_secs(10),
    );
    client.close().await.unwrap();
    while events.recv().await.is_some() {}
    driver.await.unwrap().unwrap();
    server.await.unwrap();
    venue.finish().await;
}
