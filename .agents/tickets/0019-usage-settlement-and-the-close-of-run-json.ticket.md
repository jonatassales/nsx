# 0019: usage, settlement and the close of run.json

**Spec:** a spec since removed from the tree.

**What to build:** The run records the usage, says what was observed against what was reserved, writes the money ledger and closes `run.json`.

**Blocked by:** 0018, 0013.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `✓ completed  implementation · $9.80 observed of $11.40 reserved · usage record` (`completed`, success)
- [x] The `ready` line's released amount is supplied from here. (Closed with another ticket.)
- [x] `money/settlement/line.rs` is removed: settlement prints no line.
- [x] `run.json` ends with the outcome `completed`, or `paused` after the gate. (Closed with another ticket.)
- [x] prints `✓ completed  implementation · $0.27 observed of $0.32 reserved · usage record` (`completed`, success) in `run-haiku-low`

**Note, 2026-11-11:** `usage.json` is a list of the runtime's runs: each entry keeps its paid calls in order and the totals, `null` as soon as one call reported none. The observed spend is `Estimate::compute` on those totals at the applied model's list price, and the release the reservation minus it, never clamped. The `completed` line prints after `code-review` (the runtime's ready line joins before it with another ticket), and the ledger gets the observed tokens and spend then.

**Note, 2026-11-11:** two criteria stay open until `ready` prints, with another ticket: the released amount goes into the ledger and onto that line, and `run.json` ends `completed`. Until then a run that settles its implementation leaves its record `running`.

**Note, 2026-11-11:** from the two-axis review: the usage record is read before the first paid call (`evidence/usage/kept.rs`), so one that does not read stops the run while nothing is owed, and only a file that is not there counts as no record; a call whose tokens the agent did not report stops at that call (`Unmeasured`), with the call kept in the usage record.

**Note, 2026-11-11:** with another ticket: when `ready` prints, the released amount goes onto the line and into the money ledger (`releasedCents`), and `run.json` ends `completed`. `completed` itself now prints after `runtimeReady`, as the simulation orders them.

## Dev part

**Category:** `core-domain`, because observed and released close the run's money.

**Paths:** `lib/nsx/src/money/settlement/settle.rs`, `lib/nsx/tests/money/settlement.rs`.

**Boundary:** the `Settlement` type in `money/settlement.rs`.

**Goal:** Compute observed and released from the reservation and the tokens.

- [x] $9.80 observed of $11.40 gives $1.60 released.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
