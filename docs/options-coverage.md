<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Options protocol coverage

The provider-native `options` module implements the current Options EAPI catalog:
44 REST operations, 10 routed market subscriptions, and six private event kinds.
This is Options coverage; these models do not reinterpret contracts as Spot pairs
or linear/inverse Futures. Catalog cardinality describes callable bindings, not
permission to trade, availability on every environment, or venue acceptance.

## Evidence

Protocol facts were checked on 2026-10-10 against the official
[REST catalog](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api),
[stream catalog](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/ws-streams),
[general information](https://developers.binance.com/en/docs/products/derivatives-trading-options/general-info),
[error codes](https://developers.binance.com/en/docs/products/derivatives-trading-options/error-code),
[stream connection contract](https://developers.binance.com/en/docs/products/derivatives-trading-options/websocket-market-streams/Connect),
[user-data semantics](https://developers.binance.com/en/docs/products/derivatives-trading-options/user-data-streams).

Pinned source digests and normalized wire facts live in `schema/options-rest.json`,
`schema/options-streams.json`, `schema/options-error-codes.json`, and
`schema/options-environments.json`; the latter pins current Options endpoint hosts
and baseline limits against official HTML, checked 2026-10-11. Generated
coverage records each operation and route in `schema/options-coverage.json`.
Generated bindings are reproducible with `python3 scripts/codegen/generate.py`.
Financial strings and numbers deserialize directly to exact `Decimal`, including
Greeks, strike prices, quantities, fees and underlying units. Unrepresentable data
returns an error; it is never rounded through a floating-point intermediate.

## REST and execution

REST includes account funding and margin information; public exchange information,
index, mark/Greek, exercise, depth, trades, candles, tickers and open interest; native
order submission, query/history and cancellation; batch operations; commissions,
positions and exercise records; market-maker block trades, protection and kill
switches; listen-key creation/renewal/closure; and the catalog's TradFi contract
operation. The latter is only its documented EAPI operation, not speculative
coverage of another product family.

Caller-supplied `clientOrderId` is mandatory for single and batch order placement.
The library preserves it in replies, metadata and uncertain-outcome errors. Native
order, trade, record, and block matching/settlement identities use private validated
newtypes; documented string-valued integer IDs keep their original wire form.
The catalog specifies signed 64-bit IDs without positive bounds; representable
zero and negative evidence remains intact and receives no inferred meaning.
Batch results retain every success, documented refusal, and unknown/malformed member in
input order. Native option symbols retain their full spelling; base, quote and
settlement assets come from exchange information. Financial account fields retain
their asset identities. No asset mapping is inferred from a contract's spelling.
Batch cancellation accepts exactly one nonempty identity list. The official
[downloadable REST schema](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-options/api/rest-api/1.0.0/schema.yaml)
explicitly forbids sending both lists in its Chinese parameter note; its English
note only states that at least one is required. Both notes remain in the snapshot.

Every mutation is attempted once. Timeouts, truncated responses, internal errors
and undocumented error codes remain uncertain. A caller resolves uncertainty with
venue reads. Block-trade creation has no documented client-order-ID input; its
returned matching and settlement keys remain native evidence, without inventing
an unsupported idempotency parameter.

`Config::new(environment)` selects explicit production (`eapi.binance.com`) or
[documented Options demo](https://developers.binance.com/en/docs/products/derivatives-trading-options/general-info#testnet-api-information)
(`demo-fapi.binance.com`) endpoints. The same Options documentation specifies demo
stream routes on `demo-fstream.binance.com`: `/public/`, `/market/`, and `/private/`.
These native Options environment facts are pinned in `schema/options-environments.json`.

Independent configurations share the process's Options IP pool for their environment.
`Config::with_pools(environment, &pools)` selects an explicit `WeightPools` registry;
Options, Futures, Spot, and SAPI pools and demo/production environments stay independent.
The native documented baseline is
[2,400 request weight/minute and 1,200 orders/minute](https://developers.binance.com/en/docs/products/derivatives-trading-options/common-definition).
No ten-second order ceiling is inferred. Exchange-information reads apply stated IP
limits to every client of the pool. `Config::pool_usage` and `RestClient::pool_usage`
report that shared authority, admitted usage, and reset timing.

Each independent configuration receives a separate account owner; clones share it.
`Config::budgets(owner.clone())` explicitly shares both IP and account authority;
`Budgets::for_account()` shares IP evidence while separating account scope.
`budget_limits` validates complete Options exchange-information rate authority,
requires both weight and order limits, and refuses unsupported intervals for an
explicit owner. Dynamic depth and symbol-dependent costs, order/batch counts,
response counters and cooldowns use the shared admission core.
History authority is rechecked before dispatch. For endpoints documenting three
months without a month-end convention, local admission refuses timestamps certainly
older than that window and leaves the boundary day to the venue; operator
timestamps are preserved exactly. Order history follows its documented five-day
window.

## Streams and depth evidence

The public and market routes are `/public/stream` and `/market/stream`; execution
uses `/private/ws/<listenKey>`. Distinct routes use distinct sockets. The current
Options contract allows 200 streams per socket and 10 incoming control messages
per second. The driver answers received pings, preserves accepted ingress in source
order without a fixed queue cutoff, exposes queue lag diagnostics, and reports
transport/malformed-data gaps and socket generation boundaries. The caller runs
and joins the driver, closes it explicitly, and drains its terminal boundary.

Private events are `ACCOUNT_UPDATE`, `BALANCE_POSITION_UPDATE`,
`ORDER_TRADE_UPDATE`, `GREEK_UPDATE`, `RISK_LEVEL_CHANGE`, and `listenKeyExpired`.
Changed balance/position entries remain partial updates. Commission asset identity
is retained independently. `ACCOUNT_UPDATE` exposes its documented USDT valuation
unit; consumers retain symbol-to-settlement metadata for position-only records.
Listen keys last 60 minutes and renew through REST; the caller owns renewal timing
and reconnection. Renewal does not assert continuity. Unknown future events are
retained with redacted diagnostics. No undocumented WebSocket trading API is
exposed.

Native depth payloads retain exact signed price/quantity pairs, snapshot
`lastUpdateId`, and stream `U`, `u`, and `pu` evidence. Finite snapshot rows remain
the venue's supplied rows. Consumers own depth bootstrap, continuity checks, and
any local book construction.

## Validation

Credential-free public contracts cover native identity and required order inputs,
exact and malformed decimals, batch partial evidence, partial balance identity,
native rate authority, process-wide IP sharing and explicit registry/environment
isolation, separate and explicitly shared account ownership, documented environment
hosts, cancellation identity member validation, single-attempt body loss and error-code classification,
source-ordered private ingress and joined retirement, native required nested
inputs, history admission, and native depth snapshot/update IDs and signed exact
decimals. All network fixtures are loopback.
Live mutation or credentialed verification is not performed by normal tests.
