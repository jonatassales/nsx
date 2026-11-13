# 0020: a completed run leaves an episode

**Spec:** a spec since removed from the tree.

**What to build:** Each completed run writes one episode and adds it to the index, so the next plan can recall it.

**Blocked by:** 0019.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] One episode per completed run under `.langdrift/microruntimes/episodes/`. (Closed with another ticket.)
- [x] The index gains it without a rebuild; a rebuild from the episodes gives the same index.

**Note, 2026-11-11:** from another ticket: the simulation's fixture keeps an index built by the test, so that its six similarities are the simulation's. A rebuild from the fixture's records gives other vectors. The criterion "a rebuild gives the same index" is about an index nsx itself wrote from its own episodes; `tests/rag/record.rs` already reads an episode record, and `tests/rag/episode.rs` is left free for this ticket's dev part.

**Note, 2026-11-11:** `rag/episode/write.rs` gives the episode the next free `ep-NNNN`, writes `<id>-<slug>.json` and adds its vector to the index without a rebuild; a project with no index of this embedder gets one built from its episodes. A rebuild from the episodes gives the same index. `command/run/remember.rs` writes it after a run whose outcome is `completed`: the ticket's intent, the components and the resource of the plan's cards, and nothing measured.

**Note, 2026-11-11:** the first criterion stays open until a run can end `completed`, with another ticket: the function is tested on its own here.

**Note, 2026-11-11:** from the two-axis review: an episode's slug must be a lowercase label and its file is written inside the episodes folder, or nothing is written; a number past 9999 keeps all its digits (`rag/episode/number.rs`); an index that does not list exactly the records on the disk, or lists one twice, is out of step and is built again by the next plan (`rag/index/current.rs`), so a failed index write after a record is repaired rather than hidden; a record is every JSON file of the folder and its name says the id it holds (`rag/episode/filed.rs`), which a rebuild checks, so the index and the records cannot disagree forever; the episode is written only from what the plan and the run recorded, with a stop naming what is missing (`Unplanned`).

**Note, 2026-11-11:** with another ticket: a run that prints its last line ends `completed` and leaves its episode (`command/run/remember.rs`); a run that stops anywhere in its delivery leaves none.

## Dev part

**Category:** `core-domain`, because the memory the next plan depends on.

**Paths:** `lib/nsx/src/rag/episode/write.rs`, `lib/nsx/tests/rag/episode.rs`.

**Boundary:** the `Episode` type in `rag/episode.rs`.

**Goal:** Write the episode and add it to the index.

- [x] A plan after a run cites the new episode when it is similar enough.

**Override:** on 2026-11-11 the owner asked the agent to write this dev part ("Você escreve todas"), so the agent wrote its paths.
