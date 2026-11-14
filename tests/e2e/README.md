# End-to-end run

Plays the real `nsx` binary against a real LangDrift console, with nobody at the keyboard. Each run plans a ticket of its own: a generated four-digit number and a slug that starts with `e2e-`, such as `8971 · e2e-et-ipsa-runtime`, so the console's runtimes are told apart.

The tests are ignored by default: `cargo test --locked` never runs them.

## What a run needs

| Key | Holds |
| --- | --- |
| `MCP_CONSOLE_URL` | The console to plan against, for example `https://console.langdrift.md`. No console is a default. |
| `LANGDRIFT_SECRET` | A secret generated on that console's Settings → Secrets. |

Without either, the run stops with one sentence before anything else.

## The two runs

| Run | The console's team | What it checks |
| --- | --- | --- |
| `a_generated_ticket_is_planned_to_its_handoff_and_the_console_holds_the_plan` | saved its Anthropic key, its offer and a judge model | The plan ends on its `handoff` line and exits 0, and the console's record reads `plan` with the proposed intent. The judge's calls are real and paid by the team's key. |
| `without_the_teams_key_a_generated_ticket_stops_as_the_simulation_shows` | saved its offer and no key | The plan prints the two lines of the simulation's stop and exits 1, no draft is written, and the console's record reads `error` with the judge unavailable. Nothing is paid for. |

```bash
cargo test --locked --test main e2e::plan::a_generated_ticket -- --ignored
```

```bash
cargo test --locked --test main e2e::plan::without_the_teams_key -- --ignored
```

## What it leaves

Each run leaves one runtime in the console's list, under its generated ticket. The project it planned is a temporary folder, removed when the run ends.

`nsx agent run` is not played end to end yet: it needs a scripted coding agent, the team's Play from the console, images the cluster can pull and a forge to deliver to.
