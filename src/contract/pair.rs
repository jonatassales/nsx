//! One row of the Workflow card's spend table
//!
//! What one model at one effort would cost a run, as `plan.json` carries it.
//!
//! Definition only: the `Pair` type. It carries the `model`, the `effort`,
//! `inputThousands` and `outputThousands`, `estimateCents`, `reservedCents`
//! and `overBudget`.
//!
//! ## Rules
//!
//! - Every field is required, and no other field is allowed.
//! - `model` is a price-list id, and `effort` is low, medium or high.
//! - Money is whole cents and tokens are whole thousands, never negative.
//! - `overBudget` is stated by nsx. A receiver never compares the amounts.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::fence::EFFORTS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Pair {
	#[schemars(length(min = 1))]
	pub model: String,
	#[schemars(extend("enum" = EFFORTS))]
	pub effort: String,
	#[schemars(range(min = 0))]
	pub input_thousands: i64,
	#[schemars(range(min = 0))]
	pub output_thousands: i64,
	#[schemars(range(min = 0))]
	pub estimate_cents: i64,
	#[schemars(range(min = 0))]
	pub reserved_cents: i64,
	pub over_budget: bool,
}
