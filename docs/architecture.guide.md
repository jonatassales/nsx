# nsx architecture

nsx is the LangDrift CLI, written in Rust. Its structure answers the question ADR 001 asks of the product: **where does each line of the simulation live?** Every file under `src/` opens with a preface that says what it holds. A file not built yet holds only its preface.

## The design

1. **A folder under `src/` owns one part of the session.** `command` holds the three commands, `session` how a line prints and how a command stops, and `contract` every shape a file or the console sees. `files` and `process` are the machinery the others share. Every other folder is one element of the simulation: `request`, `harness`, `rag`, `judgment`, `components`, `runtime`, `handoff`, `money`, `flow`, `driver`, `skills`, `hook`, `seat`, `evidence`, `local`, `delivery`, `checks` and `console`.
2. **A file is a type or a function, never just a word.** A word that is a variant or a field lives inside its type: the four kinds of line are `session/kind.rs`, and "paused" is an outcome of `money/gate.rs`.
3. **One pattern everywhere.** `gate.rs` holds a definition. `gate/open.rs` holds one function of it. That keeps one function per file, definitions apart from implementations, one-word names and the two-folder limit, all at once. No type sits inside another type's folder.
4. **`line.rs` is the line.** Under a type, `line.rs` holds its method `line`: the line of the simulation that type owns. A type with several lines has one method, and so one file, per line (`suggestion/effort.rs`).
5. **The two flows are scripts.** `command/plan/execute.rs` and `command/run/execute.rs` read top to bottom as the simulation plays, each step calling the element that owns the line.
6. **`contract/` holds every shape a file or the console sees.** The elements fill their part of it, and the schemas publish from one place. A name can appear in `contract/` and in an element: `contract/criteria.rs` is the answer as it is written down, `judgment/criteria.rs` is the asking.
7. **The outside world sits behind a trait**, in the element that owns it: the judge, the image resolver, the coding agent, the cluster, the forge and the console. The tests replace each with a fake, which is what lets the golden test run with no network (Rust rules, item 1).
8. **A folder never repeats its name in a file**: `request/ticket.rs`, not `ticket/ticket.rs`.
9. **Every folder directly under `src/` has a `mod.rs`.** A type's folder needs none: the type's own file declares what is in it. Every name is one lowercase word that is not a Rust keyword.

## What it costs

- **More folders.** There are 23. Three are small (`hook`, `checks`, `skills`) and could fold into `flow`.
- **Some names repeat across folders.** `plan.rs` is a command and a card; `report.rs` is a stage's report, the console's report and its shape; `criteria.rs`, `effort.rs` and `kind.rs` each appear twice. The folder tells them apart, and a search for the bare name does not.
- **One change can touch several folders.** A new field on a card touches its shape in `contract/`, the element that fills it and the line that prints it.
- **Three names are open:** `contract`, `process` and `files`.

## Left out on purpose

- **Errors**: an element defines its own error when it has its first failure. `session/stop.rs` prints any of them as one sentence.
- **The goldens and the fixture project**: `tests/simulation/golden/` holds the six goldens, the plan and five runs (`run`, `run-all-stages`, `run-over-budget`, `run-to-merge` and `run-haiku-low`), each line as its id, its kind and its text, a tab between them. `tests/simulation/project/` holds the project they play on: its ticket and linked documents under `.agents/`, its guardrail and episodes under `.langdrift/`, and its five components. The golden test plays them.
- **Speed, size and memory**: they are not criteria for nsx. No choice is made or refused for how fast nsx runs, how large its binary is or how much memory it uses.
- **The files `nsx init` lays down and the published schemas**: `langdrift/` and `schema/` each hold them beside a README that lists them.

## Prefaces

Every file under `src/` opens with a preface; a file not built yet holds nothing else. A preface is a `//!` comment with the same shape everywhere:

- a title: what the file holds;
- what it is responsible for, in a few plain sentences;
- whether it is a type's definition or one function;
- `## Simulation`: the lines of the goldens it owns, quoted as the golden has them, and its card fields;
- `## Reads and writes`: the files, programs, calls and environment variables it touches;
- `## Rules`: the constraints that bind it;
- `## Open`: a question still open that the file will have to answer.

A section appears only when it has something true to say. A preface states nothing the simulation or the code does not bear out, and holds no code block, so Cargo has no doc test to run (Rust rules, item 1).

## Open questions

Each question nsx has not settled is recorded under `## Open` in the file it touches, among them how often the checks are polled (`checks/suite.rs`), what an unknown command does (`command/cli.rs`), which folder wins when a project has both `.agents/` and `.claude/` (`request/ticket/resolve.rs`), whether `nsx init` writes a working `driver.json` (`driver/config.rs`) and who delivers the pull request (`delivery/mod.rs`).

## Files

Each table lists a folder's files, what each holds and the lines of the goldens it owns, by the id in the golden's first column. `mod.rs` is in every folder and not listed.

### command

| Files | Holds | Lines |
| --- | --- | --- |
| `cli.rs`, `cli/execute.rs` | the parser: `init` and `agent` | |
| `init.rs`, `init/execute.rs` | `nsx init`: creates `.langdrift/` with what the plan and the run need, prints nothing | |
| `agent.rs`, `agent/execute.rs` | the `agent` family: `plan` and `run` | |
| `brief.rs` | what the plan has read before its first line | |
| `resolved.rs` | what the plan act hands on: the inventory, its sizing and the draft's revision | |
| `desk.rs`, `desk/record.rs` | the writer of `plan.json` and the reports to the console, held together so no write goes untold | |
| `session.rs` | what a command needs from outside itself: the working directory, the sink, the window, the judge, the image resolver, the coding agent, the cluster, the forge, the pace of the checks' poll and the person's interrupt | |
| `plan.rs`, `plan/execute.rs` | `nsx agent plan <ticket>`, as its steps in order | `scope`, `handoff` |
| `plan/analyze.rs` | the analysis act: documents, guardrail, episodes, suggestion | `analysis` |
| `plan/resolve.rs` | the plan act: components, sizing | `planning` |
| `plan/propose.rs` | the end of the plan act: validate, write the draft, propose | |
| `run.rs`, `run/execute.rs`, `run/play.rs` | `nsx agent run`, as its steps in order: the opening, then the acts | |
| `applied.rs` | what a run applies: the authorized intent, the plan's cards and the run's id | |
| `run/reserve.rs` | the money act: estimate, budget, gate | |
| `run/authorize.rs` | waiting for the team's authorization of a proposed intent in a connected project | |
| `run/prepare.rs` | what a run needs before its first stage: the driver configuration, the linked spec, a usage record that reads, its private state kept out of Git, what earlier runs left removed, and the run's branch | |
| `run/stages.rs`, `run/call.rs`, `run/person.rs` | the stages, each with its checkpoint; one stage's paid call; the dev ticket | |
| `run/settle.rs` | the implementation's money: observed, of what was reserved | |
| `run/remember.rs` | the episode a completed run leaves | |
| `work.rs` | what the acts of a run share: the driver configuration, the linked spec, the usage record, the effort record and the run's branch | |
| `run/deliver.rs`, `run/carry.rs` | the delivery act: the run's execution ledger opened, then the act's lines in order | `next` |
| `run/host.rs`, `run/ship.rs`, `run/follow.rs` | the act's steps: the runtime brought up; the branch pushed and the pull request opened; the checks followed and Safe Seat stated | |
| `run/halt.rs` | on any stop, an interrupt included: what this run brought up taken down, with no line, and the record's runtime marked stopped | |

### session

| Files | Holds | Lines |
| --- | --- | --- |
| `line.rs`, `line/render.rs`, `line/comment.rs` | one line: its id, its kind and its shape | |
| `kind.rs` | the four kinds: comment, output, success, warning | |
| `shape.rs` | the six ways a line is laid out | |
| `sink.rs` | the trait every line is printed through | |
| `window.rs` | the trait the window's title is set through | |
| `terminal.rs`, `terminal/print.rs`, `terminal/title.rs` | the sink and window over the terminal; a line or a title that carries a control character is a stop, never printed | |
| `title.rs`, `title/of.rs` | the window title, named after the project's folder | |
| `interrupt.rs`, `signal.rs`, `signal/watch.rs`, `signal/wait.rs` | the interrupt, as a trait, and as the system sends it: Ctrl-C, a request to end or the terminal closing, watched from the start of the command | |
| `stop.rs`, `stop/exit.rs`, `stop/from.rs`, `stop/usage.rs`, `stop/interrupted.rs` | the stop rule: one sentence on stderr, then exit 1, for any folder's failure, for a command line nsx does not accept and for an interrupt | |

### contract

| Files | Holds | Lines |
| --- | --- | --- |
| `fence.rs`, `fence/parse.rs`, `fence/validate.rs` | the `runtime.md` fence | |
| `component.rs` | one component of the fence | |
| `problem.rs`, `problem/at.rs` | one problem a validation found: the field and the rule | |
| `plain.rs`, `label.rs` | what a printed text must be, one plain line; and what a runtime's id must be, a lowercase DNS label | |
| `error.rs` | what reading a shape can refuse | |
| `decode.rs`, `encode.rs` | strict decoding: an unknown field is refused; and a record written as tab-indented JSON | |
| `reference.rs` | the one rule for an image reference | |
| `schema.rs` | the JSON Schemas, generated from these types | |
| `nullable.rs` | the schema of a required field that may be `null` | |
| `empty.rs` | a section whose fields have not joined yet | |
| `cards.rs`, `cards/documents.rs`, `cards/episodes.rs`, `cards/suggestion.rs`, `cards/components.rs`, `cards/sizing.rs` | `plan.json`: the four cards, and finding a section of the Analysis or the Plan card for the folder that fills it | |
| `ticket.rs`, `linked.rs`, `analysis.rs`, `plan.rs`, `workflow.rs` | the Ticket card with its linked artifacts, and the Analysis, Plan and Workflow cards | |
| `documents.rs`, `product.rs`, `design.rs`, `specified.rs`, `guardrail.rs`, `envelope.rs`, `rule.rs` | the Analysis card's DOCUMENTS: its counts and one row per linked document | |
| `retrieval.rs`, `candidate.rs` | the Analysis card's EPISODES: its counts and one row per candidate | |
| `recommendation.rs` | the Analysis card's suggested model and effort | |
| `components.rs`, `resolved.rs` | the Plan card's COMPONENTS: one row per component, and the cross-check's counts | |
| `figures.rs`, `effort.rs`, `seat.rs` | `run.json`: a stage's figures, the effort record and the Safe Seat section | |
| `sizing.rs`, `bound.rs`, `amount.rs`, `resource.rs`, `validation.rs`, `source.rs` | the Plan card's sizing (the bound, the proposed totals, the headroom, the resource, the limits requested and the validation) and its price source | |
| `pair.rs` | one row of the Workflow card's spend table | |
| `authorization.rs`, `evidence.rs` | the Plan card's AUTHORIZATION and EVIDENCE | |
| `intent.rs`, `intent/validate.rs`, `human.rs` | `intent.json`: what the team chose, proposed or authorized, and who takes part in each stage | |
| `run.rs`, `step.rs`, `hosted.rs`, `delivery.rs`, `pull.rs`, `checks.rs`, `outcome.rs` | `run.json`: its stages, its runtime, its delivery with the pull request and its checks, and how a run ended | |
| `judged.rs`, `used.rs`, `cost.rs` | the console judge's response to one question: the answer or a refusal, what it used and what it cost | |
| `report.rs`, `status.rs`, `reply.rs`, `requested.rs`, `asked.rs`, `choice.rs`, `choice/of.rs` | the report to the console, its status, and the console's reply with the team's pending request and the choice a Play authorizes | |
| `criteria.rs`, `criteria/check.rs`, `criterion.rs`, `episodes.rs`, `episodes/check.rs`, `episode.rs`, `suggestion.rs`, `suggestion/check.rs` | the judge's three answers, each read strictly and checked by its own rules | |
| `stage.rs` | the stages, as the records spell them | |
| `planning.rs`, `spec.rs`, `tickets.rs`, `implement.rs`, `review.rs` | the five stage reports | |

### handoff

| Files | Holds | Lines |
| --- | --- | --- |
| `writer.rs`, `writer/write.rs` | the one writer of `plan.json` | |
| `error.rs` | what the hand-off can refuse | |
| `proposal.rs`, `proposal/write.rs`, `proposal/line.rs`, `proposal/card.rs` | the proposed intent, written to `intent.json`, and the Plan card's authorization and evidence | `draft` |
| `authorization.rs`, `authorization/choose.rs`, `authorization/check.rs`, `authorization/grant.rs` | the intent the team authorized, chosen among the recorded ones, written authorized after the team's Play, and checked before a run | |
| `authorization/scope.rs` | the run's opening comment | `scope` (run) |
| `authorization/line.rs` | the intent's line | `intent` |

### evidence

| Files | Holds | Lines |
| --- | --- | --- |
| `clock.rs`, `watch.rs`, `watch/now.rs` | the time, as a trait, and the machine's own clock | |
| `id.rs`, `id/mint.rs`, `id/text.rs` | the run's id | |
| `error.rs` | what keeping a run's records can refuse | |
| `record.rs`, `record/write.rs`, `record/stage.rs` | the one writer of `run.json`, which tells the console each write, and where a stage stands in it | |
| `usage.rs`, `usage/kept.rs`, `usage/write.rs`, `usage/line.rs`, `used.rs` | the usage record: the tokens each paid call of a run used, and their totals | `completed` |

### request

| Files | Holds | Lines |
| --- | --- | --- |
| `ticket.rs`, `ticket/resolve.rs`, `ticket/parse.rs`, `ticket/line.rs`, `ticket/card.rs`, `ticket/intent.rs` | the ticket, found from its bare id, its card and its intent | `readTicket` |
| `links.rs`, `links/parse.rs`, `links/line.rs`, `links/card.rs` | the ticket's four links, and their place on the Ticket card | `links` |
| `link.rs`, `link/locate.rs`, `kind.rs` | one link: its kind, its file name, its short id and where it points; and the four kinds of linked document | |
| `prd.rs`, `prd/read.rs`, `prd/line.rs`, `prd/card.rs` | the PRD and its use case | `readPrd` |
| `design.rs`, `design/read.rs`, `design/line.rs`, `design/card.rs` | the design doc, with the envelope and the budget per run | `readDesignDoc` |
| `spec.rs`, `spec/read.rs`, `spec/line.rs`, `spec/card.rs`, `named.rs` | the spec, with each component it names and its count of acceptance criteria | `readSpec` |
| `documents.rs` | those three documents together, as the judge reads them | |
| `field.rs` | the value of a document's `**Field:** value` line | |
| `reading.rs`, `reading/count.rs`, `reading/line.rs`, `reading/card.rs` | what was read, and nothing outside the links | `reads` |
| `error.rs` | what reading the ticket, its links and its documents can refuse | |

### harness

| Files | Holds | Lines |
| --- | --- | --- |
| `guardrail.rs`, `guardrail/load.rs`, `guardrail/line.rs`, `guardrail/card.rs` | the guardrail file, and its row on the Analysis card | `guardrail` |
| `criterion.rs` | one criterion: its id, its rule and where it is checked | |
| `envelope.rs`, `envelope/of.rs`, `envelope/line.rs`, `envelope/card.rs` | the bound on cpu and memory, taken from the design doc, as the plan states it and as the Plan card shows it | `bounds` |
| `error.rs` | what reading the harness can refuse | |

### money

| Files | Holds | Lines |
| --- | --- | --- |
| `cents.rs`, `cents/format.rs`, `cents/parse.rs` | money, in whole cents: printed as dollars, and read from a dollar amount a team wrote | |
| `error.rs` | what reading the money files can refuse | |
| `price.rs`, `price/load.rs`, `price/line.rs`, `price/card.rs`, `model.rs` | the price list and one of its models, and the Plan card's price source | `price` |
| `forecast.rs`, `forecast/load.rs`, `tokens.rs` | the tokens a run is expected to use, per effort | |
| `budget.rs`, `budget/of.rs`, `budget/line.rs`, `budget/cite.rs` | the budget per run, taken from the design doc, as the plan states it and as the run cites it | `budget` (plan and run) |
| `estimate.rs`, `estimate/compute.rs`, `estimate/line.rs` | the estimate | `estimate` |
| `reservation.rs`, `reservation/total.rs` | the estimate plus its margin | |
| `gate.rs`, `gate/open.rs`, `gate/line.rs` | the gate: opened or paused | `gateOpened`, `gatePaused`, `paused` |
| `spend.rs`, `spend/build.rs`, `spend/card.rs`, `pair.rs` | the spend table: one row per model and effort, priced before the first line and put on the Workflow card | |
| `ledger.rs`, `ledger/open.rs`, `ledger/write.rs` | the money ledger: a run's entry, opened from the plan's spend table with its figures checked, and written as the run goes | |
| `settlement.rs`, `settlement/settle.rs` | observed and released | the amounts in `completed` and `ready` |

### rag

| Files | Holds | Lines |
| --- | --- | --- |
| `embedder.rs`, `embedder/embed.rs` | turning text into a vector, on the machine: a plain type, run for real in the tests | |
| `index.rs`, `index/load.rs`, `index/current.rs`, `index/search.rs`, `index/rebuild.rs`, `index/write.rs`, `entry.rs` | the index of episodes and one episode in it | |
| `candidate.rs` | an episode and its similarity | |
| `threshold.rs`, `threshold/card.rs` | how similar is similar enough, shown on the card from the start | |
| `episode.rs`, `episode/write.rs`, `episode/number.rs`, `episode/filed.rs`, `episode/recorded.rs`, `episode/load.rs`, `part.rs`, `resource.rs`, `measured.rs` | one past run, with the components it ran, its workload and what it measured | |
| `error.rs` | what the retrieval can refuse | |
| `retrieval.rs`, `retrieval/retrieve.rs` | the search and what it kept | |
| `retrieval/rows.rs`, `retrieval/verdicts.rs`, `retrieval/citations.rs` | the EPISODES of the Analysis card, each part written when its line prints | |
| `retrieval/memory.rs`, `retrieval/embedding.rs`, `retrieval/search.rs`, `retrieval/candidates.rs`, `retrieval/dropped.rs`, `retrieval/cited.rs` | its six lines | `memory`, `embed`, `vectorSearch`, `candidates`, `drop`, `episodes` |

### judgment

| Files | Holds | Lines |
| --- | --- | --- |
| `judge.rs` | the judge, as a trait with one function | |
| `question.rs`, `question/name.rs`, `question/put.rs`, `question/prompt.rs` | one question for the judge, with everything the judge gets to answer it; putting it and reading the answer strictly; and the question in nsx's own words | |
| `absent.rs`, `absent/ask.rs` | no judge: every question is refused, and the plan stops there | |
| `error.rs`, `error/refused.rs` | what asking the judge can refuse | |
| `client.rs`, `client/open.rs`, `client/ask.rs` | the console's judge, reached over HTTP with the recorded connection | |
| `criteria.rs`, `criteria/ask.rs`, `criteria/line.rs`, `criteria/card.rs` | judging the guardrail's criteria | `judgeGuardrail` |
| `relevance.rs`, `relevance/ask.rs`, `relevance/explain.rs`, `relevance/line.rs` | judging which episodes are relevant, and giving each candidate its reason | `judgeRelevance` |
| `suggestion.rs`, `suggestion/ask.rs`, `suggestion/card.rs` | the suggested model and effort | |
| `suggestion/resources.rs`, `suggestion/effort.rs`, `suggestion/model.rs`, `suggestion/line.rs` | its four lines | `reasonResources`, `weighEffort`, `weighModel`, `suggest` |

### components

| Files | Holds | Lines |
| --- | --- | --- |
| `component.rs`, `component/inspect.rs`, `component/card.rs` | one component: path, manifest, stack, role, port, image and share, and its row on the Plan card | |
| `component/line.rs`, `component/row.rs` | its two lines | `inspectRust`, `inspectNode`, `inspectNats`, `inspectRedis`, `inspectNext`; `rust`, `node`, `nats`, `redis`, `next` |
| `manifest.rs`, `manifest/find.rs`, `manifest/file.rs`, `manifest/classify.rs`, `manifest/port.rs`, `manifest/code.rs`, `manifest/keyed.rs`, `manifest/scripted.rs` | the file that says what a component is: found, named, classified, and read for the port it declares, in a configuration file's key (its comments, quotes and blocks read first) or a `package.json` start script | |
| `stack.rs`, `role.rs` | the stacks, each with its line ids and its usual port, and the roles | |
| `error.rs` | what resolving the components can refuse | |
| `image.rs`, `image/resolve.rs` | the image a component runs, asked of the resolver and checked | |
| `resolver.rs`, `derived.rs`, `derived/resolve.rs` | the image resolver's trait, and the one nsx plans with: `<runtime id>/<folder>:r<revision>`, not verified | |
| `inventory.rs`, `inventory/line.rs`, `inventory/card.rs` | the components the spec names, and their placeholder rows on the Plan card | `resolve` |
| `collisions.rs`, `collisions/check.rs`, `collisions/line.rs`, `collisions/card.rs` | the cross-check of ports, images and paths, and its counts on the Plan card | `crossCheck` |

### runtime

| Files | Holds | Lines |
| --- | --- | --- |
| `sizing.rs`, `sizing/sum.rs`, `sizing/fit.rs`, `sizing/line.rs`, `sizing/proposed.rs`, `sizing/card.rs` | the summed shares and the headroom: the totals on the Plan card at each row, the fit inside the bound before the line, the headroom and the resource at the line | `size` |
| `limits.rs`, `limits/line.rs`, `limits/card.rs` | what `runtime.md` does not carry | `limits` |
| `revision.rs`, `revision/next.rs` | the revision of a plan: 1, or the earlier draft's plus one | |
| `draft.rs`, `draft/of.rs`, `draft/validate.rs`, `draft/line.rs`, `draft/card.rs`, `draft/write.rs` | the `runtime.md` draft: made from the plan, validated, and written as a title, the fence and the sizing report | `validate` |
| `error.rs` | what sizing the plan and writing its draft can refuse | |

### flow

| Files | Holds | Lines |
| --- | --- | --- |
| `workflow.rs`, `workflow/load.rs`, `entry.rs` | the ordered stages as the workflow file lists them, and the default choice | |
| `error.rs` | what reading the workflow can refuse | |
| `stage.rs` | one stage of this run and where it stands | |
| `choice.rs` | the choice per run: checkpoints, model, effort | |
| `reader.rs`, `keyboard.rs`, `keyboard/line.rs` | where a person's answer is read: the run's standard input, behind a trait | |
| `checkpoint.rs`, `checkpoint/wait.rs`, `checkpoint/confirm.rs`, `checkpoint/line.rs` | a stage a person confirms | `waiting-planning`, `confirmed-planning`; when the stage is checked, `waiting-spec`, `confirmed-spec`, `waiting-tickets`, `confirmed-tickets`, `waiting-implement`, `confirmed-implement` |
| `ending.rs`, `ending/line.rs` | how the run ends | `next` (run) |

### driver

| Files | Holds | Lines |
| --- | --- | --- |
| `agent.rs` | the coding agent, as a trait | |
| `launcher.rs`, `launcher/start.rs` | the coding agent started on this machine: the configured executable, its arguments filled, a small explicit environment | |
| `config.rs`, `config/load.rs`, `paths.rs` | `driver.json`, and where the agent's output holds its tokens | |
| `call.rs`, `call/prompt.rs`, `call/run.rs`, `call/log.rs`, `ended.rs`, `ended/accept.rs` | one call per stage, with the model and the effort; its words, its log, how it ended and whether that gives the stage a line | |
| `tokens.rs`, `tokens/read.rs` | the tokens a call used | |
| `error.rs` | what a stage call can refuse | |

### skills

| Files | Holds | Lines |
| --- | --- | --- |
| `skill.rs`, `skill/install.rs` | the skill a stage follows, laid down by `nsx init` | |
| `report.rs`, `report/check.rs`, `report/line.rs` | a stage's checked report, as one line | `grilling`, `toSpec`, `toTickets`, `implement`, `codeReview` |
| `error.rs` | what a stage's report can be refused for | |

### hook

| Files | Holds | Lines |
| --- | --- | --- |
| `effort.rs`, `effort/record.rs`, `effort/permit.rs`, `effort/line.rs` | the effort, recorded before implement may start, and the gate that asks for it | `hook` |
| `error.rs` | what the effort hook can refuse | |

### seat

| Files | Holds | Lines |
| --- | --- | --- |
| `part.rs`, `part/wait.rs`, `part/close.rs`, `part/of.rs`, `part/line.rs` | the part a person writes, and the same part as the run's record keeps it | `devTicketWaiting`, `devTicketClosed` |
| `verdict.rs`, `verdict/reach.rs`, `verdict/line.rs`, `verdict/card.rs` | whether Safe Seat was met, and its section of `run.json` | `safeSeatMet` |
| `error.rs` | what the dev ticket can refuse | |

### local

| Files | Holds | Lines |
| --- | --- | --- |
| `cluster.rs` | the cluster, as a trait: a space for the run, its Deployments, and the space's absence | |
| `kind.rs`, `kind/cluster.rs` | the machine's shared kind cluster, with a namespace per run | |
| `kind/reach.rs`, `kind/kubeconfig.rs`, `kind/kubectl.rs` | whether the shared cluster exists, with the project's own kubeconfig pointed at it; where that kubeconfig is; a kubectl call aimed through it | |
| `kind/open.rs`, `kind/apply.rs`, `kind/delete.rs`, `kind/close.rs`, `kind/absent.rs` | the five operations, through kind, kubectl and Docker | |
| `deployment.rs`, `deployment/of.rs`, `deployment/manifest.rs` | one running component, from the fence's component to the manifest the cluster applies | |
| `execution.rs`, `execution/open.rs`, `execution/space.rs`, `execution/placed.rs`, `execution/write.rs`, `execution/load.rs`, `execution/record.rs` | the execution ledger: what was created, written first; the name of the run's space; and whether a ledger read back is its folder's | |
| `execution/up.rs`, `execution/cleanup.rs`, `execution/left.rs`, `execution/sweep.rs`, `execution/card.rs` | the bring-up; the removal of exactly what the ledger lists; the ledgers earlier runs left unsettled, and their removal; the record's runtime marked stopped | |
| `state.rs`, `state/of.rs`, `state/stopped.rs`, `state/line.rs`, `state/card.rs` | the runtime: ready or stopped | `runtimeReady`, `runtimeStopped` |
| `error.rs` | what the local cluster can refuse | |

### delivery

| Files | Holds | Lines |
| --- | --- | --- |
| `forge.rs` | the forge, as a trait: start the branch, push, open a draft pull request, read its checks, mark it ready | |
| `github.rs`, `github/forge.rs` | the forge reached through `git` and `gh` | |
| `github/start.rs`, `github/push.rs`, `github/open.rs`, `github/checks.rs`, `github/ready.rs` | the five operations, each as the tool it runs | |
| `branch.rs`, `branch/start.rs`, `branch/push.rs`, `branch/short.rs`, `branch/line.rs`, `branch/card.rs` | the branch | `pushed` |
| `pull.rs`, `pull/open.rs`, `pull/ready.rs`, `pull/line.rs`, `pull/card.rs` | the pull request | `pullRequest`, `ready` |
| `readiness.rs`, `readiness/word.rs` | where a pull request stands: draft, ready for review or ready to merge | |
| `error.rs` | what the delivery can refuse | |

### checks

| Files | Holds | Lines |
| --- | --- | --- |
| `suite.rs`, `suite/list.rs`, `suite/poll.rs`, `suite/passed.rs`, `suite/line.rs`, `suite/card.rs` | the client's CI checks: first listed, polled until they pass, said and recorded | `checks`, `checksPassed` |
| `check.rs`, `standing.rs` | one check, and where it stands | |
| `error.rs` | what following the checks can refuse | |

### console

| Files | Holds | Lines |
| --- | --- | --- |
| `client.rs` | the console, as a trait | |
| `remote.rs`, `remote/open.rs`, `remote/report.rs` | the console over the network: one JSON-RPC tool call per report | |
| `offline.rs`, `offline/report.rs` | the console of a project with no connection: nothing is sent | |
| `connection.rs`, `connection/record.rs`, `connection/load.rs`, `origin.rs` | the console's origin and the secret, recorded by `nsx init` in a private file Git does not track; and where nsx will send a secret | |
| `report.rs`, `report/open.rs`, `report/ask.rs`, `report/send.rs`, `report/end.rs` | asking the console, sending each record as it is written, and telling it how a command ended, keeping the runtime's id the console answers | |
| `error.rs` | what reaching the console can refuse | |
| `play.rs`, `play/wait.rs` | waiting, silently, for the team's Play of a proposed intent | |
| `hold.rs`, `hold/wait.rs` | holding before a stage's work while the team paused the run | |

### process

| Files | Holds | Lines |
| --- | --- | --- |
| `runner.rs`, `runner/finish.rs` | running a program, as a trait; running one that must end well | |
| `local.rs`, `local/run.rs` | running it on this machine | |
| `environment.rs`, `environment/allow.rs` | the few variables a program is given | |
| `program.rs`, `program/tool.rs` | a program to run: its name, arguments, environment, folder and input; a tool of the machine, ready to run | |
| `output.rs`, `output/ended.rs` | what a program printed and how it ended; whether it ended well | |
| `error.rs` | what can go wrong with a program, as one sentence | |

### files

| Files | Holds | Lines |
| --- | --- | --- |
| `root.rs`, `root/find.rs` | the project's root, found from `.langdrift/` | |
| `layout.rs`, `layout/under.rs` | the paths under `.langdrift/` | |
| `write.rs` | writing a file whole or not at all | |
| `read.rs` | reading a regular file up to a limit | |
| `text.rs` | reading a text: inside the project, bounded by its caller and UTF-8 | |
| `within.rs` | refusing a path that leaves the project | |
| `inside.rs` | a path of the project, checked, given whole or from the root | |
| `folder.rs` | the folder the disk names for a path, so two spellings of one folder give one answer | |
| `private.rs`, `sealed.rs`, `hide.rs` | whether a file is its owner's alone, writing one that is, and keeping the private state out of Git | |
| `error.rs` | what can go wrong with the project's files, each as one sentence | |

## Card fields and records

The cards are sections of `plan.json`, and a run is sections of `run.json`. `contract/` holds each section's shape; one element fills it.

| Record | Section | Filled by |
| --- | --- | --- |
| `plan.json` | the Ticket card: id, number, slug, status, title, and the linked artifacts | `request` |
| `plan.json` | the Analysis card: documents | `request`, then `harness` for the guardrail's row and `judgment` for its statements |
| `plan.json` | the Analysis card: episodes | `rag` |
| `plan.json` | the Analysis card: suggestion | `judgment` |
| `plan.json` | the Plan card: components | `components` |
| `plan.json` | the Plan card: sizing | `harness` for the bound, then `runtime` |
| `plan.json` | the Plan card: price | `money` |
| `plan.json` | the Plan card: authorization and evidence | `handoff` |
| `plan.json` | the Workflow card: the choices | `flow` |
| `plan.json` | the Workflow card: the spend table | `money` |
| `run.json` | the run's id and its outcome | `evidence` |
| `run.json` | the intent | `handoff` |
| `run.json` | the stages and their reports | `flow`, `skills` |
| `run.json` | the effort | `hook` |
| `run.json` | Safe Seat | `seat` |
| `run.json` | the runtime | `local` |
| `run.json` | the delivery: the pull request and its checks | `delivery`, `checks` |

`handoff/writer.rs` is the only writer of `plan.json`, and `evidence/record.rs` the only writer of `run.json`. A folder fills its part through a function named for it (`card`, or the part's own name when a type fills several), which the command hands to the writer: no command builds a card.

## Tests

Outside `src/` (Rust rules, item 1), as one test target. `tests/main.rs` declares a folder per area of `src/`, and each folder's `mod.rs` declares its files. One target means a fake or a fixture is written once and every test reaches it through `crate::`: no path climbs, and nothing only the tests use sits under `src/`.

The owner allows `../` inside this crate and asks that it be avoided where there is a way. This layout is that way: no file of the crate climbs.

| Files | Holds |
| --- | --- |
| `tests/main.rs` | the one test target: it declares the folders below |
| `tests/simulation/` | the golden test, and what every other folder borrows: the fixture project, the goldens and the fakes |
| `tests/simulation/plan.rs`, `run.rs`, `stages.rs`, `budget.rs`, `merge.rs`, `haiku.rs` | one case per golden |
| `tests/simulation/pending.rs` | the pending lists, one per golden, all empty: nsx prints the six goldens whole |
| `tests/simulation/diff.rs`, `rules.rs`, `sketch.rs` | comparing what nsx printed with a golden, that comparison's own rules, and the short session they play on |
| `tests/simulation/frozen.rs`, `golden.rs`, `printed.rs` | a line as a golden freezes it, a golden's lines, and what nsx printed |
| `tests/simulation/fixture.rs`, `project.rs`, `indexed.rs`, `ran.rs` | the fixture project read in place, a copy of it, its episode index built for the simulation's six similarities, and a run played in a copy |
| `tests/simulation/fake/` | the fakes, declared by `fake/mod.rs`: `recorder.rs` (the sink and window that keep what they are given), `refuser.rs` (a sink that refuses every line), `session.rs` (the session the tests play a command over), `witness.rs` (a sink that looks at `plan.json` each time a line prints), `recorded.rs` (the judge that answers with recorded texts), `judge.rs` (the golden test's judge, which answers with the samples of `tests/contract/samples/judgment/`), `console.rs` (the console that keeps every report, answers one runtime id and the requests a test scripts, can fall silent, rewrite a file as a report arrives, and pause before a stage, `console/pausing.rs`), `answers.rs` (the person who answers each wait), `silence.rs` (the person who never does), `spender.rs` (the person whose answers come with a call paid behind their back), `host.rs` (the cluster that does as it is asked and keeps nothing), `machine.rs` (the cluster that keeps what is created in it), `forged.rs` (the forge that delivers as the simulation shows), `hub.rs` (the forge that keeps what it is asked and answers a script), `calm.rs` (the person who never interrupts), `hasty.rs` (the person who interrupts as soon as the command first waits), `coder.rs` (the coding agent that writes each stage's report and says what it used), `moment.rs` (the clock that always shows one moment) and `programs.rs` (the runner that starts nothing), `answering.rs` (the image resolver that gives one answer) and `served.rs` (a console that answers one request on this machine and keeps what it got). |
| `tests/contract/` | the fence, the hand-off records and the report read strictly; each generated schema against its published file; each sample of `tests/contract/samples/` read and written back |
| `tests/command/` | the command line, `nsx init`, and what a run leaves past its gate, through its delivery, and where it stops |
| `tests/local/` | a Deployment from its component, the execution ledger written first and found again, the cleanup of exactly what it lists, the runtime's two lines, and kind, kubectl and Docker as they are started |
| `tests/delivery/` | the branch named, pushed and said, the pull request opened and said, ready only when checked and met, and git and gh as they are started |
| `tests/checks/` | the first listing, the poll until every check passed or one did not, and the two lines |
| `tests/driver/` | `driver.json` read back, the tokens a call used, a call started with the run's choice, and how a call ended |
| `tests/skills/` | each stage's report accepted or refused, and its line |
| `tests/hook/` | the effort record as the gate on implement |
| `tests/seat/` | the dev ticket waited for and closed, and the verdict |
| `tests/console/` | the origin nsx will send a secret to, the connection recorded and read back, the wire of the console's and the judge's clients against a local server, the reports a connected plan and run send, a hold on its own (`hold.rs`), and the team's Play and Pause through a run (`played.rs`, on a project whose intent is still proposed, `proposed.rs`) |
| `tests/components/` | the manifest found and classified, the port's three sources, a component's two lines and its row on the Plan card, an image asked and checked, and every collision named |
| `tests/session/`, `tests/files/`, `tests/process/` | the lines, the stop rule and the system's interrupt; the project's files; and other programs |
| `tests/runtime/` | the shares summed against the bound, the limits statement, the next revision, and the draft made, validated and written as `runtime.md` |
| `tests/request/` | the ticket found from its id, its four links, the facts of each document, and the plan stopping on any of them |
| `tests/harness/` | the guardrail's criteria table and the envelope |
| `tests/rag/` | the embedder run for real, an episode record, the index and its rebuild, the search, the retrieval's lines, and the episode a run leaves |
| `tests/judgment/` | each of the judge's three answers, read and checked before its line prints |
| `tests/money/` | an amount printed as dollars, the price list and the forecast read back, the budget's two lines, the spend table's nine rows, and the estimate, the reservation and the gate |
| `tests/evidence/` | the run's id, the usage record with its totals, and the machine's clock |
| `tests/handoff/` | `plan.json` whole and valid after every section, the proposed intent, and the authorized intent chosen, checked and applied by a run |
| `tests/flow/` | the workflow read back, with its stages and its default choice, and a checkpoint answered, declined or left unanswered |

## Commands

Run from the repository root.

```bash
cargo check
```

```bash
cargo test
```
