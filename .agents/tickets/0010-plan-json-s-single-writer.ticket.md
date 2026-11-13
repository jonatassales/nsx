# 0010: plan.json's single writer

**Spec:** a spec since removed from the tree.

**What to build:** `plan.json` fills section by section as the plan's lines print, always written whole.

**Blocked by:** 0005.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] Each element hands its section to the one writer; the file is rewritten atomically when a section lands.
- [x] `plan.json` validates against its schema after every write.
- [x] The Ticket card's header, number, slug, status and title are in `plan.json` before the plan's first line.

**Note, 2026-11-11:** the criterion above comes from ticket 0005, which reads the header before the first line but writes no file: this ticket's writer writes `plan.json` first.

**Note, 2026-11-11:** the writer takes what a folder does to the card data, applies it, and writes the whole file: `writer.write(&root, |cards| …)`. A section that fills in parts, or that several folders fill, needs no merge rule of its own that way. The file is `microruntimes/runtimes/<slug>/plan.json`, tab-indented, and goes through `files/inside.rs`, the containment check for a path given whole or from the root, which `files/text.rs` now shares.

**Note, 2026-11-11:** the Ticket card's header is written before the plan's first line, from `request/ticket/card.rs`; a test plays the plan on a sink that refuses its first line and reads the header back. The `workflow` section, the other one written that early, arrives with ticket 0009.

**Note, 2026-11-11:** after the two-axis review. The tests are one target (`tests/main.rs`), so the fakes and the fixtures are shared through `crate::` and nothing of the tests sits under `src/`: the recorder, the recorded judge and the test session moved to `tests/simulation/fake/`. The owner allowed `../` in this crate on 2026-11-11 and asked that it be avoided where there is a way; this is that way. A test sink, `Witness`, reads `plan.json` at every line and proves that each field is absent until its line has printed. No control character reaches the terminal: a line or a window title that carries one is a stop, and a stop's sentence shows it as its escape. Only a regular file is read: a named pipe where a document should be hung the plan. A ticket whose slug is not a lowercase DNS label stops the plan, since the slug names the runtime and its folder (`0031-` wrote `runtimes/plan.json`).

## Dev part

**Category:** `hard-part`, because one writer, many sections, always valid.

**Paths:** `lib/nsx/src/handoff/writer/write.rs`, `lib/nsx/tests/handoff/writer.rs`.

**Boundary:** the `Writer` type in `handoff/writer.rs`.

**Goal:** Merge a section into `plan.json` and write it atomically.

- [x] After each section lands, the file on disk is valid and holds every earlier section.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
