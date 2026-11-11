//! An amount of cpu and memory
//!
//! A pair of figures the Plan card shows together: the proposed totals, or
//! the headroom the bound leaves.
//!
//! Definition only: the `Amount` type. It carries `cpuMillis` and
//! `memoryMiB`.
//!
//! ## Rules
//!
//! - Both are whole numbers, never negative: a headroom may be zero, and a
//!   plan over the bound stops before it writes one.
//! - No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Amount {
	#[schemars(range(min = 0))]
	pub cpu_millis: i64,
	#[serde(rename = "memoryMiB")]
	#[schemars(range(min = 0))]
	pub memory_mib: i64,
}
