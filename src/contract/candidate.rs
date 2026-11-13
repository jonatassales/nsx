//! One row of EPISODES
//!
//! An episode the search found, as the Analysis card's EPISODES rows show it.
//!
//! Definition only: the `Candidate` type. It carries the episode's `id`, its
//! `slug`, its `similarity`, its `verdict` and its `reason`.
//!
//! ## Rules
//!
//! - `similarity` is a number with two decimals.
//! - `verdict` is `kept` or `dropped`, absent until the `drop` line prints.
//!   `reason` is the judge's, from its closed set, absent until then too.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::episode::{REASONS, VERDICTS};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Candidate {
	pub id: String,
	pub slug: String,
	pub similarity: f64,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("enum" = VERDICTS))]
	pub verdict: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("enum" = REASONS))]
	pub reason: Option<String>,
}
