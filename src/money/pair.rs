//! One row of the spend table
//!
//! What one model at one effort would cost a run: the pair the team may pick
//! on the Workflow card, priced before the plan's first line.
//!
//! Definition only: the `Pair` type: the model's id, the effort, the expected
//! tokens, the estimate in cents, the reservation in cents and whether the
//! reservation is over the budget.
//!
//! ## Simulation
//!
//! - Card: Workflow · ESTIMATED SPEND, and its over-budget form
//!
//! ## Rules
//!
//! - Amounts are whole cents, and tokens are whole thousands.
//! - `over` is the gate the run will meet: the run pauses on a pair that is
//!   over the budget.
//!
//! Spec 0079.

use crate::money::{cents::Cents, tokens::Tokens};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
	pub model: String,
	pub effort: String,
	pub tokens: Tokens,
	pub estimate: Cents,
	pub reserved: Cents,
	pub over: bool,
}
