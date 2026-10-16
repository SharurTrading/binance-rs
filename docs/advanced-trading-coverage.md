<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Advanced Trading coverage

Checked **2026-10-17** against the [official catalog](https://developers.binance.com/en/docs/catalog).
These are provider-native bindings with deterministic local fixtures. They have
not been exercised against live Advanced Trading accounts. Catalog completeness
and synthetic fixtures do not establish live execution readiness.

| Module under `advanced_trading` | REST | Public stream definitions | Execution event kinds |
| --- | ---: | ---: | ---: |
| `portfolio_margin` | 102 | — | 12 |
| `portfolio_margin_pro` | 24 | — | 2 |
| `algo` | 12 | — | — |
| `copy_trading` | 2 | — | — |
| `alpha` | 6 | 13 | — |
| `stocks` | 16 | 6 | 1 |

Institutional Loan is excluded from the retail trading scope. External-wallet
withdrawal submission and destination configuration are excluded crate-wide;
read-only withdrawal records and internal Binance transfers remain available.
[Wallet coverage](wallet-coverage.md) lists the excluded existing operations.
`schema/advanced-trading-scope.json` records these scope decisions.

## Protocol facts and generation

Each product has independent `schema/*-rest.json`, `*-coverage.json` and
`*-error-codes.json` facts. Streaming products also have `*-streams.json`.
REST and stream facts retain the official schema URL, SHA-256 of the downloaded
YAML and check date. Examples, translations and provider prose are removed;
annotations retain exact financial types, native asset identity, conditional
absence and required reconciliation evidence. The generator is offline and uses
only Python's standard library and the pinned rustfmt:

```sh
python3 scripts/codegen/generate.py --check
```

The shared configuration machinery is sealed: each product alias fixes its own
production REST host, stream routes and budget pool. No demo endpoint or account
mode is inferred from another product. Callers can override secure endpoints or
exact loopback fixtures, supply an explicit proxy for REST, and share a named
account with `Config::with_pools_for_account`. Clients sharing a registry must
use the same clock. WebSocket transport refuses a configured HTTP proxy.

## Product contracts

### Algo Trading

[Algo REST catalog](https://developers.binance.com/en/docs/catalog/advanced-trading-algo-trading/api/rest-api)
and [general info](https://developers.binance.com/en/docs/products/algo/general-info)
document separate Spot and Futures TWAP/VP operations on `api.binance.com`.
This service is separate from conditional Futures algo orders. Every placement
requires a caller-owned, exactly 32-character `ClientAlgoId`; the venue's optional
automatic ID generation is never used. Cancellations require `algoId` or
`clientAlgoId`. TWAP and Spot VP duration bounds are 300–86,400 seconds.
Quantities, execution amounts, prices and fees remain exact, with fee asset
identity preserved. `success: true` acknowledges the service request and does
not guarantee execution: query the corresponding native historical/sub-order
operations to resolve the final result.

### Portfolio Margin

[PAPI REST catalog](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin/api/rest-api),
[general info](https://developers.binance.com/en/docs/products/derivatives-trading-portfolio-margin/general-info)
and [user streams](https://developers.binance.com/en/docs/products/derivatives-trading-portfolio-margin/user-data-streams)
define a separate account/API product at `papi.binance.com`. UM, CM and Margin
balances, asset identities, native position sides and stream business unit `fs`
remain separate evidence. Empty unavailable financial fields remain `None`;
reported zero remains exact zero. There is no fabricated position or portfolio
valuation. Account and balance updates remain partial native updates.

Order/list submissions require caller IDs for every native leg. Conditional
orders validate their native trigger/quantity/price combinations, hedge-mode
restrictions, close-position rules and GTD timing without supplying defaults.
All-symbol Margin open-order reads require a current caller-provided
`TradingSymbolCount`; its expiry is rechecked before sending and is never sent
to Binance. The request costs five weight units per currently trading symbol.

PAPI uses its own process IP/account pool. Its general-info page states 6,000
IP weight/minute and 1,200 orders/minute. Those explicit current limits take
precedence over the older 2,400-weight common-definition example. Parameter-dependent weights include all-symbol algo reads. OCO charges
two placements. Download-job creation is a mutation despite using GET and counts
against the documented calendar-month quota. Account order-limit queries adopt
the native per-minute limit without requiring an undocumented usage count,
preserve local usage and bind all clients sharing that account. BNB transfers enforce the rolling
10-per-10-minute ceiling; auto-collection enforces 500/hour. Other interfaces,
including the venue UI, may consume account capacity outside this process.

Execution connects to `wss://fstream.binance.com/pm/ws/<listenKey>`. Listen-key
creation/keepalive/close use the PAPI API-key-only operations. The caller owns
renewal and reconnect; neither creates a continuity claim.

### Portfolio Margin Pro

[Pro REST catalog](https://developers.binance.com/en/docs/catalog/advanced-trading-derivatives-trading-portfolio-margin-pro/api/rest-api)
and [user streams](https://developers.binance.com/en/docs/products/derivatives-trading-portfolio-margin-pro/user-data-streams)
retain their own SAPI account models, collateral tiers, SPAN fields and risk
updates. BNB transfers enforce two calls per rolling ten minutes, independently
of endpoint weight; auto-collection enforces 500/hour. SAPI IP/UID endpoint
weight scopes remain shared with the other SAPI clients. Execution connects to
`wss://fstream.binance.com/pm-classic/ws/<listenKey>`; the documented key is
issued through the existing USDⓈ-M `POST /fapi/v1/listenKey` binding.

### Copy Trading

[Copy Trading REST catalog](https://developers.binance.com/en/docs/catalog/advanced-trading-copy-trading/api/rest-api)
provides the two signed Futures lead-trader status and symbol-whitelist reads.
The service's native string response code, boolean status, timestamps and
base/quote assets remain visible; no order-copying strategy is implemented.

### Alpha Trading

[Alpha REST catalog](https://developers.binance.com/en/docs/catalog/advanced-trading-alpha-trading/api/rest-api),
[general info](https://developers.binance.com/en/docs/products/alpha/general-info)
and [market streams](https://developers.binance.com/en/docs/products/alpha/websocket-market-data)
define public unsigned BAPI data on `www.binance.com` and the native
`wss://nbstream.binance.com/w3w/wsa/stream` endpoint. Callers obtain native
`ALPHA_<id><quote>` symbols from venue metadata; the crate does not infer them
from token display names. Token metadata, all 13 native streams, full-depth
`lastUpdateId`/`U`/`u`/`pu`, and twelve-column klines are retained exactly.
UI-only partial depth and full depth remain distinct kinds; no local book is
built, synchronized or substituted for the other kind.

Alpha's current pages specify no numeric weight or message ceiling. The Alpha
pool therefore imposes no invented venue ceiling, retains venue cooldowns, and
can enforce an explicit caller-provided raw-request budget. Financial magnitudes
outside Decimal's representable range fail decoding rather than rounding.

### Stocks Trading

[Stocks REST catalog](https://developers.binance.com/en/docs/catalog/advanced-trading-stocks-trading/api/rest-api),
[general info](https://developers.binance.com/en/docs/products/stocks/general-info)
and [stream protocol](https://developers.binance.com/en/docs/products/stocks/websocket-streams-general-info)
define native equity tickers, USD prices and separate quote assets. Order
validation follows the documented LIMIT/MARKET × BUY/SELL field matrix,
two-decimal limit-price ceiling and fractional GTC session restriction. The
caller must separately accept the venue's disclaimer; the client never accepts
it automatically. Trading and tokenization submissions require caller IDs.
Mint/redeem are internal venue conversions and expose no withdrawal destination.

SAPI endpoint weight and additional UID request caps both bind: 200 orders/minute
and 50 mint/redeem requests/minute, conservatively enforced as rolling windows.
Latest quote's documented empty HTTP body returns `None`; a malformed nonempty
quote fails explicitly. Native failed submission statuses become typed venue
errors. Accepted `S` receipts and in-flight `P` conversions remain native evidence
and do not establish a fill or completed conversion.

Public stocks streams use `wss://nbstream.binance.com/equity/ws`; execution uses
`/equity/ws/<listenKey>@orderReport`, keeping token-bearing names redacted.
The dedicated key-creation/renewal operation is explicit and caller-owned.

## Transport, ownership and verification

Every REST call uses a caller deadline. Mutations are attempted once, with safe
rate/status and caller identity evidence retained after failure. Unknown errors,
ambiguous 5xx, truncated replies and undecodable receipts remain uncertain.
Portfolio Margin's exact documented HTTP 503 refusal messages and code -1008
remain definitive rejections, without automatic replay. Each product's
error-code snapshot independently sources its validation/authentication refusal
classification; no product borrows another product's error list.

`Streams::connect(config)` opens a public socket without subscriptions. Run the
returned driver on the caller's runtime, then call `subscribe` with native typed
stream names. Execution uses a separate `Streams::user_data` connection. Close,
drain through `StreamEvent::Retired`, and join the driver. Ingress is unbounded
and source ordered, with lag diagnostics and explicit gaps. No reconnect,
resubscription, renewal, mutation replay or background runtime is hidden.
Known malformed financial evidence fails; future events/fields remain redacted
native evidence. Raw frames carry no fabricated combined-stream identity.

`tests/advanced_trading_contract.rs`, `tests/advanced_streams_contract.rs` and
`core::rate::rolling` fixtures cover native routes, exact financial parsing,
caller identities, conditional orders, empty quotes, rolling quota boundaries,
all-symbol authority expiry, subscription correlation and joined retirement.
All ordinary verification is synthetic and credential-free. No live trading,
account-mode change, tokenization, transfer or withdrawal is performed.

Portfolio Margin alone documents a 24-hour stream lifetime. Alpha, Stocks and
Portfolio Margin Pro impose no numeric lifetime in these sources, so the client
does not retire them on an inferred timer. Listen-key renewal remains caller-owned.
Stocks caller IDs retain the documented 32–36-character ASCII alphanumeric,
underscore and hyphen grammar. The legacy formatted `totalAvailableBalance` field
in the Pro REST account response stays in `extra`; documented replacement fields
remain exact decimals. HTTP 200 business failures become typed venue errors;
unknown execution statuses remain uncertain. Aggregate Stocks cancellation code
486447 preserves partial outcome without inventing per-order results.

Read-only probes on 2026-10-17 did not establish authenticated live coverage.
The crate's configured-proxy probes failed before response headers. Independent
requests using the environment's trusted HTTP stack received HTTP 451 from both
demo time endpoints and HTTP 200 (`000000`, `success: true`) from public Alpha
exchange information. No live mutation was attempted.
