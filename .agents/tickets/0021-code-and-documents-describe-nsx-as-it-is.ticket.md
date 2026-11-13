# 0021: Code and documents describe nsx as it is

**Spec:** None. The owner's direction of 2026-11-13: the LangDrift CLI is nsx, written in Rust, and nothing an agent reads describes another CLI before it. Records about earlier tooling leave the tree, and Git keeps them.

**What to build:** A person or an agent reading the context file, an ADR, a PRD, a README or a code comment meets nsx as the only LangDrift CLI. Comments state the rule the code holds, not where it came from, and no live file cites a record that left the tree.

**Blocked by:** None (can start immediately).

**Status:** completed

**Alignment:** remove. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. Nothing the simulation shows changes here.

- [x] LangDrift's context file, its ADRs on the simulation and the backend, its MVP PRD, its open spec on the session panel and every README name nsx as the LangDrift CLI and describe only what exists.
- [x] The comments in `nsx/`, `apps/` and `sst.config.ts` state the rule they hold without naming its origin.
- [x] No live file cites a spec, ticket, ADR, PRD, design doc or research note that left the tree.
- [x] No comment, test or README under `nsx/` cites a spec or ticket of this repository, kept or removed: the CLI will eventually live in a repository of its own.
- [x] The README and website screenshots' alt texts and captions describe the run without naming a command, in all four locales.
- [x] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked`, `pnpm turbo run check typecheck test`, `drift check localization` and `drift quality dead-code` pass.

**Note, 2026-11-13:** the owner widened the ticket while it ran: no spec or ticket number stays anywhere under `nsx/`, kept records included, so the golden test's pending lists hold line ids only. A wider scan for implicit references also found the console's catalog keys and comments about its earlier stub; the unused keys were removed in the four locales.

## Dev part

**Override:** on 2026-11-13 no part qualifies: the ticket rewrites documentation, comments, test names and catalog copy that describe earlier tooling, which holds no problem of the policy's categories (`core-domain`, `hard-part`, `new-stack`).
