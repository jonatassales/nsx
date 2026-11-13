//! The Analysis card
//!
//! The `analysis` section of `plan.json`: what the plan read, which episodes
//! it cited and what the judge suggested.
//!
//! Definition only: the `Analysis` type. It carries three sections:
//! `documents`, `episodes` and `suggestion`.
//!
//! ## Rules
//!
//! - Each section is absent until its line prints.
//! - A similarity is a number with two decimals.
//! - The suggestion is a recommendation. It authorizes nothing.
//! - No other section is allowed. `documents` is a `Documents`, `episodes` a
//!   `Retrieval` and `suggestion` a `Recommendation`.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	documents::Documents, recommendation::Recommendation, retrieval::Retrieval,
};

#[derive(
	Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Analysis {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Documents")]
	pub documents: Option<Documents>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Retrieval")]
	pub episodes: Option<Retrieval>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Recommendation")]
	pub suggestion: Option<Recommendation>,
}
