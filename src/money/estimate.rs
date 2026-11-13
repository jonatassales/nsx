//! The estimate
//!
//! What a run is expected to spend on model calls: the expected tokens of the
//! chosen effort at the chosen model's list price. A run states it before any
//! paid call.
//!
//! Definition only: the `Estimate` type: the model, the effort, the expected
//! input and output tokens, in thousands, and the amount in cents. Its
//! functions are the files in `estimate/`.
//!
//! ## Rules
//!
//! - The estimate comes before a paid call (FIN-02).
//! - The run takes its estimate from the plan's spend-table row for the applied
//!   pair, so the run's figure is the one the card showed.
//! - Amounts are whole cents.

pub mod compute;
pub mod line;

use crate::money::{cents::Cents, tokens::Tokens};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Estimate {
	pub model: String,
	pub effort: String,
	pub tokens: Tokens,
	pub amount: Cents,
}
