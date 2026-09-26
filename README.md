<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Binance Rust Client

[![CI](https://github.com/SharurTrading/binance-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/binance-rs/actions/workflows/ci.yml)

An **unofficial**, independent Rust inner client for Binance APIs, maintained by
SharurTrading. This project is not affiliated with or endorsed by Binance.

USDⓈ-M Futures is the first implementation: **95 REST operations, 18 catalog
WebSocket API methods plus 3 session methods, 20 market streams, and 10 user-data
event types**. Coverage follows the official catalog checked on 2026-09-26,
including conditional/algo orders and the current Public/Market/Private routes.
[Coverage and sources](docs/coverage.md) distinguish bindings from live verification.
The crate is unreleased and `publish = false` remains in force.

## Boundary and architecture

The caller owns the Tokio runtime, credentials, client order IDs, and recovery
of uncertain mutations. The client owns transport, signing, exact provider-native
DTOs, deadline/rate admission, safe outcome evidence, and streaming lifecycle.
Trading decisions, canonical instruments, risk, persistence, and portfolio
accounting belong to consumers. There is no dependency on a consuming platform.

```text
core: identities, credentials, signing, time, HTTP, sockets, rate budgets
  └── usdm: configuration, REST/WS requests, responses, streams, depth bootstrap
```

Spot and COIN-M will reuse the core and get separate product modules. Asset balances,
linear versus inverse settlement, position modes, and product routes stay native.
Portfolio Margin Pro is another API/account product, not a USDⓈ-M configuration
switch. It can reuse transport and signing without pretending its account or order
models are interchangeable. Current CM migration can share position-mode settings
with UM; distinct API modules do not imply independent account settings. No OMS Toolkit, rebates, or platform policy is included.

## Public demo metadata

```rust,no_run
use binance_client::usdm::{Config, Environment, RestClient};
use binance_client::usdm::rest_requests::ExchangeInformation;
use std::time::Duration;
use tokio::time::Instant;

# async fn example() -> Result<(), binance_client::Error> {
let client = RestClient::new(Config::new(Environment::Demo)?)?;
let response = client.exchange_information(
    &ExchangeInformation::new(),
    Instant::now() + Duration::from_secs(10),
).await?;
let definitions = response.data.symbols;
# let _ = definitions;
# Ok(())
# }
```

Financial fields use `Decimal`, parsed without floats or silent rounding. Request
builders validate required/conditional inputs on `build()` and again at dispatch.
Outgoing enum values are checked; incoming enum strings and unknown fields remain
open to provider additions. Callers use current exchange filters to check tick,
lot, notional, and other symbol-dependent rules; precision digits are not tick sizes.

## Execution and ownership

- Every operation requires a caller deadline. Validation, credentials, budget, and
  unsent expiration failures return `Outcome::NotSent`.
- Mutations are attempted once. Timeout, disconnect, malformed/truncated replies,
  and unknown future errors remain `Outcome::Unknown` unless documented evidence
  proves rejection. Query venue truth; a timeout does not cancel an order.
- Responses/errors retain safe status, venue code, rate counters, and retry timing.
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
  Listen-key renewal and reconnect are explicit caller operations; no automatic
  replay, recovery, credential loading, or hidden continuity claim.
- `DepthBook` implements snapshot bridging and the Futures `pu` chain. It retains
  bootstrap events and marks real gaps unproven. Its finite snapshot is always partial.

HMAC, RSA PKCS#8, Ed25519 PKCS#8, and external signers are supported. WebSocket
session logon requires Ed25519. TLS is required except exact loopback fixtures;
ambient HTTP proxies are ignored. Explicit HTTP proxies are supported; WebSocket
proxies are not. Sensitive strings and credentials redact `Debug`; callers must
not log explicit secret access or account payloads.

REST uses fresh HTTP/1 connections to eliminate Hyper's independent retry path on
reused connections, as well as disabling reqwest retries and redirects. This costs
connection setup per REST call; use the persistent WebSocket API for latency-sensitive
operations. Reqwest owns its internal HTTP transport machinery on the caller's
runtime; caller-owned, joinable drivers govern all client WebSocket lifecycles.

## Budgets

Clone a `Config` to share budgets between REST and WebSocket clients. Across accounts
on the same IP, use one `Budgets` owner and `for_account()`; reuse the resulting owner
for every credential/client of that account. REST and WebSocket general IP budgets
are distinct, account order limits are shared, and documented cross-transport order
weight is charged to both scopes. Separate independent `Config::new` values do not
coordinate IP usage automatically.

Baseline limits are conservative documented values, with demo REST limits from
exchange metadata. Configure `BudgetLimits` from current venue evidence. The client
also tracks funding/history, conversion, and monthly download-job limits. External
clients and frontend usage can consume the same budgets; local admission cannot
guarantee venue acceptance. It never waits, retries, or sends a command after expiry. The catalog omits
`testOrder` quota weights; the client conservatively reserves one IP unit and one
order slot pending [verification #4](https://github.com/SharurTrading/binance-rs/issues/4).
That validation endpoint does not submit to the matching engine.

## Development and verification

See [AGENTS.md](AGENTS.md), [CONTRIBUTING.md](CONTRIBUTING.md), and
[SECURITY.md](SECURITY.md). CI runs deterministic fault tests, strict Rust checks on
Linux/macOS/Windows, generated-binding freshness, documentation, packaging,
licenses/advisories, and full-history secret scanning. No credentialed CI or publishing.

Read-only demo probes are ignored by default and require explicit invocation:

```sh
cargo test --test demo_read_only -- --ignored
```

These probes verify public metadata/depth only. They do not prove authenticated
execution, account modes, or all long-tail endpoints work on demo or production.
Human review and separately authorized demo trading are required before claiming
execution readiness. There are no credentials or captured user data in the fixtures.

## License

[MIT No Attribution (MIT-0)](LICENSE).
