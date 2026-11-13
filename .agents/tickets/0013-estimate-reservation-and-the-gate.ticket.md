# 0013: estimate, reservation and the gate

**Spec:** a spec since removed from the tree.

**What to build:** The run estimates the spend, reserves it plus 20%, and opens the gate or pauses with nothing paid. The over-budget golden passes.

**Blocked by:** 0012, 0007.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `console › estimate   $9.50 · 3.5M input · 250K output · list prices · FIN-02` (`estimate`, output)
- [x] prints `console › gate       reserved $11.40 of $25.00 · opened` (`gateOpened`, success)
- [x] prints `console › gate       reservation $40.80 exceeds the $25.00 budget · paused` (`gatePaused`, warning)
- [x] prints `# needs a decision: raise the budget or lower the effort · nothing was paid for` (`paused`, comment)
- [x] A paused run exits 0.
- [x] The over-budget golden passes with an empty pending list.
- [x] prints `console › estimate   $0.26 · 2M input · 120K output · list prices · FIN-02` (`estimate`, output) in `run-haiku-low`
- [x] prints `console › gate       reserved $0.32 of $25.00 · opened` (`gateOpened`, success) in `run-haiku-low`
- [x] prints `console › estimate   $34.00 · 6M input · 500K output · list prices · FIN-02` (`estimate`, output) in `run-over-budget`
- [x] prints `console › budget     $25.00 per run · set by the team · FIN-03` (`budget`, output) in every run golden, from the `Budget` type ticket 0007 built

**Note, 2026-11-11:** the criterion above comes from ticket 0007, which built and tested the line but could not print it: the run reads the budget the plan recorded here.

**Note, 2026-11-11:** ticket 0009 prices the spend table in `money/spend/build.rs`, with the forecast's thousands of tokens: the estimate rounded half up and the reservation rounded up. `estimate/compute.rs` is still this ticket's dev part and is untouched: it prices whole tokens per million, for the observed spend, and must give the same cents as the table's row.

**Note, 2026-11-11:** the run prices nothing again: its estimate, reservation and budget are the row of the plan's spend table for the applied model and effort, read from `plan.json`. A plan that priced no such row stops the run. `estimate/compute.rs` prices whole tokens per million, rounded half up, for the observed spend of ticket 0019, and is tested equal to the table's rows. The money ledger's entry (`finops/ledger/<run>.ledger.json`) opens at the `estimate` line and is written again with the gate's outcome; a figure the run did not reach is `null`.

**Note, 2026-11-11:** from the review. The run opens its money entry before its first line (`money/ledger/open.rs`): a plan with no Workflow card or no row for the applied pair stops the run with nothing printed and no id spent, and so does a negative amount or a token count nsx cannot hold, since `plan.json` is a file anyone can edit. The estimate line's token counts are exact (`1.05M`, never `1M` for 1,050K). The spend table now calls `Estimate::compute` and `Reservation::total`, so each formula is written once. `reservedCents` in the ledger is the reservation the gate judges, stated with the estimate as another spec lists it; whether it was held is `gate`.

## Dev part

**Category:** `core-domain`, because the estimate is the number the budget is held against.

**Paths:** `lib/nsx/src/money/estimate/compute.rs`, `lib/nsx/tests/money/estimate.rs`.

**Boundary:** the `Estimate` type in `money/estimate.rs`.

**Goal:** Compute the estimate from the effort's forecast and the model's list prices.

- [x] Sonnet at medium gives $9.50; Opus at high gives $34.00.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
