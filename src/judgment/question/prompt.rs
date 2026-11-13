//! A question, in words
//!
//! One function: it gives back the question as the console's judge reads it:
//! the instructions and the facts, as two texts.
//!
//! ## Rules
//!
//! - The words are nsx's own, written for the three answers it reads.
//! - The instructions state the answer's shape and every rule nsx checks the
//!   answer against, so that a judge that follows them is never refused.
//! - The facts are only what the plan has read: the guardrail's criteria,
//!   the linked documents' facts, the candidates of the search and the
//!   price list. No secret and no file's whole text is sent.

use crate::contract::{episode::REASONS, fence::EFFORTS};
use crate::judgment::question::Question;

const SHAPE: &str =
	"Answer with one JSON object and nothing else: no prose, no code fence.";

impl Question<'_> {
	pub fn prompt(&self) -> (String, String) {
		match self {
			Self::Criteria {
				guardrail,
			} => {
				let system = format!(
					"You state what each criterion of a team's guardrail requires of a plan. {SHAPE} Its shape is {{\"criteria\": [{{\"id\": string, \"statement\": string}}]}}. Give every criterion listed, once, in the order listed, with its id unchanged. Each statement is one short line of plain text, a few words, lower case, saying what the criterion requires, such as \"estimate before a paid call\"."
				);
				let listed = guardrail
					.criteria
					.iter()
					.map(|criterion| format!("- {}: {}", criterion.id, criterion.rule))
					.collect::<Vec<_>>();
				let prompt = format!(
					"The guardrail {} lists these criteria:\n{}",
					guardrail.name,
					listed.join("\n")
				);

				(system, prompt)
			}
			Self::Relevance {
				retrieval,
				documents,
			} => {
				let [shared, differing @ ..] = REASONS;
				let system = format!(
					"You say why each earlier run of a project does or does not bear on a new ticket. {SHAPE} Its shape is {{\"episodes\": [{{\"id\": string, \"verdict\": \"kept\" or \"dropped\", \"reason\": string}}]}}. Judge every candidate listed, once, with its id unchanged. The verdict is already decided for each candidate and is given to you: repeat it. For a kept candidate the reason is exactly \"{shared}\". For a dropped candidate the reason is exactly one of: {}.",
					differing
						.iter()
						.map(|reason| format!("\"{reason}\""))
						.collect::<Vec<_>>()
						.join(", ")
				);
				let candidates = retrieval
					.candidates
					.iter()
					.map(|candidate| {
						let record = retrieval
							.records
							.iter()
							.find(|record| record.id == candidate.id);
						let stacks = record
							.map(|record| {
								record
									.components
									.iter()
									.map(|part| format!("{} {}", part.stack, part.role))
									.collect::<Vec<_>>()
									.join(", ")
							})
							.unwrap_or_default();
						let verdict = if candidate.kept { "kept" } else { "dropped" };

						format!(
							"- {} ({}): verdict {verdict}; components: {stacks}",
							candidate.id, candidate.slug
						)
					})
					.collect::<Vec<_>>();
				let prompt = format!(
					"The ticket's use case is \"{}\". Its spec names {} components.\nThe candidates:\n{}",
					documents.prd.use_case,
					documents.spec.components.len(),
					candidates.join("\n")
				);

				(system, prompt)
			}
			Self::Suggestion {
				retrieval,
				documents,
				prices,
			} => {
				let mut models = prices.models.iter().collect::<Vec<_>>();

				models.sort_by_key(|model| (model.input_cents, model.output_cents));

				let system = format!(
					"You suggest the model and the effort a coding agent should use for a ticket. {SHAPE} Its shape is {{\"model\": string, \"effort\": string, \"effortReason\": string, \"modelReason\": string}}. The model is the id of one model listed. The effort is one of {}. The effort reason is one line of plain text that names each of the two efforts you did not choose, as whole words, the lower one first, and says why not, such as \"low skips the broker wiring · high is not warranted by 5 components\". The model reason is one line of plain text that names every model you did not choose by its short name exactly as listed, from the cheapest, and says why not.",
					EFFORTS.join(", ")
				);
				let listed = models
					.iter()
					.map(|model| {
						format!(
							"- id {}, short name {}: {} cents input and {} cents output per million tokens",
							model.id,
							model.short_name,
							model.input_cents.0,
							model.output_cents.0
						)
					})
					.collect::<Vec<_>>();
				let prompt = format!(
					"The ticket's use case is \"{}\". Its spec names {} components and {} acceptance criteria. The bound is {} millicores and {} MiB. {} earlier runs of the project bear on it.\nThe models, from the cheapest:\n{}",
					documents.prd.use_case,
					documents.spec.components.len(),
					documents.spec.criteria,
					documents.design.cpu_millis,
					documents.design.memory_mib,
					retrieval.kept,
					listed.join("\n")
				);

				(system, prompt)
			}
		}
	}
}
