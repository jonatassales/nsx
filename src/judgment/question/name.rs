//! The name of a question
//!
//! One function: it gives back the word a refusal names the question by,
//! the one its answer's schema carries: `criteria`, `episodes` or
//! `suggestion`.

use crate::judgment::question::Question;

impl Question<'_> {
	pub fn name(&self) -> &'static str {
		match self {
			Self::Criteria {
				..
			} => "criteria",
			Self::Relevance {
				..
			} => "episodes",
			Self::Suggestion {
				..
			} => "suggestion",
		}
	}
}
