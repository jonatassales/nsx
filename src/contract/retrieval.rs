//! The EPISODES of the Analysis card
//!
//! The `episodes` section of `plan.json`: the candidates the search found,
//! which of them the threshold kept, and how many of the cited ones measured
//! resources.
//!
//! Definition only: the `Retrieval` type. It carries the `threshold`, the
//! `candidates` in descending similarity, the counts `kept`, `dropped` and
//! `total`, and the counts `cited` and `citedMeasured`.
//!
//! ## Rules
//!
//! - `threshold`, `candidates` and `total` are absent until the `candidates`
//!   line prints; `kept` and `dropped` until the `drop` line; `cited` and
//!   `citedMeasured` until the `episodes` line.
//! - No other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::candidate::Candidate;

#[derive(
	Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Retrieval {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "f64")]
	pub threshold: Option<f64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Vec<Candidate>")]
	pub candidates: Option<Vec<Candidate>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub kept: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub dropped: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub total: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub cited: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub cited_measured: Option<u64>,
}
