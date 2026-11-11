//! The pull request of the record's delivery
//!
//! What `run.json` keeps of the pull request the run opened: its number, its
//! title, its state and, once the forge lists them, its checks.
//!
//! Definition only: the `Pull` type, with the three states beside it.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::checks::Checks;

pub const DRAFT: &str = "draft";

pub const REVIEW: &str = "ready for review";

pub const MERGE: &str = "ready to merge";

pub const STATES: [&str; 3] = [DRAFT, REVIEW, MERGE];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Pull {
	#[schemars(range(min = 1))]
	pub number: u64,
	#[schemars(length(min = 1))]
	pub title: String,
	#[schemars(extend("enum" = STATES))]
	pub state: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Checks")]
	pub checks: Option<Checks>,
}
