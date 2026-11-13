//! The money ledger
//!
//! The record of one run's money, kept in the project. An auditor can
//! recompute every line of the run from it, and the card reads the run's
//! money figures from it.
//!
//! Definition only: the `Ledger` type: one entry per run, with the model, the
//! effort, the expected tokens, the estimate, the budget, the margin, the
//! reservation and the gate's outcome, then, once the run reaches them, the
//! observed tokens, the observed spend and the release. Its functions are the
//! files in `ledger/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · RESERVED, once the `gateOpened` line has printed
//! - Card: Workflow · the paused result, read from the recorded gate
//!
//! ## Rules
//!
//! - `reservedCents` is the reservation the gate judges, stated with the
//!   estimate. Whether it was held is `gate`: `opened`, `paused`, or `null`
//!   until the gate is decided.
//! - Every amount is whole cents. A figure the run did not reach is unknown,
//!   never zero: a run that stops after the gate keeps its reservation and
//!   whatever usage was recorded, and its release stays unknown.
//! - Money is not copied into `run.json`. RESERVED, OBSERVED, RELEASED and the
//!   paused result come from this record.
//! - The entry keeps no laps, billing, totals or planning spend.

pub mod open;
pub mod write;

use serde::{Deserialize, Serialize};

pub const OPENED: &str = "opened";

pub const PAUSED: &str = "paused";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Ledger {
	pub run: String,
	pub model: String,
	pub effort: String,
	pub input_thousands: u32,
	pub output_thousands: u32,
	pub estimate_cents: i64,
	pub budget_cents: i64,
	pub budget_source: String,
	pub margin_percent: i64,
	pub reserved_cents: i64,
	pub gate: Option<String>,
	pub observed_input_tokens: Option<u64>,
	pub observed_output_tokens: Option<u64>,
	pub observed_cents: Option<i64>,
	pub released_cents: Option<i64>,
}
