//! The criteria answer
//!
//! The judge's answer to its first question: what each criterion of the
//! guardrail requires, as one short statement. It feeds the `judgeGuardrail`
//! line and the guardrail row of the Analysis card.
//!
//! Definition only: the `Criteria` type. It carries `criteria`: one entry per
//! criterion, each with its `id`, such as `FIN-02`, and its `statement`.
//!
//! ## Rules
//!
//! - Each `id` is not empty and appears once.
//! - Each `statement` is not empty. It is free text.
//! - No other field is allowed.
//! - That no criterion is skipped and none invented is checked where the
//!   question is asked, in `judgment` (spec 0051).
//!
//! Spec 0077.

pub mod check;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::criterion::Criterion;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(
	title = "Criteria answer",
	description = concat!(
		"The guardrail criteria the judge finds apply to the ticket, each ",
		"with its id and what it requires, as \"judging · FIN-02 estimate ",
		"before a paid call · …\" prints them.",
	),
	extend("$id" = "https://langdrift.md/schemas/judgment/criteria.answer.schema.json"),
)]
pub struct Criteria {
	pub criteria: Vec<Criterion>,
}
