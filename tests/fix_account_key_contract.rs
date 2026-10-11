// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! A FIX account scope drawn for an account key shares that key's Spot account owner.
//!
//! Spot's unfilled order count is tracked by (sub)account and "shared across all IP
//! addresses, all API keys, and all APIs"
//! (<https://github.com/binance/binance-spot-api-docs/blob/master/faqs/order_count_decrement.md>),
//! so REST, WebSocket API and FIX orders of one account draw on one count. FIX
//! connection limits apply per account
//! (<https://github.com/binance/binance-spot-api-docs/blob/master/fix-api.md#connection-limits>,
//! checked 2026-10-11). Each test builds its own registry.

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
    AccountKey, Clock, Credentials, Decimal, Error, Outcome, RequestId, Signer, Symbol,
    WeightPools, spot,
    spot::fix::{
        AccountBudgets, ClientId, CompId, Config, Event, Events, Request, RequestKind, Role,
        Session, Value,
    },
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use support::{HttpFixture, deadline};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
};

const MINUTE_START: u64 = 1_700_000_040_000;
const SPOT_ORDER: &str = r#"{"orderId":7,"clientOrderId":"account-key"}"#;
const TEN_SECONDS: Duration = Duration::from_secs(10);

struct ManualClock(AtomicU64);

impl ManualClock {
    fn at(millis: u64) -> Arc<Self> {
        Arc::new(Self(AtomicU64::new(millis)))
    }
}

impl Clock for ManualClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}

struct SyntheticSigner;

impl Signer for SyntheticSigner {
    fn sign(&self, _: &[u8]) -> Result<String, Error> {
        Ok("synthetic-signature".into())
    }
}

fn assert_refused(error: &Error, retry_after: Duration) {
    assert!(
        matches!(error, Error::Admission { retry_after: actual } if *actual == retry_after),
        "{error:?}"
    );
}

fn rest_client(
    config: spot::Config,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> spot::RestClient {
    spot::RestClient::new(
        config
            .rest_url(&fixture.url)
            .unwrap()
            .clock(clock.clone())
            .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap()),
    )
    .unwrap()
}

fn keyed_rest_client(
    pools: &WeightPools,
    key: &AccountKey,
    fixture: &HttpFixture,
    clock: &Arc<ManualClock>,
) -> spot::RestClient {
    rest_client(
        spot::Config::with_pools_for_account(spot::Environment::Production, pools, key).unwrap(),
        fixture,
        clock,
    )
}

async fn rest_place(client: &spot::RestClient) -> Result<(), Error> {
    let order = spot::rest_requests::NewOrder::new()
        .symbol(Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(spot::ClientOrderId::new("account-key").unwrap());
    client.new_order(&order, deadline()).await.map(drop)
}

fn keyed_scope(pools: &WeightPools, key: &AccountKey) -> AccountBudgets {
    AccountBudgets::with_pools_for_account(spot::Environment::Production, pools, key).unwrap()
}

fn fix_config(
    budgets: AccountBudgets,
    component: &str,
    address: &str,
    clock: &Arc<ManualClock>,
) -> Config {
    Config::new(
        Role::OrderEntry,
        CompId::new(component).unwrap(),
        Credentials::external_ed25519("synthetic-key", Arc::new(SyntheticSigner)).unwrap(),
        budgets,
    )
    .unwrap()
    .heartbeat(5)
    .unwrap()
    .clock(clock.clone())
    .endpoint(address)
    .unwrap()
}

fn frame(body: &str) -> Vec<u8> {
    let body = body.replace('|', "\x01");
    let mut message = format!("8=FIX.4.4\x019={}\x01{body}", body.len()).into_bytes();
    let sum = message.iter().fold(0_u8, |sum, b| sum.wrapping_add(*b));
    message.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
    message
}

async fn read_frame(stream: &mut tokio::net::TcpStream) -> Vec<u8> {
    let mut bytes = vec![];
    loop {
        bytes.push(stream.read_u8().await.unwrap());
        if bytes.len() >= 7
            && bytes[bytes.len() - 7..].starts_with(b"10=")
            && bytes.last() == Some(&1)
        {
            return bytes;
        }
    }
}

/// A venue that acknowledges Logon, writes `after_logon`, then reads until the
/// client closes the connection.
async fn venue(after_logon: Vec<Vec<u8>>) -> (String, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("tcp://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let logon =
            binance_client::spot::fix::decode(Role::OrderEntry, &read_frame(&mut stream).await)
                .unwrap();
        assert_eq!(logon.kind.as_str(), "A");
        stream
            .write_all(&frame(
                "35=A|34=1|49=SPOT|56=CLIENT|52=20260927-01:02:03.000001|98=0|108=5|25037=synthetic-server|",
            ))
            .await
            .unwrap();
        for message in after_logon {
            stream.write_all(&message).await.unwrap();
        }
        stream.read_to_end(&mut vec![]).await.unwrap();
    });
    (address, task)
}

struct Running {
    session: Session,
    events: Events,
    owner: JoinHandle<Result<(), Error>>,
    venue: JoinHandle<()>,
}

impl Running {
    async fn start(
        budgets: AccountBudgets,
        after_logon: Vec<Vec<u8>>,
        clock: &Arc<ManualClock>,
    ) -> Self {
        let (address, venue) = venue(after_logon).await;
        let (session, mut events, driver) =
            Session::connect(fix_config(budgets, "CLIENT", &address, clock))
                .await
                .unwrap();
        let owner = tokio::spawn(driver.run());
        let first = events.recv().await;
        assert!(
            matches!(first, Some(Event::Established { .. })),
            "{first:?}"
        );
        Self {
            session,
            events,
            owner,
            venue,
        }
    }

    async fn place(&self, id: &str) -> Result<Outcome, Error> {
        let order = Request::builder(Role::OrderEntry, RequestKind::NewOrderSingle)
            .field("ClOrdID", Value::ClientId(ClientId::new(id).unwrap()))
            .unwrap()
            .field("Symbol", Value::Symbol(Symbol::new("BTCUSDT").unwrap()))
            .unwrap()
            .field("Side", Value::Code("1".into()))
            .unwrap()
            .field("OrdType", Value::Code("2".into()))
            .unwrap()
            .field("TimeInForce", Value::Code("1".into()))
            .unwrap()
            .field("OrderQty", Value::Quantity("0.125".parse().unwrap()))
            .unwrap()
            .field("Price", Value::Price("1.5".parse().unwrap()))
            .unwrap()
            .build()
            .unwrap();
        self.session
            .send(RequestId::new(id).unwrap(), order)
            .await
            .map(|attempt| attempt.outcome)
            .map_err(|failure| {
                assert_eq!(failure.attempt.outcome, Outcome::NotSent);
                failure.error
            })
    }

    /// Wait until the session has handled the venue message of `kind`.
    async fn handled(&mut self, kind: &str) {
        loop {
            match self.events.recv().await {
                Some(Event::Message { message, .. }) if message.kind.as_str() == kind => return,
                Some(_) => (),
                None => panic!("session retired before {kind}"),
            }
        }
    }

    async fn finish(mut self) {
        self.session.shutdown();
        while self.events.recv().await.is_some() {}
        self.owner.await.unwrap().unwrap();
        self.venue.await.unwrap();
    }
}

#[tokio::test]
async fn a_rest_client_and_a_fix_session_of_one_key_share_the_unfilled_order_count() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let key = AccountKey::new("account-one");
    let rest_venue = HttpFixture::keep_alive(200, "", SPOT_ORDER, None).await;
    let rest = keyed_rest_client(&pools, &key, &rest_venue, &clock);
    let fix = Running::start(keyed_scope(&pools, &key), vec![], &clock).await;

    // Spot admits 50 orders per ten seconds per account: 49 over REST, one over FIX.
    for _ in 0..49 {
        rest_place(&rest).await.unwrap();
    }
    assert_eq!(fix.place("fix-1").await.unwrap(), Outcome::Unknown);
    assert_refused(&fix.place("fix-2").await.unwrap_err(), TEN_SECONDS);
    assert_refused(&rest_place(&rest).await.unwrap_err(), TEN_SECONDS);
    fix.finish().await;
    rest_venue.finish().await;
}

#[tokio::test]
async fn a_fix_session_of_another_key_keeps_its_own_order_count() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let rest_venue = HttpFixture::keep_alive(200, "", SPOT_ORDER, None).await;
    let rest = keyed_rest_client(&pools, &AccountKey::new("account-one"), &rest_venue, &clock);
    let fix = Running::start(
        keyed_scope(&pools, &AccountKey::new("account-two")),
        vec![],
        &clock,
    )
    .await;

    for _ in 0..50 {
        rest_place(&rest).await.unwrap();
    }
    assert_refused(&rest_place(&rest).await.unwrap_err(), TEN_SECONDS);
    assert_eq!(fix.place("fix-1").await.unwrap(), Outcome::Unknown);
    fix.finish().await;
    rest_venue.finish().await;
}

#[tokio::test]
async fn a_fix_limit_response_holds_the_rest_clients_of_its_key_only() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let key = AccountKey::new("account-one");
    let rest_venue = HttpFixture::keep_alive(200, "", SPOT_ORDER, None).await;
    let same_key = keyed_rest_client(&pools, &key, &rest_venue, &clock);
    let other_key = keyed_rest_client(&pools, &AccountKey::new("account-two"), &rest_venue, &clock);
    // An ORDER_LIMIT indicator: 50 of 50 orders used in a 10-second interval.
    let mut fix = Running::start(
        keyed_scope(&pools, &key),
        vec![
            frame(
                "35=XLR|34=2|49=SPOT|56=CLIENT|52=20260927-01:02:03.000002|6136=limits-1|25003=1|25004=1|25005=50|25006=50|25007=10|25008=s|",
            ),
            frame("35=0|34=3|49=SPOT|56=CLIENT|52=20260927-01:02:03.000003|"),
        ],
        &clock,
    )
    .await;
    fix.handled("XLR").await;
    fix.handled("0").await;

    assert_refused(&rest_place(&same_key).await.unwrap_err(), TEN_SECONDS);
    rest_place(&other_key).await.unwrap();
    fix.finish().await;
    rest_venue.finish().await;
}

#[tokio::test]
async fn an_order_count_a_rest_reply_reports_holds_the_fix_session_of_its_key() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let key = AccountKey::new("account-one");
    let rest_venue = HttpFixture::new(
        200,
        "X-MBX-ORDER-COUNT-10S: 50\r\n",
        SPOT_ORDER,
        None,
        false,
    )
    .await;
    let rest = keyed_rest_client(&pools, &key, &rest_venue, &clock);
    let fix = Running::start(keyed_scope(&pools, &key), vec![], &clock).await;

    rest_place(&rest).await.unwrap();
    assert_refused(&fix.place("fix-1").await.unwrap_err(), TEN_SECONDS);
    fix.finish().await;
    rest_venue.finish().await;
}

#[tokio::test]
async fn a_keyed_fix_session_shares_the_spot_pools_ip_state() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let limited = HttpFixture::new(
        429,
        "Retry-After: 2\r\n",
        "{\"code\":-1003,\"msg\":\"fixture rate limit\"}",
        None,
        false,
    )
    .await;
    let unkeyed = rest_client(
        spot::Config::with_pools(spot::Environment::Production, &pools).unwrap(),
        &limited,
        &clock,
    );
    let fix = Running::start(
        keyed_scope(&pools, &AccountKey::new("account-one")),
        vec![],
        &clock,
    )
    .await;

    assert!(matches!(rest_place(&unkeyed).await, Err(Error::Venue(_))));
    assert_refused(
        &fix.place("fix-1").await.unwrap_err(),
        Duration::from_secs(2),
    );
    fix.finish().await;
    limited.finish().await;
}

#[tokio::test]
async fn fix_scopes_drawn_for_one_key_share_its_connection_limit() {
    let pools = WeightPools::new();
    let clock = ManualClock::at(MINUTE_START);
    let key = AccountKey::new("account-one");
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("tcp://{}", listener.local_addr().unwrap());
    let first = keyed_scope(&pools, &key);
    let second = keyed_scope(&pools, &key);

    // Order entry allows ten concurrent connections per account.
    let mut open = vec![];
    for n in 0..10 {
        let config = fix_config(first.clone(), &format!("CLIENT{n}"), &address, &clock);
        open.push(Session::connect(config).await.unwrap());
    }
    let Err(refused) = Session::connect(fix_config(second, "CLIENT10", &address, &clock)).await
    else {
        panic!("an eleventh order-entry connection of the account was admitted");
    };
    // The refusal waits two heartbeat intervals of five seconds.
    assert_refused(&refused, TEN_SECONDS);
    let other = keyed_scope(&pools, &AccountKey::new("account-two"));
    open.push(
        Session::connect(fix_config(other, "CLIENT11", &address, &clock))
            .await
            .unwrap(),
    );
    drop(open);
}
