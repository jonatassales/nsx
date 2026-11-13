# Filesystem rules

Naming and path conventions for every file and directory in this repository, in every language. Loaded in every session.

1. **Names use kebab-case.** Files and directories outside Rust code: `stage-report`, not `stageReport` or `stage_report`. Where a language toolchain requires another form, the toolchain wins: Rust modules are snake_case, `Cargo.toml` and `Cargo.lock` keep their names. Conventional files keep their names: `README.md`, `AGENTS.md`, `LICENSE`, `NOTICE`, `SECURITY.md`, `SKILL.md`.
2. **Markdown documents carry a type suffix** before the extension, naming what kind of document they are: `.rule.md`, `.prd.md`, `.spec.md`, `.tdd.md`, `.ticket.md`, `.adr.md`, `.research.md`, `.policy.md`, `.prompt.md`, `.guide.md`. A rule is `<name>.rule.md`.
3. **Paths avoid climbing with `../`.** A design that needs no climbing path comes first: in Rust, `crate::` paths, and one test target that reaches its fakes through `crate::`. `../` is allowed only where no such design exists, and the change says why. Shell `source` never climbs.
4. **No bucket directories.** No `utils/`, `helpers/`, `common/` or `misc/`. Organize by domain, feature or infrastructure concern.

```text
Do:     stage-report.schema.json  commit-rules.sh  src/money/settlement/settle.rs
Don't:  stageReport.schema.json   commitRules.sh   src/money/Settlement/Settle.rs
```
