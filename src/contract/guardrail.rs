//! The guardrail's row of DOCUMENTS
//!
//! What the Analysis card shows of the guardrail the ticket links, set when
//! the guardrail's line prints.
//!
//! Definition only: the `Guardrail` type. It carries the document's `shortId`,
//! its `file`, its `criteria` in table order, `planCriteria`, the ids the plan
//! step answers to, and `enforcement`.
//!
//! ## Rules
//!
//! - `enforcement` has one value, `review`: nsx enforces no criterion.
//! - All five are required. No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::rule::Rule;

pub const ENFORCEMENT: &str = "review";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Guardrail {
	pub short_id: String,
	pub file: String,
	pub criteria: Vec<Rule>,
	pub plan_criteria: Vec<String>,
	#[schemars(extend("const" = ENFORCEMENT))]
	pub enforcement: String,
}
