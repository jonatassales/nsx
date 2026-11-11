//! Asking the suggestion question
//!
//! One function: it takes the judge, the retrieval with its cited episodes,
//! the documents read with their facts, and the price list, whose models
//! carry their short names (spec 0056). It gives back the suggestion: a
//! model, an effort, an effort reason and a model reason.
//!
//! ## Reads and writes
//!
//! - calls the judge, through the `Judge` trait
//!
//! ## Rules
//!
//! - The answer is refused when the model or the effort is not one of the
//!   three.
//! - It is refused when the effort reason does not name each effort not
//!   chosen, in ascending order. A name counts as a whole word: `follow` does
//!   not name `low`.
//! - It is refused when a reason is not one line of plain text.
//! - It is refused when the model reason does not name each model not chosen,
//!   by its short name, in ascending order of list price.
//! - A refused answer, or a judge that cannot be reached or declines, is a
//!   stop: the plan stops before the `weighEffort` line and writes no draft.
//!
//! Spec 0078.

use crate::contract::{
	fence::EFFORTS, plain::plain, problem::Problem,
	suggestion::Suggestion as Answer,
};
use crate::judgment::{
	error::Error, judge::Judge, question::Question, suggestion::Suggestion,
};
use crate::money::price::Price;
use crate::rag::retrieval::Retrieval;
use crate::request::documents::Documents;

impl Suggestion {
	pub async fn ask(
		judge: &dyn Judge,
		retrieval: &Retrieval,
		documents: Documents<'_>,
		prices: &Price,
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
			prices,
		};
		let refused = |field: &str, rule: &str| {
			Error::refused(&question, Problem::at(field, rule))
		};
		let answer = question.put::<Answer>(judge).await?;

		answer
			.check()
			.map_err(|problem| Error::refused(&question, problem))?;

		if !prices.models.iter().any(|model| model.id == answer.model) {
			return Err(refused("model", "must be a model of the price list"));
		}

		let mut others = prices
			.models
			.iter()
			.filter(|model| model.id != answer.model)
			.collect::<Vec<_>>();

		others.sort_by_key(|model| (model.input_cents, model.output_cents));

		let efforts = EFFORTS
			.into_iter()
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
