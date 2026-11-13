//! The Plan card's evidence
//!
//! The EVIDENCE row of the Plan card: "runtime.md draft · revision 1 ·
//! proposed to the console". It says which file holds the plan, in what state,
//! at which revision, and to whom it was proposed.
//!
//! Definition only: the `Evidence` type. It carries the `file`, its `state`,
//! its `revision` and `proposedTo`.
//!
//! ## Rules
//!
//! - A value not known yet is `null`, never an empty text or zero.
//! - `revision` is at least 1.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Evidence {
	pub file: Option<String>,
	pub state: Option<String>,
	#[schemars(range(min = 1))]
	pub revision: Option<i64>,
	pub proposed_to: Option<String>,
}
