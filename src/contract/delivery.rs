//! What the run delivers, as `run.json` records it
//!
//! The `delivery` of `run.json`: the branch, the commit and the pull request
//! the run opened, as the Workflow card's PULL REQUEST row reads them.
//!
//! Definition only: the `Delivery` type. It carries the `branch`, the
//! `commit` and the `pullRequest`.
//!
//! ## Rules
//!
//! - `branch` and `commit` are absent until the run pushes, and
//!   `pullRequest` until the run opens it.
//! - `commit` is the pushed head's seven-character short hash.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::pull::Pull;

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Delivery {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", length(min = 1))]
	pub branch: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", pattern(r"^[0-9a-f]{7}$"))]
	pub commit: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Pull")]
	pub pull_request: Option<Pull>,
}
