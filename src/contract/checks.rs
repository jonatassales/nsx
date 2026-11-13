//! The checks of the record's pull request
//!
//! What `run.json` keeps of the client's CI checks: the state PULL REQUEST
//! shows, and nothing of each check.
//!
//! Definition only: the `Checks` type, with the two states beside it.
//!
//! ## Rules
//!
//! - `passed` and `total` join when the checks have passed, and are absent
//!   while they run.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const RUNNING: &str = "running";

pub const PASSED: &str = "passed";

pub const STATES: [&str; 2] = [RUNNING, PASSED];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Checks {
	#[schemars(extend("enum" = STATES))]
	pub state: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub passed: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub total: Option<u64>,
}
