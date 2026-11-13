//! The suggestion answer
//!
//! The judge's answer to its third question: the model and the effort the
//! ticket warrants, and why. It feeds the `weighEffort`, `weighModel` and
//! `suggest` lines, and SUGGESTED MODEL and SUGGESTED EFFORT on the Analysis
//! card.
//!
//! Definition only: the `Suggestion` type. It carries the `model`, the
//! `effort`, the `effortReason` and the `modelReason`.
//!
//! ## Rules
//!
//! - All four fields are required and not empty. No other field is allowed.
//! - `model` is a price-list id. `effort` is `low`, `medium` or `high`.
//! - The two reasons are free text, each in its own field. There is no free
//!   list of reasons.
//! - The suggestion is a recommendation. The team authorizes the choice.

pub mod check;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::fence::EFFORTS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Suggestion answer",
	description = concat!(
		"The model and the effort the judge suggests, and why, as \"weighing ",
		"effort · …\", \"weighing model · …\" and \"✓ suggests …\" print ",
		"them. The suggestion is a recommendation; the team authorizes the ",
		"choice.",
	),
	extend("$id" = "https://langdrift.md/schemas/judgment/suggestion.answer.schema.json"),
)]
pub struct Suggestion {
	#[schemars(length(min = 1))]
	pub model: String,
	#[schemars(extend("enum" = EFFORTS))]
	pub effort: String,
	#[schemars(length(min = 1))]
	pub effort_reason: String,
	#[schemars(length(min = 1))]
	pub model_reason: String,
}
