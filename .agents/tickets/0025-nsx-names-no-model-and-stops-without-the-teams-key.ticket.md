# 0025: nsx names no model and stops without the team's key

**Spec:** None. The owner's direction of 2026-11-14: LangDrift offers no model and pays for none; the team brings its Anthropic key and models to the console, and the console's judge judges with them. The simulation gained the plan that stops without the team's key the same day.

**What to build:** nsx's judge question names no model: the console answers with the team's judge model and key. When the console answers that the team has no Anthropic key, `nsx agent plan` prints the simulation's two lines in place of the judge's first line, records the judge as unavailable on the Analysis card, reports `error` and stops with no draft. The refusal for an offer without a priced model no longer names models for the team to offer.

**Blocked by:** None. Before this is released, the console must accept a judge question without `model`, which it does once its settings change is deployed, and must pin nsx at this change, since it validates every report against nsx's schemas and would refuse a report carrying `judge` until then.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 003): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. With the key switched to none, the simulation plays the plan through `guardrail`, then `✗ judge      no Anthropic key · add one on the console's Settings → AI Models` and `# plan stopped · no draft written · nothing was paid for`; its Analysis card reads `judge unavailable · no Anthropic key` with `Judge` · `no key · Settings → AI Models`.

- [x] The judge question carries five fields, `kind`, `system`, `prompt`, `maxTokens` and `promptVersion`, and no `model`; the deprecated constant is gone.
- [x] A console answer with the reason `model-key-missing` gives `Error::KeyMissing`, whose sentence is `The console has no Anthropic key: add one on the console's Settings → AI Models.`
- [x] Without the key, the plan prints the golden `plan-keyless`: the plan's lines through `guardrail`, then `judgeStopped` (a new `error` kind and `✗` failure shape) and `planStopped`.
- [x] `plan.json`'s `analysis` gains `judge`, absent unless the judge is unavailable: `{ "unavailable": "model-key-missing" }`. `schema/handoff/plan.schema.json` is published with it.
- [x] The plan's last report has status `error` and the `judge` section; no `runtime.md` and no `intent.json` are written. The stop rule names this shown stop, which exits 1 with its sentence on standard error.
- [x] An offer with no model nsx holds a list price for stops with a sentence that names no model.
- [x] The six existing goldens are unchanged.
- [x] `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, `cargo deny check`, `vtec/vtec.sh quality check` and `vtec/vtec.sh git lint-history origin/main HEAD` pass.

## Dev part

**Category:** `hard-part`: stopping a plan mid-act on one refusal, with its two lines, its card and its last report, without breaking the stop rule.

**Paths:** `src/judgment/keyless/line.rs` and `src/judgment/keyless/card.rs`.

**Boundary:** `Keyless::lines(&self) -> [Line; 2]` and `Keyless::card(&self, cards: &mut Cards)`, which the analysis act calls.

**Goal:** the two lines and the card record of a plan without the team's key.

- [x] The lines render as the golden `plan-keyless` freezes them, and the card records `model-key-missing`.

**Override:** on 2026-11-14 the owner asked the agent to write the whole change, dev parts included: "you can go ahead and write it all!".
