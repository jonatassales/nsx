---
status: accepted
---

# Perfect Simulation Alignment

The session that plays on the home page of https://langdrift.md is the definition of what nsx does: `nsx agent plan <ticket>`, then `nsx agent run`, in a terminal beside the Ticket, Analysis, Plan and Workflow panel. nsx does exactly what that simulation shows, in the same order, with the same information. Doing more than it shows is wrong and is removed. Doing less is wrong, and what is missing becomes a ticket that builds it. Doing it differently is both: the difference is removed and the simulated behavior is built. Specs, tickets, ADRs and READMEs that disagree are stale. The simulation's script and copy live in the LangDrift website, in the `jonatassales/lang-drift` repository; the six goldens in `tests/simulation/golden/` hold its lines, and the golden test plays them.

The data matters more than the screen that shows it. The console is an interface that receives what nsx produces. What has to be right is what nsx and the client's `.langdrift/` folder produce and reason: every line of the terminal and every field of the four cards is data they produce, and the card is the specification of that data. The `console ›` prefix on the terminal's lines is part of the output, not a statement that the console does the reasoning.

nsx has three commands: `nsx init`, which creates the `.langdrift/` folder the other two need, `nsx agent plan` and `nsx agent run`. A human checkpoint that the simulation shows ("waiting planning · the owner confirms") is answered inside `nsx agent run`.

Each ticket's acceptance criteria quote the exact lines and fields it must produce, and every ticket carries the canonical rule: what is done beyond the simulation is removed, what is missing becomes a new ticket.

## Exceptions the owner decided

The simulation shows a terminal and a panel. The owner decided the following, which it does not show; each is allowed, and nothing beyond them is.

- **The console receives the records.** Whenever `nsx agent plan` or `nsx agent run` writes `plan.json`, `intent.json` or `run.json`, nsx sends the records written so far with the run's status, as one call of the MCP tool `langdrift.runtime.report` to the console's `/api/mcp`. The files stay the hand-off. `nsx init` records the connection: the secret from `LANGDRIFT_SECRET`, and the console from `MCP_CONSOLE_URL` or, without it, `https://langdrift.md`. None of this prints a line.
- **Play and hold.** A connected `nsx agent run` waits silently for the team's Play before its header, and holds silently before a stage's work while the team has paused it. Neither prints a line, so the terminal prints exactly the simulation's lines.

## Contradictions inside the simulation

Where the simulation contradicts itself, nsx follows it literally, and the contradiction stays open for the owner:

1. The Plan card reads "pending team authorization" after the run, whose header says "the recorded intent, authorized by the team".
2. The terminal's "read 4 documents" counts the ticket, the PRD, the DD and the SPEC, then reads the guardrail on its own line. The card's "4 documents read" lists the PRD, the DD, the SPEC and the guardrail.
3. `reads … FIN-02 · FIN-04` omits FIN-03. The next line, and the card, judge FIN-02, FIN-03 and FIN-04.

## Considered Options

- **Treating the specs as the truth and the simulation as marketing.** That is how code drifts: specs add gates, commands and checks the product never shows, and leave out what it shows, such as the dev ticket and Safe Seat lines.
- **Allowing extra behavior that does not contradict the simulation.** Every such allowance is a place where the product and its definition part ways. Information the product needs and the simulation lacks enters the simulation first, or becomes an exception the owner states here.

## Consequences

- A component is found by its manifest (`next.config.ts`, `Cargo.toml`, `package.json`, `nats.conf`, `redis.conf`), which gives its stack and role, and nsx resolves its image. No Dockerfile takes part in the plan.
- A failure the simulation does not show is a stop: no further line, so the terminal stays a prefix of the simulation, one sentence on standard error that carries no secret, and exit status 1. An over-budget run pauses at the gate, as the simulation shows, and exits 0.
