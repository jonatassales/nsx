//! The ticket, its links and the linked documents
//!
//! What `nsx agent plan <ticket>` reads first: the one ticket of that id, the
//! four documents the ticket links, and the exact facts the simulation shows
//! from the PRD, the design doc and the spec. Every fact is read by a fixed
//! rule, with no model involved. Nothing outside the ticket's links is read.
//!
//! - `Ticket`: the ticket, found from its bare id
//! - `Links`: the ticket's four links, each a `Link` of one `Kind`
//! - `Prd`, `Design` and `Spec`: the three linked documents the plan reads,
//!   each with its own facts, among them each component the spec has `Named`
//! - `Documents`: those three, together, as the judge reads them
//! - `field`: the value of a document's `**Field:** value` line
//! - `Reading`: what was read, counted
//! - `Error`: what reading the ticket and its links can refuse
//!
//! ## Rules
//!
//! - This folder writes no file under `.langdrift/`. Its values reach the
//!   plan's card data through `handoff/`: the whole Ticket card, and the
//!   DOCUMENTS header and the PRD, design doc and spec rows of the Analysis
//!   card.
//! - The guardrail link is found here. Its criteria are read in `harness/`.
//! - A ticket or a link that cannot be read as the simulation needs stops the
//!   plan before the line that would print next: no further line, no draft, one
//!   sentence on stderr, exit 1 (the stop rule).

pub mod design;
pub mod documents;
pub mod error;
pub mod field;
pub mod kind;
pub mod link;
pub mod links;
pub mod named;
pub mod prd;
pub mod reading;
pub mod spec;
pub mod ticket;
