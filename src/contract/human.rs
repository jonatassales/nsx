//! Who takes part in each stage
//!
//! The HUMAN IN THE LOOP ticks of the Workflow card, as `intent.json` records
//! them: per stage, whether a person takes part in it.
//!
//! Definition only: the `Human` type. It carries one flag for each of
//! `planning`, `spec`, `tickets`, `implement` and `review`.
//!
//! ## Rules
//!
//! - Every flag is required, and no other field is allowed.
//! - `planning` is always `true`: the simulation's Planning cannot be
//!   unchecked. The intent's `validate` names a `false`.
//!
//! Spec 0077.

pub mod ticked;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Human {
	#[schemars(extend("const" = true))]
	pub planning: bool,
	pub spec: bool,
	pub tickets: bool,
	pub implement: bool,
	pub review: bool,
}
