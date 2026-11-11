//! The ticket
//!
//! The one ticket a plan is about. A person names it by its bare id, without
//! knowing where its file sits. Its header is read before the plan prints
//! anything, so the Ticket card is filled from the moment the panel appears.
//!
//! Definition only: the `Ticket` type: its id, number, slug, status, title and
//! project-relative path. Its functions are the files in `ticket/`.
//!
//! An id that opens with no number leaves the number unknown, never empty.
//!
//! ## Simulation
//!
//! - Card: Ticket · the header, the number, the slug, the status and the title
//!
//! ## Rules
//!
//! - The status is printed as read and gates nothing.
//! - The id also feeds the run's header. The slug also feeds the runtime's id
//!   and the branch.
//!
//! Spec 0078.

pub mod card;
pub mod intent;
pub mod line;
pub mod parse;
pub mod resolve;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
	pub id: String,
	pub number: Option<String>,
	pub slug: String,
	pub status: String,
	pub title: String,
	pub path: PathBuf,
}
