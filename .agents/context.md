# Context

Project knowledge for agents working in this repository. [`AGENTS.md`](/AGENTS.md) holds the ideas; this file holds the mechanics and the vocabulary.

## Repository

A single Rust crate, `nsx`, at the root, built with Cargo.

- `src/`: the crate, one folder per part of the session; [`README.md`](/README.md) maps every file.
- `tests/`: one test target, `tests/main.rs`, with a folder per area, the fakes, the fixture project and the six goldens.
- `schema/`: the JSON Schemas of the records nsx writes and sends, published with the npm package; the LangDrift console validates reports against them.
- `langdrift/`: the files `nsx init` lays down in a client's `.langdrift/`, embedded in the binary.
- `bin/nsx.sh`: the POSIX launcher the npm package runs, which picks the binary for the machine.
- `package.json`: the npm package `@langdrift/nsx`, nothing more; publishing it is the owner's.
- `drift/`: this repository's own POSIX shell tooling, the commit guardrails, the quality check and the pull request widgets. It never ships.
- `.github/`: the workflows, the composite action and the widgets.

## Toolchain and commands

- `rust-toolchain.toml` pins the Rust version, with `clippy` and `rustfmt`. `rustfmt.toml` formats with tabs two wide, 80 columns and one struct field per line.
- Every command runs from the repository root, with `--locked`.

| Command | Effect |
| --- | --- |
| `cargo build --release --locked` | Builds `target/release/nsx`. |
| `cargo test --locked` | Runs every test, the six goldens included. |
| `cargo clippy --all-targets --locked -- -D warnings` | Lints the crate and its tests. |
| `cargo fmt --check` | Checks the formatting. |
| `cargo deny check` | Checks licences, advisories and banned crates against `deny.toml`. |
| `drift/drift.sh git setup` | Points Git at the commit-msg hook in `drift/hooks/`. |
| `drift/drift.sh quality check` | Lists each path that does not apply a rule file. |
| `drift/drift.sh widget dashboard` | Prints the pull request's CI Gates dashboard. |

## Spec-driven development

Local Markdown under `.agents/`:

| Artifact | Path |
| --- | --- |
| Spec | `specs/<NNNN>-<slug>.spec.md` |
| Ticket | `tickets/<NNNN>-<slug>.ticket.md` |
| PRD | `plan/prd/<NNN>-<slug>.prd.md` |
| ADR | `plan/adr/<NNN>-<slug>.adr.md` |
| Research note | `plan/research/<NNN>-<slug>.research.md` |

- Each kind numbers its own sequence without gaps: specs and tickets from `0001`; PRDs, ADRs and research notes from `001`. A new record takes the number after the highest one in its folder.
- A ticket names its spec in a `**Spec:**` line, and its blockers in `**Blocked by:**`.
- A `**Status:**` line holds one of `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `completed` or `wontfix`.
- A commit touching `src/`, `tests/`, `schema/`, `langdrift/` or `bin/` names its ticket in a `Ticket: <NNNN>` trailer; the commit-msg hook and CI check it.

## Continuous integration

| File | Runs on | Does |
| --- | --- | --- |
| `workflows/pull-request.yml` | a pull request into `main` | `validate.yml`, then Harness - Guardrails, Harness - Score and Delivery - Dashboard |
| `workflows/main.yml` | a push to `main` | `validate.yml`, then the `nsx` binaries for Linux and macOS on x64 and arm64, attested and kept as a workflow artifact for 30 days |
| `workflows/validate.yml` | a call from the two above | Check, Test (Ubuntu and macOS) and Security |
| `actions/setup-workspace/` | a step | installs the toolchain `rust-toolchain.toml` names and restores the cargo cache |
| `widgets/` | a step, through `drift widget` | the pull request widgets and the two ways they are published |

Actions are pinned to full commit SHAs with the version in a comment, and the workflow tokens are read-only unless a job asks for more.

## Glossary

**LangDrift CLI**:
`@langdrift/nsx`, which plans, runs and records microruntimes from a team's terminal. Its command is `nsx`.
_Avoid_: the agent (for the whole CLI)

**nsx**:
The LangDrift CLI's command, as a person types it: `nsx init`, `nsx agent plan`, `nsx agent run`.
_Avoid_: drift

**drift**:
This repository's own tooling, the POSIX shell CLI in `drift/` that runs the guardrails, the quality check and the widgets. It never ships.
_Avoid_: nsx

**Microruntime**:
One bounded workload around a team's own software that `nsx agent plan` defines from a ticket in `runtime.md` and `nsx agent run` brings up on the project's local cluster and removes again.
_Avoid_: Runtime (alone, in new writing), service

**Connected agent**:
`nsx agent plan` or `nsx agent run` in a project whose connection `nsx init` recorded: it reports each record to the console and honours the team's Play and Pause while it runs.
_Avoid_: Daemon, worker

**Run**:
One `nsx agent run`: the authorized intent played through the workflow's stages, from the gate to the pull request and the release of what was not spent, recorded in `run.json`.
_Avoid_: Job, session

**Execution ledger**:
The on-disk record of what an operation created, written before side effects and settled afterwards. Cleanup acts only on what the ledger owns.
_Avoid_: State file, journal

**Evidence**:
The token-free record nsx keeps of every lifecycle operation and inspection, for later audit.
_Avoid_: Log, report

**Episode**:
One earlier run of the project as the retrieval remembers it, with its stacks, component paths, measured peaks and tokens, embedded for retrieval when a new ticket is planned.
_Avoid_: Example, case, memory (for the whole)

**Judge**:
The model the plan asks, through the console, to keep or discard each source, weigh the episodes and suggest a model and effort. Its calls are paid work under the money gates; a plan with no connection stops at its first question.
_Avoid_: Planner model, LLM, evaluator

**Stage driver**:
The team's coding agent as nsx runs it for a stage: a trusted process started from `harness/driver.json` with its arguments and environment, whose usage nsx reads back.
_Avoid_: Agent runner, executor, worker
