//! A stage's report
//!
//! What a stage's skill says it did, as figures. The skill writes it as one
//! JSON object at the path the stage driver gives it. nsx reads it after the
//! call ends, checks it, prints the stage's one line from it and hands the
//! figures to the run's evidence.
//!
//! Definition only: the `Report` type, which carries the stage and the
//! stage's report once checked. Its functions are the files in `report/`.
//!
//! ## Rules
//!
//! - The shape of each stage's report lives in `contract/`.
//! - The evidence keeps the report's figures, never a free-text summary.
//!
//! ## Open
//!
//! - Whether nsx should check the figures it can measure itself, such as the
//!   files in the branch's diff or the acceptance criteria in the spec,
//!   rather than trust the report.

pub mod check;
pub mod line;

use crate::contract::figures::Figures;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
	pub stage: String,
	pub figures: Figures,
}
