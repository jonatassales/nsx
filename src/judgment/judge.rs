//! The judge
//!
//! The model that reasons for the plan. It is asked three questions: the
//! guardrail's criteria, the episode candidates, and the model and effort to
//! suggest. Where it runs is open, so it sits behind this one trait: deciding
//! that changes one implementation and no line.
//!
//! Definition only: the trait `Judge`, with its one function, so that tests
//! can replace it with a fake: it takes a `Question` and gives back the
//! judge's answer as it wrote it, a JSON text the asking function reads and
//! checks.
//!
//! ## Rules
//!
//! - The judge only recommends: it never decides. The team authorizes.
//! - Nothing about it is priced, reserved or gated in the plan, and its
//!   answers carry no cost and no usage.
//! - No test reaches a real model: the golden test uses a recorded judge.
//! - The sizing basis is not a question: it follows from the retrieval.

use std::future::Future;
use std::pin::Pin;

use crate::judgment::{error::Error, question::Question};

pub trait Judge {
	fn ask<'call>(
		&'call self,
		question: &'call Question<'call>,
	) -> Pin<Box<dyn Future<Output = Result<String, Error>> + 'call>>;
}
