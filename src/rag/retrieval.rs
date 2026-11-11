//! The retrieval
//!
//! The result of the plan's search over the project's episodes: the six
//! candidates, which of them the threshold kept, and how many of the cited
//! ones measured resources. It fills the EPISODES section and the MEASURED
//! RESOURCES field of the Analysis card, and it owns six lines of the plan.
//!
//! Definition only: the `Retrieval` type: the threshold, the candidates in
//! descending similarity, each candidate's episode record in the same order,
//! the counts kept, dropped and total, and the counts cited and cited with
//! measured resources. Its functions are the files in `retrieval/`.
//!
//! ## Simulation
//!
//! - Card: Analysis · EPISODES, its threshold and its rows
//! - Card: Analysis · MEASURED RESOURCES
//!
//! ## Rules
//!
//! - It fills the episodes of `plan.json` but does not write the file:
//!   `handoff/writer.rs` is its only writer.
//! - The candidates and the counts go to the judge, which gives back one
//!   reason per candidate (spec 0051).
//!
//! ## Open
//!
//! - What the EPISODES section shows with fewer than six episodes, or with
//!   none: the simulation only shows six.
//!
//! Spec 0078.

pub mod candidates;
pub mod citations;
pub mod cited;
pub mod dropped;
pub mod embedding;
pub mod memory;
pub mod retrieve;
pub mod rows;
pub mod search;
pub mod verdicts;

use crate::rag::{
	candidate::Candidate, episode::Episode, threshold::Threshold,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Retrieval {
	pub threshold: Threshold,
	pub candidates: Vec<Candidate>,
	pub records: Vec<Episode>,
	pub kept: u64,
	pub dropped: u64,
	pub total: u64,
	pub cited: u64,
	pub cited_measured: u64,
}
