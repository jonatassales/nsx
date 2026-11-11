//! What was read
//!
//! The reading of the ticket and its documents, as a whole. It counts what was
//! read, once for the terminal and once for the card, and it closes the reads:
//! nothing outside the ticket's links is read.
//!
//! Definition only: the `Reading` type: the count of documents read before the
//! `✓ read` line, and the count of documents the ticket links. Its functions
//! are the files in `reading/`.
//!
//! ## Rules
//!
//! - The terminal's count is the ticket, the PRD, the design doc and the spec.
//! - The card's count is the four linked documents, PRD, design doc, spec and
//!   guardrail, counted from the resolved links. It shows before the guardrail
//!   is read.
//! - No link of a linked document is followed. There is no depth, file or byte
//!   limit, and no count of refused sources.
//!
//! ## Open
//!
//! - Which set "documents read" means. The terminal and the card count
//!   different sets that agree on the simulation's ticket. The spec keeps both
//!   counts until the owner decides.
//!
//! Spec 0078.

pub mod card;
pub mod count;
pub mod line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reading {
	pub read: u64,
	pub linked: u64,
}
