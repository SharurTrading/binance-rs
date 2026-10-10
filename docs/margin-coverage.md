<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Margin coverage and protocol evidence

Checked **2026-10-11** against Binance's official Margin REST catalog: **66
catalog operations**, minus one retired Cross Margin Pro operation, plus the recommended `POST /sapi/v1/userListenToken`
operation documented on the product's new listen-token page: **66 bindings**.
The bindings use **54 distinct REST paths**.
As of October 11, **65 dispatchable operations** exclude the future open-list
query. Bindings cover
account data/configuration, cross/isolated margin, borrow/repay and liability
history, market metadata, risk stream key lifecycle, low-latency special keys,
orders and OCO/OTO/OTOCO lists, liquidation loans, and transfer history/capacity.
These are provider protocol bindings; no trading, liquidation, allocation,
leverage, portfolio equity or risk policy is selected by the library.

`schema/margin-rest.json` pins the catalog URL, SHA-256, source documentation,
quota annotations and checked date. The product changelog overrides stale catalog
quota evidence: `queryMaxBorrow` charges 750 UID since 2026-04-16. The retired
Cross Margin Pro `leverageBracket` catalog operation is excluded entirely
(retired 2026-04-13); catalog presence is not claimed as working venue coverage. `schema/margin-coverage.json` records every
REST method/path. The extra listen-token operation cites its official product
page independently. Generated request builders validate both at `build` and
at dispatch, including required parameters, enum values, time ranges, quantity
versus quote spend, positive magnitudes, isolated-symbol requirements and caller
IDs for every placement leg. Margin IDs use private-field native wrappers:
integer/text order IDs retain exact spelling and can be queried again; the
order-list `-1` sentinel remains explicit individual-order evidence. Unknown
negative membership sentinels return `OrderListKindError` carrying the unchanged
identity; they never fabricate a normal list. Caller IDs
preserve caller-selected spelling; the Margin catalog defines no Futures grammar.
OTO working/pending quantities and sides remain
caller-selected; stop/limit/iceberg conditions retain the documented contract.
Pending OTO/OTOCO trailing deltas retain native `Decimal` values, including
fractional inputs, as specified by the [OTO table](https://developers.binance.com/legacy-docs/margin_trading/trade/Margin-Account-New-OTO)
and [OTOCO table](https://developers.binance.com/legacy-docs/margin_trading/trade/Margin-Account-New-OTOCO).
The OTOCO below type remains optional and absent on the wire when omitted; no
type is invented. Its type-dependent conditions apply when supplied; the
independent trailing-price and iceberg-GTC conditions still apply. No decimal passes through a float;
unrepresentable input and malformed financial data fail rather than round.

Cross account assets retain borrowed, interest, free, locked and net amounts.
Isolated accounts retain native base/quote assets and per-symbol margin data;
symbol spelling is never parsed into assets. Venue valuation fields retain
explicit BTC/USDT names and their native account mode. Commission assets remain
separate from the traded pair. Capacity queries pair responses with the
requested asset and isolated symbol; this is request provenance, not an
invented response field. Available inventory uses validated asset map keys and
exact Decimal values. Partial events never replace a full account snapshot.

SAPI budgets follow Margin general info: independent IP **12,000/minute** or
UID **180,000/minute** per endpoint. Dynamic fees are 1/5 for cross fee queries,
1/10 for isolated fee queries, and 6/1500 UID for placements depending on borrow
side effects. [Open-order queries](https://developers.binance.com/legacy-docs/margin_trading/trade/Query-Margin-Account-Open-Orders)
have weight 10 IP, and the page counts an all-symbol query as one request for
each symbol currently trading on the exchange.
Accordingly, all-symbol admission charges `10 * N`; this multiplication derives
from the documented weight and native request multiplicity. Supply
`TradingSymbolCount::new(N, expires_at_millis)` via the request's
`trading_symbol_count` setter, using a current authoritative unfiltered venue
listing and native `TRADING` status. Counts of account orders, selected symbols or
Margin-enabled pairs do not establish this authority. The caller chooses its
freshness expiry; no arbitrary TTL or hidden listing query is selected. Missing
or expired authority refuses with `Outcome::NotSent`; HTTP rechecks expiry both
before admission and immediately before sending. Count and expiry are local
admission evidence and never become wire parameters. Symbol queries charge 10
and require no count authority. Leverage adjustment
also enforces its independent **one request/minute/IP** cap. Placements require
explicit native account order authority from `query_current_margin_order_count_usage`.
Apply the complete response with `Config::order_limits` or
`RestClient::observe_order_limits`; reads remain available before configuration.
Every attempt reserves 1/2/2/3 orders for single/OCO/OTO/OTOCO placements. Unknown
quota types/intervals fail before changing accepted authority. Order windows and
reported count floors are shared by the account owner, independently of SAPI
endpoint weight. Clones share
budgets; `for_account` retains the common IP owner and creates a new UID owner.
Margin REST uses SAPI endpoint scopes independent of the Spot aggregate IP pool.
The WebSocket API defaults to the process Spot production IP weight/connection
pool, shared with production Spot clients. The [Margin token page](https://developers.binance.com/en/docs/products/margin-trading/listen-token-data-stream)
identifies the same `ws-api.binance.com:443/ws-api/v3` API route, and the
[WebSocket API rate-limit documentation](https://developers.binance.com/legacy-docs/binance-spot-api-docs/websocket-api/rate-limits)
specifies per-IP weight shared across all connections and a per-IP connection
limit. Selecting the same process pool follows that shared endpoint authority.
`WsConfig::with_pools` selects an
explicit registry, including independent synthetic fixture pools. An explicit
`Budgets::new(BudgetLimits::spot())` owner can include the documented API weight
and connection limits while also carrying independent SAPI endpoint scopes.
Margin account order windows retain native authority under either owner.
Headers update originating scope before body reads;
`Retry-After` and 418 bans remain evidence. Missing ban timing cannot authorize
a new send. No mutation retries, hidden queues, runtime or credential acquisition
are provided. HTTP/WS deadlines and token authority are rechecked before send.

Mutations preserve client IDs and typed uncertain outcomes after a truncated
body, timeout, transport loss, malformed response or 5xx. Only pinned Margin
refusal codes are definitive; future codes remain unknown. Borrow/repay APIs
without client IDs retain venue transaction IDs for history reconciliation.
Catalog order response supersets preserve ACK/RESULT/FULL optionality without
fabricating absent financial values. Cancel-all retains individual and order-list
receipt alternatives. Nullable pending OTO order IDs stay optional, as documented.
Published changes effective **2026-10-14** are identified separately: the
open OTO/OTOCO query binding refuses before that date, rechecking authority
immediately before send; nullable fill
commission/commissionAsset fields retain explicit unavailable evidence. Current
October 10 availability is not claimed from catalog bindings or synthetic CI.
API keys, secret keys, listen keys and tokens use redacted zeroizing storage;
unknown payload diagnostics are redacted. No signed URL or raw body is logged.

## Socket coverage and lifecycle

`RiskStream` uses the separate documented `wss://margin-stream.binance.com`
route for **cross margin only**, with native margin level/status and liability
principal/interest events. Caller-owned REST listen keys require explicit
renewal. The recommended trade stream uses `WsClient` on
`wss://ws-api.binance.com:443/ws-api/v3` and the unauthenticated
`userDataStream.subscribe.listenToken` method, charging its documented weight
of 2. `ListenToken::new` accepts externally issued native receipts with explicit
source scope. REST token issuance uses the official retained endpoint heading
`USER_STREAM` and the general security table: a valid `X-MBX-APIKEY` header,
without a signature or timestamp. The modern token page supplies the same native
parameters, quota and response. The security source URL, checked date and HTML
SHA-256 are pinned in the snapshot. Missing credentials refuse before admission
or network IO. Empty tokens, missing expiry and negative expiry in a 200 issuance
response fail decoding with an unknown mutation outcome, original status and quota
evidence, and no retry.
Tokens retain the scope selected by `isIsolated` (cross when false or absent),
without deriving it from symbol presence, and millisecond expiry. Reissuing a
token and subscribing again is explicit; no reconnect or replay is automatic.
The subscription acknowledgment's expiry is retained exactly: the official
example's magnitude differs from the REST millisecond example, so it never
overrides REST token authority.

Margin-native event DTOs are generated from the separately pinned stream
catalog. Execution reports, balance deltas, partial free/locked updates,
order-list status, listen-key expiry and token termination remain visible.
Unknown events retain original evidence; malformed known events produce an
explicit gap. Every event and late reply retains its socket generation;
late subscription replies retain caller correlation and quota metadata. Unexpected
raw stream-control answers retain control kind, caller streams and the complete
result alongside an explicit typed failure; Margin exposes no raw control API.
Ingress is unbounded and source ordered with depth, oldest-item age and progress
metrics. Age/backlog and ordinary silence never claim a continuity gap.
Drivers are returned to the caller: run them on the caller's runtime, request
close, drain accepted events through `Retired`, then join the driver.
The documented control-message/connection caps and ping/pong requirements are
handled by the shared socket transport. No market-data book is invented for
Margin; consumers can use the existing Spot market-data capability explicitly.

## Documented scope awaiting complete evidence

The September 25 changelog announces UTA Margin listen-key lifecycle paths and
an `fstream.binance.com/private/ws` route effective October 14. It does not
provide lifecycle parameters, authorization, quotas or complete native UTA event
schemas, and the current official REST catalog has no UTA operations. These
bindings are blocked on complete protocol evidence in
[issue #76](https://github.com/SharurTrading/binance-rs/issues/76). Cross/isolated
Margin models are not borrowed for UTA accounts, and announced scope is not
claimed as implemented coverage.

## Sources and verification

- [Margin introduction and scope](https://developers.binance.com/en/docs/products/margin-trading/Introduction)
- [Margin transport, signing, timestamps and budgets](https://developers.binance.com/en/docs/products/margin-trading/general-info)
- [Margin REST catalog](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api)
- [Margin changelog and effective dates](https://developers.binance.com/en/docs/products/margin-trading/change-log)
- [Margin error codes](https://developers.binance.com/en/docs/products/margin-trading/error-code)
- [Risk stream lifecycle and events](https://developers.binance.com/en/docs/products/margin-trading/risk-data-stream)
- [Retained token issuance security contract](https://developers.binance.com/legacy-docs/margin_trading/trade-data-stream/Listen-Token-Websocket-API)
- [Retained authentication table](https://developers.binance.com/legacy-docs/margin_trading/general-info)
- [Recommended listen-token subscriptions and expiry](https://developers.binance.com/en/docs/products/margin-trading/listen-token-data-stream)
- [Native Margin stream event schemas](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/ws-streams/~schemas)

Synthetic, credential-free contracts exercise order/borrow validation, exact
asset balances and malformed fields, special-key redaction, body truncation,
unknown/refused mutations, shared SAPI/order quota evidence, token authority and socket
ingress/retirement. No live or credentialed mutation was invoked.
