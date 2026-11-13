# 0012: the run opens on the authorized intent

**Spec:** a spec since removed from the tree.

**What to build:** The owner types `nsx agent run`; the run finds the one authorized intent, checks it, opens `run.json` and prints its opening.

**Blocked by:** 0011.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `# run · 0031-TICKET-book-a-demo-runtime · the recorded intent, authorized by the team` (`scope`, comment)
- [x] prints `console › intent     claude-sonnet-5-5 · effort medium · runtime.md revision 1` (`intent`, output)
- [x] An intent still proposed, or naming a model the price list lacks, is a stop.
- [x] `run.json` opens with a run id and the outcome `running`.
- [x] prints `console › intent     claude-haiku-5-5 · effort low · runtime.md revision 1` (`intent`, output) in `run-haiku-low`
- [x] prints `console › intent     claude-opus-5-5 · effort high · runtime.md revision 1` (`intent`, output) in `run-over-budget`

**Note, 2026-11-11:** the run takes no argument: `handoff/authorization/choose.rs` reads every recorded intent and applies the one the team authorized whose `run.json` is not `completed`. None is a stop (when one is still proposed, the sentence says the team has not authorized it yet), and several are a stop that names their tickets. The check refuses an intent that breaks the contract's rules, one still proposed, a revision that is not the draft's and a model outside the price list, each before any line and before `run.json` exists.

**Note, 2026-11-11:** `run.json` opens when the `intent` line prints: the run's id (`<runtime id>-<NNNN>`, the highest run in the money ledger plus one), the applied intent, the seven stages as `pending` and the outcome `running`. It ends `paused` after the gate and `stopped` on any stop. `run.json`'s `intent` section and each stage's `id`, `human`, `state` and `confirmedBy` join the contract here, and `schema/handoff/run.schema.json` changes with them.

**Note, 2026-11-11:** from the review. An intent that names another runtime than the folder it sits in is a stop: its run would write into the other runtime's records, past that runtime's completed run. An intent or a run record that does not read, a folder or a broken link under that name included, is a stop and never a skip. A run's number keeps all its digits past 9999, and only a ledger file of this runtime counts, so no two runs share an id. A run that stopped states why first; a record that could not be closed is said after it.

## Dev part

**Category:** `core-domain`, because a run must never start on a proposal.

**Paths:** `lib/nsx/src/handoff/authorization/check.rs`, `lib/nsx/tests/handoff/authorization.rs`.

**Boundary:** the `Authorization` type in `handoff/authorization.rs`.

**Goal:** Check the authorized intent against the draft and the price list.

- [x] A proposed intent, a stale revision and an unknown model are each refused.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
