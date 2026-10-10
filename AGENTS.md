<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Binance Rust Client Guide

This is the coding and review contract for `binance-client`. The repository is an
independent inner client, with the same provider-native boundary as `projectx-rs`.

## Mission and scope

Own Binance protocol and transport. Consumers own trading decisions, canonical
models, risk, accounting, persistence, and presentation. USDⓈ-M Futures is the first product
target. Keep shared infrastructure independent of product-specific wire models.
Spot and COIN-M are later products; never claim coverage from scaffold CI alone.

## Non-negotiables

- **BN-BOUNDARY-01:** Depend only on general Rust ecosystem libraries. Never import
  a consuming platform's code, types, rules, credentials, fixtures, or runtime. Keep
  provider semantics in this crate and business policy in its consumer.
- **BN-DECIMAL-01:** Public financial fields use `rust_decimal::Decimal`. Parse
  decimal wire strings without an intermediate float. Checked arithmetic reports
  overflow and division by zero. Quantity, money, price, and percentage semantics
  remain distinct; never silently round operator inputs.
- **BN-ASSET-01:** Account fields retain their settlement/margin asset and position
  mode; never normalize linear and inverse contracts into one wire model.
  Spot balances retain asset identity and separate free/locked
  amounts. Preserve a commission's asset even when it differs from the pair's base
  or quote asset. Preserve base quantity versus quote spend. Never sum unlike assets,
  select a reporting currency, fabricate positions, or calculate portfolio equity.
  Symbol definitions supply base/quote assets and filters; do not infer them from
  symbol spelling. A partial balance event does not replace an entire account.
- **LAW-INVARIANT:** A supposedly impossible branch is not permission to invent
  a business value or silently abandon work. Exhaustive matches, failed
  conversions and missing required state must not fabricate a side, price,
  quantity, identity, account mode or success merely to satisfy the type checker
  or avoid a panic. Prefer a type or function boundary that makes the invalid
  combination unrepresentable; where it remains representable, report a typed
  invariant failure to the owning caller. A bare `return`, `continue`, `None`,
  `Ok(())`, log-only path or `Default` is not a fix when it disguises that
  failure as absence, success or a benign no-op. Legitimate optional values,
  declared defaults and duplicate/stale outcomes remain valid only when they
  express the operation's actual contract. An "unreachable by construction"
  comment or `debug_assert!` alone does not establish a release-build outcome.
  Review verifies the caller handles the failure explicitly, preserves accepted
  evidence and avoids partial business-state mutation. Test a representable
  failure at its owning boundary and assert its observable outcome and state
  preservation, not merely that it does not panic. Existing violations found
  outside the current change receive a tracked issue with the path and required
  outcome.
- **BN-PROTOCOL-01:** Cite current official endpoint documentation for wire behavior,
  limits, and error classification. Preserve documented timestamp units, IDs, event
  kinds, symbol status, and filter semantics. Missing required evidence, malformed
  financial data, and unknown outcome codes never become fabricated defaults.
  New product families require their own documented semantics rather than borrowed
  Spot assumptions. Do not implement speculative products or revenue frameworks.
- **BN-SECRET-01:** Credentials and signing material are redacted and zeroized on
  drop. Never persist or log secrets, signed URLs, authentication payloads, or raw
  sensitive request/response bodies, including dependency trace logs. Errors expose
  safe diagnostics. Callers acquire secrets; the library never loads `.env` files.
  HTTPS/WSS is required except exact loopback fixture hosts. Ignore ambient proxy
  variables; proxy use is explicit. Never ship partner-wide credentials to users.
- **BN-RUNTIME-01:** The caller owns the async runtime. No hidden runtime, blocking
  executor work, or synchronization guard across `.await`. Every task has a named
  owner, cancellation, and joined teardown. Dropping a task handle is not shutdown.
- **BN-ORDER-01:** Callers supply client order IDs; preserve them across acknowledgments,
  queries, errors, and late replies. Never auto-retry a money-moving mutation,
  including after response-body failure, timeout, disconnect, decode failure, or a
  5xx response. Disable dependency retries. Distinguish unsent refusal, documented
  definitive rejection, accepted response, partial success, and ambiguous outcome.
  A timeout does not cancel an order. Consumers resolve uncertainty with venue reads.
- **BN-ERROR-01:** Typed errors retain safe HTTP status, venue code, endpoint identity,
  retry timing, and partial-operation evidence. Preserve `Retry-After`, weight/order
  counters, and both legs of cancel/replace results. Unknown future codes do not
  imply definitive rejection. Never erase evidence behind a generic string error.
- **BN-RATE-01:** Documented venue budgets govern every outbound attempt, including
  retries, subscriptions, and control traffic. Client clones share budgets. Every
  client of a venue pool and environment shares one IP pool in the process unless
  given its own; sharing an account scope across clients stays explicit. Apply
  provider cooldown evidence. Admission may refuse with a typed result before
  acceptance. Recheck time-sensitive authority immediately before sending. Queued
  expiration returns an outcome and prevents a late send. Preserve causal order while
  promptly servicing cancellation and shutdown. No arbitrary account or subscription
  ceilings.
- **BN-INGRESS-01:** Retain every accepted inbound event in source order without
  fixed queue capacities. Expose queue depth, oldest-item age, and progress as lag
  diagnostics. Backlog or an old timestamp alone is not lost continuity and must not
  trigger dropping, resnapshot, or teardown. Separate market data and execution
  delivery. Report actual sequence gaps, malformed records, and transport loss
  explicitly; never only log and swallow them. Preserve snapshot/update distinctions.
- **BN-LIFECYCLE-01:** Single-writer, generation-tagged lifecycle evidence identifies
  the socket that produced each event. Retire a generation only after its accepted
  prefix drains and its socket tasks join. Reconnect/renewal must expose the boundary;
  no silent continuity claim. Pending requests retain late-answer attribution for
  their semantic lifetime. Cancellation cannot authorize replay on a new socket.
  Ordinary silence never proves failure. Honor documented ping/pong requirements.
- **BN-BOOK-01:** Depth events and snapshots keep their update IDs; synchronisation
  is the consumer's. Preserve `U`, `u`, and Futures `pu` on diff-depth events and
  `lastUpdateId` on depth replies, with exact decimal levels. Do not build, bridge,
  or claim a local order book.
- **BN-VALIDATE-01:** Normal CI is deterministic, synthetic, and credential-free.
  Test network failures locally, including truncated responses, partial outcomes,
  delayed replies, rate admission, cancellation, and reconnect overlap. Use controlled
  time for timer behavior. Live probes are ignored, read-only, and explicitly invoked;
  any mutation test needs separate operator authorization and owned cleanup.
- **BN-FOLLOWUP-01:** Complete a discovered follow-up in the same change or track it
  in a repository issue and cite that issue wherever the work is deferred. A deferred
  blocker remains a blocker. Product scope declarations do not claim implementation.
- **PROC-DEP-DECIDE:** A dependency finding is an organization decision before it is
  anything external. Whenever work in any SharurTrading repository finds that a
  dependency — crate, library, framework, toolchain, or service — may be defective,
  unsafe, or wrongly chosen, including any dependency behavior that makes a
  repository's own law impossible to follow (hidden retries that defeat mutation
  rules, hidden runtimes that defeat runtime ownership, silent pooling that defeats
  admission control, or any behavior that compromises secret handling,
  cancellation, or ordering guarantees), the discovering agent files an issue in
  the affected repository — or in the platform repository when the finding spans
  repositories — and does nothing else externally. That issue carries the evidence
  (pinned versions, dependency-source citations, affected rule IDs) and the
  operator's requirements; it does not prescribe an upstream fix. Engagement
  outside the SharurTrading organization — opening, commenting on, or reacting to
  issues or pull requests in any third-party repository — is an operator decision
  under PROC-DECIDE, taken only after the internal issue exists and the
  organization has investigated depth and blast radius. An agent may draft the
  upstream report when the operator asks, but never publishes it: no agent opens
  an out-of-organization issue on its own initiative, no matter how obvious the
  upstream responsibility, how small the fix appears, or how certain the defect.
  If the internal investigation concludes the fault is upstream, the outcome is a
  recommendation recorded on the internal issue, and external filing remains the
  operator's act. Canonical text: SharurPlatform PR #1007.

## Rust coding standards

- Document public APIs, error conditions, cancellation, ordering, ownership, and
  partial-response semantics. Use typed `thiserror` errors at library boundaries.
- Validate provider identities in newtypes with private fields. Public enums and
  response-only structs that may grow are `#[non_exhaustive]`. Requests use validated
  constructors/builders; use a builder when more than two optional settings exist.
- Borrow inputs unless ownership is required. Keep locks short and named by owner.
- No production `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`,
  assertions, or unsafe code. Tests may assert. No `catch_unwind` as error handling.
- Use `tracing` with credential-safe structured fields. Never log each market tick.
- Organize modules by capability; keep `lib.rs` to documentation and selective
  re-exports. Add a crate only for an actual directed dependency boundary.
- Use Rust 2024 and the pinned toolchain. Deny Clippy pedantic warnings; any lint
  exception is narrow and includes a reason. Never weaken gates to make a change pass.
- New behavior gets a failing test first; bug fixes reproduce the actual failure.
  Public contract tests live in `tests/`; private unit tests cover parser invariants.
  Property/workload tests use recorded seeds and assert named invariants. Tests must
  prove behavior rather than restate implementation or pin arbitrary performance.
- Authored text uses the MIT-0 SPDX headers demonstrated here. Generated `Cargo.lock`, JSON protocol fact snapshots/coverage manifests,
  and the license text are exceptions. Never copy proprietary platform material.

## Contribution and review procedure

- **BN-REVIEW-01:** After the initial repository bootstrap, all changes land through
  pull requests into `main`. No force pushes to protected branches. The operator
  initiates review; resolve every finding with an explicit disposition before merge.
  Human review is required for execution semantics, credentials, new dependencies,
  and releases. Report findings as `[BLOCKER]`, `[MAJOR]`, or `[MINOR]` with rule ID,
  file, line, risk, and concrete fix. Blockers and majors require changes.
- **BN-TRIAGE-01:** Every issue has a native type (`Bug`, `Feature`, `Task`), the matching
  kind label (`bug`, `enhancement`, `task`), an organization Priority issue field
  (`Urgent`, `High`, `Medium`, `Low`; default Medium), and one difficulty label
  (`difficulty: hard`, `difficulty: medium`, `difficulty: easy`). Hard covers protocol,
  concurrency, and execution semantics; medium covers specified implementation;
  easy covers mechanical or operator-only work. PRs inherit the linked issue's kind
  and Priority; standalone PRs set both and explain why no issue is linked.
- **BN-ATTRIBUTION-01:** AI-authored GitHub content and commit messages disclose the
  exact model identifier reported by the harness in the human-readable body. Never
  invent a model name or present agent judgment as the account owner's statement.
- **BN-CI-01:** Every PR and main push runs formatting, strict Clippy, public/private
  tests, doctests, rustdoc, package verification, Markdown, SPDX, dependency/license/
  advisory checks, and a full-history secret scan. Rust tests run on Linux.
  No credentialed CI, automated publishing, or trading operations.
- **PROC_REVIEW_AGENTS** — If you are reviewing an open github PR, leave inline and outside diff comments (and a summary if needed), the type of comment should depend on if it is possible or suitable to do inline. On each comment attribute your model name in line with PROC-ATTRIB.
- **PROC-POST_REVIEW** Assess all inline and outside diff comments + summaries. You have final say on what is valid but all comments must be resolved before we can merge due to rulesets.

## Release gate

`publish = false` remains until an explicitly authorized release PR. Confirm the
package name, public API, dependency licenses, provider terms/branding, synthetic
fixtures, and public-safe Git history before publication. Classify the SemVer impact;
update version, lockfile, and documentation together. Create `CHANGELOG.md` for the
first crates.io release and maintain it for releases; pre-release development changes
are recorded in PR descriptions. Require reviewed PR and post-merge main CI success.
Publish only the exact clean merged commit; annotated version tags are immutable.
No release is implied by repository creation.
