# langdrift

What `nsx init` lays down under `.langdrift/` in a team's project: the workflow, the skills it runs, the FinOps harness and the price list that plan and run need. The list is spec 0048's, with the content each element spec defines, as the technical design doc gathers it under "The .langdrift/ tree" (`.agents/dd/0001-the-engine-that-reproduces-the-simulation.tdd.md`). `nsx init` asks nothing and prints nothing; it keeps any file already there byte for byte, so running it twice changes nothing. It writes exactly these files and no other. `harness/driver.json` is not among them: the team writes it (spec 0058). The files sit here under the paths below and are built into the binary. Five of them carry another name here, because this repository's naming rules give every Markdown file a type suffix: the guardrail is `harness/guardrails/0001-finops.policy.md`, and the templates are `templates/empty.ticket.md`, `empty.prd.md`, `empty.tdd.md` and `empty.spec.md`. `nsx init` lays each down under the name in the table.

| File | Holds | Spec |
| --- | --- | --- |
| `finops/prices.json` | The three Claude models, each with its name, its short name and its input and output list prices in cents per million tokens, read on 2026-10-06. | 0056 |
| `finops/forecasts.json` | The tokens a run is expected to use, input and output, for each effort: low, medium and high. | 0056 |
| `harness/workflow.json` | The seven stages in order, each with its skill and who acts, and the default choice: `claude-sonnet-5-5` at `medium`, with Planning and Review. | 0057 |
| `harness/guardrails/0001-finops.gr.md` | The FinOps guardrail: one criteria table with `ID`, the rule and `Checked at`, holding FIN-02, FIN-03 and FIN-04. A review rule with no automatic enforcement. From then on it is the team's document. | 0054 |
| `skills/grilling/` | The skill the Planning stage follows. It ends by writing its stage report. | 0059 |
| `skills/to-spec/` | The skill the Spec stage follows. It ends by writing its stage report. | 0059 |
| `skills/to-tickets/` | The skill the Tickets stage follows. It ends by writing its stage report. | 0059 |
| `skills/implement/` | The skill the Implement stage follows. It ends by writing its stage report. | 0059 |
| `skills/tdd/` | The skill the Implement stage follows with `implement`. | 0059 |
| `skills/code-review/` | The skill the Review stage follows. It ends by writing its stage report. | 0059 |
| `templates/ticket.md` | An empty ticket, carrying only the field lines and sections spec 0053 reads. | 0048 |
| `templates/prd.md` | An empty PRD, carrying only the field lines and sections spec 0053 reads. | 0048 |
| `templates/dd.md` | An empty design doc, carrying only the field lines and sections spec 0053 reads. | 0048 |
| `templates/spec.md` | An empty spec, carrying only the field lines and sections spec 0053 reads. | 0048 |

The specs name each skill's folder and no file inside it. The skills are the ones this repository runs from `.claude/skills/`, copied as they stood on 2026-11-11: each `SKILL.md`, and `tdd`'s two guides. They do not write a stage report yet; that ending joins with the ticket that reads the reports (spec 0080). Whether `nsx init` keeps writing the templates, which plan and run never read, is an open question of spec 0048. The console's connection that spec 0073 has `nsx init` record comes from `LANGDRIFT_SECRET`, not from this folder.
