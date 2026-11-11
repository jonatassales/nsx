//! One question for the judge
//!
//! What the plan asks its judge, with everything the judge gets to answer
//! it. There are three questions and no other: what the guardrail's criteria
//! ask, which episode candidates are relevant, and which model and effort the
//! ticket warrants.
//!
//! Definition only: the `Question` type. The criteria question carries the
//! guardrail, with each criterion's rule text. The relevance question
//! carries the retrieval, with each candidate's record, similarity and
//! verdict, and the documents read. The suggestion question carries the
//! same, and the price list with its three models. Its functions are the
//! files in `question/`.
//!
//! ## Rules
//!
//! - The sizing basis is not a question: it follows from the retrieval.
//! - A question carries no secret.
//!
//! Spec 0078.

pub mod name;
pub mod prompt;
pub mod put;

use crate::harness::guardrail::Guardrail;
use crate::money::price::Price;
use crate::rag::retrieval::Retrieval;
use crate::request::documents::Documents;

#[derive(Debug, Clone, Copy)]
pub enum Question<'plan> {
	Criteria {
		guardrail: &'plan Guardrail,
	},
	Relevance {
		retrieval: &'plan Retrieval,
		documents: Documents<'plan>,
	},
	Suggestion {
		retrieval: &'plan Retrieval,
		documents: Documents<'plan>,
		prices: &'plan Price,
	},
}
