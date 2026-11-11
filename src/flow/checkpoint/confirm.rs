//! Reading the owner's answer
//!
//! One function: it takes the line the owner entered and says whether it
//! confirms the checkpoint.
//!
//! ## Rules
//!
//! - `yes` or `y` confirms, in any case, with spaces around it or not.
//! - Anything else declines, an empty line too.
//!
//! Spec 0080.

use crate::flow::checkpoint::Checkpoint;

impl Checkpoint {
	pub fn confirm(answer: &str) -> bool {
		let entered = answer.trim().to_lowercase();

		matches!(entered.as_str(), "yes" | "y")
	}
}
