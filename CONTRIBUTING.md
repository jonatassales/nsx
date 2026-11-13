# Contributing

nsx is built from tickets: read [`AGENTS.md`](AGENTS.md), the rules in
[`.claude/rules/`](.claude/rules/) and [`.agents/context.md`](.agents/context.md)
before a change.

## Set up

```bash
rustup toolchain install
```

```bash
drift/drift.sh git setup
```

The first installs the toolchain `rust-toolchain.toml` pins; the second points
Git at the commit-msg hook in `drift/hooks/`.

## Before a pull request

```bash
cargo fmt --check
```

```bash
cargo clippy --all-targets --locked -- -D warnings
```

```bash
cargo test --locked
```

```bash
cargo deny check
```

```bash
drift/drift.sh quality check
```

Commits follow Conventional Commits, carry no AI signature, and a commit that
touches `src/`, `tests/`, `schema/`, `langdrift/` or `bin/` names its ticket in
a `Ticket: <NNNN>` trailer. A golden under `tests/simulation/golden/` changes
only when the simulation does (ADR 001).
