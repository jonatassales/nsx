//! The suggestion of the Analysis card
//!
//! The `suggestion` section of `plan.json`: the model and the effort the
//! judge suggests, and why, as SUGGESTED MODEL and SUGGESTED EFFORT show them.
//!
//! Definition only: the `Recommendation` type. It carries the `model`, the
//! `effort`, the `effortReason`, the `modelReason` and `by`.
//!
//! ## Rules
//!
//! - Every field is absent until the `suggest` line prints: the card shows
//!   placeholders until then.
//! - `by` has one value, `recommendation`: the team authorizes, never the
//!   judge.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::fence::EFFORTS;

pub const BY: &str = "recommendation";

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Recommendation {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String")]
	pub model: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("enum" = EFFORTS))]
	pub effort: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String")]
	pub effort_reason: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String")]
	pub model_reason: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("const" = BY))]
	pub by: Option<String>,
}
