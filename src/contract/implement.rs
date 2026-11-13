//! The implement report
//!
//! What the `implement` skill writes, at the path `LANGDRIFT_REPORT` names,
//! when the Implement stage ends. The run prints its `implement` line from
//! these figures.
//!
//! Definition only: the `Implement` type. It carries the `method`, two counts,
//! `components` and `files`, and two flags, `boundaryOnly` and
//! `stoppedAtReservedPart`.
//!
//! ## Rules
//!
//! - Every field is required and never `null`. No other field is allowed.
//! - `method` is `tdd`. No other method is accepted.
//! - The counts are never negative.
//! - Whether a `false` flag fails the stage is `skills`' check.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// The method is text, so that the stage's check can name another method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Implement report",
	description = concat!(
		"The implement skill's report, for the stage line \"console › ",
		"implement  tdd · 5 components · 14 files · boundary only · stops at ",
		"the reserved part\".",
	),
	extend("$id" = "https://langdrift.md/schemas/stages/implement.report.schema.json"),
)]
pub struct Implement {
	#[schemars(extend("const" = "tdd"))]
	pub method: String,
	pub components: u64,
	pub files: u64,
	pub boundary_only: bool,
	pub stopped_at_reserved_part: bool,
}
