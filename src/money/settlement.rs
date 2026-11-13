//! Observed and released
//!
//! How a run's money ends. Once the stages have run, the tokens they used are
//! priced at the same list price as the estimate: that is the observed spend.
//! What the reservation held and the run did not spend is released.
//!
//! Definition only: the `Settlement` type: the observed input and output
//! tokens, the observed spend in cents and the released amount in cents. Its
//! functions are the files in `settlement/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · OBSERVED, once the `completed` line has printed
//! - Card: Workflow · RELEASED, once the `ready` line has printed
//!
//! ## Rules
//!
//! - The observed tokens come from the usage record that `evidence/` keeps
//!   from the stage calls. This folder does not measure them.
//! - The observed spend is recorded when the `completed` line prints, the
//!   release when the `ready` line prints.
//! - The settlement prints no line of its own: it supplies the observed
//!   amount to the `completed` line and the released one to `ready`.
//! - A run paused at the gate has no settlement: nothing was observed.
//! - A run that stops after the gate keeps its reservation. Its release stays
//!   unknown, never zero.

pub mod settle;

use crate::money::cents::Cents;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settlement {
	pub input_tokens: u64,
	pub output_tokens: u64,
	pub observed: Cents,
	pub released: Cents,
}
