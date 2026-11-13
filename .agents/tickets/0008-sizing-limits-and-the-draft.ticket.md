# 0008: sizing, limits and the draft

**Spec:** a spec since removed from the tree.

**What to build:** The plan sums the shares against the envelope, warns that gpu and disk are not `runtime.md` fields, validates the draft and writes `runtime.md`.

**Blocked by:** 0006.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `✓ size       1450m of 2000m · 2304 of 3072 MiB · 5 one-replica Deployments · proposed shares` (`size`, success)
- [x] prints `limits     gpu and disk are not runtime.md fields · none requested` (`limits`, warning)
- [x] prints `console › tool call: validate runtime.md draft against the schema · 0 errors` (`validate`, output)
- [x] A plan over the envelope is refused.
- [x] `runtime.md` is written with revision 1, and a second plan raises it.

**Note, 2026-11-11:** the plan sums the shares at each component's row, to put the running totals on the Plan card, and once more after the cross-check, where `sizing/fit.rs` says whether the sum fits the bound. A plan over the bound stops there, before `size`, with every component's lines and the cross-check already printed; a sum equal to the bound fits. The stop's sentence states the sum of all the components, the bound and the design doc that sets it. Until the review of pull request 208 the plan stopped at the row that crossed the bound, and its sentence stated only the sum so far.

**Note, 2026-11-11:** `plan.sizing` in `plan.json` gets its fields, each when its line prints: `bound` (`cpuMillis`, `memoryMiB`, `source`) at `bounds`, `proposed` from the first component's row, `headroom` and `resource` (`kind` Deployment, `count`, `replicas` 1) at `size`, `limitsRequested` (an empty list: none requested) at `limits`, and `validation` (`schema`, the schema's `$id`, and `errors`) at `validate`. `plan.price` gets `source`, `null` while the runtime has no price source, at `price`, which now prints after `limits`. The names are the agent's proposal, approved with the plan of 2026-11-11 ("Preencher agora").

**Note, 2026-11-11:** `runtime.md` is a title (the runtime's id), the `langdrift` block with the fence as tab-indented JSON, and `## Sizing`: a table of the proposed totals, the bound and the headroom, then one line with the resource, the limits statement and the validation result (owner, 2026-11-11: "Fence e relatório em tabela"). It is written after `validate`, only at 0 errors; any other count stops the plan before `validate` with the count and each broken rule in the sentence.

**Note, 2026-11-11:** the revision is 1 for a first plan and the earlier draft's plus one after it, read from the earlier draft's block alone, whatever its version; an earlier draft with no readable revision gives revision 1, and one nsx cannot read as a file stops the plan. The plan act ends once the draft is written: the `draft` line, the intent and the Plan card's authorization and evidence are ticket 0011's.

## Dev part

**Category:** `core-domain`, because sizing decides whether the plan fits what the team allowed.

**Paths:** `lib/nsx/src/runtime/sizing/sum.rs`, `lib/nsx/tests/runtime/sizing.rs`.

**Boundary:** the `Sizing` type in `runtime/sizing.rs`.

**Goal:** Sum the shares and the headroom against the envelope.

- [x] The fixture sums to 1450m of 2000m and 2304 of 3072 MiB.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
