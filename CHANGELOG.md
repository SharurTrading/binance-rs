<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Changelog

## Unreleased

- Add a read-only Spot changelog check on every PR, `main` push, and weekly CI run,
  comparing Binance's last update date with our last review date. Newer updates and
  unavailable/malformed evidence fail visibly; offline checker tests run on all
  three operating systems. Exclude generated Python caches from package contents.

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
