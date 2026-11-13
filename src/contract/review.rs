//! The review report
//!
//! What the `code-review` skill writes, at the path `LANGDRIFT_REPORT` names,
//! when the Review stage ends. The run prints its `codeReview` line from these
//! figures.
//!
//! Definition only: the `Review` type. It carries `findings`, a count, and
//! `fixed`, a flag.
//!
//! ## Rules
//!
//! - Both fields are required and never `null`. No other field is allowed.
//! - The count is never negative.
//! - Whether a `false` flag fails the stage is `skills`' check.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Review report",
	description = concat!(
		"The code-review skill's report, for the stage line \"console › ",
		"code-review 2 findings · fixed\".",
	),
	extend("$id" = "https://langdrift.md/schemas/stages/review.report.schema.json"),
)]
pub struct Review {
	pub findings: u64,
	pub fixed: bool,
}
