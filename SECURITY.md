<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Security

Report vulnerabilities through [GitHub private vulnerability reporting](https://github.com/SharurTrading/binance-rs/security/advisories/new).
Do not post credentials, signing material, signed URLs, account identities, or raw
provider captures in public issues.

The current repository is a scaffold and implements no authentication or transport.
The credential, proxy, TLS, logging, and lifecycle requirements for implementation
are specified in [AGENTS.md](AGENTS.md). Callers own secret acquisition and storage.
Only synthetic fixtures belong in this repository.
