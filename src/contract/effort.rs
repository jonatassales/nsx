//! The effort record of `run.json`
//!
//! The `effort` section of `run.json`: at what model and effort the run
//! implemented, beside what the judge suggested, and when it was recorded.
//!
//! Definition only: the `Effort` type. It carries `appliedModel`,
//! `appliedEffort`, `suggestedModel`, `suggestedEffort` and `recordedAt`.
//!
//! ## Rules
//!
//! - The applied pair and the moment are required. The suggested pair is
//!   required and `null` when the plan carried no suggestion.
//! - No other field is allowed, and the record carries no token.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{fence::EFFORTS, nullable::nullable};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Effort {
	#[schemars(length(min = 1))]
	pub applied_model: String,
	#[schemars(extend("enum" = EFFORTS))]
	pub applied_effort: String,
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable, length(min = 1))]
	pub suggested_model: Option<String>,
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable, length(min = 1))]
	pub suggested_effort: Option<String>,
	#[schemars(length(min = 1))]
	pub recorded_at: String,
}
