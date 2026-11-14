//! The Analysis card
//!
//! The `analysis` section of `plan.json`: what the plan read, which episodes
//! it cited, what the judge suggested, and why the judge could not be asked
//! when it could not.
//!
//! Definition only: the `Analysis` type. It carries four sections:
//! `documents`, `episodes`, `suggestion` and `judge`.
//!
//! ## Rules
//!
//! - Each section is absent until its line prints.
//! - A similarity is a number with two decimals.
//! - The suggestion is a recommendation. It authorizes nothing.
//! - `judge` is present only when the plan stopped before the judge's first
//!   question.
//! - No other section is allowed. `documents` is a `Documents`, `episodes` a
//!   `Retrieval`, `suggestion` a `Recommendation` and `judge` an
//!   `Availability`.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	availability::Availability, documents::Documents,
	recommendation::Recommendation, retrieval::Retrieval,
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
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Availability")]
	pub judge: Option<Availability>,
}
