<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Binance behaviour guide

> The Binance behaviours we had to discover the hard way, in plain words, so you do
> not have to discover them again.

## Who this is for

You are wrapping this client inside a trading platform, a bot or a data pipeline,
and you want to know how Binance actually behaves before you rely on it. The
[README](../README.md) says what the client does; this guide says what the venue
does and what you must do about it.

Most findings come from Binance's own pages, from this client's tests, and from a
small number of USDⓈ-M, COIN-M and Spot Demo sessions recorded in October 2026. Where
something was seen only on Demo, or only a few times, the text says so. Nothing here
is a promise about production behaviour.

| Word | Meaning here |
| --- | --- |
| Spot, USDⓈ-M, COIN-M | The three products this guide covers; Convert and Wallet get brief notes, and Margin and Options are not covered |
| Statement | A venue push that states a balance or position (`ACCOUNT_UPDATE`, `outboundAccountPosition`) |
| Fill | One trade on one of your orders, carried in `ORDER_TRADE_UPDATE` or `executionReport` |
| Pool | A rate-limit counter the venue keeps per IP address, shared by every client in your process |

## Connection

### Keys and authentication

| Fact | What to do |
| --- | --- |
| HMAC, RSA and Ed25519 keys all sign REST. WebSocket API session logon needs Ed25519 | Use Ed25519 if you want a logged-on WebSocket API session |
| Error `-2015` covers a bad key, a wrong IP or a missing permission. It does not prove the secret is wrong | Show the operator all three possibilities |
| A cheap signed read proves a key: Spot `GET /api/v3/account` (weight 20, add `omitZeroBalances=true`), USDⓈ-M `GET /fapi/v1/accountConfig` (5), COIN-M `GET /dapi/v1/balance` (1) | Prove the key once at connect, before you open any stream |
| The Spot account reply carries the account `uid`. No signed Futures read we know of returns it | Futures callers must be told the UID; you cannot discover it |
| The Spot `uid` need not match the UID supplied for the same key's USDⓈ-M account, since no Futures read returns one. On Demo (2026-10-16) the Spot `uid` differed from the UID supplied for the USDⓈ-M Demo account on one key. Demo Spot and Futures are separate hosts, and production keys were not compared | Identify a Spot account from its own signed `GET /api/v3/account` reply; do not reuse a Futures UID for it |
| Venue error text can echo URLs, IP addresses or tokens. The client keeps it behind `VenueMessage`, redacted in `Debug` and `Display` | Call `expose_sensitive_text()` only when you mean to, and never log or store the result automatically |

### Demo and production

Demo and production are separate hosts with separate rate-limit counters. Demo may
lack features that production has, and its replies can differ: the USDⓈ-M Demo
`testOrder` reply zeroes every field it does not echo (`orderId: 0`, empty symbol).
Pick the hosts with each product's `Environment`; production uses `api`, `fapi` and
`dapi.binance.com`.

The SAPI clients (Wallet, Convert, Margin) have no `Environment`: their configuration
starts at production `api.binance.com`, so point one at Demo with `rest_url`. That
changes only the host: a Demo-pointed SAPI client still shares the process's production
SAPI IP and UID pools unless given its own `budgets`. Seen once on Demo (2026-10-17): `POST /sapi/v1/asset/transfer` (`UMFUTURE_MAIN`, 1 USDT) sent to
`demo-api.binance.com` was answered HTTP 404 with a body that did not decode, which the
client reports as an `Unknown` outcome. Nothing moved: the USDⓈ-M wallet afterwards
equalled the stream's last statement. On 2026-10-17 this own-wallet transfer and its
`ACCOUNT_UPDATE` could not be exercised on Demo; an `Unknown` transfer still needs fresh
wallet evidence before its outcome can be established.

| Product | Demo REST | Demo streams | Demo WebSocket API |
| --- | --- | --- | --- |
| Spot | `demo-api.binance.com` | `demo-stream.binance.com` | `demo-ws-api.binance.com` |
| USDⓈ-M | `demo-fapi.binance.com` | `demo-fstream.binance.com` | `testnet.binancefuture.com/ws-fapi/v1` |
| COIN-M | `demo-dapi.binance.com` | `demo-dstream.binance.com` | `testnet.binancefuture.com/ws-dapi/v1` |

### Rate limits

Binance counts request weight per IP address, not per key, and counts orders per
account. The client keeps one pool per venue (Spot, Futures, Options) and environment
for the whole process, so every client you build shares it. The details are in the README's
[Budgets](../README.md#budgets) section; the parts that surprised us are below.

| Behaviour | What it means for you |
| --- | --- |
| USDⓈ-M and COIN-M share one IP weight counter and one order budget per account | A busy COIN-M reader slows USDⓈ-M trading on the same IP |
| The documented limits are examples. Production `exchangeInfo` stated different figures on 2026-10-11 | Read exchange information once per pool before trusting a limit |
| Every reply's `X-MBX-USED-WEIGHT-1M` states the whole IP's usage, including other processes | A second process on the same IP is seen only through this header, one reply late. Run one process per egress IP if you can |
| A `418` ban can last "from 2 minutes to 3 days" with no end signal | Without `Retry-After` the client refuses every send until you call `release_unknown_ban()` |
| All clients on one pool must share one clock | Give every client of a registry the same `Arc<dyn Clock>`, or they charge different windows |
| Futures order limits are per account, across keys and across USDⓈ-M and COIN-M | Give every config for one account the same `AccountKey`, or each gets its own count and you over-send |
| A full USDⓈ-M account read cost 95 weight on Demo (2026-10-16): Account V3 (5), Balance V3 (5), Position Information V3 (5) and both open-order lists, regular and algo, without a symbol (40 each). The symbol-scoped read cost 7: Position Information V3 (5) and both lists with a symbol (1 each), with no balance read. Both matched the documented weights, so you can price your own read. Polled each second, the full read exhausts the documented 2,400-weight Futures minute, which USDⓈ-M and COIN-M share | Poll the symbol, and take the full read only when you need it, such as at connect or after a gap. The scoped read does not refresh balances, so read them separately. An open-order list is documented at 40 weight unscoped and 1 with a symbol |

### Keeping capacity for exits

You can reserve part of each budget for cancellations, position-reducing orders and
the reads an exit makes first with `set_exit_reserve`, described in the README's
[Reserving capacity for exits](../README.md#reserving-capacity-for-exits). Four
things matter when you wrap it:

1. One owner per shared budget. Replacement is last-write-wins, so two profiles of one
   account that each set their own reserve undo each other.
2. A reserve cannot give back quota that is already spent, and it does not beat a
   venue ban or cooldown.
3. On Futures, a sell is not an exit. Only an explicit `reduceOnly=true`,
   `closePosition=true`, a pure cancellation, or an exact hedge-mode close
   (`LONG`+`SELL`, `SHORT`+`BUY`) qualifies.
4. A read is not an exit either. An exit that does not yet know what it holds, such
   as one started before your own state was seeded, must read the open orders and
   positions before it can name a cancel or a close. Call the read's `_for_exit`
   method for those reads: USDⓈ-M and COIN-M open orders, open algo orders and
   position risk, and Spot open orders. The ordinary method is never admitted from
   the reserve, so a busy poller cannot spend it, and an exit that uses the ordinary
   read is refused once the ordinary share is spent, before it can cancel anything.
   These are REST methods. WebSocket API cancels are eligible, but its reads, such as
   Spot `openOrders.status` and Futures `account.position`, have no exit variant, so
   an exit over the WebSocket API reads over REST.

For bounded startup or recovery work, use an explicitly named `_for_recovery`
REST method from the README's [recovery-read list](../README.md#reserving-capacity-for-exits).
This admits only the listed account-evidence and startup metadata reads from the
same reserved capacity; metadata still validates and adopts stated venue limits.
A malformed statement changes no limit and returns `ReadFailed` with typed rate
failure and received response metadata, including safe header counters.
Every native weight, deadline and cooldown still applies, with no wait or retry.
The caller controls the finite recovery workload: a recovery read can consume
capacity an exit would otherwise use. Ordinary polling keeps ordinary admission.

The same section lists a separate closed set of account-stream lifecycle APIs.
Futures start, keepalive and close listen-key controls and Spot's recovery socket
handshake, signed subscribe and unsubscribe may spend reserved capacity. They
retain native mutation uncertainty, costs and cooldowns, and never mutate orders
or balances. Other requests on a recovery socket retain ordinary admission. The
caller still owns stream renewal, driver tasks and joined teardown.

WebSocket connection attempts retain rejected HTTP upgrade status and safe rate
headers. A received `429` cooldown or untimed `418` ban refuses the next attempt
locally, whether ordinary or recovery. A single owned deadline covers name lookup,
TCP, TLS and native upgrade validation; no reconnect or retry is hidden.
When the receipt clock fails, HTTP and upgrade responses retain their safe metadata
and report `RateEvidenceFailed` with the original mutation/read outcome. The shared
IP pool refuses later attempts with `CooldownTimingUnknown`; it does not date
counters or relative retry timing at the earlier send time. Restore valid time and
resolve the received evidence before explicitly releasing the unknown timing hold.

### User data streams

| Product | How you receive account events |
| --- | --- |
| Spot | WebSocket API subscription (`userDataStream.subscribe`); the old listen-key REST calls are gone |
| USDⓈ-M, COIN-M | A listen key from `POST /fapi/v1/listenKey` or `/dapi/v1/listenKey`, then a private socket |

- A listen key lapses after 60 minutes without a keepalive. Starting a stream while
  one is active returns the same key and extends it. The client does not renew it
  for you.
- A lapsed key arrives as a `listenKeyExpired` event. Treat everything after it as
  unknown until you have read the account again.
- Closing a key ended the stream without `listenKeyExpired`. Seen once on USDⓈ-M
  Demo, on 2026-10-15: after `DELETE /fapi/v1/listenKey` no `listenKeyExpired` event
  arrived. The socket closed about 4 ms after the close reply, and a new stream had
  to be opened. Treat the socket closing as the signal; do not wait
  for `listenKeyExpired`.
- Core trading sockets and Portfolio Margin have a documented 24-hour life.
  The client ends those generations with a gap at the limit. Other Advanced
  products use their own documented lifetime; see the
  [Advanced stream policies](advanced-trading-coverage.md#transport-ownership-and-verification).
- Renewing a key is not continuity. Neither is a new socket: nothing is replayed, and
  events sent while you were away are gone from the stream.

### Socket lifecycle and failure causes

Each socket generation reports `Established`, then data, then `Retired`. Anything
that breaks continuity becomes one `Gap` with a reason.

| Gap reason | What happened |
| --- | --- |
| `venue closed socket`, `WebSocket EOF`, `WebSocket transport loss` | The connection ended or broke |
| `documented 24-hour generation lifetime` | The socket reached its 24-hour limit |
| `pong timeout`, `documented ping/pong rate exceeded` | The connection could not answer the venue's pings in time |
| `malformed JSON event`, `malformed execution/account event` | A known event did not match its documented shape |
| `account event has no balance/position evidence` | An `ACCOUNT_UPDATE` carried neither `B` nor `P` |

A new event kind the client does not model is not a gap: it arrives as
`UserPayload::Unknown` and the socket keeps going. Do the same in your layer. We once
lost continuity because a consumer treated the documented `TRADE_LITE` event as
unsupported and turned it into a gap.

Message ceilings: Spot sockets accept 5 incoming messages a second, Futures 10, and
pongs count. The client keeps the last slot for a pong, so you can send at most four
control messages a second on Spot and nine on Futures. A message past that is
refused unsent with a retry delay.

On REST, an HTTP `200` does not mean the reply arrived. We saw a USDⓈ-M listing fail
after about ten seconds with status `200` while a later read succeeded. Since
[#128](https://github.com/SharurTrading/binance-rs/pull/128) the error's
`TransportFailure` says which step failed: headers, rate evidence, body read, decode
or response model.

### Shutdown

1. Call `close()` on the handle.
2. Keep receiving events until `Retired`.
3. Join the driver task you own.

Do not drop the event receiver first. The driver then cannot deliver its final
events, ends with `Closed`, and anything still queued is lost. If the socket already retired itself after a gap,
`close()` returns `Closed`. That is expected once you have drained `Retired` and the
driver joined without error. REST needs no logout.

## Data

### Exchange information and filters

| Fact | Detail |
| --- | --- |
| Tick size | `PRICE_FILTER.tickSize`. Precision digits such as `pricePrecision` are not tick sizes |
| Quantity step | `LOT_SIZE.stepSize`. `minQty` can be zero, which is a valid lower bound |
| Market orders | Have their own `MARKET_LOT_SIZE` bounds |
| Notional | Futures `MIN_NOTIONAL`; Spot `NOTIONAL`, else `MIN_NOTIONAL` |
| Zero bounds | A `minPrice` or `maxPrice` of zero means no bound, not a bound at zero |
| Margin rates | `requiredMarginPercent` and `maintMarginPercent` are percentages; divide by 100 |
| Fees | No listing row states a commission; read the commission-rate endpoint |
| Precision seen | USDⓈ-M Demo ticks went down to `0.00000001` (eight places) on 2026-10-13; all fit `Decimal` |
| Pending contract | A futures listing can include a contract in `PENDING_TRADING`, not yet tradable, whose `PRICE_FILTER.tickSize` is `"0"`. We saw it once: USDⓈ-M Demo's `ELSAUSDT`, inspected on 2026-10-17. Whether other pending contracts, or a production listing, carry zero filters is not recorded. A zero tick is no tick: a row like that cannot be priced, so read the listing again once it trades |

The minimum order quantity moves with the price, because the notional filter is in
quote currency. Work it out from the current price and the filter immediately before
you send. A minimum we derived one day was too small the next.

### Symbols and contract types

- Dated futures on USDⓈ-M and COIN-M are named `<PAIR>_<YYMMDD>` after their UTC
  delivery date (`BTCUSDT_261225`, `BTCUSD_261225`). A USDⓈ-M perpetual keeps the
  plain pair (`BTCUSDT`); a COIN-M perpetual is spelled `<PAIR>_PERP`
  (`BTCUSD_PERP`). A name alone tells a perpetual from a dated contract, but not a
  weekly from a monthly or a quarterly: only the listing row's `contractType` says.
- Two weekly contracts can deliver in one month. Binance documents no delivery day
  for monthlies, so a monthly and a quarterly could share a month.
- Listing rows contained `contractType` values no enum lists: `CURRENT_WEEK`,
  `NEXT_WEEK`, and a space-spelled `CURRENT_QUARTER DELIVERING`. The client keeps
  these as `Unknown` with the exact text ([#80](https://github.com/SharurTrading/binance-rs/issues/80)).
- `PERPETUAL_DELIVERING` is a perpetual being delisted. Its `deliveryDate` has no
  documented meaning; do not read it as an expiry.
- Statuses `PRE_SETTLE`, `TRADING_HALT` and `TRADING_CANCEL_ONLY` appear in the enums
  but no page defines them, and we have not seen one live.
- Binance documents no roll notice. `!contractInfo` pushes listings, settlements and
  bracket changes; whether it pushes a quarterly roll is unknown. Re-read the listing
  to see a roll.

### Prices and quantities

- Prices are signed. A level, settlement or strike can be zero or negative; keep it.
- Futures depth quantities are absolute, so the client refuses a negative one. Spot
  states no such rule, so the client keeps the sign and leaves the decision to you.
- The Spot reference price `r` need not sit on the tick, and `null` means there is no
  reference price at that moment. It is not zero.
- USDⓈ-M aggregate quantity `q` includes Retail Price Improvement trades; `nq`
  excludes them.

### Time fields

| Event | Times it carries |
| --- | --- |
| Spot `trade`, Futures `aggTrade` | `T`, the trade time |
| Spot `bookTicker` | None. Use your own receipt clock |
| Futures `bookTicker` | `T` and update id `u` |
| Spot diff depth | `E` only |
| Futures diff depth | `E` and `T`; use `T` |
| Depth snapshot | Futures replies carry `T`; Spot replies carry no time |
| Kline | Open time; the close time is the last included millisecond (`...999`) |
| Mark price | `E` |
| User events | `E` (event) and `T` (transaction); see [Event order](#event-order-statements-and-fills) |

Klines are UTC. Days open at 00:00 UTC and weeks on Monday 00:00 UTC (documented).
Months opened on the first UTC day in every sample we read, but no page says so.
Spot klines accept a `timeZone` parameter; leave it out to stay on UTC. A live kline
is closed only when `k.x` is true.

### Depth books

The client keeps update ids and exact levels but builds no book. Spot and Futures
use different joining rules, so do not share one routine between them.

| Step | Spot | Futures |
| --- | --- | --- |
| Discard buffered events | Final id `u` at or before `lastUpdateId` | `u` before `lastUpdateId` |
| First kept event | `U` at most `lastUpdateId + 1` | `U` at most `lastUpdateId`, and `u` at least |
| Each next event | `U` at most the book's id + 1 | `pu` equals the previous event's `u` |
| Extra check | Re-fetch if the snapshot is older than the first buffered event | — |

A 1,000-level snapshot costs 50 weight on Spot and 20 on Futures. Levels deeper than
the snapshot appear once they change.

### Trades and history

| Question | Spot | USDⓈ-M and COIN-M |
| --- | --- | --- |
| Live prints | Individual trades (`trade`) | Aggregates only (`aggTrade`); no individual-trade stream |
| What an aggregate groups | Fills of one taker order at one time and price | Market trades within 100 ms at one price on one taking side |
| `aggTrades` by time | `startTime` alone works, any age | Only the past 48 hours; with both `startTime` and `endTime`, under one hour |
| Individual history | `historicalTrades`, 1,000 a page, no key needed | `historicalTrades`, 500 a page, weight 200, key header required, last "one month" only |
| Left out | — | Insurance-fund and ADL trades, from both live and history |

Trade ids rise with time within a symbol, and an aggregate's `f` and `l` are the
first and last individual trade ids it covers. That lets you join history to a live
feed by trade id with no double counting: note the first live `f`, keep history below
it, take live from it.

Past 48 hours on Futures there is no index from time to trade id. You must search the
id space page by page, and each page costs 200 weight, so a deep search can spend
more than a minute's budget.

When a history page is full, the next page can repeat or skip prints that share the
last millisecond. Drop the newest millisecond from a full page and ask again from it.

## Execution

### Order types and modes

| Topic | Behaviour |
| --- | --- |
| Client order ids | The client requires `newClientOrderId` on every new order on USDⓈ-M, COIN-M and Spot, and `clientAlgoId` on every new algo order on USDⓈ-M and COIN-M (Spot has no algo orders), although the venue treats both as optional (documented). A Spot order list needs `listClientOrderId` and a client id for each leg. Keep your own and query by it later |
| USDⓈ-M conditional orders | Stop, take-profit and trailing orders go to the algo endpoint (`/fapi/v1/algoOrder`) with `clientAlgoId`. Regular `newOrder` takes only `LIMIT` and `MARKET` |
| Two namespaces | Regular and algo orders have separate ids and open-order lists, and on USDⓈ-M separate cancel-all calls (COIN-M has no conditional cancel-all, below). Check both lists before you call an account clear |
| COIN-M conditional orders | Binance documents no COIN-M conditional cancel-all. The [UM/CM integration notice](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice), section C.5, names only `POST` and `DELETE /dapi/v1/algoOrder` and `GET /dapi/v1/openAlgoOrders`, where USDⓈ-M also has `DELETE /fapi/v1/algoOpenOrders`. The client offers `new_algo_order`, `cancel_algo_order` (one order) and `open_algo_orders`: list the open algo orders and cancel each one. For an exit, list them with `open_algo_orders_for_exit`; each `cancel_algo_order` is already admitted from the exit reserve |
| Post-only | Futures `GTX`; Spot uses order type `LIMIT_MAKER` instead of a time in force |
| GTD | Expiry must be more than 600 seconds after venue time; the client rechecks just before sending |
| GTC lifetime | Futures GTC orders live one calendar year ([2024-04-09 change log](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/change-log#2024-04-09)) |
| `RPI` | A USDⓈ-M time in force open only to approved accounts |
| Futures batches | At most five orders. Each member succeeds, fails or stays unknown on its own; never replay the whole batch |
| Spot amend | Only reduces quantity ([keep priority](https://github.com/binance/binance-spot-api-docs/blob/master/faqs/order_amend_keep_priority.md)) |
| Self-trade prevention | Futures modes and defaults are per product; Spot reads them per symbol from exchange information. A missing reported mode is unknown, not `NONE` |

### One-way and hedge mode

| | One-way | Hedge |
| --- | --- | --- |
| `positionSide` | `BOTH` | `LONG` or `SHORT` on orders; COIN-M position and account reads also list a `BOTH` row per symbol ([Reading account state](#reading-account-state)) |
| How to close | `reduceOnly=true` on the opposite side | `LONG`+`SELL` or `SHORT`+`BUY`; `reduceOnly` is forbidden and the client refuses it |
| `closePosition=true` | Cannot be combined with `quantity` or `reduceOnly` | Same |

Since the CM migration, USDⓈ-M and COIN-M share one position-mode setting: changing
it on one flips both.

Seen once in hedge mode on USDⓈ-M Demo (`BTCUSDT`, 2026-10-17): LONG 0.0007 and SHORT
0.0007 opened, SHORT closed, then LONG closed, with `MARKET` orders.

| Fact | What to do |
| --- | --- |
| Each `ACCOUNT_UPDATE` carried one position row, for the side that order changed; the closed side was stated at `pa` 0 | Key a statement by symbol and `positionSide`; a push says nothing of the other side |
| No `ACCOUNT_UPDATE` on the stream carried a `BOTH` row, flat or not, unlike COIN-M's hedge-mode position and account reads ([Reading account state](#reading-account-state)) | Do not wait for a `BOTH` row on the stream in hedge mode |
| The closes were sent without `reduceOnly`, which hedge mode forbids, and their `ORDER_TRADE_UPDATE` rows came back with `R` true | Read `R` as stated; do not infer from it that you sent `reduceOnly` |
| Each statement's `T` equalled its fill's and arrived before it, as in one-way mode | Place statements by `T` ([Event order](#event-order-statements-and-fills)) |

Futures have no close request. In one-way mode a close is a `reduceOnly` order, so a
refused close there is a refused `reduceOnly` order. We recorded none. In the client's
copies of Binance's documented USDⓈ-M and COIN-M REST response samples, the only
reduce-only refusal is `-2022` `ReduceOnly Order is rejected.`, a failed member of a
batch order (on Spot, the client's tests use `-2022` for a failed cancel-replace, which
is not a reduce-only refusal). The client's copies of the USDⓈ-M user-data-stream samples
also show a conditional order cancelled with `rm` `Reduce Only reject` in an
`ALGO_UPDATE`. Neither says anything about the position. Do not read either as "the
position is already flat": read the position, from the stream's statement or Position
Information, before you decide what the account holds.

### Conditional orders and cancel-all

Seen on Demo on 2026-10-16: USDⓈ-M in one-way mode on `BTCUSDT`, and Spot.

| Fact | What to do |
| --- | --- |
| A resting `STOP_MARKET` sent with `reduceOnly=true` and a quantity equal to the whole position was not cancelled when another order closed the position. It stayed working on the flat position for the 30 seconds we watched, until we cancelled it. A `closePosition=true` stop was not recorded | After any close that did not go through the protective order, cancel the protection yourself, then read the algo open-order list |
| `ALGO_UPDATE` arrived on the USDⓈ-M user stream when a conditional order was placed and when it was cancelled. We recorded no other transition (trigger, expiry or trigger reject), and whether an `ORDER_TRADE_UPDATE` also arrived for the placement or the cancel is not recorded | Handle it as its own event. Only the USDⓈ-M client decodes it, as `usdm::UserPayload::AlgoUpdate`; `coinm::UserPayload` has no such variant, so an `ALGO_UPDATE` on a COIN-M stream would arrive as `UserPayload::Unknown`, which is not a gap |
| USDⓈ-M `DELETE /fapi/v1/allOpenOrders` and `DELETE /fapi/v1/algoOpenOrders` each answer `{"code":200,"msg":...}` with no order list. The cancellations show only on the user stream | Keep the stream open across the call and take the cancellations from it, then read both open-order lists |
| Spot `DELETE /api/v3/openOrders` answers with cancelled orders and one entry for each cancelled order list. A list entry carries no `orderId` | Match `BatchResult::Success` as `DeleteOpenOrdersResponseItem::Order` or `OrderList`. List receipts retain their member orders and reports. Unknown future or malformed members remain `BatchResult::Unknown`; resolve those members with venue reads |
| Spot WebSocket API `openOrders.cancelAll` answers with the same two kinds of entry | Match `OpenOrdersCancelAllResponseItem::Order` or `OrderList` inside each successful batch member. Keep failures and unknown receipts alongside the known receipts |

### Event order: statements and fills

On the USDⓈ-M user stream, a fill produces a compact `TRADE_LITE`, a balance and
position statement (`ACCOUNT_UPDATE`) and the full fill (`ORDER_TRADE_UPDATE`).

> [!IMPORTANT]
> In every USDⓈ-M and COIN-M Demo sample we recorded (2026-10-13 to 2026-10-17), the
> statement arrived before its full fill, by under two milliseconds (0.2 to 1.7 ms as
> received, over the 27 pairs counted below). This is a small Demo sample, not a guarantee: do
> not depend on either order or on a bound for the gap.

What held in every sample:

- Across the 25 USDⓈ-M statement and fill pairs we recorded, the statement's
  transaction time `T` equalled its fill's in 21 and was one millisecond after the
  fill's in the other four (two on 2026-10-13, two on 2026-10-14). It was never before
  the fill's. Their event time `E` matched in all 25. The two COIN-M pairs
  (`BTCUSD_PERP`, 2026-10-17) had the same shape: `T` one millisecond after the entry's
  fill and equal to the exit's, `E` matching, the statement first by 0.38 and 0.31 ms.
- A statement covered every fill stamped at or before its own `T`. Place statements
  by `T`, not by the order they arrive in.
- One market order filled in two parts. Both parts had the same `E` and `T` but
  distinct trade ids `t`, prices and fees. Deduplicate by trade id, never by time.
- `TRADE_LITE` arrived earlier still. It has no fee and no realised PnL, so book
  money only from `ORDER_TRADE_UPDATE`, and do not count the same trade twice.

COIN-M has `ACCOUNT_UPDATE` and `ORDER_TRADE_UPDATE` but no `TRADE_LITE`, and its user
events name the account by its alias in a root `i` field, which USDⓈ-M's do not. One
COIN-M round trip is recorded (2026-10-17), so partial fills and bursts are not.

Spot has no positions. Seen once on Spot Demo (`BTCUSDT`, a market buy and sell,
2026-10-17): each order's `executionReport` `NEW`, then its `TRADE`, arrived before one
`outboundAccountPosition`, and within each leg the report's `E` and `T` and the push's
`E` and `u` were the same millisecond. The push listed the leg's base and quote asset and
BNB at zero, which had not changed, but not the rest of the wallet: it never replaces your
whole balance list.

### Fees, PnL and balances

| Field | Meaning |
| --- | --- |
| `n`, `N` | The fill's commission and its asset; the asset can differ from the pair's |
| `rp` | Realised PnL the venue states for that fill |
| `ep` | Entry price in the position statement. It was rounded to eight places in our samples (see [Entry price precision](#entry-price-precision)), so work out averages from the fills yourself |
| `wb` | Wallet balance in the statement |
| `ma` | Margin asset of a position row in the statement. It was `USDT` in all 19 USDⓈ-M Demo position rows we recorded (2026-10-13 to 2026-10-16). This is a gap in the client: `usdm::stream_models::AccountUpdateEventAUpperPItem` has no field for it, so it stays only in the row's `extra` (tracked in [#144](https://github.com/SharurTrading/binance-rs/issues/144)) |

USDⓈ-M `ORDER_TRADE_UPDATE` carries no margin asset. `N` is the commission's asset,
which can differ from the contract's, and `rp` names no currency: it is in the
contract's margin asset. These USDⓈ-M payloads state that asset, and we recorded each:

- the listing row (`marginAsset`);
- each position row of an `ACCOUNT_UPDATE` (`ma`, see the table above);
- each Position Information V3 row (`marginAsset`);
- each `userTrades` row (`marginAsset`).

The client's Account V3 position row has no margin asset field. A fill that arrives
with none of these at hand needs the listing before its money can be booked.

The COIN-M client leaves `ma` in `extra` too. Our COIN-M `ACCOUNT_UPDATE` captures of
2026-10-11 and 2026-10-17 show it on a position row (`BTC` on `BTCUSD_PERP`). COIN-M's
`ORDER_TRADE_UPDATE` also carries `ma` on the order row (`BTC` on 2026-10-17), which the
COIN-M client models; USDⓈ-M's does not.

In our USDT-margined samples, and in the BTC-margined COIN-M round trip, the wallet moved
by exactly the stated `rp` minus the fee on every fill. Use the venue's figures as stated
and compute derived values from exact inputs, not from rounded statement fields.

`rp` is stated to eight places after the venue's own arithmetic, so it need not equal
the exact figure from the fills: a USDⓈ-M SHORT 0.0007 closed from 81482.4 at 81487
(exactly -0.00322) was stated `-0.00321999`, a partial close of 0.0001 (exactly -0.00072)
`-0.00071999`, and a COIN-M inverse close of one 100 USD contract from 81384 at 81361.9
(about -0.000000333758 BTC) `-0.00000033`. In these three recorded closes the stated
figure was within one unit of its eighth place of the figure computed from the fills;
an exact comparison reported all three as differences.

Seen once on USDⓈ-M Demo (`BTCUSDT`, long 0.0007, cross margin, 2026-10-17): the 16:00
UTC funding fee arrived as one `ACCOUNT_UPDATE` with `m` `FUNDING_FEE`, a single `USDT`
balance row whose `bc` was the fee (`-0.00569254`), `a.S` naming the symbol, and an empty
`P`: no position was restated. Its `T` was 511 ms after the funding time and its `E`
205 ms after that. Binance's [USDⓈ-M balance and position update documentation](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/user-data-streams#event-balance-and-position-update)
says an isolated position's funding update also carries that position's row; that was
not recorded.

### Entry price precision

Seen on USDⓈ-M Demo (`BTCUSDT`, 2026-10-13 to 2026-10-16): Binance stated a position's
average entry price at different precisions on different endpoints, and did not keep
one spelling for one number. After three market entries of 0.0007 on 2026-10-14, the
same long 0.0021 was stated twice:

| Source | Field | Value | Decimals |
| --- | --- | --- | --- |
| Private stream `ACCOUNT_UPDATE` after the third entry (`pa` 0.0021) | `ep` | `81648.27142857` | 8 |
| REST Position Information V3, read at 2026-10-14T16:18:04Z | `entryPrice` | `81648.27142857143` | 11 |

The statement after the second entry's second partial fill (`pa` 0.0014) states
`81652.55714286`, also eight places. The stream's break-even price `bep` was stated at
eleven places on 2026-10-13 (`83455.76896000002`), at ten on 2026-10-14
(`81685.2181657143`) and at eleven on 2026-10-16 (`81399.88901333335`), so one row does
not hold one precision either. Trailing zeros were not kept consistently: on 2026-10-14
the stream stated `83387` where both REST reads of the same position stated `83387.0`,
and on 2026-10-16 stream and REST both stated `81351.1`, as the SDK decoded them.

So a consumer can neither assume one precision nor read it from the digits sent.
Compare a stated average with one derived from the fills within a tolerance, never for
equality.

### Reading account state

| Read | Behaviour |
| --- | --- |
| USDⓈ-M Position Information V3 | Returns only symbols with a position or an open order, so a flat account gives an empty list. Pair it with both open-order lists before you call the account flat |
| COIN-M Position Information | Sent without `marginAsset` or `pair`, it returns only symbols in `TRADING` status, so a position on a contract that is delivering, settling or halted is left out ([endpoint page and connector](#sources)). Any contract can leave `TRADING` after your last exchange information read: a quarterly entering its delivery window, or a perpetual settling before a delisting or halted. So read once per margin asset (or pair) of every listed contract you may hold before you call the account flat. That such a read lists contracts outside `TRADING` is inferred from the note, not recorded live. In hedge mode the read also lists each symbol's `BOTH` row beside `LONG` and `SHORT`, as COIN-M Account Information does; treat a flat row of the side the mode does not use as no position |
| USDⓈ-M Account V3 | Balances; its position rows have no entry price, so use position risk for that |
| Position `updateTime` | A position row in USDⓈ-M Position Information V3 and Account V3 carries `updateTime`. On USDⓈ-M Demo (2026-10-14 to 2026-10-16), 26 of the 28 non-empty position reads (14 of each endpoint) carried an `updateTime` equal to the transaction time `T` of the latest `ACCOUNT_UPDATE` that stated that position, even where that `T` was one millisecond after the fill's. The other two, one of each, followed a fill made after we closed the listen key, so no statement was recorded for it; their `updateTime` equalled that fill's trade time. COIN-M Position Information and Account Information rows have the same field in the client's models, but no COIN-M row was recorded. The client models it as optional, so treat a row without it as unstamped |
| Separate reads | Account, positions, orders and trades are separate requests with no shared instant. A trade page read during a fill can end before that fill |
| `userTrades` (Futures) | Seven days at most per request, past three months only, 1,000 rows; send `fromId` or a time range, never both |
| `myTrades` (Spot) | 24 hours at most per request, 1,000 rows; the client refuses `fromId` with a time range |
| A full page | Proves only that the limit was reached, not that you have everything |
| Time-range bounds | Send `startTime` and `endTime` as whole milliseconds. A page covers trades stamped up to its own `endTime`, not up to when the reply leaves the venue: seen once on USDⓈ-M Demo (2026-10-15), a page whose reply arrived after an exit fill still omitted that fill, because the fill was stamped after the page's `endTime` |
| A position opened and closed between two reads | Position Information V3 never names it, because the symbol has no position at either read; only the trade pages and the user stream show it |

Assume venue ids can repeat across accounts. Key orders, trades and positions by
account as well as by id.

### Unknown outcomes and cancels

| Outcome | Meaning |
| --- | --- |
| `NotSent` | Refused before sending: validation, credentials, budget or an expired deadline |
| Definitive rejection | A documented error code that proves the venue did not act |
| `Unknown` | Timeout, disconnect, truncated or undecodable reply, or an unknown code. The order may exist |

- The client sends each mutation once and never retries it. A timeout does not cancel
  an order. Resolve `Unknown` by querying with your client order id.
- On Futures, a `503` with "Service Unavailable." or "Internal error; unable to
  process your request. Please try again." is documented as not accepted. Other `503`
  and `5xx` texts are documented as unknown. Spot `5xx` replies are always unknown.
- Before trusting a cancel, check that the reply names your order and a cancelled
  status. Otherwise query the order.
- A rate-evidence failure after a reply now reports `Unknown` for a mutation and
  `ReadFailed` for a read; before [#128](https://github.com/SharurTrading/binance-rs/pull/128)
  a sent Spot order could wrongly report `NotSent`.

### Convert and Wallet

- A Convert quote is not an exchange. An accepted or processing status is not a
  completed one; confirm with status or history.
- Wallet universal transfer has no caller transaction id. Its `tranId` receipt is not
  a booked transfer; confirm through the paginated history.

## Wrapping the SDK

What this client deliberately leaves to you:

| You own | Why |
| --- | --- |
| The Tokio runtime and every driver task | The client starts no tasks. Run each driver, drain to `Retired`, join it |
| Credentials and the account UID | The client loads nothing from files or the environment, and Futures cannot tell you the UID |
| Client order ids | Generate unique ids and keep them, so `Unknown` outcomes can be resolved |
| Listen-key renewal and reconnects | Neither happens automatically, and nothing is resubscribed |
| Recovery after any gap | Re-read balances, positions, both order lists and recent trades; do not assume the stream caught up |
| Server time | Not synced automatically; inject an adjusted clock |
| Filters at send time | Check tick, step, notional and price bands against fresh exchange information |
| Order books | Join snapshots and diffs yourself, with the right rule for each product |
| Your budget owner | One `AccountKey` per real account, one clock per pool, one reserve owner per budget |
| Logging | Never log secrets, signed URLs, account payloads or exposed venue text |

## Known gaps

| Gap | Status |
| --- | --- |
| Statement-to-fill lag | Seen at 0.2 to 1.7 milliseconds on Demo only; no bound is known |
| Fills during a lost connection | What the stream drops and what history then returns is not proven |
| COIN-M and Spot fill order | One round trip recorded on each (2026-10-17); partial fills and bursts are not |
| Transfer updates | An own-wallet transfer attempt answered 404 on Demo on 2026-10-17; no `ASSET_TRANSFER` or `MARGIN_TRANSFER` update is recorded |
| Hedge-mode closes | Accepted hedge-mode `MARKET` closes recorded once (USDⓈ-M Demo, 2026-10-17); a refused hedge-mode close is not recorded |
| Refused reduce-only close | Not recorded; neither `-2022` `ReduceOnly Order is rejected.` nor an `ALGO_UPDATE` `rm` `Reduce Only reject` says why the order was refused |
| COIN-M positions outside `TRADING` | That a `marginAsset` or `pair` position read lists a delivering, settling or halted contract is inferred from Binance's note, not recorded live |
| Algo order events | Whether COIN-M pushes `ALGO_UPDATE` is not recorded, nor is any USDⓈ-M transition beyond placement and cancel, or whether an `ORDER_TRADE_UPDATE` accompanies them |
| `closePosition=true` stop on a flat position | Not recorded; the stop seen on Demo used `reduceOnly=true` |
| Futures roll notices | Unknown whether `!contractInfo` pushes a roll |
| Undefined statuses and contract types | Tracked in [#80](https://github.com/SharurTrading/binance-rs/issues/80) |
| Old Trades Lookup reach | "One month" has no stated length; the edge has not been measured |
| COIN-M WebSocket API weight | The pages do not say whether it counts against the USDⓈ-M WebSocket limit; the client charges both |
| Production rate headers | Demo header accounting may differ from production |
| Margin user-data migration | Tracked in [#76](https://github.com/SharurTrading/binance-rs/issues/76) |
| Monthly kline opens | Observed on the first UTC day, not documented |

## Sources

- This repository: [README](../README.md), [USDⓈ-M coverage](coverage.md),
  [Spot and COIN-M coverage](spot-coinm-coverage.md),
  [Order window evidence](order-window-evidence.md),
  [Convert coverage](convert-coverage.md), [Wallet coverage](wallet-coverage.md)
- Binance: [USDⓈ-M general info](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/general-info),
  [USDⓈ-M user data streams](https://developers.binance.info/docs/derivatives/usds-margined-futures/user-data-streams),
  [USDⓈ-M account trade list](https://developers.binance.info/docs/derivatives/usds-margined-futures/trade/rest-api/Account-Trade-List),
  [USDⓈ-M aggregate trades](https://developers.binance.info/docs/derivatives/usds-margined-futures/market-data/rest-api/Compressed-Aggregate-Trades-List),
  [Spot REST API](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md),
  [Spot streams](https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md),
  [UM/CM integration notice](https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice),
  [COIN-M Position Information](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#position-information)
  and its note as fetched from Binance's
  [COIN-M connector](https://github.com/binance/binance-connector-java/blob/4fb3c790b7450f7595515f414d9ee0052bb1413a/clients/derivatives-trading-coin-futures/src/main/java/com/binance/connector/client/derivatives_trading_coin_futures/rest/api/TradeApi.java#L3354-L3360),
  [COIN-M Account Information](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/account#account-information)
  and its
  [connector note](https://github.com/binance/binance-connector-java/blob/4fb3c790b7450f7595515f414d9ee0052bb1413a/clients/derivatives-trading-coin-futures/src/main/java/com/binance/connector/client/derivatives_trading_coin_futures/rest/api/AccountApi.java#L201-L205)
