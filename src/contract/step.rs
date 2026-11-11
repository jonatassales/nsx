//! One stage of the run, as `run.json` records it
//!
//! An entry of the `stages` of `run.json`: one stage of the workflow, the dev
//! ticket among them, with where the run stands in it and the report its
//! skill wrote.
//!
//! Definition only: the `Step` type. It carries the stage's `id`, whether a
//! person confirms it (`human`), its `state`, who confirmed it
//! (`confirmedBy`), for the dev ticket who closed it and how many paid
//! calls it took (`closedBy`, `paidCalls`), and its `report`. The four
//! states sit beside the type.
//!
//! ## Rules
//!
//! - Every stage is listed from the run's opening, `pending`: a run paused
//!   at the gate leaves them all so.
//! - `confirmedBy` is absent until the owner confirms the stage, and is the
//!   role `owner`, never a person.
//! - `closedBy` is the role `person` and `paidCalls` a known zero: both are
//!   the dev ticket's, absent until the person closes it.
//! - `report` is absent until the stage's skill writes it, and holds the
//!   stage's figures as its own report shapes them.
//! - No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::figures::Figures;

pub const PENDING: &str = "pending";

pub const WAITING: &str = "waiting";

pub const ACTIVE: &str = "active";

pub const DONE: &str = "done";

pub const STATES: [&str; 4] = [PENDING, WAITING, ACTIVE, DONE];

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Step {
	#[schemars(length(min = 1))]
	pub id: String,
	pub human: bool,
	#[schemars(extend("enum" = STATES))]
	pub state: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("const" = "owner"))]
	pub confirmed_by: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", extend("const" = "person"))]
	pub closed_by: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub paid_calls: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Figures")]
	pub report: Option<Figures>,
}
