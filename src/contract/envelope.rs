//! An envelope, as a card shows it
//!
//! The cpu and memory the team allows a runtime, as the design doc sets them
//! and as the Analysis card's design doc row shows them.
//!
//! Definition only: the `Envelope` type. It carries `cpuMillis` and
//! `memoryMiB`.
//!
//! ## Rules
//!
//! - Whole millicores and MiB. Both are required.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Envelope {
	#[schemars(range(min = 1))]
	pub cpu_millis: i64,
	#[serde(rename = "memoryMiB")]
	#[schemars(range(min = 1))]
	pub memory_mib: i64,
}
