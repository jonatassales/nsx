//! Letting implement start
//!
//! One function: it takes the effort record, if the run has one, and the
//! applied model and effort, and says whether the implement call may start.
//!
//! ## Rules
//!
//! - Implement starts only when the record exists and its model and effort
//!   equal the applied choice. Anything else is a stop: implement is never
//!   called.
//! - The hook is a gate, not a note: the implement call asks here before the
//!   coding agent starts, and no other path reaches the agent.

use crate::hook::{effort::Effort, error::Error};

impl Effort {
	pub fn permit(
		recorded: Option<&Self>,
		model: &str,
		effort: &str,
	) -> Result<(), Error> {
		let Some(record) = recorded else {
			return Err(Error::Unrecorded);
		};

		let matches = record.model == model && record.effort == effort;

		if !matches {
			return Err(Error::Mismatch {
				recorded: format!("{} at {}", record.model, record.effort),
				applied: format!("{model} at {effort}"),
			});
		}

		Ok(())
	}
}
