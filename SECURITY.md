<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Security

Report vulnerabilities through [GitHub private vulnerability reporting](https://github.com/SharurTrading/binance-rs/security/advisories/new).
Do not post credentials, signing material, signed URLs, account identities, or raw
provider captures in public issues.

Callers acquire and store credentials. The library never reads environment variables
or credential files. API keys/HMAC key bytes and supplied PKCS#8 DER use zeroizing
storage; parsed asymmetric keys are owned by AWS-LC. Credentials and sensitive
provider strings redact `Debug`. Explicit access/serialization can reveal their
contents and must not be logged. Ordinary financial DTOs contain private account
data; their debug representation is not a safe logging format.

TLS uses an explicit Rustls provider and trusted roots. Insecure URLs are refused
except exact loopback fixture hosts; URL credentials, query strings, and fragments
in endpoint configuration are refused. Ambient proxies, automatic redirects, and
mutation retries are disabled. WebSocket dependency logging was audited to avoid
raw handshake/frame logging. Errors retain safe operation evidence, not sensitive
URLs or raw bodies. Re-audit transport dependencies when changing versions.

Consumers own socket driver tasks and must close/drain/join them. Dropping or
aborting a driver is not orderly retirement. Actual transport loss is explicit;
unresolved mutation outcomes require venue reads before further action.
Only synthetic fixtures belong in this public repository. Credentialed tests and
trading operations require explicit operator authorization; ordinary CI has neither.
