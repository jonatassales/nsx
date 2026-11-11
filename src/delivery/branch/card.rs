//! The branch in the run's record
//!
//! One function: it takes the branch and the run's record, and fills the
//! delivery section's `branch` and, once pushed, its `commit`.
//!
//! ## Simulation
//!
//! - Card: Workflow · PULL REQUEST, whose `#19 · 3f9c2e1` reads the commit
//!
//! ## Rules
//!
//! - `commit` is the short hash, as the card shows it.
//! - What the section already holds of the pull request stays.
//!
//! Spec 0081.

use crate::contract::run::Run;
use crate::delivery::branch::Branch;

impl Branch {
	pub fn card(&self, run: &mut Run) {
		let delivery = run.delivery.get_or_insert_with(Default::default);

		delivery.branch = Some(self.name.clone());
		delivery.commit = self.short().map(String::from);
	}
}
