//! `run.json`: the run's record
//!
//! What a run leaves for the Workflow card: one section per part of the run,
//! filled as the run reaches it. It holds the latest run of the runtime.
//!
//! Definition only: the `Run` type. It carries `run`, the run's id, the
//! applied `intent`, the `stages`, each with its `report`, the `effort`
//! record, `safeSeat`, the `runtime`, the `delivery` with its `pullRequest`,
//! and the `outcome`.
//!
//! ## Rules
//!
//! - `run` and `outcome` are always present, and `null` until known.
//! - Every other section is absent until the run reaches it.
//! - Money is not copied here. RESERVED, OBSERVED and RELEASED come from the
//!   run's entry in the money ledger, which the run's id names.
//! - Of the checks, only the state PULL REQUEST shows is kept.
//! - Every value is in units and carries no secret.
//! - No other section is allowed.
//!
//! - `intent` is the applied intent, referred to by its `$id`.
//! - `runtime` is ready, then stopped (`hosted.rs`). `delivery` gains its
//!   branch and commit at the push, its pull request when it opens, and the
//!   pull request's checks as the forge lists them (`delivery.rs`).
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	delivery::Delivery, effort::Effort, hosted::Hosted, intent::Intent,
	nullable::nullable, outcome::Outcome, seat::Seat, step::Step,
};

// `run` and `outcome` are required and `null` until known; every other
// section is absent until the run reaches it, never `null`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Run record",
	description = concat!(
		"The run's record (run.json): the run's id and outcome, null until ",
		"known, and one section per part of the run, absent until the run ",
		"reaches it.",
	),
	extend("$id" = "https://langdrift.md/schemas/handoff/run.schema.json"),
)]
pub struct Run {
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable)]
	pub run: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Intent")]
	pub intent: Option<Intent>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Vec<Step>")]
	pub stages: Option<Vec<Step>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Effort")]
	pub effort: Option<Effort>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Seat")]
	pub safe_seat: Option<Seat>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Hosted")]
	pub runtime: Option<Hosted>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Delivery")]
	pub delivery: Option<Delivery>,
	#[serde(deserialize_with = "Option::deserialize")]
	#[schemars(required, transform = nullable)]
	pub outcome: Option<Outcome>,
}
