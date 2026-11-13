# 0018: the dev ticket and Safe Seat

**Spec:** a spec since removed from the tree.

**What to build:** Implement stops at the reserved part; the run waits for the person to write it, closes the dev ticket with no paid call, and later says Safe Seat was met.

**Blocked by:** 0016, 0014.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `console › waiting    dev-ticket · the person writes the reserved part` (`devTicketWaiting`, output)
- [x] prints `✓ closed     dev-ticket · written without an agent · no paid call` (`devTicketClosed`, success)
- [x] prints `✓ Safe Seat  met` (`safeSeatMet`, success) (Closed with another ticket.)
- [x] `run.json`'s Safe Seat section is filled. (Closed with another ticket.)

**Note, 2026-11-11:** the person closes the dev ticket by entering a line in the run's terminal, any line; the end of the input stops the run after the `waiting` line with the stage `active` (owner, plan of 2026-11-11). `paidCalls` is measured from the usage record between the stage's two lines, not assumed. `seat/verdict/reach.rs` decides Safe Seat met when the person closed the dev ticket with no paid call during it; a count that is unknown does not meet it.

**Note, 2026-11-11:** two criteria stay open: `✓ Safe Seat  met` follows `✓ checks     4 of 4 passed`, so the line prints and `run.json`'s Safe Seat section is written with another ticket. The verdict, its line and its section are built and tested here.

**Note, 2026-11-11:** from the two-axis review: `paidCalls` is measured, not assumed. The run counts its calls in `usage.json` on the disk before the wait and after it; a call paid meanwhile stops the run before the `closed` line, which says "no paid call", and the stage stays `active` with no `paidCalls`; a usage record that holds fewer of the run's calls after the wait than before it is a stop too.

**Note, 2026-11-11:** `✓ Safe Seat  met` prints after `✓ checks     4 of 4 passed`, with another ticket (`command/run/follow.rs`): the verdict is reached from the dev ticket as `run.json` closed it, and the Safe Seat section is written then. A verdict that is not met prints no line and stops the run, with the pull request left a draft.

## Dev part

**Category:** `core-domain`, because Safe Seat is the rule that keeps the team in its code.

**Paths:** `lib/nsx/src/seat/verdict/reach.rs`, `lib/nsx/tests/seat/verdict.rs`.

**Boundary:** the `Verdict` type in `seat/verdict.rs`.

**Goal:** Decide whether Safe Seat was met.

- [x] A dev ticket closed after a paid call is not met.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
