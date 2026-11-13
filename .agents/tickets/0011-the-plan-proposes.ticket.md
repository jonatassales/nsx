# 0011: the plan proposes

**Spec:** a spec since removed from the tree.

**What to build:** The plan ends by writing the intent as proposed and saying the draft was proposed. The plan golden passes.

**Blocked by:** 0008, 0009, 0010.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `✓ plan       runtime.md draft, revision 1 · proposed to the console` (`draft`, success)
- [x] `intent.json` holds status `proposed`, the suggested model and effort, and Planning as a human stage.
- [x] The plan golden passes with an empty pending list.

**Note, 2026-11-11:** the proposed intent takes the workflow's default choice (`harness/workflow.json`, `defaults`), not the judge's suggestion, as `contract/intent.rs` says; Planning is forced to a human stage. The plan reads the workflow before its first line, so a missing or malformed file stops it there (the spec lists no such failure: provisional, in `flow/workflow/load.rs`). A file that does not list the seven stages in order, whose default effort is not low, medium or high, or whose default model is not in the price list stops the plan there too, so the plan never reaches its draft with a choice it could not propose. The criterion and the dev part's goal above say "suggested" and "from the suggestion": they predate that reading, and the note holds.

**Note, 2026-11-11:** order at the end of the plan: `runtime.md` is written, then `intent.json`, then `draft` prints, then the Plan card gets `authorization` (`pending`) and `evidence` (`runtime.md`, `draft`, the revision, `console`). A plan that stops writes neither the draft nor the intent. The Workflow card's stages and default choice still join with another spec's tickets.

## Dev part

**Category:** `core-domain`, because the plan never authorizes; it only proposes.

**Paths:** `lib/nsx/src/handoff/proposal/write.rs`, `lib/nsx/tests/handoff/proposal.rs`.

**Boundary:** the `Proposal` type in `handoff/proposal.rs`.

**Goal:** Write the proposed intent from the suggestion and the workflow.

- [x] The written intent validates and its status is `proposed`.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
