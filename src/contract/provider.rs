//! A model provider
//!
//! Definition only: the `Provider` type and the three providers a team may
//! offer models from: `anthropic`, `openai` and `gemini`.
//!
//! ## Rules
//!
//! - The price list holds only Anthropic's models, so a model of another
//!   provider is never priced, and never priced at zero.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
#[schemars(inline)]
pub enum Provider {
	Anthropic,
	Openai,
	Gemini,
}
