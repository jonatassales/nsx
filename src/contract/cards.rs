//! `plan.json`: the four cards
//!
//! The plan's card data: what the Ticket, Analysis, Plan and Workflow cards
//! show, as the plan produces it. The card is the specification of this data.
//! A receiver reads it and computes nothing.
//!
//! Definition only: the `Cards` type. It carries four sections, one per card:
//! `ticket`, `analysis`, `plan` and `workflow`. Its functions are the files in
//! `cards/`: each finds one section of a card for the folder that fills it.
//!
//! ## Rules
//!
//! - A section is absent until its card has something to show. Absent means
//!   not revealed yet.
//! - `ticket`, `workflow` and the EPISODES threshold are there before the
//!   plan's first line. Every other field arrives when its line prints.
//! - Values are units, never formatted text. Labels, copy and thousands
//!   separators are the receiver's.
//! - No other section is allowed.

pub mod components;
pub mod documents;
pub mod episodes;
pub mod sizing;
pub mod suggestion;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	analysis::Analysis, plan::Plan, ticket::Ticket, workflow::Workflow,
};

// A section absent until it is revealed is not `null`: each optional section
// takes its inner type's schema, and its default keeps it optional.
#[derive(
	Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields)]
#[schemars(
	title = "Plan card data",
	description = concat!(
		"The plan's card data (plan.json): one section per card, each absent ",
		"until it is revealed. A section whose fields have not joined yet ",
		"allows no field; the element spec that fills it changes this schema ",
		"with its type.",
	),
	extend("$id" = "https://langdrift.md/schemas/handoff/plan.schema.json"),
)]
pub struct Cards {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Ticket")]
	pub ticket: Option<Ticket>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Analysis")]
	pub analysis: Option<Analysis>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Plan")]
	pub plan: Option<Plan>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Workflow")]
	pub workflow: Option<Workflow>,
}
