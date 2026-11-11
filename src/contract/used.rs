//! What a judgment used
//!
//! Definition only: the `Used` type: the input and output tokens of one
//! judgment.
//!
//! Spec 0078.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Used {
	pub input_tokens: u64,
	pub output_tokens: u64,
}
