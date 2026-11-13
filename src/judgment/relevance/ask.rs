//! Asking the episodes question
//!
//! One function: it takes the judge, the retrieval, with its candidates,
//! their records, similarities and the threshold's verdicts, and the
//! documents read with their facts. It gives back a verdict and a reason for
//! every candidate.
//!
//! ## Reads and writes
//!
//! - calls the judge, through the `Judge` trait
//!
//! ## Rules
//!
//! - The answer is refused when a verdict differs from the threshold's, a
//!   reason is outside the closed set, or a candidate is missing.
//! - A refused answer, or a judge that cannot be reached or declines, is a
//!   stop: the plan stops before the `judgeRelevance` line and writes no
//!   draft.

use crate::contract::{
	episode::VERDICTS, episodes::Episodes, problem::Problem,
};
use crate::judgment::{
	error::Error, judge::Judge, question::Question, relevance::Relevance,
};
use crate::rag::retrieval::Retrieval;
use crate::request::documents::Documents;

impl Relevance {
	pub async fn ask(
		judge: &dyn Judge,
		retrieval: &Retrieval,
		documents: Documents<'_>,
	) -> Result<Self, Error> {
		let [kept, _dropped] = VERDICTS;
		let question = Question::Relevance {
			retrieval,
			documents,
		};
		let refused = |problem: Problem| Error::refused(&question, problem);
		let answer = question.put::<Episodes>(judge).await?;

		answer.check().map_err(refused)?;

		// The judge never contradicts the threshold, and judges no episode
		// the search did not find.
		for (index, episode) in answer.episodes.iter().enumerate() {
			let found = retrieval
				.candidates
				.iter()
				.find(|candidate| candidate.id == episode.id);
			let Some(candidate) = found else {
				return Err(refused(Problem::at(
					&format!("episodes[{index}].id"),
					"must be a candidate of the search",
				)));
			};
			let agrees = candidate.kept == (episode.verdict == kept);

			if !agrees {
				return Err(refused(Problem::at(
					&format!("episodes[{index}].verdict"),
					"must be the threshold's verdict",
				)));
			}
		}

		if answer.episodes.len() != retrieval.candidates.len() {
			return Err(refused(Problem::at(
				"episodes",
				"must judge every candidate of the search",
			)));
		}

		Ok(Self {
			judged: answer.episodes,
		})
	}
}
