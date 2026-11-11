//! The validation of the draft
//!
//! What validating the `runtime.md` draft found, as the Plan card's
//! VALIDATION shows it.
//!
//! Definition only: the `Validation` type. It carries `schema`, the `$id` of
//! the schema the draft was validated against, and `errors`, the count.
//!
//! ## Rules
//!
//! - Every field is required, and no other field is allowed.
//! - The count is the real one. A plan whose count is not 0 stops before it
//!   writes this.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Validation {
	#[schemars(length(min = 1))]
	pub schema: String,
	pub errors: u64,
}
