//! The design doc's row of DOCUMENTS
//!
//! What the Analysis card shows of the design doc the ticket links, set when
//! the design doc's read line prints: the envelope and the budget the team
//! set in it.
//!
//! Definition only: the `Design` type. It carries the document's `shortId`,
//! its `file`, the `envelope` and `budgetPerRunCents`.
//!
//! ## Rules
//!
//! - All four are required. The budget is whole cents.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::envelope::Envelope;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Design {
	pub short_id: String,
	pub file: String,
	pub envelope: Envelope,
	#[schemars(range(min = 0))]
	pub budget_per_run_cents: i64,
}
