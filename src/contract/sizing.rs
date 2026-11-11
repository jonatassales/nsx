//! The Plan card's sizing
//!
//! The `sizing` section of the Plan card in `plan.json`: the bound, what the
//! plan proposes inside it, and the sizing report's other statements.
//!
//! Definition only: the `Sizing` type. It carries `bound`, `proposed`,
//! `headroom`, `resource`, `limitsRequested` and `validation`. The limits
//! `runtime.md` has no field for sit beside the type.
//!
//! ## Rules
//!
//! - `bound` arrives with the `bounds` line. `proposed` arrives with the
//!   first component's row and steps up at each row. `headroom` and
//!   `resource` arrive with the `size` line, `limitsRequested` with the
//!   `limits` line and `validation` with the `validate` line.
//! - `limitsRequested` lists the limits a document asked for among those
//!   `runtime.md` cannot hold. It is empty: none is requested.
//! - A field is absent until its line prints, and no other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	amount::Amount, bound::Bound, resource::Resource, validation::Validation,
};

pub const LIMITS: [&str; 2] = ["gpu", "disk"];

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Sizing {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Bound")]
	pub bound: Option<Bound>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Amount")]
	pub proposed: Option<Amount>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Amount")]
	pub headroom: Option<Amount>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Resource")]
	pub resource: Option<Resource>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(
		with = "Vec<String>",
		extend("items" = serde_json::json!({
			"type": "string",
			"enum": LIMITS,
		})),
	)]
	pub limits_requested: Option<Vec<String>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Validation")]
	pub validation: Option<Validation>,
}
