//! The choice for one run
//!
//! What the team chooses for one run: the model, the effort and the stages
//! where a person confirms. The plan records the workflow's default choice,
//! in its card data and in the proposed intent. The team may change it in the
//! intent before authorizing, and the run applies the choice that was
//! authorized.
//!
//! Definition only: the `Choice` type, which carries `model` (a price-list
//! id), `effort` (`low`, `medium` or `high`) and `human`, a flag for each of
//! `planning`, `spec`, `tickets`, `implement` and `review`.
//!
//! ## Simulation
//!
//! - Card: Workflow · HUMAN IN THE LOOP, the ticks
//! - Card: Workflow · MODEL
//! - Card: Workflow · EFFORT
//!
//! ## Rules
//!
//! - `planning` is always true, whatever the recorded choice says.
//! - Which stages wait comes from the applied choice, never from the
//!   workflow's defaults.
//! - The judge's suggestion is a recommendation and does not change the
//!   choice. No flag sets the model or the effort.
//! - The intent that carries the choice between plan, card and run is spec
//!   0050's. The run's record keeps the choice as applied (spec 0065).
//!
//! Spec 0080.

use serde::Deserialize;

use crate::contract::human::Human;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice {
	pub model: String,
	pub effort: String,
	pub human: Human,
}
