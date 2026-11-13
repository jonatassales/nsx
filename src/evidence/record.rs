//! The single writer of the run's record
//!
//! `run.json` holds the latest run of the runtime: the run's evidence and
//! the Workflow card after the run, in one place. Each folder that produces
//! run data writes its section through this writer as its line prints, and
//! the writer tells the console each time.
//!
//! Definition only: the `Record` type, which holds the run's record as it
//! fills: the run's id, the applied intent, the sections of the other
//! folders and the outcome; with the plan's cards and the state of the
//! reports to the console, so that every write is told. Its functions are
//! the files in `record/`.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/run.json`
//!
//! ## Rules
//!
//! - The run's script fills `run`, `intent` (the applied intent, from
//!   `handoff`), the stages as `pending` and `outcome`. Every other
//!   section's content is its owner's.
//! - The outcome is `running` once the run's `intent` line has printed,
//!   `paused` when the gate pauses, `completed` when the ready line prints
//!   and `stopped` on any stop.
//! - The outcome tells `command` whether an intent has a completed run, and
//!   tells `rag` when a run finished, so that its episode is written.
//! - The money is not copied here: RESERVED, OBSERVED and RELEASED, and the
//!   paused result, come from the money ledger.
//! - Every value is in units and carries no secret. What is not known is
//!   `null`, never zero.

pub mod stage;
pub mod write;

use std::path::PathBuf;

use crate::console::report::Report;
use crate::contract::{cards::Cards, run::Run};

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
	pub file: PathBuf,
	pub run: Run,
	pub cards: Cards,
	pub report: Report,
}
