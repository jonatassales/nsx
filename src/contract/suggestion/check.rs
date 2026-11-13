//! Checking a suggestion answer
//!
//! One function: it takes the judge's suggestion answer and gives back the
//! first rule it breaks, as a field and a rule, or nothing.
//!
//! ## Rules
//!
//! - A model is named, the effort is one of the three, and each reason is
//!   given.
//! - Whether the model is in the price list, and whether each reason weighs
//!   what was not chosen, is checked where the question is asked, in
//!   `judgment/`.

use crate::contract::{
	fence::EFFORTS, problem::Problem, suggestion::Suggestion,
};

impl Suggestion {
	pub fn check(&self) -> Result<(), Problem> {
		if self.model.is_empty() {
			return Err(Problem::at("model", "must name a model"));
		}

		if !EFFORTS.contains(&self.effort.as_str()) {
			let rule = format!("must be one of {}", EFFORTS.join(", "));

			return Err(Problem::at("effort", &rule));
		}

		if self.effort_reason.is_empty() {
			return Err(Problem::at("effortReason", "must weigh the effort"));
		}

		if self.model_reason.is_empty() {
			return Err(Problem::at("modelReason", "must weigh the model"));
		}

		Ok(())
	}
}
