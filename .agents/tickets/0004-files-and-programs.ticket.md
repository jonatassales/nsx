# 0004: files and programs

**Spec:** a spec since removed from the tree.

**What to build:** nsx finds the project's root, writes a file whole or not at all, reads a file up to a limit, refuses a path that leaves the project, and starts a program with only the variables it names.

**Blocked by:** 0001.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] The root is found by walking up to `.langdrift/`.
- [x] A write is atomic; a read is bounded.
- [x] A path outside the project, through `..` or a symlink, is refused.
- [x] A started program receives only the minimal base and the variables its caller names (another ADR).
- [x] `files/lock.rs` is removed: no spec asks for a lock.

**Note, 2026-11-10:** each folder words its own failures in an `error.rs`, one sentence each. A program to run is its own type, `process/program.rs`, so the runner takes one thing. Three open questions are answered provisionally and say so in their prefaces: a named variable that is not set is left out, the locale is `LANG`, `LC_ALL`, `LC_CTYPE` and `LC_MESSAGES`, and a program still running is killed when its command stops. No caller reads a file yet, so no read limit is chosen.

**Note, 2026-11-11:** after the two-axis review: `files/text.rs` reads a document from the project's root through `within`, so no reader can skip the containment check.

## Dev part

**Category:** `hard-part`, because path containment is the boundary between nsx and the rest of the machine.

**Paths:** `lib/nsx/src/files/within.rs`, `lib/nsx/tests/files/within.rs`.

**Boundary:** the `Root` type in `files/root.rs`.

**Goal:** Refuse any path that resolves outside the project.

- [x] `..`, an absolute path and a symlink pointing out are refused; a path inside is accepted.

**Override:** on 2026-11-10 the owner asked the agent to write this dev part ("You write them all"), so the agent wrote its paths.
