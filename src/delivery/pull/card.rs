//! The pull request in the run's record
//!
//! One function: it takes the pull request and the run's record, and fills
//! the delivery section's pull request: its number, its title and its state.
//!
//! ## Simulation
//!
//! - Card: Workflow · PULL REQUEST, `#19` and, from the ready line,
//!   `ready for review` or `ready to merge`
//!
//! ## Rules
//!
//! - The checks' state the section already holds stays.

use crate::contract::{pull::Pull as Recorded, run::Run};
use crate::delivery::pull::Pull;

impl Pull {
	pub fn card(&self, run: &mut Run) {
		let delivery = run.delivery.get_or_insert_with(Default::default);
		let checks = delivery
			.pull_request
			.take()
			.and_then(|recorded| recorded.checks);

		delivery.pull_request = Some(Recorded {
			number: self.number,
			title: self.title.clone(),
			state: String::from(self.state.word()),
			checks,
		});
	}
}
