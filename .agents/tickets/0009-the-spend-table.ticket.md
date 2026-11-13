# 0009: the spend table

**Spec:** a spec since removed from the tree.

**What to build:** The Workflow card shows what each model and effort would cost against the budget, before the plan's first line.

**Blocked by:** 0007.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] `plan.json`'s workflow section holds nine rows, one per model and effort, with `budgetCents` 2500 and `marginPercent` 20.
- [x] A price list missing a model is a stop.

**Note, 2026-11-11:** the `workflow` section of `plan.json` gets its fields: `budgetCents`, `budgetSource` (the design doc's path from the project's root), `marginPercent` and `spend`, nine rows of `model`, `effort`, `inputThousands`, `outputThousands`, `estimateCents`, `reservedCents` and `overBudget`. The plan reads the forecast and builds the table before its first line, and writes it with the ticket's header and the threshold. The names in the approved plan are the agent's proposal where another spec gives none.

**Note, 2026-11-11:** the rows follow the three models nsx reads (`money/price.rs`, `MODELS`), then low, medium and high. A price list without one of the three, a forecast without one of the three efforts, a negative price, a token count that is negative or not whole, and an amount too large for cents each stop the plan before its first line, with one sentence. A model beyond the three, or a model listed twice, is refused when the price list is read (review of pull request 208): the judge could suggest a fourth with no row in the table, and a second price would be ignored.

**Note, 2026-11-11:** left open for the owner: the Workflow header and the MODEL price line show a model's name, its short name and the date the prices were read, and the section does not carry them yet (`contract/workflow.rs`, Open). The stages and the default choice join with another spec.

## Dev part

**Category:** `core-domain`, because the table the team chooses from.

**Paths:** `lib/nsx/src/money/spend/build.rs`, `lib/nsx/tests/money/spend.rs`.

**Boundary:** the `Spend` type in `money/spend.rs`.

**Goal:** Build one row per model and effort from the price list and the forecasts.

- [x] The fixture's nine rows equal another spec's figures.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
