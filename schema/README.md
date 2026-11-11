# schema

The JSON Schemas nsx publishes: the shapes of `src/contract/`, for whoever reads them without nsx. A skill's author validates a stage report against them, and the console validates each report it receives. They first lived under Core's Go contracts, which `@langdrift/core` published; ticket 0434 removed Core, and `@langdrift/nsx` publishes them from this folder (PRD 0002). `src/contract/schema.rs` generates them from the types. Each file keeps the name and the `$id` it has today, `https://langdrift.md/schemas/<group>/<file>`, and sits here under that group. Of Core's older `judge` schemas only the response the console's `/api/judge` answers is carried over (`judge/response.schema.json`, from `contract/judged.rs`); the `protocol` package is not. The files below started as Core's, copied byte for byte by ticket 0403, and the contract tests prove that each schema generated from its type equals its file as a JSON document. `handoff/plan.schema.json` has since grown: its sections gain their fields as the tickets that fill them land (the DOCUMENTS of the Analysis card with ticket 0408, the Workflow card's spend table with ticket 0417), so it no longer equals Core's copy. The console validates every report against these files (tickets 0432 and 0434). Their descriptions still say `dft` and "the Go check" because they are Core's words; the cutover (ticket 0434) is where they change.

| File | Holds | Spec |
| --- | --- | --- |
| `configuration/runtime.schema.json` | The `langdrift` fence of `runtime.md`, version 3: the runtime's id and revision, the bound, the suggested model and effort, and per component its path, stack, role, port, image, share and replicas. | 0049 |
| `judgment/criteria.answer.schema.json` | The judge's criteria answer: one `id` and one `statement` per criterion of the guardrail. | 0049 |
| `judgment/episodes.answer.schema.json` | The judge's episodes answer: per candidate its `ep-NNNN` id, the verdict `kept` or `dropped`, and a reason from the closed set. | 0049 |
| `judgment/suggestion.answer.schema.json` | The judge's suggestion answer: the model, the effort, and the reason for each. | 0049 |
| `stages/planning.report.schema.json` | The report the `grilling` skill writes: the open questions settled. | 0049 |
| `stages/spec.report.schema.json` | The report the `to-spec` skill writes: the spec's path, whether it was revised, and its acceptance criteria count. | 0049 |
| `stages/tickets.report.schema.json` | The report the `to-tickets` skill writes: the ticket count, and whether the blocking edges were recorded. | 0049 |
| `stages/implement.report.schema.json` | The report the `implement` skill writes: the method `tdd`, the components and files, boundary only, stopped at the reserved part. | 0049 |
| `stages/review.report.schema.json` | The report the `code-review` skill writes: the finding count, and whether the findings were fixed. | 0049 |
| `handoff/plan.schema.json` | `plan.json`, the plan's card data: one section per card, each absent until revealed. A section whose fields have not joined yet allows no field. | 0049, 0050, 0073 |
| `handoff/intent.schema.json` | `intent.json`, the recorded intent: the ticket, the runtime id, the revision, the model, the effort, the human stages and the status `proposed` or `authorized`. | 0049, 0050, 0073 |
| `handoff/run.schema.json` | `run.json`, the run's record: the run's id and outcome, `null` until known, and one section per part of the run. | 0049, 0065, 0073 |
| `report/report.schema.json` | The arguments of the tool `langdrift.runtime.report`: the runtime's console id, the status (`held` among them, spec 0076), and the three hand-off records written so far. | 0073, 0076 |
| `report/answer.schema.json` | The tool's answer: the runtime's console id and, when one is pending, the team's request: a play with the choice it authorizes, or a pause. | 0073, 0076 |
| `judge/response.schema.json` | The console judge's answer to one question: the validated answer text, or a refusal with its reason, with the tokens used, the cost in cents and the judge's version. | 0045, 0078 |

## What the console needs unchanged

The console validates a report against these files, never against a copy of its own (`apps/console/src/app/api/mcp/invalid-report-field.compute.ts` and `report-tool.data.ts`), and the `runtimes` function's answer against `report/answer.schema.json` before it reaches nsx (`valid-report-answer.compute.ts`).

- **The paths it imports.** `handoff/plan.schema.json`, `handoff/intent.schema.json`, `handoff/run.schema.json`, `report/report.schema.json`, `report/answer.schema.json` and the five stage report schemas, under the package's name.
- **The `$id` of the three hand-off schemas.** The console registers them first, then compiles the report's schema. The report refers to each record by a `$ref` to its `$id`: `https://langdrift.md/schemas/handoff/plan.schema.json`, `…/intent.schema.json` and `…/run.schema.json`. A changed `$id` or `$ref` leaves the reference unresolved.
- **The dialect.** Every schema declares JSON Schema draft 2020-12, and the console compiles them with a 2020-12 validator.
- **The report's schema as it is.** The console serves it as the tool's `inputSchema`.
- **What they refuse.** Each object refuses a field it does not define, so that a report the console accepts is one nsx's own decoding accepts.

The samples sit beside the tests under `tests/contract/samples/`, and `@langdrift/nsx` exports them as `@langdrift/nsx/samples/*` for the console's tests; whether they ship once the package is published is decided with its packaging (ticket 0434).
