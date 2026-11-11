//! The spec report
//!
//! What the `to-spec` skill writes, at the path `LANGDRIFT_REPORT` names, when
//! the Spec stage ends. The run prints its `toSpec` line from these figures.
//!
//! Definition only: the `Spec` type. It carries `spec`, the path of the spec,
//! `revised`, a flag, and `acceptanceCriteria`, a count.
//!
//! ## Rules
//!
//! - Every field is required and never `null`. No other field is allowed.
//! - `spec` is a clean path relative to the project. It never climbs out.
//! - The count is never negative.
//! - The line prints the spec's short id, not this path: the first two
//!   dash-separated parts of the file name, such as `0011-SPEC`.
//! - Whether a `false` flag or another spec fails the stage is `skills`'
//!   check (spec 0059).
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Spec report",
	description = concat!(
		"The to-spec skill's report, for the stage line \"console › to-spec    ",
		"0011-SPEC revised · 3 acceptance criteria\". spec is the spec's ",
		"project-relative path; the line prints the first two dash-separated ",
		"parts of its file name. A clean path that never climbs is a rule ",
		"JSON Schema cannot fully express; nsx enforces it.",
	),
	extend("$id" = "https://langdrift.md/schemas/stages/spec.report.schema.json"),
)]
pub struct Spec {
	#[schemars(length(min = 1))]
	pub spec: String,
	pub revised: bool,
	pub acceptance_criteria: u64,
}
