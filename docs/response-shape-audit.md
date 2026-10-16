<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Response shape audit

Reviewed against official documentation on **2026-10-16**, including every
supported Core Trading and Advanced Trading family. Institutional Loan and the
seven external-withdrawal operations remain excluded.

The [REST General](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/general)
and [WebSocket General](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general)
links on issues #135 and #136 lead to the authoritative sections. Their endpoint
references, supplemented by Binance's official Spot Markdown and SBE definitions,
establish the heterogeneous cancellation receipts required to fix both issues.

## Inventory and executable evidence

[The generated ledger](../schema/response-shape-coverage.json) maps every method
to its native Rust response model, official source and independent fixture IDs.
Its JSON method inventory contains **624 methods**: 534 REST, 80 catalog WebSocket
methods, nine manual session methods and Margin's manual listen-token subscription.
There are **979 JSON method cases**, **245 stream cases covering 138 routes/event
kinds**, and **323 binary cases**. All fixtures are synthetic or derived from public
documentation; authentication samples and download URLs are replaced.

| Product | REST | Catalog WS | Manual WS |
| --- | ---: | ---: | ---: |
| USDⓈ-M | 95 | 18 | 3 sessions |
| COIN-M | 67 | 10 | 3 sessions |
| Spot | 48 | 52 | 3 sessions |
| Wallet | 43 | 0 | 0 |
| Convert | 9 | 0 | 0 |
| Margin | 66 | 0 | 1 token subscription |
| Options | 44 | 0 | 0 |
| Portfolio Margin | 102 | 0 | 0 |
| Portfolio Margin Pro | 24 | 0 | 0 |
| Algo Trading | 12 | 0 | 0 |
| Copy Trading | 2 | 0 | 0 |
| Alpha Trading | 6 | 0 | 0 |
| Stocks Trading | 16 | 0 | 0 |

- [JSON fixtures](../tests/fixtures/response-shapes.json) include whole official
  examples, declared schema alternatives, conditional omissions, empty receipts,
  nullable values, dynamic dictionary keys and unauthenticated session replies.
  Schema field examples assembled into bodies are labeled separately from whole
  response examples. Spot Markdown fixtures retain the actual endpoint heading,
  source line and source hash; broad parent sections are not endpoint evidence.
- [Stream fixtures](../tests/fixtures/stream-shapes.json) cover every exposed
  market route and private event kind, including raw payloads and documented
  combined envelopes. Array roots retain their native array shape. Stream control
  acknowledgments, errors and late replies also have loopback transport coverage
  in `product_websocket_contract`, `advanced_streams_contract` and `margin_contract`.
- [Binary fixtures](../tests/fixtures/binary-shapes.json) cover all 92 API SBE,
  29 FIX SBE and four market SBE templates, plus all 19 order-entry and 14 market-data
  ASCII FIX definitions, with absent/present optional forms. There are 393 native
  decoding checks across 193 template/model associations and 32 explicit projection
  assertions. Source hashes match the pinned official XML/dictionaries.

`tests/ci/test_response_shapes.py` compares these inventories to the executable
snapshots, public private-event alternatives and pinned binary definitions. Adding
a method, route, private DTO or binary definition without evidence fails CI.
`scripts/codegen/generate.py --check` checks the dispatchers and ledger offline.
The wire fixtures are independent of the generated DTO definitions, so a schema
mistake cannot silently change the test input at regeneration time.

Shared REST/WS envelopes, rate evidence, negative venue codes, partial operations,
malformed/truncated bodies and late responses remain covered by `http_contract`,
`product_http_contract`, `websocket_contract`, `product_websocket_contract`,
`advanced_spot_contract` and `advanced_trading_contract`. Successful endpoint data
fixtures deliberately exclude error bodies; a successful HTTP status alone does
not turn a negative venue code into a successful receipt.

## Corrected contracts

Spot REST cancel-all and WebSocket `openOrders.cancelAll` now expose explicit
`Order` and `OrderList` success alternatives. Member failures and unknown evidence
stay in input order. List receipts retain every member and report. The equivalent
Portfolio Margin cancellation and Margin alternatives use the same explicit
shape distinction. Single cancellation can carry a complete order receipt and
list receipt together; list markers select the list model, which retains both
sets of typed fields. A malformed nested report cannot fall through to a simpler
order variant and bypass Decimal validation.

Additional corrections from the all-product audit:

- Spot WS trading-day tickers accept scalar and array results. Reference prices
  require a present, nullable price and no fabricated success `code`. SOR accepts
  the documented WS array and native SBE scalar, retaining typed commission asset,
  match type and allocation ID. Cancel/replace preserves failed and null legs.
- USDⓈ-M WS order-book levels reside inside the result and are exact typed levels.
  Five analytics timestamps accept documented integer and quoted-integer forms,
  rejecting fractional or overflowing values. Futures and Portfolio Margin income
  allow an empty symbol for non-trade records. PM UM transaction identity accepts
  documented integer and string forms without a float conversion.
- USDⓈ-M and PM quantitative indicators use a typed map for arbitrary instrument
  keys and `ACCOUNT`, with exact Decimal values and thresholds. Shared-host mark
  prices accept the documented unavailable funding-rate sentinel. COIN-M index
  prices retain either documented native identity key, `i` or `s`, and refuse a
  missing identity. USDⓈ-M exposes both documented asset-index routes.
- Margin conditionally omitted isolated-account, transfer, collateral, delisting
  and aggregate fields remain optional. Its list execution/status fields remain
  text. Options price and lot-size filters have separate required fields; its
  cancellation status code remains native text.
- PM untriggered conditional orders do not require a nonexistent resulting order.
  ACK responses and conditional execution-report fields are accepted.
- API SBE projects cancellation arrays, subscription arrays and amend/list-status
  groups into the documented native DTOs. Documented finite nested cancel/replace
  errors decode without allowing arbitrary recursive envelopes. Binary32 values
  become Decimal only when exactly representable; no rounding is introduced.
  The optional binary balance-clear time remains explicitly nullable.

Public response types change to represent these alternatives: consumers must
update their matches and conditional field access. Request identity validation,
mutation retry policy and withdrawal exclusions are unchanged.

## Source disagreements and limits

The migrated catalog omits some whole-body unions and misplaces USDⓈ-M WS book
levels. Legacy official pages supply the missing examples. Both documented
representations are accepted when sources disagree, including SOR scalar/array,
COIN-M index identity keys and quoted timestamps. A homogeneous depth item example
containing only a price is not a valid price/quantity tuple; whole official examples
establish both columns. These source artifacts do not justify relaxing native
financial validation.

Three COIN-M algo methods have integration-notice evidence rather than standalone
COIN-M response pages; their fixtures retain that limitation. PM declares ACK/RESULT/
FULL but illustrates FULL; the minimal ACK evidence is cross-referenced with its
official Margin sibling. A single-asset asset-index route has the documented array
payload; an undocumented object form is not inferred. Documentation of a blank
response is represented by the existing transport-normalized empty receipt, not
a claim about whether the venue emits zero bytes or `{}`.

Binary template 999 deliberately refuses. Structurally nullable subscription IDs
do not establish a valid current subscription: those native checks still refuse.
Structural cancel/replace smoke cases with null legs retain explicit unknown
evidence. Legacy listen-key binary templates are structural coverage, not restored
Spot endpoints. Allocation events retain explicit unknown evidence because the
current JSON user-event reference does not advertise a typed allocation event.

This audit establishes coverage of the pinned documented shapes, not live venue
conformance or every future field/business combination. Unknown evidence remains
explicit. No credentialed requests or exchange mutations were needed for this audit.
Final source and implementation review used **gpt-6-astra, high reasoning** across
Spot/binary, Futures, Wallet/Margin/Options/Convert and Advanced Trading. Review
findings were corrected and checked with focused native assertions.
