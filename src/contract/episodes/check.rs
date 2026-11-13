//! Checking an episodes answer
//!
//! One function: it takes the judge's episodes answer and gives back the
//! first rule it breaks, as a field and a rule, or nothing.
//!
//! ## Rules
//!
//! - An id of the form `ep-NNNN`, each episode judged once, a verdict of
//!   `kept` or `dropped`, and a reason that fits the verdict.
//! - Whether the verdicts are the threshold's is checked where the question
//!   is asked, in `judgment/`.

use std::collections::HashSet;

use crate::contract::{
	episode::{REASONS, VERDICTS},
	episodes::Episodes,
	problem::Problem,
};

impl Episodes {
	pub fn check(&self) -> Result<(), Problem> {
		let [kept, dropped] = VERDICTS;
		let [shared, differing @ ..] = REASONS;
		let mut seen = HashSet::new();

		for (index, episode) in self.episodes.iter().enumerate() {
			let field = |name: &str| format!("episodes[{index}].{name}");
			let digits = episode.id.strip_prefix("ep-").unwrap_or_default();
			let numbered =
				digits.len() == 4 && digits.chars().all(|digit| digit.is_ascii_digit());
			let fits = match episode.verdict.as_str() {
				verdict if verdict == kept => episode.reason == shared,
				_ => differing.contains(&episode.reason.as_str()),
			};

			if !numbered {
				return Err(Problem::at(
					&field("id"),
					"must be an episode id, ep-NNNN",
				));
			}

			if !seen.insert(&episode.id) {
				return Err(Problem::at(&field("id"), "must judge each episode once"));
			}

			if episode.verdict != kept && episode.verdict != dropped {
				return Err(Problem::at(&field("verdict"), "must be kept or dropped"));
			}

			if !fits {
				let rule = format!(
					"must be the verdict's reason: {shared} when kept; {} when dropped",
					differing.join(", ")
				);

				return Err(Problem::at(&field("reason"), &rule));
			}
		}

		Ok(())
	}
}
