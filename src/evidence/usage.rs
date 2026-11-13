//! The usage record
//!
//! What each paid stage call of a run used, as the stage driver reports it,
//! and the run's totals. `money` prices the totals into the observed spend,
//! so the figure a run shows is measured, not estimated.
//!
//! Definition only: the `Usage` type, one run's entry: the run's id, the
//! applied model and effort, one `Used` per stage-driver call in order, and
//! the run's token totals. Its functions
//! are the files in `usage/`.
//!
//! ## Rules
//!
//! - Tokens are integers, or `null` when a call reported none. The totals
//!   are `null` when any call's tokens are `null`: unknown is never zero.
//! - The dev ticket makes no call and adds no entry to the calls.
//! - A run paused at the gate makes no call and adds no entry.
//! - The record holds tokens, not money: pricing them is `money`'s.

pub mod kept;
pub mod line;
pub mod write;

use serde::{Deserialize, Serialize};

use crate::evidence::used::Used;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Usage {
	pub run: String,
	pub model: String,
	pub effort: String,
	pub calls: Vec<Used>,
	pub input_tokens: Option<u64>,
	pub output_tokens: Option<u64>,
}
