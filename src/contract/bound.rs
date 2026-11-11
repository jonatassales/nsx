//! The bound on the Plan card
//!
//! What the team allows the runtime, as the Plan card's BOUNDED shows it.
//!
//! Definition only: the `Bound` type. It carries `cpuMillis`, `memoryMiB`
//! and `source`, the design doc's path from the project's root.
//!
//! ## Rules
//!
//! - Every field is required, and no other field is allowed.
//! - The bound is at least 1 millicore and 1 MiB.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Bound {
	#[schemars(range(min = 1))]
	pub cpu_millis: i64,
	#[serde(rename = "memoryMiB")]
	#[schemars(range(min = 1))]
	pub memory_mib: i64,
	#[schemars(length(min = 1))]
	pub source: String,
}
