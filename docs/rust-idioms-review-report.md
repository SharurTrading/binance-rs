<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Rust idioms review disposition report

This is a new implementation and disposition audit for
[issue #150](https://github.com/SharurTrading/binance-rs/issues/150), whose baseline
is revision `40e54d6`. The issue describes a report prepared on 2026-10-20 but not
committed; that original report is unavailable. This document does not reproduce
its authors' evidence, line-by-line coverage claim, or independent review method.
It records the issue's 12 major findings, 19 minor findings, and seven adjacent
observations against the baseline and the implementation in this change. Paths
identify the relevant code; baseline line numbers remain available in the issue.

The baseline count is the issue author's classification (0 blockers, 12 majors,
19 minors), not a fresh claim that all 31 require the initially proposed fix.
Each disposition below must stand on its own evidence. In particular, the
F12 quantity invariant applies to Futures; Options and advanced products preserve
signed native quantities and must not inherit a Futures restriction.

## Findings and implementation evidence

All findings below have an implementation or a corrected, evidence-based disposition.
No item is deferred. F12 is a constructor-invariant bypass for Futures under
RP-IDIOM-CTOR; the baseline zero-blocker count is not adopted by this review.

| Finding | Guide check | Baseline site | Disposition and implementation evidence |
| --- | --- | --- | --- |
| F1 | RP-ANTI-CLONE | `src/core_trading/{spot,usdm,coinm,options}/streams.rs`, `src/advanced_trading/streams.rs`, `src/core_trading/{spot,margin}/websocket.rs`, `src/core_trading/options/event_payloads.rs` | Fixed: market and private decoders move `data`/`event` from owned envelopes after resolving names, kinds and subscription identity. Unknown payloads and missing-versus-null failures retain their previous outcomes. |
| F2 | RP-ANTI-CLONE, RP-PAT-COMPOSE | `src/core_trading/{spot,usdm,coinm,margin,options,wallet,convert}/wire.rs`, `src/advanced_trading/wire.rs` | Fixed: `core::wire` owns the exact parser and shared decimal deserializers. Strings borrow through `Cow`; numbers allocate their exact spelling. Request validation also reuses the parser. Product-only sentinel behavior stays explicit. |
| F3 | RP-ANTI-CLONE | `src/core/http.rs:130,159-163,224,231`; `src/core/request.rs:61-71` | Fixed: HTTP dispatch owns one top-level parameter snapshot through generated/native validation, attribution, cost calculation, signing and both authority checks. Standalone builders and weight queries use the same borrowed-map hooks; WebSocket dispatch also reuses its snapshot. It signs one encoded buffer and appends the signature last. The regression fails on the baseline with three serializations and passes with one. |
| F4 | RP-ANTI-CLONE | `src/core/socket.rs:680,690,764,769,798,803,812` | Fixed: pending lookup borrows the ID; replies move payload, metadata and identities, recovering failed oneshot sends as late evidence. A payload clone remains only when malformed rate evidence requires both the parsed reply and the untouched original frame before the gap; regression tests pin both deliveries. |
| F5 | RP-ANTI-CLONE | `src/core_trading/spot/sbe/schema.rs:68-85,345,349,428`; `src/core_trading/spot/fix/binary.rs:194,258,291` | Fixed: schema nodes borrow from loaded schemas, including pre-registered primitives. One push/truncate exponent scope retains only borrowed names and optional exponent values, with nested shadowing and null-exponent regressions; no per-group scope clone. |
| F6 | RP-ANTI-CLONE | `src/core_trading/spot/fix/codec.rs:276,545,561-564,592` | Fixed: dictionary loading compiles component expansion, group rules, allowed tags and delimiters once. Encode/decode borrow compiled rules; requiredness and group-boundary regression covers the compilation contract. |
| F7 | RP-ANTI-CLONE, RP-IDIOM-MEM | `src/core_trading/spot/fix/session.rs:794,811`; `src/core_trading/spot/fix/budgets.rs:213-225` | Fixed: `MessageAdmission` checks and prunes before account admission, then commits without another fallible step. Dropping the admission spends no message capacity. Account/message refusal tests preserve both-or-neither charging. |
| F8 | RP-PAT-COMPOSE, RP-PAT-YAGNI | `src/core_trading/usdm/` vs `src/core_trading/coinm/` (eight hand-written file pairs, ~87% line-identical) | Fixed: `core_trading::futures` shares configuration, REST/WS wrappers, late-response mapping and stream/control plumbing; `futures_wire` shares identical level and batch shapes. Native Kline volumes, routes, endpoints, account/event DTOs and rate tables remain separate. Dead COIN-M USDⓈ-M-only branches are removed. See the modification evidence record; the route-check claim is rejected because every public COIN-M constructor selects its sole market route. |
| F9 | RP-PAT-SMALL-CRATES | `scripts/codegen/generate.py:335,620-637`; `scripts/codegen/advanced.py:65`; `src/core_trading/{margin,options,wallet,convert}/event_payloads.rs` | Fixed: one neutral `core::wire::UnknownMessage` and its redacting Debug implementation are re-exported from product payload modules. Generators use the neutral path, removing the advanced-to-Wallet dependency and all duplicate definitions. |
| F10 | RP-IDIOM-PRIVACY | `src/core/error.rs:288-299` | Fixed: `Error::outcome` explicitly lists every current variant; existing outcomes are preserved and additions require a compiler-visible decision. |
| F11 | RP-IDIOM-PRIVACY | `src/core_trading/spot/fix/request.rs:126,355` | Fixed: FIX `order_count` and `validate_contract` explicitly list current `RequestKind` variants, including the valid cancel guard fallback. |
| F12 | RP-IDIOM-CTOR, RP-PAT-NEWTYPE | `src/core_trading/usdm/wire.rs:80,82,86-98`; `src/core_trading/coinm/wire.rs:92,94,105-109`; same shape in `options/wire.rs`, `advanced_trading/wire.rs` | Fixed for USDⓈ-M/COIN-M: shared Futures `PriceLevel` has private fields and read-only accessors, with validated construction/deserialization and a compile-fail mutation example. Existing negative-quantity refusal stays intact. Options/Spot deliberately preserve signs; advanced native levels have no non-negative invariant and remain unchanged. |
| N1 | RP-IDIOM-FORMAT | `src/core/socket.rs:484-488` | Fixed: WebSocket signing writes sorted parameters into one preallocated buffer, borrowing strings and formatting other JSON values directly. |
| N2 | RP-IDIOM-DOCS | `src/lib.rs:15-36` | Fixed: `src/lib.rs` includes the README as public crate documentation; the Demo example explains why external network access requires `no_run`. |
| N3 | RP-IDIOM-DOCS | `README.md:115,395-545,692` | Fixed: the crate documentation includes every README Rust block. Visible `fn main() -> Result<(), Error>` wrappers render correctly on GitHub; local examples run as doctests, including pool-key assertions. The external Demo example alone is `no_run`. |
| N4 | RP-ANTI-CLONE | `src/core_trading/usdm/wire.rs:221,224,226`; `coinm/wire.rs:233,236,238`; `options/wire.rs:167-183` | Fixed: Futures, Spot and Options batch alternatives deserialize by reference and move the original value into Unknown only on failure. Batch order and failure classification remain unchanged. |
| N5 | RP-IDIOM-ERROR-ARG | `src/core_trading/spot/fix/session.rs:284` | Fixed: `SendFailure::request` returns the exact owned FIX request for provably unsent refusals, including deadline overflow, driver loss and queued retirement. Interrupted attribution or attempted writes return no retryable request. Ownership and uncertainty regressions cover these branches. |
| N6 | RP-ANTI-CLONE | `src/core_trading/spot/sbe/schema.rs:703,710` | Fixed: amendment projection destructures and reuses the taken object and list-status array; its ownership regression covers the resulting native shape. |
| N7 | RP-IDIOM-CTOR | `src/core_trading/spot/sbe/streams.rs:81-86` | Fixed wording and code: the baseline validated once; `MarketStream` now retains the validated name without cloning a discarded temporary. |
| N8 | RP-PAT-COMPOSE | `src/core_trading/margin/config.rs:13` | Fixed: Margin re-exports the shared SAPI `Config` used by Wallet and Convert. Native order evidence is installed with the existing Margin `RestClient::observe_order_limits`; call sites and coverage documentation are migrated. |
| N9 | RP-PAT-COMPOSE | `src/core_trading/options/streams.rs:105,155`; `src/core_trading/margin/websocket.rs:242,485` | Fixed: Options and Margin each define one product-local socket-policy function with pinned limit citations, reused by both connection paths. |
| N10 | BN-PROTOCOL-01, RP-PAT-NEWTYPE | `src/core_trading/convert/identity.rs:54` | Fixed: Convert `OrderId` accepts the full pinned `int64` domain with an infallible constructor. Signed boundary serialization/query conversion is covered; out-of-range and wrong wire types still fail. The boundary regression fails on baseline `i64::MIN` before the fix. |
| N11 | RP-ANTI-DENY-WARNINGS | `tests/account_key_contract.rs:13`, `ban_release_contract.rs:12`, `futures_ws_weight_contract.rs:17`, `internal_transfer_contract.rs:5`, `price_boundary_contract.rs:5`, `response_shapes_contract.rs:5`, `shared_pools_contract.rs:14`, `stated_order_limits_contract.rs:15`, `stream_shapes_contract.rs:5`, `ws_exchange_limits_contract.rs:8`, `invariant_contract.rs:9` | Fixed: unused panic/expect lint arms removed after checking actual macro/method uses. `tests/invariant_contract.rs` retains its used panic allowance and drops only the unused expect allowance. |
| N12 | BN-IDIOMS-01 | `tests/product_websocket_contract.rs:48-50`; `tests/ws_exchange_limits_contract.rs:26-31` | Fixed: product WebSocket and advanced stream suites use the shared three-second deadline. WebSocket exchange-limit and pool-key suites use shared `FixedClock(u64)` with their original timestamps. Operator-invoked live probe deadlines retain their distinct 10/20-second contracts. |
| N13 | RP-ANTI-CLONE | `scripts/codegen/generate.py:229,231` (`portfolio_margin/rest_models.rs:2765,2770` and others) | Fixed in the generator: discriminator decoders borrow the retained JSON tree and move it into Unknown on failed decoding. |
| N14 | RP-IDIOM-DEFAULT | `scripts/codegen/generate.py:329` (`stocks/rest_models.rs:16-17` example) | Fixed in the generator: removed 4,534 redundant Option defaults while retaining the 2,220 defaults required by custom deserializers. Required non-Option schema defaults are unaffected. |
| N15 | BN-IDIOMS-01 | `scripts/codegen/generate.py:751`, `scripts/codegen/advanced.py:126` | Fixed: every generated Rust output declares its generated origin and names its generator, including the seven generated event dispatch modules. |
| N16 | RP-PAT-YAGNI | `scripts/codegen/generate.py:126-128`, `advanced.py:127` | Fixed: generators delete empty enum modules and empty stream constructor files; product module declarations expose only nonempty generated modules. |
| N17 | BN-IDIOMS-01 | `scripts/codegen/generate.py:43-49,208` | Fixed with evidence-backed schema annotations: depth update IDs and Futures account balances/positions have descriptive Rust names while wire keys stay exact. Explicit schema `x-variant-names` also name alternative DTO types, including proven Order/OrderList and Multiple/Single shapes. Ambiguous short fields and unnamed alternatives retain stable wire-based/positional names instead of invented domain semantics; field docs show known pinned meanings. |
| N18 | BN-IDIOMS-01 | `src/advanced_trading/{algo,alpha,copy_trading,portfolio_margin,portfolio_margin_pro,stocks}/mod.rs` and four `validation.rs` copies | Fixed: advanced products share generic REST construction and common validation/classification helpers with explicit native code catalogs. Portfolio Margin intentionally excludes -1131 and retains its documented definitive 503 cases; tests compare all six pinned catalogs. |
| N19 | RP-IDIOM-DOCS | `scripts/codegen/generate.py:333,433` | Fixed in the generator: setter docs state required/optional status; DTO field docs include pinned descriptions and protocol sources when available. |

## Adjacent observations

These seven observations are separate from the 31 guide findings and do not add
to the major/minor count.

| Observation | Rule or reason | Disposition and evidence |
| --- | --- | --- |
| A1: Futures duplication drift | BN-PROTOCOL-01; LAW-INVARIANT | Fixed with F8. The new USDⓈ-M batch regression reproduces the baseline quantity+priceMatch loophole and now requires explicit price. `docs/futures-modify-order-evidence.md` records catalog contradictions and migrated COIN-M requirements. The unreachable cross-route claim is explicitly rejected. |
| A2: admission scratch `Vec` allocations | Measurement required; no RP check | Fixed: REST/IP/account and SAPI scratch windows use fixed stack arrays and optional entries, preserving window order and atomic checks. An ignored, threshold-free five-sample admission benchmark records the same synthetic workload before/after; timings are illustrative debug measurements, not a production throughput guarantee. |
| A3: socket retirement matches gap text | Typed local control flow | Fixed: private `CloseHandshake` state distinguishes an exhausted incoming-message budget. Retirement no longer parses public diagnostic strings; public gap text and outcomes remain unchanged. |
| A4: SBE market JSON round trip | Redundant conversion; no clone finding | Fixed: the market decoder transfers its native model through an owned, opaque binary payload; SBE ingress checks the type explicitly. No serialize-to-Value/decode round trip or duplicate binary decode remains. A public ordering test covers binary data, JSON shutdown, malformed data and retirement. |
| A5: generated dispatch Debug sniffing | Test assertions must inspect semantic evidence | Fixed in `response_shapes.py`: generated typed predicates inspect Unknown variants recursively instead of Debug strings. Python regressions cover aliases and multiline nested definitions; the independent binary/stream/response corpora continue to assert known-versus-unknown evidence. |
| A6: guide repository vocabulary | BN-IDIOMS-01 | Fixed: severity mapping moved from the pure Rust guide to `AGENTS.md` beside BN-REVIEW-01. Repository-specific review identifiers and the crate-specific applicability sentence also live in AGENTS.md. |
| A7: fixture dead-code lint reasons | Explicit lint justification | Fixed: both `tests/support/mod.rs` allowances use `reason = "shared fixture is compiled into other test binaries"`. |

## Deliberate patterns and corrected scope

- External Demo documentation compiles without running because it needs a live
  service. Local README assertions run without credentials or outbound requests.
- Shared test fixture modules deliberately allow unused helpers: each integration
  test binary compiles its own copy and exercises a different subset.
- Fixtures retain distinct clock values and live probe timeout contracts; sharing
  a helper must not change tested time windows.
- Options and advanced depth evidence preserve signed quantities. Futures require
  non-negative absolute quantities and enforce that rule at construction/decode.
- Portfolio Margin's pinned `-1131` handling is a product fact, not duplication
  drift. Shared code accepts explicit product-specific facts.
- A retained clone must have a concrete dual-owner evidence requirement; rare
  diagnostic-frame retention is different from copying every correlated answer.

## Validation and checklist disposition

Validation commands and their final results are recorded below after integration.
The Convert signed-ID, HTTP serialization-count and USDⓈ-M batch-price tests were
first run against the original revision and failed for the reported behavior.
No live or credentialed probe was run.

The guide checklist applies to changed code. Generated findings are corrected in
the generators and verified by regeneration; neutral sharing must preserve native
product semantics. Runtime ownership, credentials, exact decimals, ingress order,
and ambiguous mutation evidence remain requirements rather than assumed audit
results. This report makes no claim of fresh full-repository line coverage.

## Compatibility

This unpublished 0.1.0 crate changes source-level APIs: Futures depth fields use
read-only accessors; generated annotated depth/account fields and response-alternative DTOs use
descriptive Rust names; Convert integer IDs have an infallible constructor; Margin quota evidence
is installed on its REST client; empty generated modules are removed; FIX send
failures can return an unsent request. Shared product aliases retain existing
import paths and native serialized keys. No dependency or release change is made.

## Admission allocation measurement

The ignored `core::rate::admission_bench::admission_throughput` harness runs five
samples of 250,000 ordinary REST admissions and 250,000 SAPI admissions, with one
warm-up per path per sample. A temporary glibc allocator profiler counted
`malloc`/`calloc`/`realloc` calls for the same test binaries at the baseline and
this change: **6,250,349 versus 324** calls. The 6,250,025 removed calls match three
scratch allocations per ordinary admission and two per SAPI admission including
its per-second window. The profiler was outside the repository; no unsafe code or
dependency was added. Remaining allocations include test harness/setup and real
budget state. No zero-allocation claim is made for the entire admission API.

Debug timing medians for 250,000 admissions were REST 347.99 ms before / 411.38 ms
after and SAPI 235.82 ms before / 209.51 ms after. Concurrent compilation and
unoptimized iterator overhead make these noisy diagnostic measurements; they do
not establish a throughput improvement. The benchmark has no timing threshold.

## Final validation

- Generated bindings checked with `python3 scripts/codegen/generate.py --check`.
- Thirteen offline Python CI tests, formatting, Markdown lint and SPDX checks pass.
- `cargo test --all-targets --all-features --locked`: 606 passed across 54 suites;
  13 intentional ignores (live probes/manual benchmark).
- `cargo clippy --all-targets --all-features --locked -- -D warnings`: passes.
- `cargo test --doc --all-features --locked`: 17 passed, including eight
  compile-fail examples and the README behavior assertions.
- `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps --locked`: passes.
- `cargo package --allow-dirty --locked`: package verification passes before
  committing; plain `cargo package --locked` is also run on the clean commit.
- `cargo deny --locked check` passes advisories, bans, licenses and sources.
- `cargo audit --file Cargo.lock --deny warnings` finds no vulnerabilities.
- The full-history Gitleaks scan passes; the final commit is scanned before push.
- Spot changelog freshness passes (official update 2026-09-18, reviewed 2026-09-27).
  Its official source was fetched through the managed proxy and supplied to the
  unchanged checker with `--input`; the checker's direct-network helper was not
  used in this proxy-bound environment.
