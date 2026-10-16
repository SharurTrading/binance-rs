<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Contributing

Read [AGENTS.md](AGENTS.md) before changing code. This repository is an independent
Binance inner client; consuming application policy does not belong here.

The checked-in toolchain installs Rust, Clippy, and rustfmt. Local checks are:

```sh
python3 scripts/codegen/generate.py --check
python3 -m unittest discover -s tests/ci -p 'test_*.py'
cargo fmt --all -- --check
bash scripts/ci/check_spdx_headers.sh
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo test --doc --all-features --locked
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps --locked
cargo package --locked
cargo deny --locked check
cargo audit --file Cargo.lock --deny warnings
npx --yes markdownlint-cli2@0.23.3
gitleaks git --no-banner --redact --log-opts="--all" .
```

Install `cargo-deny`, `cargo-audit`, Gitleaks, and Node.js separately when running
their checks locally; CI pins its tool versions. Package verification requires a
clean checkout. Before the first commit, `cargo package --locked --allow-dirty`
can validate local work without publishing anything.

Optionally enable the formatting/Clippy pre-commit hook:

```sh
git config core.hooksPath hooks
```

Use synthetic fixtures and local mock servers. Live provider access does not belong
in ordinary tests. Describe validation and relevant protocol sources in every PR;
disclose AI authorship with the exact model name when applicable.

Bindings derive from pinned protocol facts in `schema/`. The generator uses the
Python standard library and the pinned rustfmt; it performs no downloads. Read
[coverage and provenance](docs/coverage.md) before updating facts, required event
evidence, financial classification, or rate weights. Generated code remains under
the same strict Rust gates as handwritten transport.

## Local build cache

Kache is optional and speeds up only local builds of this repository. Crates that
depend on this one never use it, and CI does not use it (it caches with
`Swatinem/rust-cache`). To opt in, install it and run its one-time setup, which
covers every repository on the machine:

```sh
cargo install kache --locked
kache init
kache doctor
```

[.kache.toml](.kache.toml) keeps the cache local-only, with no remote backends or
planner, and isolates this repository's entries under its own key salt;
`kache doctor` lists it as the config file. The
[Kache quick start](https://ninja.kunobi.com/docs/kache/getting-started/quick-start)
covers the rest.
