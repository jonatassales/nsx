# Changelog

Every notable change to nsx, newest first, in the shape of
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/jonatassales/nsx/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/jonatassales/nsx/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jonatassales/nsx/releases/tag/v0.1.0
