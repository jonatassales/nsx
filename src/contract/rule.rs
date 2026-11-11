//! One criterion of the guardrail's row
//!
//! A criterion of the guardrail the ticket links, as the Analysis card's FIN
//! line shows it.
//!
//! Definition only: the `Rule` type. It carries the criterion's `id` and its
//! `statement`.
//!
//! ## Rules
//!
//! - `id` is set when the guardrail's line prints. `statement` is the
//!   judge's, absent until the judge's line on the criteria prints.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Rule {
	pub id: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String")]
	pub statement: Option<String>,
}
