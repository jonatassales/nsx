//! The Workflow card
//!
//! The `workflow` section of `plan.json`: what the Workflow card shows before
//! a run, from the moment the panel appears. The card after a run reads
//! `run.json` and the money ledger instead.
//!
//! Definition only: the `Workflow` type. It carries `budgetCents` with its
//! `budgetSource`, `marginPercent` and `spend`, one `Pair` per model and
//! effort.
//!
//! ## Rules
//!
//! - The section is written before the plan's first line.
//! - Every field is required, and no other field is allowed.
//! - Money is whole cents and tokens are whole numbers.
//! - `budgetSource` is the design doc's path from the project's root.
//! - Every phase-1 figure of the card is a lookup in `spend`.
//!
//! ## Open
//!
//! - The stages and the default choice are not in the section yet.
//! - The price list's model names and its read date are not in the section
//!   yet: the card's header and its MODEL price line need them.

pub mod pair;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::pair::Pair;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Workflow {
	#[schemars(range(min = 0))]
	pub budget_cents: i64,
	#[schemars(length(min = 1))]
	pub budget_source: String,
	#[schemars(range(min = 0))]
	pub margin_percent: i64,
	pub spend: Vec<Pair>,
}
