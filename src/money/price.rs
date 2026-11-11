//! The price list
//!
//! The three Claude models a team may choose, each with its input and output
//! list price per million tokens. The prices were read on 2026-10-06 and are
//! uncached. The model choice, every money figure and the model ids the rest
//! of nsx uses come from this one list.
//!
//! Definition only: the `Price` type: the date the prices were read, that they
//! are uncached, the currency and the unit, and per model its id, name, short
//! name, input cents and output cents, each a `Model`. The ids of the three
//! models nsx reads sit beside the type. Its functions are the files in
//! `price/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · the MODEL price line, under the choice
//!
//! ## Rules
//!
//! - The list holds exactly three models: `claude-opus-5-5`,
//!   `claude-sonnet-5-5` and `claude-haiku-5-5`. nsx reads only them.
//! - The unit is cents per million tokens, in USD.
//!
//! ## Open
//!
//! - Who keeps the price list current, and whether a team may edit or extend
//!   `finops/prices.json`. Until that is decided, a model beyond the three is
//!   refused when the list is read, never silently skipped.
//!
//! Spec 0079.

pub mod card;
pub mod line;
pub mod load;

use serde::{Deserialize, Serialize};

use crate::money::model::Model;

pub const MODELS: [&str; 3] =
	["claude-opus-5-5", "claude-sonnet-5-5", "claude-haiku-5-5"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Price {
	pub read_on: String,
	pub cache: String,
	pub currency: String,
	pub unit: String,
	pub models: Vec<Model>,
}
