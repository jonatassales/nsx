# Tooling rules

Scripts, the toolchain, mechanical checks and commits. Loaded in every session.

1. **POSIX shell for automation.** Repository automation, orchestration and verification scripts are POSIX shell with a `.sh` extension. `.mjs`, `.js`, `.ts`, Python and other non-shell automation scripts are prohibited, and a `.sh` wrapper around embedded code in another language does not satisfy this rule. The crate keeps Rust, and `bin/nsx.sh` stays the shell launcher it is.
2. **Cargo builds, the toolchain is pinned.** `rust-toolchain.toml` names the one Rust version, with `clippy` and `rustfmt`; every build, test and lint runs through `cargo` with `--locked`. `package.json` exists only to ship nsx on npm: its scripts call `cargo` directly. pnpm is the only Node package manager, for that packaging and for `pnpm dlx skills`; never `npm`, `npx`, `yarn` or `bun`.
3. **Atomic leaf commands.** A script invokes the tool for its operation, with the tool's own flags (`cargo build --release --locked`). No script calls another script, and no shell wrapper stands in front of one cargo command.
4. **Mechanical checks.** `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, `cargo deny check` and `drift quality check` run in CI, and the commit guardrails run in the commit-msg hook and in CI. A rule they do not cover is enforced by review (General, Authority 7).

## Commit messages

- **Conventional Commits:** `type(scope): subject`, in lowercase.
- **Subject:** imperative mood, at most 72 characters (ideally 60), no trailing period.
- **Blank lines:** exactly one between the subject and the body, and between the body and the trailers.
- **Body:** hard-wrapped at 72 characters.
- **Ticket trailer:** a commit touching `src/`, `tests/`, `schema/`, `langdrift/` or `bin/` ends with `Ticket: <NNNN>` after a blank line.
- **No AI signatures:** no `Co-Authored-By`, `Generated with` or assistant markers in commits or pull requests.
