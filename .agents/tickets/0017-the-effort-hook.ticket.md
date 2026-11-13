# 0017: the effort hook

**Spec:** a spec since removed from the tree.

**What to build:** The effort is recorded before implement may start.

**Blocked by:** 0015.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `console › hook       effort recorded · implement unblocked` (`hook`, output)
- [x] `run.json`'s effort section is written before implement runs.

**Note, 2026-11-11:** the effort record is the `effort` section of `run.json`: the applied model and effort, the judge's suggested pair as the plan's cards carry it (`null` when the plan carried none) and `recordedAt`, an RFC 3339 timestamp in UTC (the form is open in `evidence/clock.rs`). `hook/effort/permit.rs` is the gate: the implement call asks it first and is refused without a record, or with one of another choice.

**Note, 2026-11-11:** from the two-axis review: the `hook` line prints only once `Effort::record` has written the effort section of `run.json`. The gate itself, `Effort::permit`, is asked by the implement call before the coding agent starts, and nowhere else: a second ask right after the record was written could never refuse, so it was removed.

## Dev part

**Category:** `core-domain`, because the hook is a gate, not a note.

**Paths:** `lib/nsx/src/hook/effort/record.rs`, `lib/nsx/tests/hook/effort.rs`.

**Boundary:** the `Effort` type in `hook/effort.rs`.

**Goal:** Record the effort and refuse implement until it is recorded.

- [x] Implement cannot start before the record exists.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
