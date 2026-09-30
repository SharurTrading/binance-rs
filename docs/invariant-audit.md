<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Invariant audit record

Audit of [issue #28](https://github.com/SharurTrading/binance-rs/issues/28): production
code was reviewed for invented business values in supposedly impossible branches and
for silently swallowed invariant failures. The rule under test: a branch that cannot
happen is never permission to invent a value or abandon accepted work.

Method: every module family was read (`core`, `usdm`, `coinm`, `spot`, `wallet`,
`convert`, `spot/fix`, `spot/sbe`), candidates were classified against the business
contract and the consuming caller, and each repair landed with a discriminating
synthetic regression that fails on the pre-repair source. Provider facts changed in
this sweep were verified byte-for-byte against the hash-recorded upstream catalogs
before any snapshot was touched. No policy scanner was introduced.

## Repairs in this sweep

Each entry records the owner, the call-path invariant, and the required outcome.

| # | Site | Violation and repair |
| --- | --- | --- |
| R1 | `spot/rest_models.rs` `MyFiltersResponse*Item` | **Fabricated filter identity and absence.** The REST `myFilters` filter enums were greedy `untagged` alternatives whose members are all-optional, so every filter decoded as the first alternative: a `LOT_SIZE` reported itself as `PRICE_FILTER` with `min_qty`/`step_size` "absent" while the venue's values sat redacted in `extra`. Owner: any consumer gating orders on symbol filters. Invariant: the variant named is the kind the venue sent. Repair: the snapshot now carries the provider's OpenAPI `discriminator` vocabulary (verified against the recorded source hash), so both products dispatch on `filterType`, require each filter's own evidence, and retain unknown filter kinds. Regressions: `tests/invariant_contract.rs` (REST and socket lists, refused incomplete filters). |
| R2 | `coinm/{rest,ws}_models.rs` `PositionInformationResponseItem` | **Unrepresentable position mode.** `marginType`/`isAutoAddMargin` were annotated `x-decimal`, typing the inverse margin mode (`"isolated"`, `"false"`) as a quantity, so the whole position list failed to decode and the mode could not exist in the model. Repair: the annotation was removed after the pinned COIN-M source (hash-verified) confirmed both are strings; absence stays absence. Regression: `tests/invariant_contract.rs` (mode retained verbatim, absent stays `None`, numeric mode refused). |
| R3 | `core/rate.rs` `observe`, `core/sapi.rs` `observe_cost` | **Discarded venue answer on unrepresentable cooldown.** A `Retry-After` beyond the representable range produced `Error::Configuration("cooldown overflow")` after the response arrived, so a `200` acknowledgment was replaced by `Transport { Unknown }` and its body was never read. Owner: every REST caller. Invariant: limiter bookkeeping never overwrites a received response. Repair: the delay saturates at the widest value (refuses longer, never shorter). Regressions: `tests/invariant_contract.rs` (the acknowledgment survives; the next send is refused). |
| R4 | `core/socket.rs` `response`/`settle` | **Correlated answer dropped after `pending.remove`.** Any failure in the post-removal chain (unparseable status, clock, limiter, missing payload) dropped the venue's frame: the caller got `Transport { Unknown }` and the event stream got nothing. Repair: the frame is emitted as accepted ingress in source order before the failure propagates. Regression: `tests/invariant_contract.rs` (frame precedes the reported gap). |
| R5 | `core/socket.rs` `retire` | **Fabricated deadline reason for unsent commands.** Commands still queued at retirement received `Error::Expired` — "deadline expired before sending" — although the caller's deadline had not passed. Repair: `Error::NotSent { operation, reason }` whose `outcome()` is `Outcome::NotSent`; nothing claims a venue or deadline verdict. Regression: `tests/invariant_contract.rs`. |
| R6 | `core/socket.rs` `run`/`retire` | **False continuity claim.** Every driver failure, including owner-requested cancellation and local refusals, was wrapped in `SocketEvent::Gap` ("stream continuity lost"). Repair: only proven continuity failures (gap, malformed record, transport loss, decode) are gaps; an owner stop is the normal retirement boundary and a local failure is returned to the caller. |
| R7 | `core/socket.rs` `ws_rates`, `core/rate.rs` `observe_ban`, `core/error.rs` `RateEvidence` | **Venue retry timing read as "no cooldown".** A `retryAfter` deadline at or behind the local clock saturated to `Duration::ZERO`, admitting the next send immediately; the refusal evidence was silently narrowed. Repair: unusable timing sets `retry_after_unusable` and the owner refuses sends with `Error::CooldownTimingUnknown` (no expiry invented), exactly like a ban without timing. Regression: `tests/invariant_contract.rs` (past deadline refuses the next send; evidence retained on the refusal). |
| R8 | `spot/fix/codec.rs` `Message::outcome` | **Acceptance fabricated over a definitive error code.** `ErrorCode` was consulted only to reject *unknown* codes, so a definitive code (`-2011`) paired with an accept-shaped `ExecType`/`OrdStatus` reported `Outcome::Accepted`. Repair: any non-zero code is never acceptance — documented codes are `Rejected`, others stay `Unknown` — and the unreachable trailing match (non-ExecutionReport kinds claiming execution evidence) was removed. Regression: `tests/fix_contract.rs` (fails pre-repair). |
| R9 | `spot/fix/session.rs` `candidates` | **Dead evidence path.** `RefSeqNum` matched only `Value::Unsigned`, but both encodings decode tag 45 as `Integer`, so sequence-based attribution of receipts without a client ID never fired. Repair: both arms match. Regression: `tests/fix_contract.rs` (fails pre-repair). |
| R10 | `spot/fix/session.rs` `incoming` | **Recovery requests silently unhandled.** `35=2` ResendRequest and `35=4` SequenceReset fell through `_ => ()`; the boundary surfaced later as an unrelated sequence gap. Repair: both are reported as explicit typed gaps at the request. Regression: `tests/fix_contract.rs` (fails pre-repair). |
| R11 | `sbe/schema.rs`, `fix/binary.rs`, `sbe/market.rs` | **Metadata defaults that could invent framing.** A present-but-unparsable declared array width silently became one element (desynchronizing every later field); the group dimension type defaulted to divergent invented names between encoder and decoder; a zero venue `blockLength` silently became one byte; an unparsable `jsonDefaultValue` silently degraded to text; and the depth-diff arm was an unguarded catch-all. All are now explicit typed refusals; the remaining dimension assumption is [issue #33](https://github.com/SharurTrading/binance-rs/issues/33). Regression: `tests/sbe_contract.rs` template boundary. |
| R12 | `scripts/codegen/generate.py` (spot `event_payloads.rs`) | **Cross-product borrowing.** The generated Spot user-data dispatcher carried USDⓈ-M `ACCOUNT_UPDATE`/`ACCOUNT_CONFIG_UPDATE` evidence guards; a foreign event name on a Spot socket became a gap that retired the generation. Repair: evidence guards are emitted only for events the product's own pinned stream schema documents, so an unmodeled Spot event is retained as `UserPayload::Unknown`. Regressions: `tests/product_websocket_contract.rs` (fails pre-repair). |
| R13 | `core/http.rs` `header_rates` | **Dropped venue counter evidence.** The interval-less `x-mbx-used-weight`/`x-mbx-order-count` spellings were filtered out of `RateEvidence`. Repair: they are retained as evidence; they still map to no budget window, since the venue named no interval. |

## Recorded as legitimate (no repair)

- `unwrap_or_default()` on counter lookups (`core/rate.rs`, `core/sapi.rs`, `fix/session.rs`, `fix/codec.rs`): the default is immediately re-derived against a real interval bucket and can never be mistaken for venue evidence.
- `let _ = reply.send(...)` on oneshot channels: closed receivers are late-answer decisions; the attempt survives in `pending` and is re-emitted as a late answer or a settled `Unknown`. `fix/budgets.rs`'s drop-time `retire` is a conservative over-count.
- `http.rs` post-response clock fallback to the attempt's own pre-send reading: a real reading, never an invented epoch, and propagating a clock failure there would discard the received answer (R3's rule); documented in place.
- `fix/config.rs` `set_port` discard: unreachable by construction — the endpoint is built with an explicit port and `endpoint()` refuses any URL without one.
- Documented sentinels (empty algo fields, `"null"` iceberg, empty delivery funding/interest) map to absence, not zero; orders, balances, commissions, and position fields retain exact `Decimal` or exact text with no defaults; request builders omit unset parameters rather than defaulting them.
- Order-book bootstrap (`usdm/coinm/spot book.rs`): gaps force a blocking `Gap` state, no mirror claims completeness, and no gap becomes invented depth. Deferred observability: [issue #36](https://github.com/SharurTrading/binance-rs/issues/36).
- `Decimal`/quantity parsing refuses malformed strings everywhere; no `Option<Decimal>` collapses to zero anywhere in the response surface.
- Rate admission's fixed-window counters are a documented model layered with a venue-observed floor; the outstanding framing ambiguities and evidence gaps are the issues below.

## Deferred findings

| Issue | Scope |
| --- | --- |
| [#30](https://github.com/SharurTrading/binance-rs/issues/30) | Futures 503 message clause claims definitive rejection without pinned evidence |
| [#31](https://github.com/SharurTrading/binance-rs/issues/31) | Wallet/Convert definitive code lists and Convert status literals lack pinned evidence |
| [#32](https://github.com/SharurTrading/binance-rs/issues/32) | Wallet `withdrawHistory` request-rate evidence is unpinned |
| [#33](https://github.com/SharurTrading/binance-rs/issues/33) | SBE repeating-group dimension encoding is undocumented by the provider |
| [#34](https://github.com/SharurTrading/binance-rs/issues/34) | Unknown market stream names retire the generation instead of being retained |
| [#35](https://github.com/SharurTrading/binance-rs/issues/35) | Unused non-API socket ping limit carries no citation |
| [#36](https://github.com/SharurTrading/binance-rs/issues/36) | Book updates discarded at a bootstrap gap are not countable |
| [#37](https://github.com/SharurTrading/binance-rs/issues/37) | Live USDⓈ-M order placement cannot express `workingType` |

Audit authored by `nvidia/z-ai/glm-5.3` (opencode agent) on the operator's account.
