# Security Policy

## Reporting a vulnerability

Use either channel, whichever you prefer:

- **GitHub private vulnerability reporting**, from the
  [Security tab](https://github.com/jonatassales/nsx/security/advisories/new).
  It opens a private thread attached to this repository.
- **Email security@langdrift.md**, if you would rather not use GitHub.

Please do not open a public issue, pull request or discussion for a security
problem: a public report puts every nsx user at risk before a fix ships.

A useful report includes the version or commit you tested, what an attacker
can do and what access they need, and the steps to reproduce. A clear
description of the weakness is enough to open a case.

## Scope

### In scope

- **Budget boundaries**: a crafted ticket, document, judge answer or stage
  report that makes nsx skip the reservation, spend beyond the gate, or treat
  a recommendation as an authorization.
- **Execution bounds**: command injection, a path that escapes the project,
  or a process nsx starts with more of the environment than it names.
- **Secrets and evidence**: `LANGDRIFT_SECRET` or the recorded connection
  leaking into a line, a sentence on standard error, a record, the evidence or
  an episode.
- **The local cluster**: a run that removes or changes what its execution
  ledger does not own.
- **The npm package and its launcher** (`bin/nsx.sh`), and the JSON Schemas in
  `schema/`.

### Out of scope

- The LangDrift console and site, reported at
  [jonatassales/lang-drift](https://github.com/jonatassales/lang-drift/security/advisories/new).
- Third-party model providers, cloud vendors and the tools nsx drives (`kind`,
  `kubectl`, `git`, `gh`): report to them directly.
- Reports from automated scanners with no demonstrated attack path.
- Outdated dependencies with no reachable path in nsx; `cargo deny` tracks them.
