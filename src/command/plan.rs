//! The plan command, phase 1 of the simulation
//!
//! `nsx agent plan <ticket>` reads the team's documents, reasons over them
//! and drafts a sized `runtime.md` that it proposes to the team. Nothing is
//! built, started or paid for.
//!
//! Definition only: the `Plan` type, the command and its one argument: the
//! ticket's bare id, as in `0031-TICKET-book-a-demo-runtime`. Its functions
//! are the files in `plan/`.
//!
//! ## Rules
//!
//! - Exactly one argument. No argument, or more than one, is a stop.
//! - The argument is the bare id, never a file path. `request` resolves it to
//!   the ticket's file.
//! - The command takes no flag: the judge always judges, the envelope is the
//!   design doc's and the components are the ones the spec names.

pub mod analyze;
pub mod execute;
pub mod propose;
pub mod resolve;

use clap::Args;

#[derive(Debug, Args)]
pub struct Plan {
	/// The ticket's bare id, as in 0031-TICKET-book-a-demo-runtime
	pub ticket: String,
}
