# 0022: nsx stands in its own repository

**Spec:** None. The owner's direction of 2026-11-13: nsx moves from the LangDrift monorepo to `jonatassales/nsx`, with its gates, its harness structure, its CI, the records and rules that are its alone, the Rust practices a CLI repository keeps, and the most-installed Rust skills.

**What to build:** A person who clones `jonatassales/nsx` builds, tests and releases nsx from the repository root, and an agent working here finds nsx's rules, its decision, its tickets and its skills in this repository, without reaching into another one.

**Blocked by:** None (can start immediately).

**Status:** completed

**Alignment:** change. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. Nothing the simulation shows changes here.

- [x] The crate builds, tests and lints from the repository root, and its manifests name this repository.
- [x] `.agents/` and `.claude/` keep the harness structure: the rules nsx follows, ADR 001, tickets 0001 to 0022, and empty folders for what has not arrived.
- [x] Comments cite ADR 001 and the Rust rules of this repository.
- [x] `drift` runs the commit guardrails, the quality check and the pull request widgets, and CI validates every pull request and builds the four binaries on `main`.
- [x] The repository keeps a licence, a security policy, a changelog, contribution notes and a dependency policy, and Dependabot watches cargo and the workflows.
- [x] The most-installed Rust skills sit in `.claude/skills/`, recorded in `skills-lock.json`.

**Note, 2026-11-13:** tickets 0001 to 0021 are the LangDrift monorepo's tickets about the CLI, in their order. ADR 001 states for nsx the decision the monorepo records for its product. Of the gates, the Workflow, Contracts and Code Quality widgets report what they measure; the monorepo's FinOps and Agentic widgets print fixed figures, so they did not come. Of skills.sh's Rust skills, `rust-review` needs Python scripts that ship only with its plugin, and `m15-anti-pattern`, `rust-skills` and `rust-engineer` are broad collections that overlap the ones installed; the owner may add them.

## Dev part

**Override:** on 2026-11-13 no part qualifies: the ticket moves a crate, its records and its tooling, work with no problem of its own to hand-write.
