<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Binance Rust Client

[![CI](https://github.com/SharurTrading/binance-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SharurTrading/binance-rs/actions/workflows/ci.yml)

An **unofficial**, independent Rust inner client for Binance APIs, maintained by
SharurTrading. This project is not affiliated with or endorsed by Binance.

**Status: repository scaffold only.** No API operations or runtime dependencies are
implemented. The crate is not published, and `publish = false` prevents accidental
crates.io publication. The local package name is `binance-client`; registry name
availability is not claimed.

## Boundary

The client owns provider transport, request signing, authentication, provider-native
DTOs, request outcomes, documented rate budgets, and streaming lifecycle. Its caller
owns the async runtime and supplies credentials and client order IDs.

Consumers own order routing decisions, portfolio accounting, risk, persistence,
valuation, UI, and translation to their own domain. There are no dependencies on a
consuming trading platform and no universal broker abstraction.

## Initial implementation target: Spot

Spot is the first product target. Its contract must preserve:

- Asset identities and separate free and locked amounts for each asset.
- Base and quote assets from symbol definitions, without parsing symbol spelling.
- Base-asset order quantities and quote-asset spend as distinct request semantics.
- Fill commission amounts together with their actual commission assets.
- Exact decimal prices, quantities, balances, fees, and exchange filters.
- Snapshot and incremental update semantics, order/trade identities, and explicit
  transport continuity boundaries.

A balance is not an account-wide cash scalar. The client does not choose a reporting
currency, convert balances, calculate portfolio equity/P&L, or invent futures-style
positions for Spot holdings. This bootstrap adds no futures, margin, OMS Toolkit,
rebate, or builder-fee implementation.

## Development

See [AGENTS.md](AGENTS.md) for the coding and review contract,
[CONTRIBUTING.md](CONTRIBUTING.md) for local checks, and
[SECURITY.md](SECURITY.md) for private vulnerability reporting.

CI validates Rust on Linux, macOS, and Windows; Markdown, licensing, dependencies,
and Git history secret scanning run independently. All checks run on every pull
request and push to `main`, including documentation-only changes. A weekly run also
checks for newly disclosed dependency advisories.

The empty crate deliberately has no placeholder behavior tests. A green bootstrap
build proves the scaffold works; it does not prove a Binance integration exists.

## Sources

Protocol decisions must cite the applicable Binance documentation and record when
it was checked. Start with:

- [Binance developer documentation](https://developers.binance.com/en/docs/)
- [Spot REST API](https://developers.binance.com/en/docs/products/spot/rest-api)
- [Spot user data streams](https://developers.binance.com/en/docs/products/spot/user-data-stream)
- [Spot filters](https://developers.binance.com/en/docs/products/spot/filters)
- [Official Spot protocol reference](https://github.com/binance/binance-spot-api-docs)

## License

[MIT No Attribution (MIT-0)](LICENSE).
