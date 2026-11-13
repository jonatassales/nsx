//! A stage's figures
//!
//! What a stage's skill reported, as `run.json` keeps it beside the stage:
//! one of the five stage reports, by its own shape.
//!
//! Definition only: the `Figures` type: the report of Planning, Spec,
//! Tickets, Implement or Review.
//!
//! ## Rules
//!
//! - Each report keeps its own schema, referred to by its `$id`. No two
//!   share a field, so a report is told apart by its fields alone.
//! - Only figures are kept, never a free-text summary.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	implement::Implement, planning::Planning, review::Review, spec::Spec,
	tickets::Tickets,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(inline)]
pub enum Figures {
	Planning(Planning),
	Spec(Spec),
	Tickets(Tickets),
	Implement(Implement),
	Review(Review),
}
