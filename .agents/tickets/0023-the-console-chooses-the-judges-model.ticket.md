# 0023: The console chooses the judge's model

**Spec:** None. The owner's direction of 2026-11-13: the model key and the LLM options belong to the console, where the team sets them on Settings → AI Models; nsx only ever holds a LangDrift secret.

**What to build:** A person reading nsx's code, its schemas' notes or its docs learns that the console's `/api/judge` answers with the team's own judge model and key, and ignores the `model` nsx sends. nsx keeps sending `model` while the console still requires it, and the console's secret page is named as it is now, Settings → Secrets.

**Blocked by:** None (can start immediately).

**Status:** completed

**Alignment:** change. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. Nothing the simulation shows changes here.

- [x] The judge client's comments say the team's setting on the console decides which model judges, and that `model` is deprecated: sent because the console still requires it, and ignored.
- [x] `schema/README.md` says the same of the judge request, which has no schema of its own.
- [x] The connection's comment names the console's secret page Settings → Secrets.
- [x] The request nsx posts is unchanged: the six fields, `model` among them.
- [x] `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, `cargo deny check`, `vtec/vtec.sh quality check` and `vtec/vtec.sh git lint-history origin/main HEAD` pass.

**Note, 2026-11-13:** nsx can stop sending `model` once the console accepts a request without it; until then a request without the field is refused, because the console reads the request strictly.

## Dev part

**Override:** on 2026-11-13 no part qualifies: the ticket rewrites comments and documentation, which holds no problem of the policy's categories (`core-domain`, `hard-part`, `new-stack`).
