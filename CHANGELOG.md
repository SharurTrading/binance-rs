<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

## Unreleased

- Add initial Spot and COIN-M JSON clients with separate REST/WS models, exact
  asset balances, base versus quote order amounts, inverse contract volumes,
  source-ordered user events, and product-specific depth bootstrap rules.
- Pin official Spot/COIN-M protocol facts checked on 2026-09-27 and extend
  deterministic generation/freshness checks across all three products.
- Add Spot Unicode signing, fractional receive windows, current subscription and
  shutdown events, shared REST/WS quotas, daily orders, and successful order/cancel
  weight refunds. Share UM/CM budget ownership explicitly and retain reconciliation
  IDs on transport and venue errors, including nested batch identities.
- Cover product signing, ambiguous/truncated replies, quota boundaries, late
  responses, retirement, and malformed execution evidence with synthetic fixtures.
  Advanced coverage remains tracked in
  [issue #11](https://github.com/SharurTrading/binance-rs/issues/11); no live execution
  or release readiness is claimed.

- Refresh all direct Cargo requirements to current stable releases, including
  reqwest 0.13.5 and base64 0.23.1, and update the dependency lockfile.

- Implement USDⓈ-M Futures first: 95 REST operations, 18 catalog WebSocket API
  methods plus session logon/status/logout, 20 market streams, and 10 user-data events.
- Share transport, HMAC/RSA/Ed25519 signing, clocks, deadlines, safe outcome evidence,
  and scoped venue budgets without unifying product account or settlement models.
- Preserve exact decimals, validated caller IDs, batch member results, late response
  attribution, unbounded ingress, generation boundaries, and finite depth continuity.
- Add deterministic local HTTP/WebSocket fault tests, explicit read-only demo probes,
  protocol fact snapshots, reproducible bindings, documentation, and freshness CI.
- Retain MIT-0 licensing, cross-platform gates, human review, and unpublished status.
