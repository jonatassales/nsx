//! Checking a recorded intent
//!
//! One function: it takes an intent and gives back every problem it finds, in
//! field order. An intent with no problem is one a run may apply.
//!
//! ## Rules
//!
//! - `ticket`, `runtimeId` and `model` are not empty.
//! - `revision` is at least 1.
//! - `effort` is `low`, `medium` or `high`.
//! - `human.planning` is true: Planning always has a person.
//! - `status` is `proposed` or `authorized`.
//! - It does not compare the model with the price list or the revision with
//!   the draft's. Those are the run's checks.

use crate::contract::{
	fence::EFFORTS,
	intent::{Intent, STATUSES},
	problem::Problem,
};

impl Intent {
	pub fn validate(&self) -> Vec<Problem> {
		let mut problems = Vec::new();

		if self.ticket.is_empty() {
			problems.push(Problem::at("ticket", "is missing"));
		}

		if self.runtime_id.is_empty() {
			problems.push(Problem::at("runtimeId", "is missing"));
		}

		if self.revision < 1 {
			problems.push(Problem::at("revision", "must be at least 1"));
		}

		if self.model.is_empty() {
			problems.push(Problem::at("model", "is missing"));
		}

		if !EFFORTS.contains(&self.effort.as_str()) {
			let rule = format!("must be one of {}", EFFORTS.join(", "));

			problems.push(Problem::at("effort", &rule));
		}

		if !self.human.planning {
			problems.push(Problem::at(
				"human.planning",
				"must have a person in the loop",
			));
		}

		if !STATUSES.contains(&self.status.as_str()) {
			problems.push(Problem::at("status", "must be proposed or authorized"));
		}

		problems
	}
}
