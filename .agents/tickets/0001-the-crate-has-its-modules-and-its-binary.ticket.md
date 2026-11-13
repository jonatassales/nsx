# 0001: the crate has its modules and its binary

**Spec:** a spec since removed from the tree.

**What to build:** A developer builds `nsx` and runs `nsx --help`: the three commands exist, `nsx init`, `nsx agent plan <ticket>` and `nsx agent run`, and each one parses its arguments and calls its script. No other command exists.

**Blocked by:** None (can start immediately).

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] Every folder of `src/` is declared as a module, and each file's preface stays above its code.
- [x] The crate has a library target and a binary named `nsx`; the binary's entry file carries the architecture header.
- [x] `nsx agent plan` takes one bare ticket id; `nsx agent run` takes none.
- [x] An unknown command or a missing argument is a stop (ticket 0003) with clap's message.
- [x] `cargo clippy -- -D warnings` passes.

**Note, 2026-11-10:** every command's `execute` takes a `Session` and gives back a `Stop` when it stops, so `command/session.rs` is added and the `Stop` type's definition, the boundary of ticket 0003, arrives here. `--help` stays as clap prints it; the `help` command is disabled. Usage errors are routed through the stop rule by ticket 0003. `rustfmt.toml` and item 3 of the Rust rules record the formatting the owner chose.

**Note, 2026-11-11:** after the two-axis review: a refused command line is a stop in nsx's own words, not clap's. Ticket 0003 words a missing command and a missing ticket itself and keeps the first line of clap's message for the rest (`session/stop/usage.rs`).

## Dev part

**Category:** `new-stack`, because the first Rust the owner writes in nsx: the dispatch from the parsed command to its script.

**Paths:** `lib/nsx/src/command/cli/execute.rs`, `lib/nsx/tests/command/cli.rs`.

**Boundary:** the `Cli` type in `command/cli.rs` and the `execute` of each command, which the agent writes.

**Goal:** Dispatch the parsed command to `init`, `agent plan` or `agent run`.

- [x] Each of the three commands reaches its own `execute`, and nothing else is reachable.

**Override:** on 2026-11-10 the owner asked the agent to write this dev part ("You write them all"), so the agent wrote its paths.
