//! Every question is refused for lack of the team's key

use std::future::Future;
use std::pin::Pin;

use nsx::judgment::{error::Error, judge::Judge, question::Question};

use crate::simulation::fake::keyless_judge::KeylessJudge;

impl Judge for KeylessJudge {
	fn ask<'call>(
		&'call self,
		_question: &'call Question<'call>,
	) -> Pin<Box<dyn Future<Output = Result<String, Error>> + 'call>> {
		Box::pin(async { Err(Error::KeyMissing) })
	}
}
