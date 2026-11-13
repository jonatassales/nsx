# nsx

nsx is the LangDrift CLI: it plans, runs and records a bounded execution, a microruntime, around a team's own software, from the team's terminal. It is written in Rust and ships on npm as `@langdrift/nsx`. This file holds the ideas an agent must carry into any task here; the mechanics live in [`.agents/context.md`](.agents/context.md).

## The idea

- **The simulation is the definition.** The session that plays on the home page of https://langdrift.md defines what nsx does, line by line and field by field ([ADR 001](.agents/plan/adr/001-perfect-simulation-alignment.adr.md)). The six goldens under `tests/simulation/golden/` hold its lines.
- **Three commands.** `nsx init`, `nsx agent plan <ticket>` and `nsx agent run`, nothing else.
- **The console receives, nsx reasons.** nsx reports every record it writes to the LangDrift console, whose code lives in `jonatassales/lang-drift`, and takes the team's Play or Pause back. The console validates those reports against this repository's JSON Schemas in `schema/`.
- **Unknown is never zero.** Missing evidence reads as missing, never as zero spend or a confirmed stop.
- **Recommendation is not authorization.** The judge suggests; the team's authorization and the money gate decide.

## How we work

- **Rules first.** [`.claude/rules/`](.claude/rules/) holds the rules every task follows. Read them before touching code.
- **The owner writes application code.** Agents edit code only when the task asks for it.
- **Records.** Tickets, specs, PRDs, ADRs and research notes live under `.agents/`, numbered without gaps; [`.agents/context.md`](.agents/context.md) says how.
- **Nothing carries an AI signature**: no `Co-Authored-By`, no "Generated with", in commits, pull requests or contributions. Commit, push and pull request each need explicit authorization outside the agreed workflow; merge and release always do.
- **Secrets stay out of reach.** Never read or print `LANGDRIFT_SECRET` or a recorded connection, and keep credentials out of code, logs and evidence.

## Commands

Run from the repository root.

```bash
cargo build --release --locked
```

```bash
cargo test --locked
```

```bash
cargo clippy --all-targets --locked -- -D warnings
```

```bash
cargo fmt --check
```
