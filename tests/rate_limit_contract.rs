// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Rate admission must use the full available budget without exceeding it on the wire.
//!
//! Synthetic limits keep workloads small; endpoint weights and scope semantics follow
//! <https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/general-info>
//! and the market-data/trade catalog, checked 2026-09-27.

#[cfg(test)]
mod support;

use binance_client::usdm::{
    RestClient,
    rest_requests::{CheckServerTime, NewOrder, OrderBook},
};
use binance_client::{
    BudgetLimits, Budgets, ClientOrderId, Clock, Decimal, Error, Outcome, Symbol,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use support::{HttpFixture, config, deadline};

struct ManualClock(AtomicU64);

impl ManualClock {
    fn set(&self, millis: u64) {
        self.0.store(millis, Ordering::SeqCst);
    }
}

impl Clock for ManualClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}

fn assert_refused(error: &Error, millis: u64) {
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert!(
        matches!(error, Error::Admission { retry_after }
        if *retry_after == Duration::from_millis(millis)),
        "{error:?}"
    );
}

#[cfg(test)]
fn order(id: u64) -> NewOrder {
    NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(ClientOrderId::new(format!("rate-fixture-{id}")).unwrap())
}

#[tokio::test]
async fn weighted_requests_use_remaining_capacity_and_reset_at_the_exact_boundary() {
    let mut fixture = HttpFixture::new(
        200,
        "",
        "{\"lastUpdateId\":1,\"bids\":[],\"asks\":[]}",
        None,
        false,
    )
    .await;
    let clock = Arc::new(ManualClock(AtomicU64::new(1000)));
    let client = RestClient::new(
        config()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone())
            .budgets(Budgets::new(BudgetLimits::usdm().weight_per_minute(7)).unwrap()),
    )
    .unwrap();
    let depth = OrderBook::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .limit(100);

    // A 100-level read costs five units; a five-level read costs two.
    client.order_book(&depth, deadline()).await.unwrap();
    assert_refused(
        &client.order_book(&depth, deadline()).await.unwrap_err(),
        59_000,
    );
    client
        .order_book(&depth.limit(5), deadline())
        .await
        .unwrap();
    assert_eq!(fixture.connections_accepted(), 2);
    assert!(fixture.requests.recv().await.unwrap().contains("limit=100"));
    assert!(fixture.requests.recv().await.unwrap().contains("limit=5"));

    clock.set(59_999);
    assert_refused(
        &client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap_err(),
        1,
    );
    assert_eq!(fixture.connections_accepted(), 2);
    clock.set(60_000);
    for _ in 0..7 {
        client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap();
    }
    assert_refused(
        &client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap_err(),
        60_000,
    );
    assert_eq!(fixture.connections_accepted(), 9);
    fixture.finish().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_clones_and_accounts_fill_but_never_exceed_the_shared_ip_budget() {
    const CAPACITY: usize = 17;
    const CONTENDERS: usize = 64;
    let fixture = HttpFixture::new(200, "", "{\"serverTime\":1000}", None, false).await;
    let clock = Arc::new(ManualClock(AtomicU64::new(1000)));
    let budgets =
        Budgets::new(BudgetLimits::usdm().weight_per_minute(u64::try_from(CAPACITY).unwrap()))
            .unwrap();
    let base = config()
        .rest_url(&fixture.url)
        .unwrap()
        .clock(clock.clone());
    let clients = [
        RestClient::new(base.clone().budgets(budgets.clone())).unwrap(),
        RestClient::new(base.budgets(budgets.for_account())).unwrap(),
    ];

    for round in 0..2 {
        clock.set(round * 60_000 + 1000);
        let gate = Arc::new(tokio::sync::Barrier::new(CONTENDERS + 1));
        let mut tasks = tokio::task::JoinSet::new();
        for i in 0..CONTENDERS {
            let client = clients[i % clients.len()].clone();
            let gate = gate.clone();
            tasks.spawn(async move {
                gate.wait().await;
                client
                    .check_server_time(&CheckServerTime::new(), deadline())
                    .await
            });
        }
        gate.wait().await;
        let mut accepted = 0;
        let mut refused = 0;
        while let Some(result) = tasks.join_next().await {
            match result.unwrap() {
                Ok(_) => accepted += 1,
                Err(error) => {
                    assert_refused(&error, 59_000);
                    refused += 1;
                }
            }
        }
        assert_eq!(
            accepted, CAPACITY,
            "full capacity must be available under contention"
        );
        assert_eq!(refused, CONTENDERS - CAPACITY);
        assert_eq!(
            fixture.connections_accepted(),
            usize::try_from(round + 1).unwrap() * CAPACITY,
            "refused attempts must never reach the wire"
        );
    }
    fixture.finish().await;
}

#[tokio::test]
async fn order_windows_allow_every_slot_without_resetting_the_longer_window_early() {
    let fixture = HttpFixture::new(200, "", "{\"orderId\":7}", None, false).await;
    let clock = Arc::new(ManualClock(AtomicU64::new(1000)));
    let client = RestClient::new(
        config()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone())
            .budgets(Budgets::new(BudgetLimits::usdm().orders(3, 5)).unwrap()),
    )
    .unwrap();
    for id in 0..3 {
        client.new_order(&order(id), deadline()).await.unwrap();
    }
    assert_refused(
        &client.new_order(&order(3), deadline()).await.unwrap_err(),
        9000,
    );
    clock.set(9999);
    assert_refused(
        &client.new_order(&order(4), deadline()).await.unwrap_err(),
        1,
    );
    clock.set(10_000);
    for id in 5..7 {
        client.new_order(&order(id), deadline()).await.unwrap();
    }
    assert_refused(
        &client.new_order(&order(7), deadline()).await.unwrap_err(),
        50_000,
    );
    clock.set(20_000);
    assert_refused(
        &client.new_order(&order(8), deadline()).await.unwrap_err(),
        40_000,
    );
    clock.set(59_999);
    assert_refused(
        &client.new_order(&order(9), deadline()).await.unwrap_err(),
        1,
    );
    assert_eq!(fixture.connections_accepted(), 5);

    clock.set(60_000);
    for id in 10..13 {
        client.new_order(&order(id), deadline()).await.unwrap();
    }
    assert_refused(
        &client.new_order(&order(13), deadline()).await.unwrap_err(),
        10_000,
    );
    assert_eq!(fixture.connections_accepted(), 8);
    fixture.finish().await;
}

#[tokio::test]
async fn venue_counter_evidence_leaves_exactly_the_reported_remaining_capacity() {
    let fixture = HttpFixture::new(
        200,
        "X-MBX-USED-WEIGHT-1M: 7\r\n",
        "{\"serverTime\":1000}",
        None,
        false,
    )
    .await;
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(10)).unwrap();
    let client = RestClient::new(
        config()
            .rest_url(&fixture.url)
            .unwrap()
            .clock(Arc::new(ManualClock(AtomicU64::new(1000))))
            .budgets(budgets),
    )
    .unwrap();
    // The first response reports seven units including traffic outside this client.
    for _ in 0..4 {
        client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap();
    }
    // Subsequent stale/lower evidence must not erase locally reserved units.
    assert_refused(
        &client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap_err(),
        59_000,
    );
    assert_eq!(fixture.connections_accepted(), 4);
    fixture.finish().await;
}

#[tokio::test]
async fn venue_cooldown_blocks_shared_clients_until_the_exact_expiry() {
    let fixture = HttpFixture::new(
        429,
        "Retry-After: 2\r\n",
        "{\"code\":-1003,\"msg\":\"fixture rate limit\"}",
        None,
        false,
    )
    .await;
    let clock = Arc::new(ManualClock(AtomicU64::new(1000)));
    let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
    let base = config()
        .rest_url(&fixture.url)
        .unwrap()
        .clock(clock.clone());
    let first = RestClient::new(base.clone().budgets(budgets.clone())).unwrap();
    let second = RestClient::new(base.budgets(budgets.for_account())).unwrap();
    assert!(matches!(
        first
            .check_server_time(&CheckServerTime::new(), deadline())
            .await,
        Err(Error::Venue(_))
    ));
    assert_refused(
        &second
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap_err(),
        2000,
    );
    clock.set(2999);
    assert_refused(
        &first
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap_err(),
        1,
    );
    assert_eq!(fixture.connections_accepted(), 1);
    clock.set(3000);
    // The fixture rejects again, proving that the attempt is admitted at expiry.
    assert!(matches!(
        second
            .check_server_time(&CheckServerTime::new(), deadline())
            .await,
        Err(Error::Venue(_))
    ));
    assert_eq!(fixture.connections_accepted(), 2);
    fixture.finish().await;
}
