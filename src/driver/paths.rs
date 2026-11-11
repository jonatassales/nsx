//! Where the coding agent reports its tokens
//!
//! The `usage` entry of `driver.json`: the two places in the coding agent's
//! JSON output that hold what a call used.
//!
//! Definition only: the `Paths` type, which carries `inputTokens` and
//! `outputTokens`, each a dotted path into the agent's output.
//!
//! Spec 0080.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Paths {
	pub input_tokens: String,
	pub output_tokens: String,
}
