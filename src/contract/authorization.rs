//! Where the plan's authorization stands
//!
//! The AUTHORIZATION row of the Plan card, as `plan.json` records it when the
//! plan is proposed.
//!
//! Definition only: the `Authorization` type and its one value, `pending`.
//!
//! ## Rules
//!
//! - nsx records that the team has not authorized yet. It never decides an
//!   authorization: the team authorizes in the console.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
#[schemars(inline)]
pub enum Authorization {
	Pending,
}
