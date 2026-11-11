//! The suggested resource
//!
//! What the plan proposes to run the components as, as the Plan card's
//! SUGGESTED RESOURCE shows it: one one-replica Deployment per component.
//!
//! Definition only: the `Resource` type. It carries the `kind`, the `count`
//! and the `replicas` of each. The one kind sits beside the type.
//!
//! ## Rules
//!
//! - `kind` is `Deployment` and `replicas` is 1: the plan proposes nothing
//!   else.
//! - Every field is required, and no other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const KIND: &str = "Deployment";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Resource {
	#[schemars(extend("const" = KIND))]
	pub kind: String,
	#[schemars(range(min = 1))]
	pub count: u64,
	#[schemars(extend("const" = 1))]
	pub replicas: i64,
}
