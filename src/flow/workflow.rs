//! The workflow
//!
//! LangDrift's spec-driven workflow, as `nsx init` sets it up in a project.
//! It is one fixed list of seven stages, each with the skill the card names
//! and who acts in it, and the default choice a plan starts from.
//!
//! Definition only: the `Workflow` type, which carries the seven stages in
//! order (`planning`, `spec`, `tickets`, `implement`, `dev-ticket`, `review`,
//! `merge`), each an `Entry` with its `id`, `skill` and `person`, and the
//! default choice, `defaults`. The seven ids and their skills sit beside
//! the type. Its functions are the files in `workflow/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · HUMAN IN THE LOOP: the rows, their skill texts, which
//!   rows carry a box and their initial ticks
//! - Card: Workflow · MODEL and EFFORT, as shown before any suggestion
//!
//! ## Rules
//!
//! - The skill texts are `grilling`, `to-spec`, `to-tickets`,
//!   `implement · tdd` and `code-review`. Dev ticket and Merge have none.
//! - `person` is `required` for Planning, `optional` for Spec, Tickets,
//!   Implement and Review, `writes` for Dev ticket and `owner` for Merge.
//! - `defaults` is `claude-sonnet-5-5` at `medium`, with Planning and Review.
//!   It only seeds the plan's default choice, and the judge's suggestion does
//!   not change it.
//! - A connected plan proposes `defaults` only when the team offers it on the
//!   console's Settings → AI Models (`within`).
//! - Merge is never played: it is always the owner's.
//!
//! ## Open
//!
//! - Whether a team may change the stages, their skills or their order. Until
//!   the owner decides, nsx plays these seven.
//! - Whether the default follows the judge's suggestion when the two differ.

pub mod load;
pub mod within;

pub const STAGES: [&str; 7] = [
	"planning",
	"spec",
	"tickets",
	"implement",
	"dev-ticket",
	"review",
	"merge",
];

// The skill each stage follows, in the stages' order. The dev ticket is the
// person's own work and Merge the owner's: neither has one.
pub const SKILLS: [Option<&str>; 7] = [
	Some("grilling"),
	Some("to-spec"),
	Some("to-tickets"),
	Some("implement · tdd"),
	None,
	Some("code-review"),
	None,
];

use serde::Deserialize;

use crate::flow::{choice::Choice, entry::Entry};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workflow {
	pub stages: Vec<Entry>,
	pub defaults: Choice,
}
