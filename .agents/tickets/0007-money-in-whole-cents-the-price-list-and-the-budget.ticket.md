# 0007: money in whole cents, the price list and the budget

**Spec:** a spec since removed from the tree.

**What to build:** The plan states the budget per run and who set it, and that the runtime has no price source. Amounts are whole cents and print as the simulation prints them.

**Blocked by:** a ticket since removed from the tree.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `budget     $25.00 per run for model calls · set by the team in the design doc` (`budget`, output) (Closed with another ticket.)
- [x] prints `estimate   runtime has no price source · cpu and memory stay in millicores and MiB` (`price`, output)
- [x] prints `console › budget     $25.00 per run · set by the team · FIN-03` (`budget`, output) in every run golden, from the same type as the plan's line (Closed with ticket 0013.)
- [x] `$9.50` and `$11.40` format from 950 and 1140 cents.

**Note, 2026-11-11:** the budget's two lines are built and tested from one type, but neither command prints one yet: the budget is read out of the design doc by another ticket, and the run reads the one the plan recorded in ticket 0013. Their two criteria above stay open and move with those tickets, and `budget` stays on every pending list until then. The plan's line is `budget/line.rs`; the run's, which cites FIN-03, is `budget/cite.rs`, one file per line as the README's rule 4 has it.

**Note, 2026-11-11:** an amount formats as the simulation's formatter does, US English: thousands grouped with a comma (`$1,234.56`) and an amount below zero with a leading minus (`-$1.60`). The goldens show neither case. `Cents` is a whole number that may be negative, until the open question on a release below zero is settled.

**Note, 2026-11-11:** the plan loads the price list before its first line, as the plan's script says of the money files, so a project where `nsx init` never ran stops with one sentence and nothing printed. A field the price list does not define is refused. Whether the list holds exactly the three models is not checked: who may edit it is still open. The plan now prints 7 of its 42 lines.

**Note, 2026-11-13:** the plan golden prints the `budget` line with another ticket, and each of the five run goldens prints `console › budget` with ticket 0013, from the same type.

## Dev part

**Category:** `core-domain`, because every amount on screen goes through this.

**Paths:** `lib/nsx/src/money/cents/format.rs`, `lib/nsx/tests/money/cents.rs`.

**Boundary:** the `Cents` type in `money/cents.rs`.

**Goal:** Format whole cents as the simulation shows dollars.

- [x] 0, 950, 1140, 2500 and 4080 cents format as the goldens show.

**Override:** on 2026-11-10 the owner asked the agent to write this dev part ("You write them all"), so the agent wrote its paths.
