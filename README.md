<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Binance Rust Client

[![CI](https://github.com/SharurTrading/binance-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/binance-rs/actions/workflows/ci.yml)

> [!IMPORTANT]
> Read the [behaviour guide](docs/behaviour-guide.md) before wrapping this client. It
> explains, in plain words, the venue behaviours an integrator must handle.

An **unofficial**, independent Rust inner client for Binance APIs, maintained by
SharurTrading. This project is not affiliated with or endorsed by Binance.

> **Warning: unreleased and untested for live trading.** This crate is under active
> development and is not ready for production use. Synthetic tests and read-only
> public probes do not establish live trading readiness. `publish = false` remains
> in force.

External-wallet withdrawals and withdrawal destination/configuration APIs are
excluded from this crate. Internal Binance transfers and read-only withdrawal
history/status remain available. A platform withdrawal shield also requires API
credentials without withdrawal permission.

The [response-shape audit](docs/response-shape-audit.md) records source-linked
fixtures for every supported JSON method, market route and private event kind,
plus the pinned SBE and FIX definitions.

Seven separate JSON product clients are available under `binance_client::core_trading`:

| Product | REST | WebSocket API | Market streams | User event kinds |
| --- | ---: | ---: | ---: | ---: |
| USDⓈ-M | 95 | 18 + 3 session methods | 21 | 10 |
| Spot (JSON catalog) | 48 | 52 + 3 session methods | 15 | 6 |
| Wallet | 43 | — | — | — |
| Convert | 9 | — | — | — |
| COIN-M (initial coverage) | 67 | 10 + 3 session methods | 19 | 7 |
| Margin | 66 bindings | Listen-token subscription | Risk/execution streams | Native Margin events |
| Options | 44 | — | 10 | 6 |

Advanced Trading clients are available under `binance_client::advanced_trading`:

| Product | REST | Market stream definitions | User event kinds |
| --- | ---: | ---: | ---: |
| Portfolio Margin | 102 | — | 12 |
| Portfolio Margin Pro | 24 | — | 2 |
| Algo Trading (Spot/Futures TWAP and VP) | 12 | — | — |
| Copy Trading | 2 | — | — |
| Alpha Trading | 6 | 13 | — |
| Stocks Trading | 16 | 6 | 1 |

[Advanced Trading coverage](docs/advanced-trading-coverage.md) records the native
routes, protocol sources, quota rules and synthetic verification limits.
Institutional Loan is outside the crate's retail trading scope.

[USDⓈ-M coverage](docs/coverage.md) and
[Spot/COIN-M coverage](docs/spot-coinm-coverage.md), and
[Wallet coverage](docs/wallet-coverage.md), and
[Convert coverage](docs/convert-coverage.md),
[Margin coverage](docs/margin-coverage.md), and
[Options coverage](docs/options-coverage.md) record official sources and
verification limits. Margin has 65 currently dispatchable REST operations; the
October 14 query is date-gated. Token issuance uses the documented API-key-only
`USER_STREAM` contract. Announced UTA stream scope is tracked in
[issue #76](https://github.com/SharurTrading/binance-rs/issues/76).
Advanced Spot JSON bindings include order lists, SOR, amend,
cancel/replace partial evidence, and explicit microsecond units. COIN-M migrated
algo evidence remains tracked in
[issue #11](https://github.com/SharurTrading/binance-rs/issues/11).
[Spot FIX/SBE protocols](docs/core-protocols.md) add native ASCII/binary sessions,
SBE API responses and four binary market streams, with explicit fixture limits.

## Boundary and architecture

The caller owns the Tokio runtime, credentials, client order IDs, and recovery
of uncertain mutations. The client owns transport, signing, exact provider-native
DTOs, deadline/rate admission, safe outcome evidence, and streaming lifecycle.
Trading decisions, canonical instruments, risk, persistence, and portfolio
accounting belong to consumers. There is no dependency on a consuming platform.

```text
src/
├── core/: shared identities, credentials, time, transport and rate budgets
└── core_trading/
    ├── usdm/: linear Futures models, requests, streams
    ├── wallet/: native balances, networks, histories, internal transfers, SAPI scopes
    ├── convert/: native quote/limit amounts, expiry authority, SAPI endpoint scopes
    ├── margin/: cross/isolated balances, borrowing, orders, execution/risk events
    ├── options/: native option contracts, Greeks, orders, routed streams
    ├── spot/: balances, base quantity/quote spend, depth updates, FIX and SBE
    └── coinm/: inverse Futures models, requests, streams, depth updates

```

`core_trading` groups the existing products within this single crate. Future API
sections can sit alongside it, and `core` remains independent shared infrastructure.
Root imports such as `binance_client::spot` remain compatible re-exports of the
same modules and types; new code can use `binance_client::core_trading::spot`.

Spot and COIN-M reuse the core through separate product modules. Asset balances,
linear versus inverse settlement, position modes, and product routes stay native.
Portfolio Margin Pro is another API/account product, not a USDⓈ-M configuration
switch. It can reuse transport and signing without pretending its account or order
models are interchangeable. Current CM migration can share position-mode settings
with UM; distinct API modules do not imply independent account settings. No OMS Toolkit, rebates, or platform policy is included.

## Public demo metadata

Create one client at application startup and reuse it for subsequent requests.
The application owns the Tokio runtime. This example uses Tokio's `macros` and
`rt` features with a current-thread runtime.

```rust,no_run
use binance_client::Error;
use binance_client::core_trading::usdm::rest_requests::{CheckServerTime, ExchangeInformation};
use binance_client::core_trading::usdm::{Config, Environment, RestClient};
use std::time::Duration;
use tokio::time::Instant;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let client = RestClient::new(Config::new(Environment::Demo)?)?;
    let metadata = client
        .exchange_information(
            &ExchangeInformation::new(),
            Instant::now() + Duration::from_secs(10),
        )
        .await?;
    println!("Symbol definitions: {:?}", metadata.data.symbols);

    let server_time = client
        .check_server_time(
            &CheckServerTime::new(),
            Instant::now() + Duration::from_secs(10),
        )
        .await?;
    println!("Server time: {:?}", server_time.data.server_time);
    Ok(())
}
```

Financial fields use `Decimal`, parsed without floats or silent rounding. Futures
depth arrays apply the same exact parser to every price and quantity token; values
outside Decimal coefficient or scale limits fail decoding. Request
builders validate required/conditional inputs on `build()` and again at dispatch.
Outgoing enum values are checked; unknown incoming fields remain open to provider
additions. Contract status and contract type (USDⓈ-M and COIN-M), Spot symbol status
and the USDⓈ-M funding rate type decode to typed enums in each product's `enums`
module: one variant per documented value, with the exact venue spelling kept on
encode, and `Unknown(String)` holding any value the venue's pages do not list.
Mapping these to trading meaning is the consumer's. Other enum-valued fields remain
strings ([#77](https://github.com/SharurTrading/binance-rs/issues/77),
[#78](https://github.com/SharurTrading/binance-rs/issues/78)). Callers use current exchange filters to check tick,
lot, notional, and other symbol-dependent rules; precision digits are not tick sizes.

## Execution and ownership

- Every operation requires a caller deadline. Validation, credentials, budget, and
  unsent expiration failures return `Outcome::NotSent`.
- Mutations are attempted once. Timeout, disconnect, malformed/truncated replies,
  and unknown future errors remain `Outcome::Unknown` unless documented evidence
  proves rejection. Query venue truth; a timeout does not cancel an order.
- Responses/errors retain safe status, venue code, rate counters, retry timing, and
  caller order IDs (parameter paths identify nested batch members).
  `Error::Transport.failure` provides an optional `TransportFailure` category for
  HTTP failures: response headers, rate evidence, body read, JSON/binary decoding,
  content type, or response model decoding. It holds no raw dependency error,
  response body, or URL. WebSocket transport errors use `None`, apart from the
  shared response model decoder's `ResponseDecodeFailed` category. Adding this
  public field changes exhaustive `Error::Transport` matches and construction
  literals; callers should use `..` when matching and continue to use `Outcome`
  to decide whether a mutation needs venue reconciliation. A failure applying
  rate evidence after a response now reports `Unknown` for a sent mutation or
  `ReadFailed` for a read, retaining the received metadata and caller IDs; it
  previously surfaced as a configuration error classified `NotSent`.
  Venue failures and market-stream control refusals also retain an exact optional
  venue explanation as `VenueMessage`. It is redacted in error formatting and
  debugging; it may echo a URL, IP address, or token. Access through
  `expose_sensitive_text()` is explicit and caller-owned: do not automatically log
  or persist the returned text.
  Batch results preserve input order and each member's success/refusal/uncertainty.
  Malformed members retain redacted `BatchResult::Unknown` evidence without hiding
  successful receipts. Resolve uncertain members with venue reads; never replay
  the entire batch because one member failed to decode.
- WebSocket connections return a handle/events and a **caller-owned driver**.
  Run its future alongside calls, request close, drain through `Retired`, and join
  the task you own. The client starts no runtime or socket task. Do not abort the
  driver as normal shutdown. [Example](examples/demo_ws.rs).
- Ingress is unbounded and source ordered. Queue depth, oldest age, and counts
  distinguish lag from actual loss. Generations, gaps, and retirement are explicit.
- Market and execution connections are separate. Mixed market routes are refused.
  Spot execution events arrive through WebSocket API subscriptions; Futures use
  listen keys. Renewal and reconnect are explicit caller operations; no automatic
  replay, recovery, credential loading, or hidden continuity claim.
- A market socket may connect with no streams. `subscribe`, `unsubscribe` and
  `list_subscriptions` send the venue's `SUBSCRIBE`, `UNSUBSCRIBE` and
  `LIST_SUBSCRIPTIONS` control messages on the open socket, within the same
  generation, in Spot, USDⓈ-M (per route) and COIN-M. They share the documented
  incoming-message ceiling with pongs and are refused unsent past it. A reconnect
  resubscribes nothing: membership is the caller's.
- Depth events and snapshots keep their update IDs (`U`, `u`, Futures `pu`,
  `lastUpdateId`) and exact levels. A level's price is signed and kept as sent,
  zero and negative included; Futures refuse only a negative quantity, which the
  venue documents as absolute. Synchronising a local book from them is the
  consumer's; the client builds no book. Every finite snapshot remains partial.

HMAC, RSA PKCS#8, Ed25519 PKCS#8, and external signers are supported. WebSocket
session logon requires Ed25519. TLS is required except exact loopback fixtures;
ambient HTTP proxies are ignored. Explicit HTTP proxies are supported; WebSocket
proxies are not. Sensitive strings and credentials redact `Debug`; callers must
not log explicit secret access or account payloads.

REST reuses pooled HTTP/1.1 connections (keep-alive), so repeated reads and the
reconcile path skip per-call connection setup; use the persistent WebSocket API
for latency-sensitive trading. What if a pooled connection dies? Hyper re-sends
a request only when it was never handed to the connection's encoder — not even
a partial write reached the venue — so a resend cannot duplicate an order; any
request the venue did receive fails loudly as a typed error the caller resolves
with venue reads. One caller-visible consequence: a call that lands on a dead
idle connection is transparently redialed, so a single call can consume two
connections while the venue sees one delivery. Reqwest retries and redirects
stay disabled, and contract tests pin both directions — a venue-received
request is never resent, and a stale-idle redial delivers exactly once —
against dependency drift. Reqwest owns its internal HTTP transport machinery on
the caller's runtime; caller-owned, joinable drivers govern all client
WebSocket lifecycles.

## Budgets

Spot, Futures and Options `Config::new` draw their IP budgets from the process's pool for
that venue pool and environment, so every client counts against one weight limit:

| Pool | Products | Baseline minute weight | Source |
| --- | --- | ---: | --- |
| Spot | Spot | 6,000 | [Spot rate limiters](https://github.com/binance/binance-spot-api-docs/blob/master/enums.md#rate-limiters-ratelimittype) |
| Futures | USDⓈ-M and COIN-M together | 2,400 | [UM/CM integration notice](https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice), A.3 |
| Options | Options | 2,400 | [Options rate limiters](https://developers.binance.com/en/docs/products/derivatives-trading-options/common-definition) |

Demo and production never share a pool. Each pool starts at its documented baseline;
every REST exchange information reply and Spot WebSocket API `exchangeInfo`
result hands its `rateLimits` to the pool, and the
latest stated `REQUEST_WEIGHT` per minute (and Spot's `RAW_REQUESTS` per five
minutes) replaces it for every client of the pool; stated `ORDERS` limits replace the
account order limits (see [Order limits](#order-limits)). A counted window stated without a
positive limit is refused as `Error::Gap` and leaves the pool unchanged; of two
statements of one window in a reply, the stricter binds. Every `X-MBX-USED-WEIGHT-1M` raises
the pool's count, and a `Retry-After` or `418` holds every client of the pool. A
request the pool cannot take is refused unsent as `Error::Admission` with its retry
delay.

### Futures WebSocket API weight

The futures pool keeps two minute weight counters: REST weight, which USDⓈ-M and
COIN-M share, and the USDⓈ-M WebSocket API's own. The
[USDⓈ-M WebSocket API](https://developers.binance.info/docs/derivatives/usds-margined-futures/websocket-api-general-info)
states: "The WebSocket API IP weight limit (shared across `ws-fapi` and
`ws-fapi-mm`) is **not** shared with the REST API IP weight limit (`fapi`,
`fapi-mm`). The only overlap is that REST single/batch order place/modify/cancel
requests are counted against the same limit as WebSocket API requests." The
[COIN-M WebSocket API](https://developers.binance.info/docs/derivatives/coin-margined-futures/websocket-api-general-info)
states: "Rate limits are the same as on REST API and are shared with REST API." The
[UM/CM integration notice](https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice),
A.3, states: "Requests on either fapi or dapi count against the same
X-MBX-USED-WEIGHT-1M counter."

| Traffic | REST weight | USDⓈ-M WebSocket API weight |
| --- | :---: | :---: |
| USDⓈ-M REST | Yes | No |
| USDⓈ-M REST single/batch order place, modify and cancel | Yes | Yes |
| USDⓈ-M WebSocket API, handshake included | No | Yes |
| COIN-M REST | Yes | No |
| COIN-M REST single/batch order place, modify and cancel | Yes | Yes, conservatively |
| COIN-M WebSocket API, handshake included | Yes | Yes, conservatively |

`X-MBX-USED-WEIGHT-1M` raises REST weight. A USDⓈ-M WebSocket API reply's
`rateLimits` count raises the WebSocket API counter; a COIN-M reply's raises REST
weight. A reply that also states the window's limit — a positive `REQUEST_WEIGHT`
per-minute figure in its `rateLimits` — states it for the counter it reports, and
of two statements the stricter binds. A COIN-M client counts its WebSocket API
weight on REST weight with any budget owner, including an explicit one.

> [!NOTE]
> The documentation does not say whether COIN-M WebSocket API requests or COIN-M
> REST order requests also count against the USDⓈ-M WebSocket API limit. The client
> takes the conservative reading and charges them there too, so COIN-M traffic can
> only lower what USDⓈ-M WebSocket API clients may send. USDⓈ-M WebSocket API weight
> is never charged to REST weight: the notice puts COIN-M REST on the one REST
> counter that the USDⓈ-M WebSocket API limit is stated not to share.

`pool_usage()` reports both counters: `ws_request_weight` carries the WebSocket
API window with its limit, source, use and reset time.

Spot's [WebSocket exchange information](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general)
updates the same IP owner as REST, including clients configured separately from
the same registry. The whole counted statement is validated before either limit
changes; a missing, null, zero or negative counted limit returns `Error::Gap`,
preserving the prior limits and every charged attempt. Other implemented
WebSocket API catalogs offer no exchange-information method.

Wallet, Convert and Margin REST draw from a separate process SAPI IP pool; their
endpoint weight counters remain independent. Margin WebSocket API uses the Spot
pool. Options uses its own pool and native exchange-information limits.
`with_pools` selects an explicit registry for these products.

A registry owns no clock. Every client drawn from one registry must be given one
clock, the same `Arc<dyn Clock>` or wall time for all (the default), because each
client charges the pool's fixed, aligned windows at the instant its own `Config::clock`
reports; clients with different clocks charge different windows of one pool.

Each `Config::new` keeps its own account owner; clone a product `Config` to share
budgets between its REST and WebSocket clients. Across products or credentials of
one account, name the account with an `AccountKey` (see
[Sharing an account](#sharing-an-account)). Spot shares REST/WS weight,
daily/ten-second order counts, and connection-attempt limits. Successful ordinary
Spot submits/cancels release the documented weight reservation; failures remain
charged and observed venue counters are never reduced.

Spot FIX connection attempts count twice — once against the account's own window
and once against the IP's — because the venue checks both independently. Two
accounts trading FIX through one IP therefore share one attempt budget:
`AccountBudgets::with_pools_for_account` (or `new_for_account`) draws each key's
scope from the same registry pool its Spot clients use, and a refused attempt
never burns an account slot.

### Order limits

Binance counts orders per account. Each account owner starts from its pool's
documented order figures:

| Pool | Ten seconds | Minute | Day | Source |
| --- | ---: | ---: | ---: | --- |
| Spot | 50 | none | 160,000 | `rateLimits` examples in the [Spot REST API](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md) and [WebSocket API](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-api.md) |
| Futures | 300 | 1,200 | none | [UM/CM integration notice](https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice), A.3 |
| Options | none | 1,200 | none | [Options rate limiters](https://developers.binance.com/en/docs/products/derivatives-trading-options/common-definition) |

Exchange information `rateLimits` lists the venue's `ORDERS` limiters. Every `ORDERS`
entry for ten seconds, one minute or one day that a client of the pool reads, through
REST or Spot WebSocket API `exchangeInfo`, replaces that window's figure for every
account owner of the pool: keyed or not, drawn before the read or after. Each owner
still counts only its own account's orders against the figure. A statement that names
a window the pool has no figure for adds it; other intervals are not counted. In the
futures pool, USDⓈ-M and COIN-M statements replace one shared set of figures, window
by window.

> [!NOTE]
> The documented figures are examples and differ from what production states. On
> 2026-10-11 production `exchangeInfo` stated Spot 100 per ten seconds and 200,000 a
> day, USDⓈ-M 300 per ten seconds and 1,200 a minute, COIN-M 1,200 a minute only, and
> Options 30 per ten seconds and 100 a minute. Spot's
> [unfilled order count FAQ](https://github.com/binance/binance-spot-api-docs/blob/master/faqs/order_count_decrement.md)
> says of its own examples: "The actual configuration on the live exchange may be
> different." Read exchange information once per pool before relying on the figures.

Exchange information is a public, unsigned read, and none of the cited pages states
that an order or weight limit depends on VIP level or account tier. Margin's native
order windows (`order_limits`) and FIX `LimitResponse` windows are reported by the
venue for the account and stay as they are.

### Sharing an account

Binance counts orders per account, not per client: Spot's unfilled order count is
[shared across all IP addresses, API keys and APIs](https://github.com/binance/binance-spot-api-docs/blob/master/faqs/order_count_decrement.md),
and USDⓈ-M and COIN-M share one order budget
([UM/CM integration notice](https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice)).
An `AccountKey` names one account. It is opaque bytes or text the caller chooses;
the client only compares it, redacts it from `Debug`, and zeroizes it when its last
copy drops. A registry keeps a copy for as long as the registry lives.

`new_for_account` draws from the process's registry and `with_pools_for_account`
from an explicit one. Every configuration of one registry given an equal key, in one
venue pool and environment, shares that pool's account owner; it still shares the
pool's IP weight with every other client of the pool, keyed or not. Without a key,
each configuration gets an account owner of its own, so sharing stays explicit.

| Shared per key and pool | Pool |
| --- | --- |
| Order counts per ten seconds, minute and day, including `X-MBX-ORDER-COUNT-*` a reply reports | Spot, Futures, Options |
| USDⓈ-M conversion quote and USDⓈ-M and COIN-M monthly download quotas | Futures |
| UID endpoint weight and Margin's native order windows, including those `order_limits` installs | SAPI |
| FIX order windows a `LimitResponse` reports, and FIX connection limits | Spot |

| Configuration | Keyed constructors |
| --- | --- |
| `spot::Config`, `usdm::Config`, `coinm::Config`, `options::Config` | `new_for_account(environment, &key)`, `with_pools_for_account(environment, &pools, &key)` |
| `wallet::Config`, `convert::Config`, `margin::Config` | `new_for_account(&key)`, `with_pools_for_account(&pools, &key)` |
| `margin::WsConfig` | `new_for_account(&key)`, `with_pools_for_account(&pools, &key)` |
| `spot::fix::AccountBudgets` | `new_for_account(environment, &key)`, `with_pools_for_account(environment, &pools, &key)` |

Each pool keeps its own owner per key: a key shared by a Spot and a USDⓈ-M client
names one account in each pool, not one owner across pools. `budgets` still
replaces both scopes with an explicit owner.

```rust
use binance_client::core_trading::{coinm, usdm};
use binance_client::{AccountKey, Error};

let account = AccountKey::new("main-account");
let um = usdm::Config::new_for_account(usdm::Environment::Production, &account)?;
let cm = coinm::Config::new_for_account(coinm::Environment::Production, &account)?;
# Ok::<(), Error>(())
```

A Spot FIX session joins the same account through its `AccountBudgets`. A scope
drawn for a key counts its orders against the Spot owner that `spot::Config` of
that key, registry and environment shares, so REST, WebSocket API and FIX orders
draw on one count, and order limits a FIX `LimitResponse` reports hold the key's
REST and WebSocket API clients too. It keeps the Spot pool's IP state. Every scope
drawn for the key shares one set of FIX connection limits, which Binance
[counts per account](https://github.com/binance/binance-spot-api-docs/blob/master/fix-api.md#connection-limits)
for FIX and FIX SBE alike. `AccountBudgets::new` still takes an explicit owner.

```rust
use binance_client::core_trading::spot::{self, fix::AccountBudgets};
use binance_client::{AccountKey, Error};

let account = AccountKey::new("main-account");
let rest = spot::Config::new_for_account(spot::Environment::Production, &account)?;
let fix = AccountBudgets::new_for_account(spot::Environment::Production, &account)?;
# Ok::<(), Error>(())
```

`Config::budgets` replaces the drawn pool with an explicit owner, isolating the
client from every pool. `Config::with_pools` draws from a registry the caller builds
instead of the process's, so tests can share a pool without touching it:

```rust
use binance_client::core_trading::{coinm, usdm};
use binance_client::{Error, WeightPools};

let pools = WeightPools::new();
let um = usdm::Config::with_pools(usdm::Environment::Production, &pools)?;
let cm = coinm::Config::with_pools(coinm::Environment::Production, &pools)?;
# Ok::<(), Error>(())
```

### Naming a pool

`pool_key()` names the venue limit a client draws on as a `PoolKey`: its
`VenuePool` and `PoolEnvironment`. Clients reporting equal keys count against one
venue limit, so a caller pacing several clients can group them by key; `PoolKey`
orders, hashes and compares for use as a map key.

| Configuration | `VenuePool` | `PoolEnvironment` |
| --- | --- | --- |
| `spot::Config` | `Spot` | Its `Environment` |
| `spot::sbe::MarketConfig` | `Spot` | `Production` |
| `usdm::Config`, `coinm::Config` | `Futures` | Its `Environment` |
| `options::Config` | `Options` | Its `Environment` |
| `wallet::Config`, `convert::Config`, `margin::Config` | `Sapi` | `Production` |
| `margin::WsConfig` | `Spot` | `Production` |

The Spot, USDⓈ-M, COIN-M and Options `RestClient` report the same key, and
`pool_usage()` carries it as `key`. A configuration given an explicit owner through
`budgets` reports `None`: that owner belongs to no registry, and the caller that
built it already knows which clients share it.

```rust
use binance_client::core_trading::{coinm, usdm};
use binance_client::{Error, PoolEnvironment, PoolKey, VenuePool};

let um = usdm::Config::new(usdm::Environment::Production)?;
let cm = coinm::Config::new(coinm::Environment::Production)?;
let futures = Some(PoolKey::new(VenuePool::Futures, PoolEnvironment::Production));
assert_eq!(um.pool_key(), futures);
assert_eq!(cm.pool_key(), futures);
# Ok::<(), Error>(())
```

### Reading a pool and a request's weight

The client reports what it measures and decides nothing with it. `pool_usage()` on
a product `Config` or `RestClient` reads the pool that client draws on, at its
injected clock:

| Field | Meaning |
| --- | --- |
| `request_weight` | `REQUEST_WEIGHT` per minute. Spot and Options count REST and WebSocket API weight here together; the futures pool counts REST weight and COIN-M WebSocket API weight, not USDⓈ-M WebSocket API weight ([Futures WebSocket API weight](#futures-websocket-api-weight)) |
| `ws_request_weight` | The USDⓈ-M WebSocket API's own minute window, where the pool counts one apart from REST (the futures pool, and explicit `BudgetLimits::usdm()` owners); `None` on pools that count both transports together, whose `request_weight` already includes it |
| `raw_requests` | `RAW_REQUESTS` per five minutes, where the pool counts them (Spot) |
| `key` | The pool read, as `pool_key()` names it; `None` for an explicit owner |
| `limit`, `source` | The latest stated limit (`LimitSource::Stated`), or the documented baseline until a client of the pool reads exchange information or a WebSocket reply states one (`LimitSource::Documented`) |
| `used` | Spent in the current window: the pool's own count, raised to `X-MBX-USED-WEIGHT-1M` — or a WebSocket reply's `rateLimits` count for the separate window — whenever a reply reports more |
| `interval`, `resets_in` | The window's length and the time until it starts again, aligned to the epoch as the venue's windows are |

Every Spot, USDⓈ-M and COIN-M REST request builder has `weight()`: the weight
admission will charge that request against the minute window, read before it is
sent. It follows the parameters exactly as admission does (`limit` for klines and
depth, `symbol` or `symbols` for tickers and open orders), and refuses what dispatch
would refuse before admission. Fixed weights come from each catalog's `x-ip-weight`
in `schema/`; parameter-dependent weights are each product's `rate.rs` tables, from
the same pages. A Spot submit or cancel the venue documents as free on success is
charged its reservation and released once it succeeds.

```rust
use binance_client::core_trading::usdm::{self, rest_requests::KlineCandlestickData};
use binance_client::{Error, Symbol};

let config = usdm::Config::new(usdm::Environment::Production)?;
let pool = config.pool_usage()?;
let bars = KlineCandlestickData::new()
    .symbol(Symbol::new("BTCUSDT")?)
    .interval("1m")
    .limit(1000);
let fits = pool.request_weight.used + bars.weight()? <= pool.request_weight.limit;
# Ok::<(), Error>(())
```

Explicit `BudgetLimits` owners start from conservative documented values. USDⓈ-M
also tracks its documented funding/history, conversion, and monthly download-job limits. External
clients and frontend usage can consume the same budgets; local admission cannot
guarantee venue acceptance. It never waits, retries, or sends a command after expiry. The catalog omits
USDⓈ-M `testOrder` quota weights; an authorized demo probe
([verification #4](https://github.com/SharurTrading/binance-rs/issues/4)) found it charges
no IP weight and one slot on each order limit, which the client charges. That validation
endpoint does not submit to the matching engine.

### Reserving capacity for exits

The consumer chooses whether to reserve quota and how many native units to keep.
The SDK handles transport admission only: it maintains no positions, order engine,
accounting, or automatic flattening. Reserves default to zero. Retain the existing
shared owner or configuration to change policy at runtime through
`Budgets::set_exit_reserve` or a product `Config::set_exit_reserve`:

```rust
use binance_client::{AccountKey, Error, ExitReserve, usdm};

let account = AccountKey::new("main-account");
let config = usdm::Config::new_for_account(usdm::Environment::Production, &account)?;
let owner = config.clone();
let client = usdm::RestClient::new(config)?;
// The outer coordinator enables protection when its work needs capacity.
owner.set_exit_reserve(
    ExitReserve::default()
        .request_weight(40)
        .websocket_weight(20)
        .orders(5),
)?;
// Release protection when that work is complete; accepted usage stays charged.
owner.set_exit_reserve(ExitReserve::default())?;
# drop(client);
# Ok::<(), Error>(())
```

This setter replaces policy for every client sharing those IP, WebSocket and account
owners; constructing another default client does not clear it. IP reserves also affect
other accounts on that IP. Their account reserves remain separate; configure each account
owner explicitly. Set `ExitReserve::default()` to release a policy. One outer coordinator
should own policy for each shared quota owner: replacements are last-write-wins, not
additive leases. A lower reserve or release makes only remaining quota available;
raising it cannot reclaim already spent quota. Counters, accepted venue limits and
cooldowns remain intact when policy changes.

| Setting | Native quota units kept per applicable window |
| --- | --- |
| `request_weight` | REST/shared-IP request weight; includes WebSocket weight counted on REST |
| `websocket_weight` | Separate WebSocket API IP request weight |
| `raw_requests` | Raw REST request count |
| `orders` | Account order placements in every order window, including response-discovered and Margin/FIX windows |
| `sapi_weight` | SAPI endpoint weight in its native IP or UID scope |
| `sapi_requests` | SAPI endpoint request count in each documented second/minute window |

For each charged dimension with venue limit `L` and reserve `R`, ordinary attempts stop
at `L - R`, floored at zero; eligible attempts can use `L`. Both charge the same counter.
Every dimension must admit before any shared counter is charged. A zero-cost dimension
never blocks an attempt just because its counter is exhausted. New venue limit evidence
applies immediately, even when it falls below the configured reserve: ordinary charged
traffic then refuses, and eligible traffic still cannot exceed `L`. Reported usage and
limits remain the full venue figures, not an ordinary admission allowance.

Eligibility follows provider request semantics, independent of consumer intent:

| Product/transport | Requests eligible for reserved capacity |
| --- | --- |
| USDⓈ-M and COIN-M REST/WebSocket API | Exposed pure order/algo cancellations; new orders/algo orders with explicit `reduceOnly=true` or `closePosition=true`; regular new orders with exact `positionSide=LONG, side=SELL` or `positionSide=SHORT, side=BUY`; creation batches only when every member qualifies |
| Options REST | Pure option/block-order cancellations; `newOrder` with `reduceOnly=true`; nonempty `orders` creation batches whose every member has `reduceOnly=true` |
| Spot REST/WebSocket API | Pure single-order, order-list and all-open-order cancellations |
| Margin SAPI REST | Pure single-order, OCO and all-open-order cancellations |
| Convert SAPI REST | Pure `cancelLimitOrder` cancellation |
| Wallet SAPI REST | No exit eligibility; ordinary attempts honor reserves on any scopes they share |
| Spot FIX/FIX SBE | Pure `CancelOrder` and `CancelAll`, with a separately configured session message reserve |

Futures batches use native `batchOrders`; Options batches use native `orders`. Boolean
flags accept the native boolean `true` or exact string `"true"` where the request model
supports them. Mixed batches, cancel-replace, amendments, modifications, test orders and
auto-cancel timers remain ordinary. The hedge-side rule applies only to Futures
regular placements, not conditional algo placements or Options. A missing or `BOTH`
position side, the opposite hedge-side pairs, and spot sells remain ordinary.
Binance identifies the exact hedge-side pairs as reducing for its overload rule;
eligibility does not establish a held position, venue acceptance, or exemption from
normal order limits, IP budgets and cooldowns. Margin borrowing/repayment flags do
not make an order eligible.

A read is never eligible from its request alone. An exit that must first learn what
to cancel or close chooses an exit read explicitly: the same request through a
`_for_exit` method, admitted as eligible attempts are. The ordinary method for that
request stays ordinary, so a caller polling the same endpoint never spends the reserve.
An exit read is sent at most once, never waits for capacity, and is refused with
`Error::Admission` before sending only when the venue's whole limit is spent.

| Product | Exit reads |
| --- | --- |
| USDⓈ-M REST | `current_all_open_orders_for_exit`, `current_all_algo_open_orders_for_exit`, `position_information_v3_for_exit` |
| COIN-M REST | `current_all_open_orders_for_exit`, `open_algo_orders_for_exit`, `position_information_for_exit` |
| Spot REST | `get_open_orders_for_exit` |

Size the reserve for every read an exit makes as well as its cancels and closes. An
unscoped open-order list costs 40 weight on Futures and 80 on Spot; scoped to one
symbol it costs 1 and 6.

These classifications follow the official
[USDⓈ-M trade endpoints](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade),
[COIN-M trade endpoints](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade),
[USDⓈ-M overload rule](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/general-info),
[COIN-M overload rule](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/general-info),
[Options trade endpoints](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/trade),
[Spot trading endpoints](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/trade),
[Margin trade endpoints](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade),
[Convert limit cancellation](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#cancel-limit-order),
and [Spot FIX requests](https://github.com/binance/binance-spot-api-docs/blob/master/fix-api.md).
SAPI retains its endpoint IP/UID ownership: equal path text does not combine different
scopes. For example, Margin new-order UID weight and cancellation IP weight remain
independent. Additional endpoint IP request caps are checked independently of UID weight.

A FIX session starts with message capacity configured by
`spot::fix::Config::with_exit_reserve(messages)`. Retain its `Session` handle and call
`session.set_exit_reserve(messages).await?` to replace that policy at runtime; zero
releases it. The single session driver applies updates in FIFO command order and
acknowledges application before the setter returns success. Policy commands send no
wire message and consume no quota. Dropping the setter future after enqueue does not
retract the update. Retired sessions return a typed lifecycle gap. Use one outer
coordinator for each session's policy; replacements are not additive leases and do
not reset or refund message usage or change accepted venue limits.
Its ordinary applications stop below the session's message limit, while pure
cancellations and required protocol controls use the full limit and charge the same
counter. Required Logon, Heartbeat, TestRequest replies and Logout can consume protected
capacity. FIX order placements also honor the shared Spot account order reserve; an
account refusal does not spend session message capacity.

> [!IMPORTANT]
> Reserved capacity is a local admission policy, not guaranteed venue acceptance or
> an execution priority. Venue cooldowns and bans still refuse eligible requests.
> Other processes, manual venue traffic and required FIX controls can consume capacity.
> An admitted cancellation can still be rejected, and conditional reducing orders
> only execute when the venue's own conditions permit. No mutation waits or retries
> are introduced; an uncertain outcome still belongs to the caller to reconcile.

### Releasing a ban without timing

Binance documents a `418` ban as lasting
["from 2 minutes to 3 days"](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#http-418)
and gives no other signal that it has ended. When the reply carries no usable
`Retry-After`, the pool has no expiry to wait for, so every send on it is refused as
`Error::CooldownTimingUnknown` and no reply can clear that. The client invents no
expiry; the caller decides when to try again and says so:

| Call | Reaches |
| --- | --- |
| `release_unknown_ban()` on a Spot, USDⓈ-M, COIN-M, Options, Wallet, Convert or Margin `Config` or `RestClient` | The pool that configuration or client draws on |
| `Budgets::release_unknown_ban()` | An explicit owner |

It clears only the unknown-timing refusal, for every client of the pool at once. A
cooldown with a known `Retry-After` still holds until it expires. The release asserts
nothing to the venue: a ban that has not ended answers the next send with `418` again,
and the pool refuses again.

```rust
use binance_client::core_trading::spot;
use binance_client::Error;

let config = spot::Config::new(spot::Environment::Production)?;
// The caller has decided the ban is over, for example after waiting out three days.
config.release_unknown_ban()?;
# Ok::<(), Error>(())
```

## Development and verification

See [AGENTS.md](AGENTS.md), [CONTRIBUTING.md](CONTRIBUTING.md), and
[SECURITY.md](SECURITY.md). CI runs deterministic fault tests, strict Rust checks on
Linux, generated-binding freshness, documentation, packaging,
licenses/advisories, and full-history secret scanning. No credentialed CI or publishing.

The [CI workflow](.github/workflows/ci.yml) checks Binance's official Spot changelog
on every PR and `main` push, weekly on Mondays at 06:17 UTC, and on manual dispatch.
It compares `Last Updated` with our review date in
[spot-changelog-review.json](.github/spot-changelog-review.json). A newer update
fails the run and reports both dates and source links. Fetch/format errors also
fail; they cannot establish freshness. The check reads Binance's official
[Markdown documentation source](https://github.com/binance/binance-spot-api-docs/blob/master/CHANGELOG.md)
because the [developer page](https://developers.binance.com/en/docs/products/spot/CHANGELOG)
can return empty responses to scripted reads. Update `last_reviewed` through a PR
only after reviewing the protocol changes. Date-only comparison cannot distinguish
multiple edits made on the same day. The weekly schedule becomes active after
merging into `main`. The read-only freshness job uses public network access;
the checker tests use offline fixtures.

Read-only demo probes are ignored by default and require explicit invocation:

```sh
cargo test --test demo_read_only -- --ignored
```

These probes verify public metadata, depth and USDⓈ-M stream membership only. They do not prove authenticated
execution, account modes, or all long-tail endpoints work on demo or production.
Human review and separately authorized demo trading are required before claiming
execution readiness. There are no credentials or captured user data in the fixtures.

## License

[MIT No Attribution (MIT-0)](LICENSE).
