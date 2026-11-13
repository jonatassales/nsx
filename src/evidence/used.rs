//! What one stage call used
//!
//! One call of the usage record: the stage, the skill it followed and the
//! tokens the coding agent reported for it.
//!
//! Definition only: the `Used` type, which carries the `stage`, the `skill`,
//! `inputTokens` and `outputTokens`.
//!
//! ## Rules
//!
//! - Tokens are whole numbers, or `null` when the call reported none.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Used {
	pub stage: String,
	pub skill: String,
	pub input_tokens: Option<u64>,
	pub output_tokens: Option<u64>,
}
