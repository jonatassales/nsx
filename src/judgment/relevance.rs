//! The judgment of the episode candidates
//!
//! The judge's second question: for each episode candidate, whether it shares
//! the ticket's stack and scale, or why it differs. The reasons come from a
//! closed set, so they read the same on every plan.
//!
//! Definition only: the `Relevance` type: per candidate, its id, its verdict,
//! kept or dropped, and its reason. Its functions are the files in
//! `relevance/`.
//!
//! ## Simulation
//!
//! - Card: Analysis · EPISODES rows: each row's verdict and reason, which
//!   `rag/` shows from these judgments
//!
//! ## Rules
//!
//! - A kept candidate's reason is `shares the stack and scale`. A dropped
//!   one's is `other runtime`, `no broker` or `docs only`.
//! - The verdict equals the threshold's.
//!
//! ## Open
//!
//! - May the judge drop a candidate at or above 0.55, or keep one below it?
//!   Until the owner decides, a disagreement is a refused answer and stops the
//!   plan.

pub mod ask;
pub mod explain;
pub mod line;

use crate::contract::episode::Episode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relevance {
	pub judged: Vec<Episode>,
}
