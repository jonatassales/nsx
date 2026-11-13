# langdrift

What `nsx init` lays down under `.langdrift/` in a team's project: the workflow, the skills it runs, the FinOps harness and the price list that plan and run need. `src/files/layout.rs` names every path under `.langdrift/`, these and the ones the plan and the run write. `nsx init` asks nothing and prints nothing; it keeps any file already there byte for byte, so running it twice changes nothing. It writes exactly these files and no other. `harness/driver.json` is not among them: the team writes it. The files sit here under the paths below and are built into the binary (`src/command/init/execute.rs`). Five of them carry another name here, because this repository's naming rules give every Markdown file a type suffix: the guardrail is `harness/guardrails/0001-finops.policy.md`, and the templates are `templates/empty.ticket.md`, `empty.prd.md`, `empty.tdd.md` and `empty.spec.md`. `nsx init` lays each down under the name in the table.

| File | Holds |
| --- | --- |
| `finops/prices.json` | The three Claude models, each with its name, its short name and its input and output list prices in cents per million tokens, read on 2026-10-06. |
| `finops/forecasts.json` | The tokens a run is expected to use, input and output, for each effort: low, medium and high. |
| `harness/workflow.json` | The seven stages in order, each with its skill and who acts, and the default choice: `claude-sonnet-5-5` at `medium`, with Planning and Review. |
| `harness/guardrails/0001-finops.gr.md` | The FinOps guardrail: one criteria table with `ID`, the rule and `Checked at`, holding FIN-02, FIN-03 and FIN-04. A review rule with no automatic enforcement. From then on it is the team's document. |
| `skills/grilling/` | The skill the Planning stage follows. |
| `skills/to-spec/` | The skill the Spec stage follows. |
| `skills/to-tickets/` | The skill the Tickets stage follows. |
| `skills/implement/` | The skill the Implement stage follows. |
| `skills/tdd/` | The skill the Implement stage follows with `implement`. |
| `skills/code-review/` | The skill the Review stage follows. |
| `templates/ticket.md` | An empty ticket, carrying only the field lines and sections the plan reads (`src/request/`). |
| `templates/prd.md` | An empty PRD, carrying only the field lines and sections the plan reads. |
| `templates/dd.md` | An empty design doc, carrying only the field lines and sections the plan reads. |
| `templates/spec.md` | An empty spec, carrying only the field lines and sections the plan reads. |

Each skill is a folder with its `SKILL.md`; `tdd` also holds its two guides. They are this repository's own skills, from `.claude/skills/`. A skill says nothing of a stage report: the prompt nsx gives each stage call asks the coding agent to write the stage's report as one JSON object at the report path (`src/driver/call/prompt.rs`). Whether `nsx init` keeps writing the templates, which plan and run never read, is an open question (`src/command/init/execute.rs`). The console's connection that `nsx init` records comes from `LANGDRIFT_SECRET`, not from this folder (`src/console/connection.rs`).
