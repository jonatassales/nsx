//! The Plan card
//!
//! The `plan` section of `plan.json`: the components, how they are sized
//! inside the bound, what the runtime costs, and where the proposal stands.
//!
//! Definition only: the `Plan` type. It carries five sections: `components`,
//! `sizing`, `price`, `authorization` and `evidence`. `evidence` carries the
//! `file`, its `state`, its `revision` and `proposedTo`.
//!
//! ## Rules
//!
//! - Each section is absent until its line prints.
//! - `authorization` has one value, `pending`. nsx records that the team has
//!   not authorized yet. It never decides an authorization.
//! - In `evidence`, a value not known yet is `null`, and `revision` is at
//!   least 1. For the simulation's plan it reads `runtime.md`, `draft`, 1 and
//!   `console`.
//! - `authorization` and `evidence` are written once, when the plan is
//!   proposed. A run never changes them.
//! - No other section is allowed.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	authorization::Authorization, components::Components, evidence::Evidence,
	sizing::Sizing, source::Source,
};

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Plan {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Components")]
	pub components: Option<Components>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Sizing")]
	pub sizing: Option<Sizing>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Source")]
	pub price: Option<Source>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Authorization")]
	pub authorization: Option<Authorization>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Evidence")]
	pub evidence: Option<Evidence>,
}
