//! The report to the console
//!
//! What nsx tells the console each time it writes a hand-off record: which
//! runtime, the run's status, and the records written so far: `plan.json`
//! and `intent.json` from a plan, and `run.json` too from a run. The console
//! shows only what nsx reported.
//!
//! Definition only: the `Report` type: what nsx keeps between two reports of
//! one command. It carries the file that keeps the runtime's console id,
//! that id once the console gave it, the status to state, and whether the
//! console has stopped answering. Its functions are the files in `report/`.
//!
//! ## Rules
//!
//! - The status is nsx's statement, never derived by the console. It is one
//!   of `ready`, `analysis`, `plan`, `workflow`, `held`, `paused`, `done` and
//!   `error`.
//! - `held` means the run waits for the team's Play, before its header or
//!   before a stage's work. `paused` means the gate paused the run and the
//!   process ended.
//! - nsx sends the hand-off records themselves and no console-specific shape.
//!   The report's shape and its status live in `contract/`.
//! - A report carries no secret.
//! - The runtime's id is kept in `.langdrift/agent/state/console/<runtime
//!   id>.id`, private state Git ignores.
//! - Once a call fails, the console is unavailable: the records' sends stop,
//!   and nothing stops, until an ask is answered again. The run's waits,
//!   hold points and end always ask.

pub mod ask;
pub mod end;
pub mod open;
pub mod send;

use std::path::PathBuf;

use crate::contract::status::Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
	pub kept: PathBuf,
	pub runtime: Option<String>,
	pub status: Status,
	pub unavailable: bool,
}
