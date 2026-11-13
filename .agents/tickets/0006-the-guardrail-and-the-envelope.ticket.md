# 0006: the guardrail and the envelope

**Spec:** a spec since removed from the tree.

**What to build:** The plan reads the team's guardrail and states the envelope as the design doc sets it.

**Blocked by:** a ticket since removed from the tree.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `reads      harness/guardrails/0001-finops.gr.md · FIN-02 · FIN-04` (`guardrail`, output)
- [x] prints `bounded    cpuMillis 2000 · memoryMiB 3072 · set by the team in the design doc` (`bounds`, output)
- [x] A guardrail with no criteria table is a stop.

**Note, 2026-11-11:** the criteria table is the document's first table; `ID` and `Checked at` are found by name and the rule is the other column. A row with no id, or checked at anything but `plan`, `run` or both, is a stop, and so is a guardrail with no table or no row. The stop lands before the guardrail's own line, the one that opens with `reads`.

**Note, 2026-11-11:** the envelope is handed from the design doc through `Envelope::of`, as the budget is. The guardrail's row of DOCUMENTS is written when its line prints: `id`, `file`, `criteria` (each an `id`, its `statement` absent until the judge's line), `planCriteria` and `enforcement`, always `review`. The bound's place in the Plan card's sizing is written with ticket 0008, which adds that section.

**Note, 2026-11-11:** the two open questions on the guardrail stay open as the prefaces word them: whether leaving FIN-03 out of the line is meant, and whether `Checked at` is how a guardrail says which step a criterion governs. The line follows the simulation.

**Note, 2026-11-11:** after the two-axis review. A criteria table needs its row of dashes under the header: before, a table without one silently lost its first criterion. The guardrail's row of the card is written by `harness/guardrail/card.rs`, not by the command.

## Dev part

**Category:** `core-domain`, because the team's own rules, read from its file.

**Paths:** `lib/nsx/src/harness/guardrail/load.rs`, `lib/nsx/tests/harness/guardrail.rs`.

**Boundary:** the `Guardrail` and `Criterion` types in `harness/`.

**Goal:** Read the criteria table: `ID`, the rule and `Checked at`.

- [x] The fixture's guardrail gives FIN-02, FIN-03 and FIN-04.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
