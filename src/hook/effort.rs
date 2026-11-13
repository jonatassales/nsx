//! The effort record
//!
//! The record that says at what model and effort a run implemented: the pair
//! the team applied beside the pair the judge suggested, so the owner can see
//! whether the team followed the recommendation. It is token-free evidence,
//! kept in the run's record.
//!
//! Definition only: the `Effort` type: the applied model and effort, the
//! suggested model and effort, unknown when the plan carried none, and when
//! it was recorded. Its functions are the
//! files in `effort/`.
//!
//! ## Rules
//!
//! - The implement call starts only when the record exists and its model and
//!   effort equal the applied choice.
//! - The model is the price list's id, for example `claude-sonnet-5-5`.
//! - nsx records the applied choice: it never chooses it.
//!
//! ## Open
//!
//! - Should nsx also satisfy an effort guard inside the client's coding agent,
//!   and how would the call's session be bound to this record? The simulation
//!   shows only nsx's own gate.

pub mod line;
pub mod permit;
pub mod record;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effort {
	pub model: String,
	pub effort: String,
	pub suggested_model: Option<String>,
	pub suggested_effort: Option<String>,
	pub recorded_at: String,
}
