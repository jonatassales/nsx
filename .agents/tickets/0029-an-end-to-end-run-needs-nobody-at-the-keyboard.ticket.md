# 0029: An end-to-end plan needs nobody at the keyboard

**Spec:** None. The owner's direction of 2026-11-14: an end-to-end folder in nsx that tests against a real console without any user interaction, with a generated ticket name each time, so that runs do not all read `0031 · book-a-demo-runtime`.

**What to build:** One command plays `nsx agent plan` end to end with nobody at the keyboard. It copies the fixture project into a fresh folder under a generated ticket number and a slug prefixed `e2e-`, connects it to the console named in the environment with the real `nsx init`, plans the ticket with the real binary, and checks the console's record of the runtime. It runs on demand, never in the default test run. `nsx agent run` end to end is ticket 0030.

**Blocked by:** 0026 and 0027.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. The end-to-end plan prints the simulation's lines with its own ticket's id in `console › tool call: read <ticket>`; the unit goldens keep the simulation's ticket.

- [x] `tests/e2e/` holds the runs and a README that says what they need: `MCP_CONSOLE_URL` and `LANGDRIFT_SECRET`, and what the console's team must have saved. It is a folder of the one test target, as the testing rules require.
- [x] Without `MCP_CONSOLE_URL` or `LANGDRIFT_SECRET` a run stops before anything else with one sentence: no console is a default.
- [x] Each run generates its ticket number and slug with the `fake` crate; the slug starts with `e2e-`; the fixture's ticket file is renamed to them, which is the only place the fixture names its ticket.
- [x] With the team's key saved, the plan ends on `# next: once the team authorizes, nsx agent run applies the recorded intent` with exit 0, and the console's record reads `plan`, proposed, under the generated ticket. The judge's calls are real.
- [x] Without the team's key, the plan ends on the simulation's two stop lines with exit 1, writes no draft, and the console's record reads `error` with the judge unavailable.
- [x] The runs are ignored by default: `cargo test --locked` does not run them.
- [x] The repository's checks pass.

**Verified, 2026-11-14:** the run without the key passed against `https://console.langdrift.md`, which listed the runtime as `8971 · e2e-et-ipsa-runtime`. The run with the key was not played: the console's team had saved no Anthropic key.

## Dev part

**Override:** on 2026-11-14 no part qualifies: the ticket renames one file and reads back a record, with no problem of the policy's categories (`core-domain`, `hard-part`, `new-stack`).
