//! The checks in the run's record
//!
//! One function: it takes the suite, whether its poll has ended and the
//! run's record, and fills the checks of the delivery section's pull
//! request.
//!
//! ## Simulation
//!
//! - Card: Workflow · PULL REQUEST's state, `checks running` then
//!   `4 of 4 checks passed`
//!
//! ## Rules
//!
//! - `state` is `running`, then `passed` with `passed` and `total`. The two
//!   counts are absent while the checks run.
//! - A record with no pull request yet is left as it is: the checks belong
//!   to one.

use crate::checks::suite::Suite;
use crate::contract::{
	checks::{Checks, PASSED, RUNNING},
	run::Run,
};

impl Suite {
	pub fn card(&self, passed: bool, run: &mut Run) {
		let pull = run
			.delivery
			.as_mut()
			.and_then(|delivery| delivery.pull_request.as_mut());
		let Some(pull) = pull else {
			return;
		};

		let total = self.checks.len() as u64;
		let state = if passed { PASSED } else { RUNNING };

		pull.checks = Some(Checks {
			state: String::from(state),
			passed: passed.then_some(total),
			total: passed.then_some(total),
		});
	}
}
