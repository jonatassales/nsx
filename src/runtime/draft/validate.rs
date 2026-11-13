//! Validating the draft
//!
//! One function: it takes the draft and gives back the number of errors its
//! fence has against the `runtime.md` JSON Schema, which is 0, or the stop
//! that states another count.
//!
//! ## Rules
//!
//! - Every error is counted. Validation never stops at the first one.
//! - A count other than 0 stops the plan before the `validate` line, under
//!   the stop rule. The count goes into the one sentence on stderr, with
//!   each field and the rule it breaks, and no draft is written.
//! - The schema and the fence's own rules belong to `contract/`. This
//!   function counts what they report.

use crate::runtime::{draft::Draft, error::Error};

impl Draft {
	pub fn validate(&self) -> Result<usize, Error> {
		let problems = self.fence.validate();
		let errors = problems.len();

		if errors != 0 {
			let named = problems
				.iter()
				.map(|problem| format!("{} {}", problem.field, problem.rule))
				.collect::<Vec<_>>();

			return Err(Error::Invalid {
				errors,
				problems: named.join("; "),
			});
		}

		Ok(errors)
	}
}
