# 0029: An end-to-end run needs nobody at the keyboard

**Spec:** None. The owner's direction of 2026-11-14: an `e2e/` folder in nsx that tests a whole plan and run against a real console without any user interaction, with a generated ticket name each time, so that runs do not all read `0031 · book-a-demo-runtime`.

**What to build:** One command plays nsx end to end with nobody at the keyboard. It copies the fixture project into a fresh folder under a generated ticket number and slug prefixed `e2e-`, connects it to the console named in the environment, runs `nsx agent plan` and `nsx agent run` with a scripted coding agent, and checks the console's record of the runtime. It runs on demand, never in the default test run.

**Blocked by:** 0026 and 0027.

**Status:** ready-for-agent

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. The end-to-end run prints the simulation's lines with its own ticket's number and slug in place of `0031` and `book-a-demo-runtime`; the unit goldens keep the simulation's ticket.

- [ ] `e2e/` holds the run and a README that says what it needs: `MCP_CONSOLE_URL`, `LANGDRIFT_SECRET`, Docker, and a console whose team saved its Anthropic key and offer.
- [ ] Without `MCP_CONSOLE_URL` or `LANGDRIFT_SECRET` the run stops before anything else with one sentence: no console is a default.
- [ ] Each run generates its ticket number and slug with the `fake` crate; the slug starts with `e2e-`; the fixture's ticket, its links and its runtime id are rewritten to them.
- [ ] The coding agent is a script that writes each stage's report and usage: no coding call is paid for. The judge's calls are real.
- [ ] The run asserts that the plan ends on its `handoff` line, that the run ends on its last line with exit 0, and that the console's record of the runtime reads `done`.
- [ ] It runs with `cargo test --test e2e -- --ignored`, and `cargo test --locked` does not run it.
- [ ] The repository's checks pass.

## Dev part

**Category:** `hard-part`: renaming the fixture's ticket everywhere it is named, so that a generated name plans and runs as the simulation's does.

**Paths:** `e2e/project/rename.rs`.

**Boundary:** `rename(project: &Path, ticket: &Generated) -> std::io::Result<()>`, which the end-to-end run calls on its copy of the fixture.

**Goal:** a copy of the fixture whose ticket has the generated number and slug in its file name, its header and its runtime id.

- [ ] `nsx agent plan <generated ticket>` resolves the renamed ticket and its links.

**Override:** on 2026-11-14 the owner asked the agent to write the whole change, dev parts included: "Yes, write it all".
