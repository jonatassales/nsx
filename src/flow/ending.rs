//! How the run ends
//!
//! Whether a delivered run ends for review or to merge. The Review tick alone
//! decides: ticked, the pull request waits for the owner's review; unticked,
//! only the merge is left to the owner.
//!
//! Definition only: the `Ending` type and its two outcomes, for review and to
//! merge. Its functions are the files in `ending/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · HUMAN IN THE LOOP, the Review row: once `ready` has
//!   printed it reads `waiting for the owner` when ticked, and is done when
//!   unticked. Merge always reads `always the owner's`.
//!
//! ## Rules
//!
//! - The ending follows `reviewed`, the Review tick, and nothing else. It
//!   never depends on whether the delivery looked clean.
//! - The run's record keeps `reviewed` (spec 0065). Spec 0063 words its
//!   `ready` line from it.
//! - No proposed ticket status is written.
//! - nsx never merges.
//!
//! Spec 0080.

pub mod line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
	Review,
	Merge,
}
