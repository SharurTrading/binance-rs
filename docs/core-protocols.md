<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Spot FIX and SBE protocols

Checked **2026-09-27** against Binance's production schema pointers. These are
initial protocol implementations with deterministic synthetic fixtures. Pinned
schema counts do not prove every template's conformance or live execution readiness.
No live FIX connection, private read, or mutation probe was run.

## Sources and pinned facts

| Protocol | Pinned schema | Official source |
| --- | --- | --- |
| REST and WebSocket API SBE | 3:4 | [Production API schemas](https://github.com/binance/binance-spot-api-docs/tree/master/sbe/schemas) |
| SBE market streams | 1:0 | [Market schema](https://github.com/binance/binance-spot-api-docs/blob/master/sbe/schemas/stream_1_0.xml) |
| FIX SBE | 1:1 | [FIX binary schema](https://github.com/binance/binance-spot-api-docs/blob/master/sbe/schemas/spot-fixsbe-1_1.xml) |
| FIX ASCII order entry/drop copy | FIX 4.4 | [Order-entry dictionary](https://github.com/binance/binance-spot-api-docs/blob/master/fix/schemas/spot-fix-oe.xml) |
| FIX ASCII market data | FIX 4.4 | [Market dictionary](https://github.com/binance/binance-spot-api-docs/blob/master/fix/schemas/spot-fix-md.xml) |

Behavior comes from the [FIX API contract](https://developers.binance.com/en/docs/products/spot/fix-api),
[SBE API FAQ](https://github.com/binance/binance-spot-api-docs/blob/master/faqs/sbe_faq.md),
and [SBE market-stream contract](https://developers.binance.com/en/docs/products/spot/sbe-market-data-streams).
The API production pointer currently selects `spot_3_4.xml`; the existence of a
newer schema file does not authorize silently negotiating it.

`schema/spot/*.json` retains normalized protocol facts, original XML SHA-256 and
source URLs. `scripts/codegen/normalize_binary.py` accepts operator-supplied XML
and performs no network access. Examples, comments and descriptive provider prose
are excluded. The runtime uses these pinned facts offline. Unknown versions,
non-representable responses, unknown binary enums and malformed fields are explicit
errors; there is no guessed fallback schema.

## API SBE

`spot::RestClient::new_sbe` selects `Accept: application/sbe` and `X-MBX-SBE: 3:4`.
`spot::WsClient::connect_sbe` selects binary responses while sending normal JSON
requests. Both use the existing native requests/models, signing, caller deadlines,
shared budgets and safe outcome handling. Timestamp provenance is microseconds;
REST/WebSocket receive-window inputs remain decimal milliseconds.

Successful JSON fallback is refused. JSON negotiation errors retain status, venue
code and rate evidence. A failed/truncated binary mutation is uncertain and is
never retried. Correlated binary WebSocket replies and late answers pass through
the same generation owner as JSON replies. A ban without retry timing refuses
further outbound attempts rather than fabricating a cooldown.

Mantissas and exponents become exact Decimal strings before native deserialization;
no float or rounding intervenes. Nullable fields retain documented null/default
semantics. User events retain their event kind and subscription identity. Partial
balance deltas do not replace accounts. Binary candles omit JSON's reserved final
column; the native `Kline::reserved` is explicitly null for this omission.
Embedded response envelopes cannot recursively contain the same template.

## SBE market streams

`spot::sbe::MarketConfig` requires caller-provided Ed25519 credentials. Only the API
key is sent in the sensitive handshake header; public stream subscription does not
sign a timestamp. WSS is mandatory except exact loopback fixtures. The four native
streams preserve trades, best bid/ask, finite top-20 snapshots and incremental depth
as separate payloads. Their timestamps are microseconds.

Initial subscriptions are explicit and checked against the venue's 1024-stream
limit. Shared Spot budgets count the documented IP connection attempts. Pongs echo
ping payloads and count against five control messages per second. The caller runs
and joins the returned driver, drains accepted ingress in source order and observes
24-hour/transport/decode retirement. Consumer lag never drops data or causes renewal.
Venue best-price auto-culling is distinct from client loss; skipped best-price IDs
alone do not establish a gap. This module exposes depth evidence and does not claim
an automatically bootstrapped or complete order book.

## FIX sessions and execution evidence

`spot::fix` owns private validated component/client/opaque identities, native typed
fields, repeating groups, ASCII framing and little-endian SOFH binary framing.
Prices, base quantities, quote spend and commission assets remain distinct. Optional
binary fields use native null values. Instrument-derived `Precision` is required
for binary financial requests; unrepresentable inputs are refused without rounding.
Missing instrument evidence does not receive inferred precision.

`Config` selects order entry, drop copy or market data, with ASCII, ASCII requests /
SBE replies, or SBE in both directions. TLS verifies the hostname and certificate;
no ambient proxy or hidden runtime is used. Logon signs the exact documented Ed25519
payload once and requests sequential processing. Authentication/control traffic is
owned by the single writer. Negotiated heartbeats, exact TestRequest echoes,
sequence checks, maintenance News and Logout remain visible. Silence triggers a
probe; only an unanswered probe deadline establishes heartbeat failure. Binary
headers omit component IDs, so standalone decoding retains that omission. Session
events attribute the authenticated local component separately.

Application builders expose native single/list placement, cancellation,
cancel/replace, cancel-all, quantity reduction, limits, instruments and market-data
requests. Callers own every client/list/leg ID and contingent instruction. Required
fields, enum values, quantities, trigger constants, original identities and group
relationships are validated before send; symbol filters and current order state
remain venue evidence, not inferred consumer policy.

`Session::send` completes after a socket write. It does not wait for acceptance or
claim that a mutation succeeded. `SendFailure` distinguishes pre-send refusal from
uncertain writes and retains identities and typed causes. Queued cancellation or
expiration prevents a late send; no mutation is replayed after any failure.
`Event::Message` retains all local attribution candidates for the generation,
including reused wire IDs and late replies. Successful cancel/replace legs and
independent rejection records remain separate source-ordered evidence; one record
never completes the other leg. Future status/error codes cannot prove acceptance.

`AccountBudgets` must be shared across sessions for the same account and takes the
account's existing Spot owner, sharing order counts with HTTP/WebSocket clients.
Every outbound application/control attempt counts against role-specific message
quotas. Account connection attempts, concurrent connections and unique components
use the documented role limits. Closed connections retain their count for two
heartbeat intervals; unclean closure also retains component exclusion. Native
LimitResponse count/max/interval evidence constrains future sends. Counter floors
are held for a full observed interval instead of guessing its reset origin.

FIX market subscriptions reserve the documented 1000-stream allowance. The current
contract does not specify successful unsubscribe acknowledgment: uncertain slots
remain reserved through generation retirement, and an unambiguous subscribe
rejection can release its reservation. This limitation remains tracked in
[issue #11](https://github.com/SharurTrading/binance-rs/issues/11); a successful socket
write is insufficient evidence to erase an active venue subscription.

The caller spawns/runs `SessionDriver::run`, requests shutdown, drains through
`Retired`, and joins its task. Dispatch and ingress ownership are separate. No
connection is replaced automatically, and no accepted record is discarded because
of queue depth or age.

## Verification and remaining blockers

`tests/sbe_contract.rs` exercises independent wire fixtures, truncations, exact
prices/quantities, schema rejection, event identity, nullable candle provenance,
recursive envelopes, REST/WS negotiation, correlated replies, bans, binary ingress,
ping servicing and ordered retirement. `tests/fix_contract.rs` exercises framing,
required/financial evidence, commission assets, future-code uncertainty, exact
binary encoding, both binary session modes, Logon signing, reused IDs, control
servicing and joined teardown. Private controlled-time tests reproduce queued
expiration/cancellation, heartbeat probes, observed quota floors and delayed leases.
These tests establish their named invariants, not exhaustive venue conformance.

[Issue #11](https://github.com/SharurTrading/binance-rs/issues/11) remains open for
COIN-M algo request/response/quota contracts and funding-info quota evidence, plus
the FIX unsubscribe evidence limitation above. The current official catalogs do
not supply the missing COIN-M admission facts. Those endpoints are not exposed with
guessed quotas or USDⓈ-M assumptions. Execution/credential changes require human
review. `publish = false` remains; no release or live trading is authorized.
