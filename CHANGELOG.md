# Changelog

Every notable change to nsx, newest first, in the shape of
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `nsx agent run` brings its own kind and kubectl: pinned versions,
  downloaded once into the project's private state, each verified against
  its publisher's SHA-256, and found first by every cluster tool.
- `nsx agent run` starts Docker when it is installed and stopped, and
  waits up to three minutes for it to answer, without a line. On Linux,
  where that needs root, and on a machine with no Docker, it stops with
  one sentence that says what to do.

## [0.2.1] - 2026-11-14

### Added

- A connected `nsx agent plan` reads the team's offer, the models and
  efforts per provider set on the console's Settings → AI Models,
  through the tool `langdrift.account.offer`. The judge suggests, and the
  Workflow card prices, only the offered models of the price list at the
  offered efforts nsx knows. An offer that leaves nothing to suggest, or
  one the console cannot answer, stops the plan before its first line.
- `schema/offer/offer.schema.json`, exported as `@langdrift/nsx/offer/*`,
  with its sample.
- A plan whose team has no Anthropic key on the console stops as the
  simulation shows: after `guardrail` it prints `✗ judge      no
  Anthropic key · add one on the console's Settings → AI Models` and `#
  plan stopped · no draft written · nothing was paid for`, records the
  judge as unavailable in `plan.json`'s new `analysis.judge`, reports
  `error` to the console, writes no draft and exits 1.
- An `error` line kind, red in a terminal, and the `✗` failure shape.

### Changed

- The refusal for an offer without a model nsx holds a list price for no
  longer names models for the team to offer.

### Removed

- The judge request's `model`, and the model nsx named in it: the
  console answers with the team's judge model and key, set on its
  Settings → AI Models.

## [0.2.0] - 2026-11-13

### Changed

- The README presents nsx to a team that would use it: a banner in
  LangDrift violet, then its badges, its tagline and one link to the
  docs. The map of `src/` it held is now `docs/architecture.guide.md`.

## [0.1.1] - 2026-11-13

### Changed

- The package and the crate describe nsx as the LangDrift FinOps, LLMOps
  and Microruntimes CLI.
- The workflow that checks, tests and audits the crate is `ci.yml`, and a
  pull request shows its checks as CI.

## [0.1.0] - 2026-11-13

The first release of nsx, on npm as `@langdrift/nsx`.

### Added

- Three commands, as the simulation on https://langdrift.md plays them:
  `nsx init` lays down `.langdrift/` in a team's project, `nsx agent plan
  <ticket>` reads the ticket and proposes a sized microruntime, and
  `nsx agent run` applies the intent the team authorized.
- The npm package carries a binary for macOS and Linux, on x64 and arm64,
  each built on `main` with a signed build provenance attestation; its
  launcher, `bin/nsx.sh`, runs the one for the machine.
- The JSON Schemas of the records nsx writes and sends, and their samples,
  exported by the package for the LangDrift console.

### Changed

- nsx lives in its own repository, `jonatassales/nsx`, with its rules, its
  ADR, its tickets, its guardrails and its CI.
- The repository's tooling CLI is `vtec`, in `vtec/`, so it is not
  confused with the LangDrift monorepo's `drift`.

[Unreleased]: https://github.com/jonatassales/nsx/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/jonatassales/nsx/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/jonatassales/nsx/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/jonatassales/nsx/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jonatassales/nsx/releases/tag/v0.1.0
