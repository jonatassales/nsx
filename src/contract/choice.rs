//! The choice a Play authorizes
//!
//! What the team authorizes when it presses Play: the revision, the model,
//! the effort and the checkpoints of the intent the console holds, as nsx
//! last reported it, in the hand-off's own shapes, so that it compares field
//! by field with `intent.json`.
//!
//! Definition only: the `Choice` type. It carries `revision`, `model`,
//! `effort` and `human`.
//!
//! ## Rules
//!
//! - It never carries who pressed or when: that evidence stays in the
//!   console.
//! - No other field is allowed.
//!
//! Spec 0077.

pub mod of;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{fence::EFFORTS, human::Human};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Choice {
	#[schemars(range(min = 1))]
	pub revision: i64,
	#[schemars(length(min = 1))]
	pub model: String,
	#[schemars(extend("enum" = EFFORTS))]
	pub effort: String,
	pub human: Human,
}
