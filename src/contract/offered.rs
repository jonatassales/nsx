//! What one provider offers
//!
//! Definition only: the `Offered` type: the provider, the ids of the models
//! the team offers from it, and the efforts the team offers for them.
//!
//! ## Rules
//!
//! - Each model and each effort is a non-empty string, listed once.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::provider::Provider;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Offered {
	pub provider: Provider,
	#[schemars(inner(length(min = 1)), extend("uniqueItems" = true))]
	pub models: Vec<String>,
	#[schemars(inner(length(min = 1)), extend("uniqueItems" = true))]
	pub efforts: Vec<String>,
}
