//! How a run ended
//!
//! Where a run stands, as its record says it: the `outcome` of `run.json`. It
//! serves the choice of the intent a run applies and the episode a completed
//! run leaves, not the card.
//!
//! Definition only: the `Outcome` type and its four outcomes: `running`,
//! `paused`, `stopped` and `completed`.
//!
//! ## Rules
//!
//! - `running` is set when the run's header prints.
//! - `paused` is the pause at the money gate. It is a shown ending, not a
//!   failure: the run exits 0.
//! - `stopped` is a stop under the stop rule: the run exits 1.
//! - `completed` is set when the `ready` line prints.
//! - The card's paused result reads the money ledger, not this field.
//! - In `run.json` the outcome is `null` until known.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
#[schemars(inline)]
pub enum Outcome {
	Running,
	Paused,
	Stopped,
	Completed,
}
