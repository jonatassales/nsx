# 0003: the stop rule

**Spec:** a spec since removed from the tree.

**What to build:** When something fails that the simulation does not show, nsx prints no further line, writes one sentence to stderr and exits 1. Ctrl-C is a stop. The over-budget pause exits 0.

**Blocked by:** 0002.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] Any element's error reaching the binary's edge prints one sentence on stderr, with no secret, and exits 1.
- [x] No line reaches stdout after a stop, so stdout stays a prefix of the simulation.
- [x] Ctrl-C is handled as a stop.
- [x] A shown ending, such as the gate's pause, exits 0.

**Note, 2026-11-10:** any folder's failure becomes a stop through `stop/from.rs`, and a refused command line through `stop/usage.rs`; `--help` exits 0. No secret exists yet for a sentence to leak: another ticket adds that check with the secret. The case of a failure after printed lines is tested with the plan, in ticket 0005.

## Dev part

**Category:** `core-domain`, because the rule that keeps stdout a prefix of the simulation in every failure.

**Paths:** `lib/nsx/src/session/stop/exit.rs`, `lib/nsx/tests/session/stop.rs`.

**Boundary:** the `Stop` type in `session/stop.rs`.

**Goal:** Turn any failure into the stop: one sentence, no further line, exit 1.

- [x] A failure after three lines leaves exactly those three on stdout and one sentence on stderr. (Shown after four lines, where the plan reads the PRD: `lib/nsx/tests/request/stop.rs`.)
- [ ] The sentence never contains a value read from a secret. (Open: nsx reads no secret yet. It moves to another ticket, which reads the first one.)

**Override:** on 2026-11-10 the owner asked the agent to write this dev part ("You write them all"), so the agent wrote its paths.
