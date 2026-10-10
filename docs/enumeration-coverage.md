<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Native response enumerations

Response enums are generated from cited schema components. Documented string
values have named variants; `Unknown(String)` retains any other spelling exactly,
including case, whitespace, and empty strings. Wallet integer enums retain the
native JSON integer through named variants and `Unknown(i64)`. Neither decoder
assigns a trading policy to an unknown value. Wrong JSON types fail decoding.

This is a breaking response API change: formerly textual fields and collections
now contain their product's enum, and documented Wallet integer fields contain
integer enums. Use `as_str()` for string enums and `value()` for integer enums.
Outgoing constructors retain their native string/integer inputs and validation.

## Mapped evidence

| Product | Response fields | Primary value definitions |
| --- | --- | --- |
| USD-M | Order side/type/status, position side, time in force, working type, price match, STP; exchange-information order types, times in force, filter types; execution/algo/account reason events and kline intervals | [Common definitions](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/common-definition), [user-data events](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/user-data-streams), [new order and algo order](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade) |
| COIN-M | Same native order categories, its own interval/filter lists, and execution/account reason event values | [Common definitions](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition), [COIN-M user-data events](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/user-data-streams) |
| COIN-M integrated algo routes | Algo status and type for the announced shared USD-M API routes | [Integration notice](https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice), [shared algo API](https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade), [shared algo events](https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/user-data-streams) |
| Spot | Order/list/execution statuses and types, side, time in force, contingency, STP, permissions and nested permission sets, allocation type and working floor | [Official enum inventory](https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md) |
| Spot | Kline intervals, including distinct `1m` and `1M` | [Official streams](https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/web-socket-streams.md#klinecandlestick-streams-for-utc) |
| Wallet capital | Deposit/withdraw statuses, transfer direction, wallet type and capital deposit travel-rule status | [Deposit history](https://developers.binance.com/legacy-docs/wallet/capital/deposite-history), [withdraw history](https://developers.binance.com/legacy-docs/wallet/capital/withdraw-history) |
| Wallet local entities | Local travel-rule integer status and overall verification string status | [Travel-rule catalog](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule) |
| Wallet | System status, cloud-mining payment/refund type and status, universal-transfer type | [System status](https://developers.binance.com/legacy-docs/wallet/others/system-status), [asset catalog](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset) |
| Convert | Native `orderStatus` on quote acceptance, status reads and history/open-order rows | [Convert trade catalog](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#accept-quote) |

REST, WebSocket API and stream fields use the same product enum where their native
field has the same contract. Liquidation orders and continuous/index/mark-price
kline events retain their native surrounding payloads. COIN-M `EXPIRED_IN_MATCH`
and `LIQUIDATION` are documented by its own execution-event page. Its common
definitions omit USD-M-only `GTD`, `RPI`, and the `1s` interval, so these decode as
unknown in COIN-M. Wallet withdrawal status `5` has no listed meaning in the cited
current/legacy history contract and remains `Unknown(5)`.

USD-M and COIN-M account-update reasons use their own 17-value and 13-value
lists. USD-M-only reasons remain exact unknown values in COIN-M.

Capital deposit `travelRuleStatus` codes describe readiness/information required;
local-entity `travelRuleStatus` codes describe completed/pending/failed. They use
different enum types. Integer codes never become textual numbers on re-encoding.

Spot SBE already exposes `decode_api<T: DeserializeOwned>`. Decoding exchange
information as `spot::rest_models::ExchangeInfoResponse` now supplies the same
typed statuses, order types, STP modes and permission sets as JSON. Callers can
still explicitly request a dynamic JSON value. The pinned official
[Spot SBE 3:4 schema](https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/sbe/schemas/spot_3_4.xml)
controls binary enum codes. An unsupported binary code fails with typed binary
decode evidence; the decoder does not invent a textual spelling for an unknown
byte. A synthetic exchange-information binary contract test exercises the typed
projection and preserves an unknown textual permission exactly.

## Fields without an enumerated response contract

USD-M/COIN-M `underlyingType` and `underlyingSubType` remain strings: common
definitions give examples rather than an exhaustive value inventory. Wallet daily
snapshot response `type`, universal-transfer row `status`, delegation
`transferType`, address-verification `status`, local-entity deposit/withdrawal
status fields and requirement-status fields retain their native string/integer
shape where their own endpoint documents only examples. An uppercase request
selector does not establish a lowercase response enum. Convert limit placement
and cancellation `status` fields likewise have examples (`PROCESS`, `CANCELED`)
without an enumerated list and remain strings. These are evidence limits of
[#78](https://github.com/SharurTrading/binance-rs/issues/78), not assumed status
semantics borrowed from another product.

Exchange-information `rateLimitType` and `interval` fields are typed for USD-M,
COIN-M and Spot. USD-M's own WebSocket documentation supplies `SECOND` in addition
to the common-definition `MINUTE`; COIN-M's cited sources list `MINUTE`. Spot's
pinned SBE enum schema supplies `CONNECTIONS` and `HOUR` in addition to its common
JSON inventory. Exact `Unknown` values remain available to the existing budget
readers through `as_str()`. This completes the response fields of
[#77](https://github.com/SharurTrading/binance-rs/issues/77) without changing
admission policy. Separate rate issues [#64](https://github.com/SharurTrading/binance-rs/issues/64),
[#65](https://github.com/SharurTrading/binance-rs/issues/65), and
[#66](https://github.com/SharurTrading/binance-rs/issues/66) remain outside this change.

## Disputed contract spellings

[#80](https://github.com/SharurTrading/binance-rs/issues/80) names USD-M
`CURRENT_WEEK`, `NEXT_WEEK`, both markets' `CURRENT_QUARTER DELIVERING` with a
space, and COIN-M `TRADIFI_PERPETUAL`, `SETTLING`, `PRE_SETTLE`, and `CLOSE`.
Current product definitions do not list those spellings for those markets.
Contract tests assert exact `Unknown` retention. A missing spelling in one
exchange-information observation does not establish that the venue can never
send it.

The ignored `record_public_exchange_information_enum_evidence` test makes two
bounded public production GETs only when explicitly invoked. It writes a dated
run containing endpoint identity, public symbol/contract/status fields and safe
failure evidence to `tests/fixtures/public-exchange-info-2026-10-11.json`.
Credentialed/account bodies are never requested. Probe success or failure is
recorded as observation; a failed fetch is not evidence for a new enum variant.

The explicitly invoked October 11 run returned HTTP 200 for both markets and
recorded 924 USD-M symbols and 30 COIN-M symbols in the
[dated public facts](../tests/fixtures/public-exchange-info-2026-10-11.json).
None carried the disputed spellings. A deterministic test decodes and round-trips
every captured contract/status through its own market's enum. The absence of a
candidate in this bounded observation leaves #80 open for qualifying evidence;
it does not justify adding a variant or refusing an unknown reply.

`map_response_enums.py` applies the response annotations reproducibly;
`generate.py` emits the types and checks their freshness. The public enumeration
contracts exercise each documented value, exact unknown round trips, refusal of
wrong wire types, representative native payloads and the Spot SBE projection.
