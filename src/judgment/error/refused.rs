//! Refusing an answer
//!
//! One function: it takes the question and the rule its answer breaks, and
//! gives back the refusal that stops the plan: the question's name, the
//! field and the rule.

use crate::contract::problem::Problem;
use crate::judgment::{error::Error, question::Question};

impl Error {
	pub fn refused(question: &Question<'_>, problem: Problem) -> Self {
		Self::Refused {
			question: question.name(),
			field: problem.field,
			rule: problem.rule,
		}
	}
}
