//! The expected tokens
//!
//! How many input and output tokens a run is expected to use at each effort:
//! low, medium and high. The estimate is made from these figures, and the
//! lines and the card show them beside it.
//!
//! Definition only: the `Forecast` type: the unit, and per effort the expected
//! input and output tokens, a `Tokens`, in thousands of tokens. Its functions
//! are the files in `forecast/`.
//!
//! ## Rules
//!
//! - The forecast is per effort, not per model: low 2,000 input and 120
//!   output, medium 3,500 and 250, high 6,000 and 500, in thousands of tokens.
//! - It carries no `status` and no `illustrative` flag.
//! - nsx reads the three efforts a run may take. An effort the file lacks
//!   stops the plan when the spend table is built.
//!
//! ## Open
//!
//! - Where a ticket's expected tokens come from: a team-wide forecast per
//!   effort, a figure derived from the spec's components, or episodes that
//!   measured tokens. Until the owner decides, the file is the team's forecast
//!   per effort.

pub mod load;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::money::tokens::Tokens;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Forecast {
	pub unit: String,
	pub by_effort: BTreeMap<String, Tokens>,
}
