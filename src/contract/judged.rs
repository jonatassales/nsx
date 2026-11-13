//! The judge's response
//!
//! What the console's `/api/judge` answers one question with: the validated
//! answer text, or a refusal with its reason, with what the judgment used
//! and cost and the judge's version.
//!
//! Definition only: the `Judged` type. It carries `text`, `usage` (`Used`),
//! `refused`, `reason`, `cost` (`Cost`) and `judgeVersion`.
//!
//! ## Rules
//!
//! - The cost's provenance is `console`: the console prices it, and marks
//!   it illustrative until the owner prices a plan.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{cost::Cost, used::Used};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Judge response",
	description = concat!(
		"The console's answer to one question: the validated answer text, or ",
		"a refusal, with what the judgment used and cost.",
	),
	extend("$id" = "https://langdrift.md/schemas/judge/response.schema.json"),
)]
pub struct Judged {
	pub text: String,
	pub usage: Used,
	pub refused: bool,
	pub reason: String,
	pub cost: Cost,
	#[schemars(length(min = 1))]
	pub judge_version: String,
}
