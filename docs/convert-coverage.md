<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Convert REST coverage and protocol evidence

Checked **2026-09-27** against the official catalog: all nine SAPI REST operations.
This product is separate from USDⓈ-M Futures Convert. No credentialed or live
mutation probe was run; catalog bindings never establish execution readiness.

Financial fields parse into Decimal without an intermediate float or rounding.
Source/destination and base/quote amounts stay separate. Quote and limit builders
require exactly one amount direction. Provider asset identities are validated;
symbol names are never split to infer a pair. Wallet combinations and expiry
choices remain caller inputs, with documented venue defaults when omitted.

`send_quote_request` returns a `Quotation` with the requested assets and wallet
selection beside the native receipt. Missing quote IDs (for example, insufficient
funds) and missing venue expiry cannot authorize acceptance. `AcceptQuote::new`
retains quote authority, exact quoted amounts and asset provenance. It rechecks
`validTimestamp` against the caller's millisecond clock before admission and
immediately before sending. Expiration returns an unsent outcome. Receive windows
remain separate millisecond parameters and never extend a quote's authority.

Acceptance preserves the original quotation and native string order ID.
Integer order IDs used by query/limit operations remain their own validated type;
the client never silently converts between these native representations. Errors
retain quote/native cancellation order IDs for reconciliation, status, code, and
quota evidence. A timeout,
truncated body or malformed receipt remains ambiguous, and mutations are never
retried. Caller cancellation does not establish venue cancellation.

Acceptance outcomes recognize the documented `PROCESS`, `ACCEPT_SUCCESS`,
`SUCCESS`, and `FAIL` vocabulary. Processing/accepted acknowledgments do not prove
completion or fills. Limit placement recognizes its `PROCESS` acknowledgment;
limit cancellation recognizes `CANCELED`. Unknown statuses remain native strings
and classify as `Unknown`. Known request refusals use Convert's own error-code
documentation; unknown codes and every 5xx remain ambiguous.

SAPI admission shares the endpoint IP/UID owners described in
[Wallet coverage](wallet-coverage.md). Each endpoint independently charges its
published scope and weight. Header evidence and `Retry-After` update the originating
scope before reading the body. A `418` IP ban applies across endpoint and UID
owners sharing that IP. Missing/malformed ban retry timing refuses subsequent sends
with `CooldownTimingUnknown`; no retry delay or restored authority is fabricated.
No Spot/Futures aggregate budget is borrowed.
Clients start no tasks or hidden runtime. Production SAPI uses HTTPS, explicit
credentials/proxies and caller deadlines; dependency retries are disabled.

## Sources and verification

- [Convert general transport, signing, timing and SAPI limits](https://developers.binance.com/en/docs/products/convert/general-info)
- [Convert error codes](https://developers.binance.com/en/docs/products/convert/error-code)
- [Quote, limit and history contracts](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade)
- [Pairs and asset precision](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data)

`schema/convert-rest.json` pins normalized protocol facts, source links, the
original schema SHA-256 and check date. Financial/identity/required evidence
annotations correct catalog gaps. Examples and provider prose were excluded.
The standard-library generator produces native DTOs, eight builders, all nine
methods, and canonical acceptance operation facts. Quote authority remains a
handwritten validated request. Generated freshness checks all five products
offline. No dependencies were added.

`tests/convert_contract.rs` covers amount direction, asset provenance, exact values,
venue expiry during admission, unsent expired authority, retained quote IDs and
headers after truncated acceptance, no mutation retries, future statuses, native
cancellation vocabulary, documented rejection versus unknown/5xx outcomes,
malformed financial data, required query IDs, the 30-day history range, and IP
bans spanning endpoints and distinct account owners.
Human execution review is required before merge.
[Issue #17](https://github.com/SharurTrading/binance-rs/issues/17) tracks this client.

## REST operations

| Operation | Method | Path | Weight scope | Weight |
| --- | --- | --- | --- | ---: |
| [listAllConvertPairs](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#list-all-convert-pairs) | GET | `/sapi/v1/convert/exchangeInfo` | IP | 3000 |
| [queryOrderQuantityPrecisionPerAsset](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/market-data#query-order-quantity-precision-per-asset) | GET | `/sapi/v1/convert/assetInfo` | IP | 100 |
| [acceptQuote](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#accept-quote) | POST | `/sapi/v1/convert/acceptQuote` | UID | 500 |
| [cancelLimitOrder](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#cancel-limit-order) | POST | `/sapi/v1/convert/limit/cancelOrder` | UID | 200 |
| [getConvertTradeHistory](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#get-convert-trade-history) | GET | `/sapi/v1/convert/tradeFlow` | UID | 3000 |
| [orderStatus](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#order-status) | GET | `/sapi/v1/convert/orderStatus` | UID | 100 |
| [placeLimitOrder](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#place-limit-order) | POST | `/sapi/v1/convert/limit/placeOrder` | UID | 500 |
| [queryLimitOpenOrders](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#query-limit-open-orders) | GET | `/sapi/v1/convert/limit/queryOpenOrders` | UID | 3000 |
| [sendQuoteRequest](https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#send-quote-request) | POST | `/sapi/v1/convert/getQuote` | UID | 200 |
