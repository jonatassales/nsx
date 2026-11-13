//! The pull request
//!
//! The pull request the run opens from its branch to the repository's default
//! branch. It starts as a draft, so that nobody reviews it before the checks
//! pass and Safe Seat is met, and it is marked ready at the end of the run.
//! nsx never merges it: the owner decides.
//!
//! Definition only: the `Pull` type, which carries the pull request's number,
//! its title and its state, which is a draft, then ready for review or ready
//! to merge (`readiness.rs`). Its functions are the files in `pull/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · PULL REQUEST: a dash until the pr line, then
//!   `#19 · 3f9c2e1`. From the ready line its state reads `ready for review`
//!   or `ready to merge`. Before that it shows the checks' state (`checks`).
//! - Card: Workflow · the header, at the ready line:
//!   `#19 · ready for review · $9.80` or `#19 · ready to merge · $9.80`. The
//!   observed figure is `money`'s.
//!
//! ## Rules
//!
//! - The run's record keeps `pullRequest.number`, `pullRequest.title` and
//!   `pullRequest.state`.
//! - A stopped run leaves its pull request a draft.
//!
//! ## Open
//!
//! - Where the title comes from. Until the owner decides, it is the subject
//!   of the branch's first commit, which the stages write.
//! - Opening the pull request as a draft is a choice: the simulation shows
//!   only the pr line, then the ready line.

pub mod card;
pub mod line;
pub mod open;
pub mod ready;

use crate::delivery::readiness::Readiness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pull {
	pub number: u64,
	pub title: String,
	pub state: Readiness,
}
