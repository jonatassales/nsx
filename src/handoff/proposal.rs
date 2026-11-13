//! The proposed intent
//!
//! What the plan asks the team to authorize: a ticket, a `runtime.md`
//! revision, a model, an effort and the human-in-the-loop choices. The plan
//! records it as proposed, so that authorizing is a decision on concrete
//! values.
//!
//! Definition only: the `Proposal` type, which carries the ticket's bare id,
//! the runtime id and the draft's revision, and the workflow's default
//! choice of model, effort and checkpoints. Its functions are the files in
//! `proposal/`.
//!
//! ## Rules
//!
//! - The choice is the workflow's default (`flow`), the one the Workflow card
//!   shows. The judge's suggestion stays a recommendation.
//! - Its status is `proposed`. nsx records the team's authorization, it
//!   never decides it.
//! - A plan of the same ticket that writes a new revision writes a new
//!   proposed intent for it.

pub mod card;
pub mod intent;
pub mod line;
pub mod write;

use crate::flow::choice::Choice;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
	pub ticket: String,
	pub runtime: String,
	pub revision: i64,
	pub choice: Choice,
}
