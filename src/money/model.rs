//! One model of the price list
//!
//! A Claude model a team may choose, with what a million of its tokens cost.
//!
//! Definition only: the `Model` type: its id, its name, its short name, and
//! its input and output list prices in cents per million tokens.
//!
//! ## Rules
//!
//! - The id is what the intent, the judge's suggestion and the coding agent
//!   use. The name is what the MODEL choice shows, the short name what the
//!   Workflow header shows.
//!
//! Spec 0079.

use serde::{Deserialize, Serialize};

use crate::money::cents::Cents;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Model {
	pub id: String,
	pub name: String,
	pub short_name: String,
	pub input_cents: Cents,
	pub output_cents: Cents,
}
