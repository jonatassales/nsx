//! Asking the suggestion question
//!
//! One function: it takes the judge, the retrieval with its cited episodes,
//! the documents read with their facts, and the candidates: the models of
//! the price list the team offers, which carry their short names, each with
//! its efforts. It gives back the suggestion: a model, an effort, an effort
//! reason and a model reason.
//!
//! ## Reads and writes
//!
//! - calls the judge, through the `Judge` trait
//!
//! ## Rules
//!
//! - The answer is refused when the model is not a candidate, or the effort
//!   is not one of that candidate's efforts.
//! - It is refused when the effort reason does not name each effort of the
//!   chosen model not chosen, in ascending order. A name counts as a whole
//!   word: `follow` does not name `low`.
//! - It is refused when a reason is not one line of plain text.
//! - It is refused when the model reason does not name each candidate model
//!   not chosen, by its short name, in ascending order of list price.
//! - A reason with nothing left to weigh, because the team offers one model
//!   or one effort, names nothing: the question asks it to say that the
//!   team offers only that one, and nsx checks that it is one plain line.
//! - A refused answer, or a judge that cannot be reached or declines, is a
//!   stop: the plan stops before the `weighEffort` line and writes no draft.

use crate::contract::{
	plain::plain, problem::Problem, suggestion::Suggestion as Answer,
};
use crate::judgment::{
	error::Error, judge::Judge, question::Question, suggestion::Suggestion,
};
use crate::money::candidates::Candidates;
use crate::rag::retrieval::Retrieval;
use crate::request::documents::Documents;

impl Suggestion {
	pub async fn ask(
		judge: &dyn Judge,
		retrieval: &Retrieval,
		documents: Documents<'_>,
		candidates: &Candidates,
	) -> Result<Self, Error> {
		// Whether a reason names each of these as whole words, in this order.
		fn weighs(reason: &str, names: &[&str]) -> bool {
			let apart = |edge: Option<char>| !edge.is_some_and(char::is_alphanumeric);
			let mut from = 0;

			names.iter().all(|name| {
				let named = reason[from..]
					.match_indices(name)
					.map(|(at, _name)| from + at)
					.find(|at| {
						let before = reason[..*at].chars().next_back();
						let after = reason[at + name.len()..].chars().next();

						apart(before) && apart(after)
					});
				let Some(at) = named else {
					return false;
				};

				from = at + name.len();

				true
			})
		}

		let question = Question::Suggestion {
			retrieval,
			documents,
			candidates,
		};
		let refused = |field: &str, rule: &str| {
			Error::refused(&question, Problem::at(field, rule))
		};
		let answer = question.put::<Answer>(judge).await?;

		answer
			.check()
			.map_err(|problem| Error::refused(&question, problem))?;

		let chosen = candidates
			.models
			.iter()
			.find(|candidate| candidate.model.id == answer.model);
		let Some(chosen) = chosen else {
			return Err(refused("model", "must be a model the team offers"));
		};

		if !chosen.efforts.contains(&answer.effort) {
			return Err(refused(
				"effort",
				"must be an effort the team offers for the model",
			));
		}

		let mut others = candidates
			.models
			.iter()
			.map(|candidate| &candidate.model)
			.filter(|model| model.id != answer.model)
			.collect::<Vec<_>>();

		others.sort_by_key(|model| (model.input_cents, model.output_cents));

		let efforts = chosen
			.efforts
			.iter()
			.map(String::as_str)
			.filter(|effort| *effort != answer.effort)
			.collect::<Vec<_>>();
		let models = others
			.iter()
			.map(|model| model.short_name.as_str())
			.collect::<Vec<_>>();

		// Both reasons are printed, so each is one plain line.
		if !plain(&answer.effort_reason) {
			return Err(refused("effortReason", "must be one line of plain text"));
		}

		if !plain(&answer.model_reason) {
			return Err(refused("modelReason", "must be one line of plain text"));
		}

		if !weighs(&answer.effort_reason, &efforts) {
			return Err(refused(
				"effortReason",
				"must weigh each effort not chosen, from the lowest",
			));
		}

		if !weighs(&answer.model_reason, &models) {
			return Err(refused(
				"modelReason",
				"must weigh each model not chosen, by its short name, from the lowest list price",
			));
		}

		Ok(Self {
			model: answer.model,
			effort: answer.effort,
			effort_reason: answer.effort_reason,
			model_reason: answer.model_reason,
		})
	}
}
