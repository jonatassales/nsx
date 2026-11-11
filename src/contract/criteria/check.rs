//! Checking a criteria answer
//!
//! One function: it takes the judge's criteria answer and gives back the
//! first rule it breaks, as a field and a rule, or nothing.
//!
//! ## Rules
//!
//! - The rules are Core's, from its Go package `judgment` (removed with
//!   ticket 0434), in its words:
//!   each entry names its criterion once, and says what the criterion
//!   requires.
//! - Whether the criteria are the guardrail's is checked where the question
//!   is asked, in `judgment/`.
//!
//! Spec 0077.

use std::collections::HashSet;

use crate::contract::{criteria::Criteria, problem::Problem};

impl Criteria {
	pub fn check(&self) -> Result<(), Problem> {
		let mut seen = HashSet::new();

		for (index, criterion) in self.criteria.iter().enumerate() {
			let named_once = !criterion.id.is_empty() && seen.insert(&criterion.id);

			if !named_once {
				return Err(Problem::at(
					&format!("criteria[{index}].id"),
					"must name a criterion once",
				));
			}

			if criterion.statement.is_empty() {
				return Err(Problem::at(
					&format!("criteria[{index}].statement"),
					"must say what the criterion requires",
				));
			}
		}

		Ok(())
	}
}
