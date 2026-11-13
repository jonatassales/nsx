//! Asking the criteria question
//!
//! One function: it takes the judge and the guardrail's criteria with their
//! rule text, and gives back one statement per criterion.
//!
//! ## Reads and writes
//!
//! - calls the judge, through the `Judge` trait
//!
//! ## Rules
//!
//! - The answer is refused if it skips a criterion or names one that is not in
//!   the guardrail, or if a statement is not one line of plain text.
//! - A refused answer, or a judge that cannot be reached or declines, is a
//!   stop: the plan stops before the `judgeGuardrail` line and writes no
//!   draft.

use crate::contract::{
	criteria::Criteria as Answer, plain::plain, problem::Problem,
};
use crate::harness::guardrail::Guardrail;
use crate::judgment::{
	criteria::Criteria, error::Error, judge::Judge, question::Question,
};

impl Criteria {
	pub async fn ask(
		judge: &dyn Judge,
		guardrail: &Guardrail,
	) -> Result<Self, Error> {
		let question = Question::Criteria {
			guardrail,
		};
		let refused = |problem: Problem| Error::refused(&question, problem);
		let answer = question.put::<Answer>(judge).await?;

		answer.check().map_err(refused)?;

		// Every criterion is judged, in the guardrail's order: none skipped
		// and none invented. A statement is printed, so it is one plain line.
		for (index, criterion) in answer.criteria.iter().enumerate() {
			let asked = guardrail.criteria.get(index).map(|asked| &asked.id);

			if asked != Some(&criterion.id) {
				return Err(refused(Problem::at(
					&format!("criteria[{index}].id"),
					"must be the guardrail's next criterion",
				)));
			}

			if !plain(&criterion.statement) {
				return Err(refused(Problem::at(
					&format!("criteria[{index}].statement"),
					"must be one line of plain text",
				)));
			}
		}

		if answer.criteria.len() != guardrail.criteria.len() {
			return Err(refused(Problem::at(
				"criteria",
				"must judge every criterion of the guardrail",
			)));
		}

		Ok(Self {
			stated: answer.criteria,
		})
	}
}
