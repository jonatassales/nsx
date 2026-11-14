# 0024: nsx reads the team's offer and suggests and costs only within it

**Spec:** None. The owner's direction of 2026-11-13: the team enters, per provider, the models and the efforts it offers on the console's Settings → AI Models, and the judge works only within what the account offers.

**What to build:** A connected `nsx agent plan` reads the team's offer from the console before its first line. The judge suggests, and the Workflow card prices, only the models of the price list the team offers, each at the efforts nsx knows that the team offers for it. The suggestion is the best deal among them, as the simulation shows.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

**Alignment:** change. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. The simulation's team offers the three models at low, medium and high, so every line and card it shows is unchanged.

- [ ] `schema/offer/offer.schema.json` is generated from a contract type: per provider (`anthropic`, `openai` or `gemini`) its models and its efforts, non-empty strings, each list without repeats, no other field. A sample sits under `tests/contract/samples/offer/`, and `@langdrift/nsx` exports `./offer/*`.
- [ ] The console answers the offer through a second MCP tool, `langdrift.account.offer`, called with no arguments, on the same endpoint, bearer and 4-second patience as the report tool.
- [ ] A connected plan reads the offer before the spend table is built. The candidates are the price list's models the offer names, each at the efforts nsx knows that its provider offers; a model offered at no such effort is no candidate. A project with no connection plans with every model and effort, as before.
- [ ] A failed read, or an offer that leaves no candidate model or no known effort, stops the plan before its first line, with a sentence that names the console's Settings → AI Models.
- [ ] The workflow's default choice must be a candidate, or the plan stops before its first line.
- [ ] The suggestion question lists only the candidates; the answer must name a candidate model at one of its efforts, and its reasons weigh the candidates not chosen, however many there are.
- [ ] The Workflow card's spend rows are the candidate models, each at its efforts.
- [ ] The goldens are unchanged.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, `cargo deny check`, `vtec/vtec.sh quality check` and `vtec/vtec.sh git lint-history origin/main HEAD` pass.

## Dev part

**Override:** on 2026-11-13 no part qualifies: the ticket narrows existing choices to a list the console sends, with no problem of the policy's categories (`core-domain`, `hard-part`, `new-stack`).
