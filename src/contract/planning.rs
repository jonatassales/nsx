//! The planning report
//!
//! What the `grilling` skill writes, at the path `LANGDRIFT_REPORT` names,
//! when the Planning stage ends. The run prints its `grilling` line from these
//! figures, never from the agent's free output.
//!
//! Definition only: the `Planning` type. It carries `openQuestionsSettled`, a
//! count.
//!
//! ## Rules
//!
//! - The field is required and never `null`: unknown is not zero.
//! - The count is never negative.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Planning report",
	description = concat!(
		"The grilling skill's report, for the stage line \"console › ",
		"grilling   3 open questions settled\".",
	),
	extend("$id" = "https://langdrift.md/schemas/stages/planning.report.schema.json"),
)]
pub struct Planning {
	pub open_questions_settled: u64,
}
