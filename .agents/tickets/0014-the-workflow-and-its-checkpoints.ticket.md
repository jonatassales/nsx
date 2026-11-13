# 0014: the workflow and its checkpoints

**Spec:** a spec since removed from the tree.

**What to build:** The run waits for the owner to confirm Planning, and any other stage the team checked, reading the answer from standard input. It ends by saying what the owner decides next.

**Blocked by:** 0013.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `console › waiting    planning · the owner confirms` (`waiting-planning`, output)
- [x] prints `✓ confirmed  planning · by the owner` (`confirmed-planning`, success)
- [x] prints `console › waiting    spec · the owner confirms` (`waiting-spec`, output)
- [x] prints `✓ confirmed  spec · by the owner` (`confirmed-spec`, success)
- [x] prints `console › waiting    tickets · the owner confirms` (`waiting-tickets`, output)
- [x] prints `✓ confirmed  tickets · by the owner` (`confirmed-tickets`, success)
- [x] prints `console › waiting    implement · the owner confirms` (`waiting-implement`, output)
- [x] prints `✓ confirmed  implement · by the owner` (`confirmed-implement`, success)
- [x] prints `# next: the owner reviews #19 and decides whether to merge` (`next`, comment) (Closed with another ticket.)
- [x] prints `# next: the merge stays the owner's` (`next`, comment) in `run-to-merge`, where Review is unchecked (Closed with another ticket.)
- [x] Planning always waits.
- [x] A declined checkpoint is a stop.

**Note, 2026-11-11:** the owner's answer is read from the run's standard input, behind `flow/reader.rs`. Yes (`yes` or `y`, any case) confirms; anything else, or the end of the input, declines and stops the run right after the `waiting` line, with the stage left `waiting` in `run.json` (owner, plan of 2026-11-11). The answer is echoed as typed: it is one word and no secret.

**Note, 2026-11-11:** the two `next` criteria stay open: the line follows `ready` and carries the pull request's number, so it prints with another ticket. `flow/ending/line.rs` holds both wordings. Until tickets 0015 to 0018 add the stages' own work, only the checkpoints play, in the stages' order.

**Note, 2026-11-11:** the `next` comment prints after `ready`, with another ticket: `# next: the owner reviews #19 and decides whether to merge`, or `# next: the merge stays the owner's` when Review is unchecked (`command/run/carry.rs`).

## Dev part

**Category:** `core-domain`, because a person always stands between the plan and the work.

**Paths:** `lib/nsx/src/flow/checkpoint/wait.rs`, `lib/nsx/tests/flow/checkpoint.rs`.

**Boundary:** the `Checkpoint` type in `flow/checkpoint.rs`.

**Goal:** Wait for the person's answer on standard input.

- [x] Yes confirms; anything else declines; end of input declines.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
