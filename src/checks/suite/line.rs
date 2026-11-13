//! The checks' two lines
//!
//! One function: it gives back the line for the suite's state. Running: the
//! name of every listed check, in the forge's order. Passed: the number
//! passed and the number listed.
//!
//! ## Simulation
//!
//! - `checks`: `checks     build · check · test · guardrails · running`
//! - `checksPassed`: `✓ checks     4 of 4 passed`
//! - Card: Workflow · PULL REQUEST's state, `checks running` then
//!   `4 of 4 checks passed`. `delivery` owns the field and shows what this
//!   element records.
//!
//! ## Rules
//!
//! - `checks` prints once, after `pullRequest`, at the first answer that
//!   lists at least one check. `checksPassed` prints before `safeSeatMet`.
//! - `checksPassed` prints only when the number passed equals the number
//!   listed. The run asks for it with `passed` true once the poll has
//!   ended, and for the running line before.
//! - The names and the count are the forge's.
//! - No other checks line exists: none for unknown, none for failed.
//! - Both lines are the same with Review ticked or not and with any stages
//!   checked. Neither prints when the gate pauses the run.
//!
//! ## Open
//!
//! - Checks that appear after the running line: the passed line counts all of
//!   them, so the two lines could differ. Until the owner decides, the lines
//!   follow the forge as it answers.

use crate::checks::suite::Suite;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Suite {
	pub fn line(&self, passed: bool) -> Line {
		let total = self.checks.len();

		if passed {
			return Line {
				id: String::from("checksPassed"),
				kind: Kind::Success,
				shape: Shape::Result {
					label: String::from("checks"),
					value: format!("{total} of {total} passed"),
				},
			};
		}

		let names = self
			.checks
			.iter()
			.map(|check| check.name.as_str())
			.collect::<Vec<_>>();

		Line {
			id: String::from("checks"),
			kind: Kind::Output,
			shape: Shape::Status {
				label: String::from("checks"),
				value: format!("{} · running", names.join(" · ")),
			},
		}
	}
}
