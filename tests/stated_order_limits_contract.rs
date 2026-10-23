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

#![allow(clippy::unwrap_used, reason = "synthetic fixture assertions")]

#[allow(
    dead_code,
    reason = "fixture helpers are shared across integration suites"
)]
mod support;

use binance_client::{
    AccountKey, ClientOrderId, Clock, Credentials, Decimal, Error, Outcome, RequestId, Symbol,
    TransportFailure, WeightPools, spot, usdm,
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
async fn a_stated_order_limit_without_a_positive_limit_fails_with_evidence_and_keeps_the_previous_limit()
 {
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
                Error::Transport {
                    outcome: Outcome::ReadFailed,
                    failure: Some(TransportFailure::RateEvidenceFailed),
                    meta: Some(_),
                    ..
                }
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
        let (_, mut peer) = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(stream)
            .await
            .unwrap();
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

#[tokio::test]
async fn account_usage_reports_limits_counts_and_aligned_resets_without_crossing_owners() {
    use binance_client::LimitSource;
    let pools = WeightPools::new();
    let clock = clock();
    let key = AccountKey::new("usage-owner");
    let base = usdm::Config::with_pools_for_account(usdm::Environment::Production, &pools, &key)
        .unwrap()
        .clock(clock.clone());
    let independent = usdm_fresh(&pools).clock(clock.clone());
    let headers = "X-MBX-ORDER-COUNT-10S: 9\r\nX-MBX-ORDER-COUNT-1M: 12\r\n";
    let venue = HttpFixture::keep_alive(200, headers, USDM_ORDER, None).await;
    let trader = usdm_client(base.clone(), &venue, &clock);
    let initial = trader.account_usage().unwrap();
    assert_eq!(initial.orders.len(), 2);
    assert_eq!(
        (
            initial.orders[0].limit,
            initial.orders[0].source,
            initial.orders[0].used
        ),
        (300, LimitSource::Documented, 0)
    );
    assert_eq!(initial.orders[1].limit, 1200);
    usdm_place(&trader).await.unwrap();
    assert_eq!(
        base.account_usage().unwrap(),
        trader.account_usage().unwrap()
    );
    assert_eq!(trader.account_usage().unwrap().orders[0].used, 9);
    assert_eq!(trader.account_usage().unwrap().orders[1].used, 12);
    assert_eq!(independent.account_usage().unwrap().orders[0].used, 0);

    let exchange =
        HttpFixture::new(200, "", &usdm_statement(&[ten_seconds(20)]), None, false).await;
    usdm_read(&usdm_client(usdm_fresh(&pools), &exchange, &clock))
        .await
        .unwrap();
    for report in [
        base.account_usage().unwrap(),
        independent.account_usage().unwrap(),
    ] {
        assert_eq!(
            (report.orders[0].limit, report.orders[0].source),
            (20, LimitSource::Stated)
        );
    }
    clock.0.store(START + 10_001, Ordering::SeqCst);
    let report = trader.account_usage().unwrap();
    assert_eq!(
        (report.orders[0].used, report.orders[0].resets_in),
        (0, Duration::from_millis(9999))
    );
    assert_eq!(
        (report.orders[1].used, report.orders[1].resets_in),
        (12, Duration::from_millis(49_999))
    );
    assert_eq!(report.orders[0].source, LimitSource::Stated);
    exchange.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn spot_account_query_sets_limits_and_raises_only_the_reading_owners_counts() {
    let pools = WeightPools::new();
    let clock = clock();
    let venue = HttpFixture::new(
        200,
        "",
        &json!([
            {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":2,"count":2},
            {"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":100,"count":7}
        ])
        .to_string(),
        None,
        false,
    )
    .await;
    let base = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock.clone());
    let query = spot::RestClient::new(
        base.clone()
            .rest_url(&venue.url)
            .unwrap()
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap();
    query
        .rate_limit_order(&spot::rest_requests::RateLimitOrder::new(), deadline())
        .await
        .unwrap();
    let report = query.account_usage().unwrap();
    assert_eq!(report.orders.len(), 2);
    assert_eq!((report.orders[0].limit, report.orders[0].used), (2, 2));
    assert_eq!((report.orders[1].limit, report.orders[1].used), (100, 7));
    let order = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(spot::ClientOrderId::new("stated-limits").unwrap());
    assert_refused(
        &query.new_order(&order, deadline()).await.unwrap_err(),
        Duration::from_secs(10),
    );
    assert_eq!(
        spot::Config::with_pools(spot::Environment::Production, &pools)
            .unwrap()
            .clock(clock.clone())
            .account_usage()
            .unwrap()
            .orders[0]
            .used,
        0
    );
    venue.finish().await;
}

/// A single API reply, followed by the caller's joined socket teardown.
async fn spot_api_reply(reply: Value) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let (_, mut peer) = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(tcp)
            .await
            .unwrap();
        let frame = peer.next().await.unwrap().unwrap();
        let request: Value = serde_json::from_slice(frame.as_payload()).unwrap();
        let mut reply = reply;
        reply["id"] = request["id"].clone();
        peer.send(Message::text(reply.to_string())).await.unwrap();
        while let Some(message) = peer.next().await {
            if message.unwrap().is_close() {
                peer.flush().await.unwrap();
                break;
            }
        }
    });
    (url, server)
}

#[tokio::test]
async fn websocket_response_and_account_query_limits_govern_later_rest_orders() {
    for account_query in [false, true] {
        let pools = WeightPools::new();
        let clock = clock();
        let base = spot::Config::with_pools(spot::Environment::Production, &pools)
            .unwrap()
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
        let entries = json!([
            {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":3,"count":1},
            {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":2,"count":0}
        ]);
        let response = if account_query {
            json!({"status":200,"result":entries})
        } else {
            json!({"status":200,"result":{"serverTime":1},"rateLimits":entries})
        };
        let (url, server) = spot_api_reply(response).await;
        let (client, mut events, driver) =
            spot::WsClient::connect(base.clone().websocket_url(&url).unwrap())
                .await
                .unwrap();
        let driver = tokio::spawn(driver.run());
        if account_query {
            client
                .account_rate_limits_orders(
                    &spot::ws_requests::AccountRateLimitsOrders::new(),
                    RequestId::new("limits").unwrap(),
                    deadline(),
                )
                .await
                .unwrap();
        } else {
            client
                .time(
                    &spot::ws_requests::Time::new(),
                    RequestId::new("limits").unwrap(),
                    deadline(),
                )
                .await
                .unwrap();
        }
        let other = spot::Config::with_pools(spot::Environment::Production, &pools)
            .unwrap()
            .clock(clock.clone());
        assert_eq!(other.account_usage().unwrap().orders[0].limit, 50);
        assert_eq!(
            other.account_usage().unwrap().orders[0].source,
            binance_client::LimitSource::Documented
        );
        let report = base.account_usage().unwrap();
        assert_eq!(
            (
                report.orders[0].limit,
                report.orders[0].used,
                report.orders[0].source
            ),
            (2, 1, binance_client::LimitSource::Stated)
        );
        let venue = HttpFixture::keep_alive(200, "", SPOT_ORDER, None).await;
        let trader = spot::RestClient::new(base.rest_url(&venue.url).unwrap()).unwrap();
        let order = spot::rest_requests::NewOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(spot::ClientOrderId::new("stated-limits").unwrap());
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
}

#[tokio::test]
async fn malformed_websocket_limits_preserve_the_acknowledgment_and_change_no_limit() {
    for invalid in [json!(0), json!(-1), Value::Null] {
        let pools = WeightPools::new();
        let base = spot::Config::with_pools(spot::Environment::Production, &pools)
            .unwrap()
            .clock(clock())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
        let result: Value = serde_json::from_str(SPOT_ORDER).unwrap();
        let (url, server) = spot_api_reply(json!({"status":200,"result":result,"rateLimits":[
            {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1,"count":2},
            {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":invalid,"count":9}
        ]})).await;
        let before = base.account_usage().unwrap();
        let (client, mut events, driver) =
            spot::WsClient::connect(base.clone().websocket_url(&url).unwrap())
                .await
                .unwrap();
        let driver = tokio::spawn(driver.run());
        let order = spot::ws_requests::OrderPlace::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(spot::ClientOrderId::new("stated-limits").unwrap());
        let response = client
            .order_place(
                &order,
                RequestId::new("invalid-limits").unwrap(),
                deadline(),
            )
            .await
            .unwrap();
        assert_eq!(response.data.order_id, 7);
        assert_eq!(response.data.client_order_id.as_str(), "stated-limits");
        assert_eq!(response.meta.status, 200);
        assert!(response.meta.rates.stated_limits_unusable);
        let after = base.account_usage().unwrap();
        assert_eq!(
            (after.orders[0].limit, after.orders[0].source),
            (before.orders[0].limit, before.orders[0].source)
        );
        assert_eq!(after.orders[0].used, 9);
        assert_eq!(base.pool_usage().unwrap().request_weight.limit, 6000);
        assert_eq!(
            base.pool_usage().unwrap().request_weight.source,
            binance_client::LimitSource::Documented
        );
        let mut gap = false;
        while let Some(event) = events.recv().await {
            gap |= matches!(
                event,
                spot::ApiEvent::Gap {
                    error: Error::Gap(_),
                    ..
                }
            );
        }
        assert!(gap);
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
}

#[tokio::test]
async fn account_query_invalid_limits_are_atomic_and_later_statements_cannot_loosen_them() {
    let pools = WeightPools::new();
    let base = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock());
    for (limit, valid) in [(2, true), (4, true), (0, false), (-1, false)] {
        let entries = json!([
            {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":limit,"count":1},
            {"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":20,"count":3}
        ]);
        let fixture = HttpFixture::new(200, "", &entries.to_string(), None, false).await;
        let client = spot::RestClient::new(
            base.clone()
                .rest_url(&fixture.url)
                .unwrap()
                .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
        )
        .unwrap();
        let before = base.account_usage().unwrap();
        let result = client
            .rate_limit_order(&spot::rest_requests::RateLimitOrder::new(), deadline())
            .await;
        if valid {
            result.unwrap();
        } else {
            assert!(
                matches!(result, Err(Error::Transport { outcome: Outcome::ReadFailed, meta: Some(ref meta), .. }) if meta.status == 200 && meta.rates.stated_limits_unusable)
            );
            assert_eq!(base.account_usage().unwrap(), before);
        }
        assert_eq!(base.account_usage().unwrap().orders[0].limit, 2);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn newly_stated_spot_minute_and_day_windows_count_admitted_placements() {
    let pools = WeightPools::new();
    let clock = clock();
    let base = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock.clone())
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let venue = HttpFixture::keep_alive(200, "", SPOT_ORDER, None).await;
    let trader = spot::RestClient::new(base.clone().rest_url(&venue.url).unwrap()).unwrap();
    let order = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(spot::ClientOrderId::new("stated-limits").unwrap());
    trader.new_order(&order, deadline()).await.unwrap();
    let query_venue = HttpFixture::new(
        200,
        "",
        &json!([
            {"rateLimitType":"ORDERS","interval":"MINUTE","intervalNum":1,"limit":2,"count":0},
            {"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":100,"count":0}
        ])
        .to_string(),
        None,
        false,
    )
    .await;
    let query = spot::RestClient::new(base.rest_url(&query_venue.url).unwrap()).unwrap();
    query
        .rate_limit_order(&spot::rest_requests::RateLimitOrder::new(), deadline())
        .await
        .unwrap();
    assert_eq!(trader.account_usage().unwrap().orders[1].used, 1);
    trader.new_order(&order, deadline()).await.unwrap();
    let usage = trader.account_usage().unwrap();
    assert_eq!(
        (
            usage.orders[1].limit,
            usage.orders[1].used,
            usage.orders[2].used
        ),
        (2, 2, 2)
    );
    assert_refused(
        &trader.new_order(&order, deadline()).await.unwrap_err(),
        Duration::from_mins(1),
    );
    clock.0.store(START + 10_000, Ordering::SeqCst);
    assert_refused(
        &trader.new_order(&order, deadline()).await.unwrap_err(),
        Duration::from_secs(50),
    );
    clock.0.store(START + 60_000, Ordering::SeqCst);
    trader.new_order(&order, deadline()).await.unwrap();
    assert_eq!(trader.account_usage().unwrap().orders[1].used, 1);
    assert_eq!(trader.account_usage().unwrap().orders[2].used, 3);
    query_venue.finish().await;
    venue.finish().await;
}

#[tokio::test]
async fn missing_websocket_account_query_result_preserves_metadata_and_reports_a_gap() {
    let pools = WeightPools::new();
    let base = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock())
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let before = base.account_usage().unwrap();
    let (url, server) = spot_api_reply(json!({"status":200,"rateLimits":[
        {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1,"count":2},
        {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":1,"count":7}
    ]}))
    .await;
    let (client, mut events, driver) =
        spot::WsClient::connect(base.clone().websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    let error = client
        .account_rate_limits_orders(
            &spot::ws_requests::AccountRateLimitsOrders::new(),
            RequestId::new("missing-result").unwrap(),
            deadline(),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Transport { outcome: Outcome::ReadFailed, meta: Some(ref meta), .. }
        if meta.status == 200 && meta.rates.stated_limits_unusable
        && meta.rates.counters.get("x-mbx-order-count-10s") == Some(&7))
    );
    let after = base.account_usage().unwrap();
    assert_eq!(
        (after.orders[0].limit, after.orders[0].source),
        (before.orders[0].limit, before.orders[0].source)
    );
    assert_eq!(after.orders[0].used, 7);
    assert_eq!(base.pool_usage().unwrap().request_weight.limit, 6000);
    let mut gap = false;
    let mut retained_frame = false;
    while let Some(event) = events.recv().await {
        gap |= matches!(event, spot::ApiEvent::Gap { .. });
        retained_frame |= matches!(event, spot::ApiEvent::Unknown { ref payload, .. }
            if payload.as_value().get("id") == Some(&json!("missing-result")));
    }
    assert!(gap);
    assert!(retained_frame);
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn missing_websocket_order_payload_keeps_unknown_outcome_and_client_identity() {
    let pools = WeightPools::new();
    let base = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock())
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let (url, server) = spot_api_reply(json!({"status":200,"rateLimits":[
        {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":50,"count":1}
    ]}))
    .await;
    let (client, mut events, driver) =
        spot::WsClient::connect(base.clone().websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    let order = spot::ws_requests::OrderPlace::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(spot::ClientOrderId::new("missing-ack").unwrap());
    let error = client
        .order_place(&order, RequestId::new("missing-ack").unwrap(), deadline())
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Transport { outcome: Outcome::Unknown, ref client_order_ids, meta: Some(ref meta), .. }
        if meta.status == 200 && meta.rates.counters.get("x-mbx-order-count-10s") == Some(&1)
        && client_order_ids.get("newClientOrderId").map(String::as_str) == Some("missing-ack")
        && meta.client_order_ids == *client_order_ids)
    );
    assert_eq!(base.account_usage().unwrap().orders[0].used, 1);
    let mut gap = false;
    while let Some(event) = events.recv().await {
        gap |= matches!(event, spot::ApiEvent::Gap { .. });
    }
    assert!(gap);
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn invalid_rest_account_query_keeps_only_independent_header_counts_in_metadata() {
    let pools = WeightPools::new();
    let base = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock());
    let entries = json!([
        {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":1,"count":9},
        {"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":0,"count":3}
    ]);
    let fixture = HttpFixture::new(
        200,
        "X-MBX-ORDER-COUNT-10S: 2\r\n",
        &entries.to_string(),
        None,
        false,
    )
    .await;
    let client = spot::RestClient::new(
        base.clone()
            .rest_url(&fixture.url)
            .unwrap()
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap();
    let before = base.account_usage().unwrap();
    let error = client
        .rate_limit_order(&spot::rest_requests::RateLimitOrder::new(), deadline())
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Transport { outcome: Outcome::ReadFailed, failure: Some(TransportFailure::RateEvidenceFailed), meta: Some(ref meta), .. }
        if meta.status == 200 && meta.rates.stated_limits_unusable
        && meta.rates.counters.get("x-mbx-order-count-10s") == Some(&2)
        && !meta.rates.counters.contains_key("x-mbx-order-count-1d"))
    );
    let after = base.account_usage().unwrap();
    assert_eq!(
        (after.orders[0].limit, after.orders[0].source),
        (before.orders[0].limit, before.orders[0].source)
    );
    assert_eq!(after.orders[0].used, 2);
    fixture.finish().await;
}

#[tokio::test]
async fn malformed_websocket_account_query_refuses_with_metadata_and_keeps_limits() {
    for entry in [
        json!({"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":0,"count":9}),
        json!({"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":1}),
    ] {
        let pools = WeightPools::new();
        let base = spot::Config::with_pools(spot::Environment::Production, &pools)
            .unwrap()
            .clock(clock())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
        let (url, server) = spot_api_reply(json!({"status":200,"result":[entry],"rateLimits":[
            {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":1,"count":2}
        ]})).await;
        let (client, mut events, driver) =
            spot::WsClient::connect(base.clone().websocket_url(&url).unwrap())
                .await
                .unwrap();
        let driver = tokio::spawn(driver.run());
        let error = client
            .account_rate_limits_orders(
                &spot::ws_requests::AccountRateLimitsOrders::new(),
                RequestId::new("invalid-query").unwrap(),
                deadline(),
            )
            .await
            .unwrap_err();
        assert!(
            matches!(error, Error::Transport {outcome: Outcome::ReadFailed, meta: Some(ref meta), ..}
            if meta.status == 200 && meta.rates.stated_limits_unusable)
        );
        assert_eq!(base.account_usage().unwrap().orders[0].limit, 50);
        assert_eq!(base.account_usage().unwrap().orders[0].used, 0);
        assert_eq!(base.pool_usage().unwrap().request_weight.limit, 6000);
        let mut gap = false;
        while let Some(event) = events.recv().await {
            gap |= matches!(event, spot::ApiEvent::Gap { .. });
        }
        assert!(gap);
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
}

#[tokio::test]
async fn invalid_limits_do_not_replace_a_venue_rejection_with_a_generic_query_error() {
    let pools = WeightPools::new();
    let base = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock())
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let (url, server) = spot_api_reply(json!({"status":400,"error":{"code":-2015,"msg":"synthetic rejection"},
        "rateLimits":[{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":0,"count":7}]})).await;
    let (client, mut events, driver) =
        spot::WsClient::connect(base.clone().websocket_url(&url).unwrap())
            .await
            .unwrap();
    let driver = tokio::spawn(driver.run());
    let error = client
        .account_rate_limits_orders(
            &spot::ws_requests::AccountRateLimitsOrders::new(),
            RequestId::new("rejected-query").unwrap(),
            deadline(),
        )
        .await
        .unwrap_err();
    assert!(
        matches!(error, Error::Venue(ref failure) if failure.code == Some(-2015)
        && failure.status == 400 && failure.rates.stated_limits_unusable)
    );
    assert_eq!(base.account_usage().unwrap().orders[0].limit, 50);
    assert_eq!(base.account_usage().unwrap().orders[0].used, 7);
    let mut gap = false;
    while let Some(event) = events.recv().await {
        gap |= matches!(event, spot::ApiEvent::Gap { .. });
    }
    assert!(gap);
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn account_statements_share_by_key_and_combine_with_common_pool_limits() {
    let pools = WeightPools::new();
    let clock = clock();
    let key = AccountKey::new("order-window-owner");
    let owner = spot::Config::with_pools_for_account(spot::Environment::Production, &pools, &key)
        .unwrap()
        .clock(clock.clone())
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let other = spot::Config::with_pools(spot::Environment::Production, &pools)
        .unwrap()
        .clock(clock.clone());
    for limit in [100, 2] {
        let entries = json!([{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":limit,"count":0}]);
        let fixture = HttpFixture::new(200, "", &entries.to_string(), None, false).await;
        let query = spot::RestClient::new(owner.clone().rest_url(&fixture.url).unwrap()).unwrap();
        query
            .rate_limit_order(&spot::rest_requests::RateLimitOrder::new(), deadline())
            .await
            .unwrap();
        let same_key =
            spot::Config::with_pools_for_account(spot::Environment::Production, &pools, &key)
                .unwrap()
                .clock(clock.clone());
        assert_eq!(same_key.account_usage().unwrap().orders[0].limit, limit);
        assert_eq!(other.account_usage().unwrap().orders[0].limit, 50);
        fixture.finish().await;
    }
    let statement = json!({"rateLimits":[{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":10}]}).to_string();
    let exchange = HttpFixture::new(200, "", &statement, None, false).await;
    let reader = spot::RestClient::new(other.clone().rest_url(&exchange.url).unwrap()).unwrap();
    reader
        .exchange_info(&spot::rest_requests::ExchangeInfo::new(), deadline())
        .await
        .unwrap();
    assert_eq!(owner.account_usage().unwrap().orders[0].limit, 2);
    assert_eq!(other.account_usage().unwrap().orders[0].limit, 10);
    exchange.finish().await;
}
