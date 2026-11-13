# 0005: the plan finds the ticket and its links

**Spec:** a spec since removed from the tree.

**What to build:** The owner types `nsx agent plan 0031` and the plan finds the ticket by its bare id, reads its four links and fills the Ticket card before its first line.

**Blocked by:** a ticket since removed from the tree.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it.

- [x] prints `console › tool call: read 0031-TICKET-book-a-demo-runtime` (`readTicket`, output)
- [x] prints `console › ready-for-agent · links 0002-PRD · 0005-DD · 0011-SPEC · guardrail 0001-finops` (`links`, output)
- [x] The Ticket card's header, number, slug, status and title are in `plan.json` before the first line. (Closed with ticket 0010.)
- [x] A missing ticket or a link outside the project is a stop.

**Note, 2026-11-11:** the Ticket card's header is read before the plan's first line, but it is not in `plan.json` yet: no file is written before the hand-off's writer exists. Its criterion above stays open and moves to ticket 0010, which writes `plan.json` first.

**Note, 2026-11-11:** both `.agents/` and `.claude/` are searched, at any depth, and the id must name one file across the two; the spec's open question about a project that has both stays open, with that reading. The id is matched against file names only, never joined to a path, and a folder that is a symbolic link is never entered.

**Note, 2026-11-11:** an id that opens with no four digits and a dash has no number: it is unknown, not empty, and the slug is then the id. The title is the words of the first heading, whatever its level. Any Markdown link of another kind stops the plan, a page's address included, as another spec lists it among its safe stops.

**Note, 2026-11-11:** the plan prints `scope` and `readTicket`, then reads the links and prints `links`, so a ticket with a bad link stops after two lines; `tests/request/stop.rs` holds that case. Whether a linked document is there is found when it is read. `files/text.rs` reads a document, at most 1 MiB and UTF-8, the first read limit nsx chooses.

**Note, 2026-11-13:** the Ticket card's header is in `plan.json` before the first line with ticket 0010, as the test `the_tickets_header_is_in_plan_json_before_the_first_line` proves.

## Dev part

**Category:** `core-domain`, because resolving a bare id is the first decision of every plan.

**Paths:** `lib/nsx/src/request/ticket/resolve.rs`, `lib/nsx/tests/request/ticket.rs`.

**Boundary:** the `Ticket` type in `request/ticket.rs`.

**Goal:** Find the ticket's file from its bare id under the client's `.agents/` or `.claude/`.

- [x] The fixture's ticket resolves; an unknown id is a stop.

**Override:** on 2026-11-10 the owner asked the agent to write this dev part ("You write them all"), so the agent wrote its paths.
