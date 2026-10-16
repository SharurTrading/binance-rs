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
small number of USDⓈ-M Demo sessions recorded in October 2026. Where something was
seen only on Demo, or only a few times, the text says so. Nothing here is a promise
about production behaviour.

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
| Venue error text can echo URLs, IP addresses or tokens. The client keeps it behind `VenueMessage`, redacted in `Debug` and `Display` | Call `expose_sensitive_text()` only when you mean to, and never log or store the result automatically |

### Demo and production

Demo and production are separate hosts with separate rate-limit counters. Demo may
lack features that production has, and its replies can differ: the USDⓈ-M Demo
`testOrder` reply zeroes every field it does not echo (`orderId: 0`, empty symbol).
Pick the hosts with each product's `Environment`; production uses `api`, `fapi` and
`dapi.binance.com`.

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

### Keeping capacity for exits

You can reserve part of each budget for cancellations and position-reducing orders
with `set_exit_reserve`, described in the README's
[Reserving capacity for exits](../README.md#reserving-capacity-for-exits). Three
things matter when you wrap it:

1. One owner per shared budget. Replacement is last-write-wins, so two profiles of one
   account that each set their own reserve undo each other.
2. A reserve cannot give back quota that is already spent, and it does not beat a
   venue ban or cooldown.
3. On Futures, a sell is not an exit. Only an explicit `reduceOnly=true`,
   `closePosition=true`, a pure cancellation, or an exact hedge-mode close
   (`LONG`+`SELL`, `SHORT`+`BUY`) qualifies.

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
- Every socket has a 24-hour life. The client ends the generation with a gap when it
  is reached, so plan a reconnect before then.
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

The minimum order quantity moves with the price, because the notional filter is in
quote currency. Work it out from the current price and the filter immediately before
you send. A minimum we derived one day was too small the next.

### Symbols and contract types

- Dated futures are named `<PAIR>_<YYMMDD>` after their UTC delivery date
  (`BTCUSDT_261225`). Perpetuals keep plain names (`BTCUSDT`, `BTCUSD_PERP`).
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
| Client order ids | The client requires `newClientOrderId` on every new order. Keep your own and query by it later |
| USDⓈ-M conditional orders | Stop, take-profit and trailing orders go to the algo endpoint (`/fapi/v1/algoOrder`) with `clientAlgoId`. Regular `newOrder` takes only `LIMIT` and `MARKET` |
| Two namespaces | Regular and algo orders have separate ids, open-order lists and cancel-all calls. Check both lists before you call an account clear |
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
| `positionSide` | `BOTH` | `LONG` or `SHORT` |
| How to close | `reduceOnly=true` on the opposite side | `LONG`+`SELL` or `SHORT`+`BUY`; `reduceOnly` is forbidden and the client refuses it |
| `closePosition=true` | Cannot be combined with `quantity` or `reduceOnly` | Same |

Since the CM migration, USDⓈ-M and COIN-M share one position-mode setting: changing
it on one flips both. Every fill we recorded was in one-way mode; hedge-mode closes
follow Binance's pages and this client's tests, not a live recording.

### Event order: statements and fills

On the USDⓈ-M user stream, a fill produces a compact `TRADE_LITE`, a balance and
position statement (`ACCOUNT_UPDATE`) and the full fill (`ORDER_TRADE_UPDATE`).

> [!IMPORTANT]
> In every USDⓈ-M Demo sample we recorded, the statement arrived before its full
> fill, by about one to two milliseconds. This is a small Demo sample, not a
> guarantee: do not depend on either order or on a bound for the gap.

What held in every sample:

- Both events of a pair carried the same transaction time `T`. Their event time `E`
  matched too, except once, where `E` was one millisecond after `T`.
- A statement covered every fill stamped at or before its own `T`. Place statements
  by `T`, not by the order they arrive in.
- One market order filled in two parts. Both parts had the same `E` and `T` but
  distinct trade ids `t`, prices and fees. Deduplicate by trade id, never by time.
- `TRADE_LITE` arrived earlier still. It has no fee and no realised PnL, so book
  money only from `ORDER_TRADE_UPDATE`, and do not count the same trade twice.

COIN-M has `ACCOUNT_UPDATE` and `ORDER_TRADE_UPDATE` but no `TRADE_LITE`. We have
recorded no COIN-M fill, so its order is unknown. Spot has no positions: its `outboundAccountPosition` lists only the assets
that changed, so it never replaces your whole balance list.

### Fees, PnL and balances

| Field | Meaning |
| --- | --- |
| `n`, `N` | The fill's commission and its asset; the asset can differ from the pair's |
| `rp` | Realised PnL the venue states for that fill |
| `ep` | Entry price in the position statement. It is rounded (to eight places in our sample), so work out averages from the fills yourself |
| `wb` | Wallet balance in the statement |

In our USDT-margined samples the wallet moved by exactly the stated `rp` minus the
fee on every fill. Use the venue's figures as stated and compute derived values from
exact inputs, not from rounded statement fields.

### Reading account state

| Read | Behaviour |
| --- | --- |
| USDⓈ-M Position Information V3 | Returns only symbols with a position or an open order, so a flat account gives an empty list. Pair it with both open-order lists before you call the account flat |
| USDⓈ-M Account V3 | Balances; its position rows have no entry price, so use position risk for that |
| Separate reads | Account, positions, orders and trades are separate requests with no shared instant. A trade page read during a fill can end before that fill |
| `userTrades` (Futures) | Seven days at most per request, past three months only, 1,000 rows; send `fromId` or a time range, never both |
| `myTrades` (Spot) | 24 hours at most per request, 1,000 rows; the client refuses `fromId` with a time range |
| A full page | Proves only that the limit was reached, not that you have everything |

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
| Statement-to-fill lag | Seen at one to two milliseconds on Demo only; no bound is known |
| Fills during a lost connection | What the stream drops and what history then returns is not proven |
| COIN-M and Spot fill order | No fill recorded on either |
| Hedge-mode closes | Not recorded live |
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
  [UM/CM integration notice](https://developers.binance.info/docs/derivatives/coin-margined-futures/Important-CM-UM-Integration-Notice)
