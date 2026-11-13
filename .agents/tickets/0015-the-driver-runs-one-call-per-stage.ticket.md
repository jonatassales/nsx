# 0015: the driver runs one call per stage

**Spec:** a spec since removed from the tree.

**What to build:** Each stage runs through the coding agent, once, with the authorized model and effort and a small explicit environment. The tokens it used are read back.

**Blocked by:** 0014.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] The real driver runs the executable `harness/driver.json` names; the tests use the fake agent.
- [x] No line prints from the driver.
- [x] A missing `driver.json` is a stop.

**Note, 2026-11-11:** the coding agent sits behind `driver/agent.rs`: `Launcher` starts the executable `harness/driver.json` names, directly and with no shell, each placeholder filled inside its argument, with the minimal base environment, the `passEnv` variables and the `LANGDRIFT_*` ones. The configuration is read once the gate has opened, before the first stage waits, so a run over its budget still pauses without one (owner, plan of 2026-11-11); a missing file, an unknown field or `args` without `{{model}}` and `{{effort}}` stops the run there.

**Note, 2026-11-11:** a call that started gives back how it ended, whatever its exit status: the usage record keeps it first, then a non-zero status or a token count the agent did not report stops the run with no line for the stage. The call's output is kept in `agent/state/runs/<run>/stages/<stage>.log`, private to its owner. The call's elapsed time is not kept: nothing reads it. `nsx init` writes no `driver.json`: the file is the team's to fill.

**Note, 2026-11-11:** from the two-axis review: a run's id is the highest of the money ledger and of the private run folders plus one, so a checkout without an earlier ledger entry never reuses that run's private files; a report an earlier call left is deleted before the agent starts, so a stage is never given a line from stale figures; the placeholders of `driver.json` are filled in one pass, so a ticket or skill text that looks like `{{model}}` is passed as written; the usage record and the call's log (`driver/call/log.rs`) are both written before either failure stops the run, so a paid call is never left with neither; a brace that opens no placeholder is passed as written and does not hide the placeholder after it (`{{{model}}}`); an agent killed by a signal is its own sentence (`driver/ended/accept.rs`).

## Dev part

**Category:** `core-domain`, because observed spend comes from these tokens.

**Paths:** `lib/nsx/src/driver/tokens/read.rs`, `lib/nsx/tests/driver/tokens.rs`.

**Boundary:** the `Agent` trait in `driver/agent.rs`.

**Goal:** Read the tokens a call used from the agent's output.

- [x] Input and output tokens are read; missing ones are `null`, never zero.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
