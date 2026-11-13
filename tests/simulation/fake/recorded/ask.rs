//! Each question gets its recorded answer
//!
//! The `Judge` the tests use: whatever is asked, the answer is the text
//! recorded for that question.

use std::future::Future;
use std::pin::Pin;

use nsx::judgment::{error::Error, judge::Judge, question::Question};

use crate::simulation::fake::recorded::Recorded;

impl Judge for Recorded {
	fn ask<'call>(
		&'call self,
		question: &'call Question<'call>,
	) -> Pin<Box<dyn Future<Output = Result<String, Error>> + 'call>> {
		let recorded = match question {
			Question::Criteria {
				..
			} => &self.criteria,
			Question::Relevance {
				..
			} => &self.episodes,
			Question::Suggestion {
				..
			} => &self.suggestion,
		};

		Box::pin(async { Ok(recorded.clone()) })
	}
}
