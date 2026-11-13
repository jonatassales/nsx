//! Every question is refused
//!
//! One function: whatever the question, it gives back the same refusal, which
//! stops the plan before the line the answer would feed.

use std::future::Future;
use std::pin::Pin;

use crate::judgment::{
	absent::Absent, error::Error, judge::Judge, question::Question,
};

impl Judge for Absent {
	fn ask<'call>(
		&'call self,
		_question: &'call Question<'call>,
	) -> Pin<Box<dyn Future<Output = Result<String, Error>> + 'call>> {
		Box::pin(async { Err(Error::Unconnected) })
	}
}
