//! The spend table
//!
//! One row per candidate model and effort: nine rows when the team offers
//! the three models at the three efforts. The plan prices every pair before
//! its first line prints, so the Workflow card only looks up the pair the
//! team picks, and the run uses the same row. The console shows the data
//! and never reasons about money.
//!
//! Definition only: the `Spend` type: the budget with its source, the margin
//! in percent, and one `Pair` per candidate model and effort, in the order of
//! the three models nsx reads and then low, medium, high. The fixed margin
//! sits beside the type. Its functions are the files in `spend/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · the header, `{short name} · {effort} · {estimate} of {budget}`
//! - Card: Workflow · ESTIMATED SPEND, and its over-budget form
//!
//! ## Rules
//!
//! - Every phase-1 Workflow figure is a lookup in this table, and all of them
//!   exist from the moment the panel appears, before any line.
//! - The run takes its estimate, reservation and budget from the row of the
//!   applied pair, so the gate is the one the card described.
//! - A model or an effort missing from the table stops the command (the stop
//!   rule). A pair the team does not offer has no row.

pub mod build;
pub mod card;

use crate::money::{budget::Budget, pair::Pair};

pub const MARGIN: i64 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spend {
	pub budget: Budget,
	pub margin: i64,
	pub pairs: Vec<Pair>,
}
