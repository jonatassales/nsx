//! Putting a question to the judge
//!
//! One function: it takes the judge, asks it the question and reads the
//! answer strictly, in the shape the contract gives that answer.
//!
//! ## Reads and writes
//!
//! - calls the judge, through the `Judge` trait
//!
//! ## Rules
//!
//! - A field the answer's shape does not define, and anything after the one
//!   JSON object, is refused.
//! - Whether the answer breaks a rule is for the asking function to say: it
//!   knows what the answer must agree with.
//!
//! Spec 0078.

use serde::de::DeserializeOwned;

use crate::contract::decode::decode;
use crate::judgment::{error::Error, judge::Judge, question::Question};

impl Question<'_> {
	pub async fn put<Answer: DeserializeOwned>(
		&self,
		judge: &dyn Judge,
	) -> Result<Answer, Error> {
		let written = judge.ask(self).await?;

		decode(&written).map_err(|cause| Error::Unread {
			question: self.name(),
			cause,
		})
	}
}
