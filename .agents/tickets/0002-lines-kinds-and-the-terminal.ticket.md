# 0002: lines, kinds and the terminal

**Spec:** a spec since removed from the tree.

**What to build:** Every line nsx prints goes through one sink, as one of four kinds, with the simulation's spacing. A developer can collect lines in a test instead of printing them.

**Blocked by:** 0001.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] A line has an id, a kind (`comment`, `output`, `success`, `warning`) and a text.
- [x] `console ›` lines, lines with a bare label and `✓` lines render with the goldens' exact columns.
- [x] The terminal sink prints to stdout; a test sink collects the lines.
- [x] The window title is set as the simulation sets it.

**Note, 2026-11-10:** a line carries its shape, the words its layout needs, so `session/shape.rs` is added. The window's title goes through a trait of its own, `session/window.rs`, because a trait with two methods would put two functions in one file. The test sink, `session/recorder.rs`, lives in the library so that every test target can use it without a path that climbs. Printing gives back a stop when standard output cannot be written.

## Dev part

**Category:** `hard-part`, because the column layout is what makes nsx's output byte-equal to the simulation.

**Paths:** `lib/nsx/src/session/line/render.rs`, `lib/nsx/tests/session/render.rs`.

**Boundary:** the `Line` and `Kind` types in `session/line.rs` and `session/kind.rs`.

**Goal:** Render a line to the exact text the goldens hold, for each of the six shapes they show.

- [ ] Every line of the six goldens, built from its id, kind and parts, renders to its text byte for byte. (Open: the six shapes are proven on ten of the goldens' lines here; each other line is proven by the ticket that builds it, and the golden test closes it.)

**Override:** on 2026-11-10 the owner asked the agent to write this dev part ("You write them all"), so the agent wrote its paths.
