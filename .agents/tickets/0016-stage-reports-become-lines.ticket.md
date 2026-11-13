# 0016: stage reports become lines

**Spec:** a spec since removed from the tree.

**What to build:** Each stage's checked report becomes its one line.

**Blocked by:** 0015.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `console › grilling   3 open questions settled` (`grilling`, output)
- [x] prints `console › to-spec    0011-SPEC revised · 3 acceptance criteria` (`toSpec`, output)
- [x] prints `console › to-tickets 3 tickets · blocking edges recorded` (`toTickets`, output)
- [x] prints `console › implement  tdd · 5 components · 14 files · boundary only · stops at the reserved part` (`implement`, output)
- [x] prints `console › code-review 2 findings · fixed` (`codeReview`, output)
- [x] A report missing or failing its checks is a stop.

**Note, 2026-11-11:** each stage's report is read strictly in its own shape and then checked: `revised`, `blockingEdgesRecorded`, `boundaryOnly`, `stoppedAtReservedPart` and `fixed` must be true, the method `tdd` and the spec the one the ticket links. The outcomes the simulation does not show (findings left unfixed, an implement past the boundary, a spec written rather than revised, tickets without blocking edges) fail the stage, as the spec says until the owner decides. The figures go into the stage's entry of `run.json` (`report`, one of the five report shapes), and a count of 1 takes the singular.

**Note, 2026-11-11:** from the two-axis review: the workflow file must keep, for each stage, the skill nsx plays it with (`flow/workflow.rs`, `SKILLS`): a file with another skill, or none where one is played, stops the command with one sentence, so a stage line never names a skill the call did not follow.

## Dev part

**Category:** `core-domain`, because a line is never invented from free text.

**Paths:** `lib/nsx/src/skills/report/check.rs`, `lib/nsx/tests/skills/report.rs`.

**Boundary:** the `Report` type in `skills/report.rs`.

**Goal:** Check a stage's report against its contract.

- [x] Each of the five reports is accepted when valid and refused when not.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
