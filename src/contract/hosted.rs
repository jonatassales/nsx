//! The runtime section of the run's record
//!
//! What `run.json` keeps of the runtime the run brought up: its revision,
//! its Deployments, the applied totals and whether it is ready or stopped.
//!
//! Definition only: the `Hosted` type, with the two states beside it.
//!
//! ## Rules
//!
//! - `removed` and `left` join when the runtime is stopped, and are absent
//!   before.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const READY: &str = "ready";

pub const STOPPED: &str = "stopped";

pub const STATES: [&str; 2] = [READY, STOPPED];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Hosted {
	#[schemars(range(min = 1))]
	pub revision: i64,
	pub deployments: u64,
	#[schemars(range(min = 0))]
	pub cpu_millis: i64,
	#[serde(rename = "memoryMiB")]
	#[schemars(range(min = 0))]
	pub memory_mib: i64,
	#[schemars(extend("enum" = STATES))]
	pub state: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub removed: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub left: Option<u64>,
}
