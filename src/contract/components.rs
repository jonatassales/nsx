//! The Plan card's COMPONENTS
//!
//! The `components` section of the Plan card in `plan.json`: the components
//! the spec names, each as the plan resolved it, and the cross-check of them.
//!
//! Definition only: the `Components` type. It carries `rows`, one `Resolved`
//! per component in the spec's order, then `portsDistinct`, `imagesResolved`
//! and `collisions`, the counts of the cross-check.
//!
//! ## Rules
//!
//! - `rows` arrives with the `resolve` line, as placeholders. The three counts
//!   arrive with the `crossCheck` line.
//! - A field is absent until its line prints, and no other field is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::resolved::Resolved;

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Components {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Vec<Resolved>")]
	pub rows: Option<Vec<Resolved>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub ports_distinct: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub images_resolved: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub collisions: Option<u64>,
}
