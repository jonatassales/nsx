//! What an episode's run measured
//!
//! The peaks of cpu and memory a past run measured, when it measured them.
//!
//! Definition only: the `Measured` type: the cpu peak in millicores and the
//! memory peak in MiB.
//!
//! ## Rules
//!
//! - A run that measured neither has no `Measured` at all: unknown is never
//!   zero.
//!
//! Spec 0078.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Measured {
	pub cpu_millis: i64,
	#[serde(rename = "memoryMiB")]
	pub memory_mib: i64,
}
