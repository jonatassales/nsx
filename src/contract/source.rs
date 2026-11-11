//! The Plan card's price
//!
//! The `price` section of the Plan card in `plan.json`: where the runtime's
//! price comes from.
//!
//! Definition only: the `Source` type. It carries `source`.
//!
//! ## Rules
//!
//! - `source` is required and `null` while the runtime has no price source:
//!   an unknown price is never a zero.
//! - No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::nullable::nullable;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Source {
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable, length(min = 1))]
	pub source: Option<String>,
}
