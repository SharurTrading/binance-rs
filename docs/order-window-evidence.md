<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Order window evidence

`Config::account_usage()` and `RestClient::account_usage()` on Spot, USDⓈ-M,
COIN-M and Options read the account owner at that client's clock. `AccountUsage`
reports supported, epoch-aligned `ORDERS` windows in increasing interval order:
ten seconds, minute and day. Each entry retains `interval`, `limit`, `source`,
`used` and `resets_in`. An undocumented window is omitted until the venue states
its limit. FIX observation-relative quotas and Margin quotas remain separate.

Limits come from the documented baseline until a positive venue statement is
read. Statements replace the fallback, even if larger. Among subsequent or
duplicate statements the stricter limit remains in force. Reading the report
neither admits work nor changes a counter. Resetting a usage window leaves its
stated limit intact.

Every admitted placement counts in all supported aligned order windows, including
windows whose ceiling may be stated later. Header and WebSocket count evidence
raise only the receiving account owner's current count; lower evidence never
reduces it. Clones and clients using the same explicit `AccountKey` share counts.
Different account owners share exchange-information limits but retain separate
account-specific statements and counts. Clients sharing
a pool must share a clock, as documented on `Config::clock`.

## Decision for issue 100

Adopt the limits in WebSocket API response `rateLimits` and in Spot's REST
`GET /api/v3/rateLimit/order` and WebSocket `account.rateLimits.orders` results,
using the same complete-statement validation as exchange information. These are
statements of active limits, not just usage samples:

- The [Spot WebSocket API rate limits](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/general)
  define both `limit` and `count`, fixed interval resets, and account-wide order
  usage shared by all API keys.
- The [Spot REST account query](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/rest-api/account#query-unfilled-order-count-user_data)
  and [Spot WebSocket account query](https://developers.binance.com/en/docs/catalog/core-trading-spot-trading/api/ws-api/account)
  return order windows with their limits and counts.

Reviewed 2026-10-11 for [#100](https://github.com/SharurTrading/binance-rs/issues/100).
Adopt `ORDERS` for the receiving account owner, rather than broadcasting an
account-specific query/response statement to other account owners. The venue
explicitly describes these windows as per account. Broadcasting a higher account
allowance could over-admit a different account that has not reported its limit.
Public exchange information continues to state common limits for every account
owner of the pool. When both common and account-specific limits are stated, the
stricter binds. Counts belong only to the receiving owner.
Request weight follows its existing REST/WebSocket pool routing, including the
separate USDⓈ-M WebSocket weight window.

A counted window without a positive integer limit changes no limit from that
statement. Response and account-query arrays must retain valid window identities
and nonnegative integer counts. Unknown valid windows remain in the native reply
and do not create an unsupported admission window.

An account query with unusable evidence fails as `Outcome::ReadFailed` with safe
response metadata. Its prior limits and usage remain intact apart from ordinary
request admission and independent header evidence. A malformed WebSocket statement
preserves received counter evidence and the original acknowledgment or rejection,
marks `RateEvidence::stated_limits_unusable`, and emits a generation-tagged gap
before socket retirement. Callers must drain events and join the driver. A gap
never authorizes replaying a mutation.

Deterministic loopback contracts use a manual clock and their own `WeightPools`.
They check account isolation, stated limits, admission and header counts,
aligned resets, stricter statements, atomic invalid-limit refusal, and preservation
of received acknowledgments. Local compiler-cache validation concerns are tracked
in [#111](https://github.com/SharurTrading/binance-rs/issues/111); this change's
checks bypass that wrapper without changing global configuration.
