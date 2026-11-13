//! One criterion of the guardrail
//!
//! An entry of the judge's criteria answer: a criterion of the FinOps
//! guardrail that applies to the ticket, as "judging · FIN-02 estimate before
//! a paid call" prints it.
//!
//! Definition only: the `Criterion` type. It carries its `id`, such as
//! `FIN-02`, and its `statement`.
//!
//! ## Rules
//!
//! - Both fields are required and not empty. No other field is allowed.
//! - The statement is free text.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Criterion {
	#[schemars(length(min = 1))]
	pub id: String,
	#[schemars(length(min = 1))]
	pub statement: String,
}
