# 0030: An end-to-end run plays nsx agent run

**Spec:** None. The owner's direction of 2026-11-14: test `nsx agent run` end to end without any user interaction, and see in the console that it works.

**What to build:** After the end-to-end plan of ticket 0029, the same command plays `nsx agent run` with nobody at the keyboard, to its last line, and checks that the console's record reads `done`. A scripted coding agent writes each stage's report and usage, so no coding call is paid for.

**Blocked by:** 0029, and the four answers below.

**Status:** needs-info

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. The end-to-end run prints the simulation's `nsx agent run` lines under its generated ticket.

What a real run needs, which the unit tests fake and no real machine has exercised yet:

1. **The team's Play.** A connected run waits silently for the team's Play before its header. Who presses it: the end-to-end run through the console's API, or a console setting that authorizes it ahead?
2. **A person's confirmations.** Planning and Review wait for a person at the keyboard. Does the end-to-end run answer them through standard input, or leave only stages without a person in the loop?
3. **Images.** The run brings five Deployments up and waits for each to be available. The fixture's components have no published image. Which images does the cluster run: ones built from the fixture, or small published stand-ins?
4. **Delivery.** The run pushes a branch and opens a pull request with `gh`. Against which forge: a throwaway GitHub repository, or a scripted `gh`?

- [ ] The run ends on its last simulation line with exit 0, and the console's record reads `done`.
- [ ] The coding agent is a script: no coding call is paid for.
- [ ] The run needs nobody at the keyboard and nobody at the console.

## Dev part

**Override:** on 2026-11-14 the dev part is not proposed yet: the ticket waits for the owner's four answers, which decide where its difficulty sits.
