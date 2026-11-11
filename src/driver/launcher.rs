//! The coding agent, started on this machine
//!
//! The agent the binary runs: the executable `driver.json` names, started as
//! a program of its own.
//!
//! Definition only: the `Launcher` type, which carries the runner of
//! programs. Its function is the file in `launcher/`.
//!
//! Spec 0080.

pub mod start;

use crate::process::runner::Runner;

pub struct Launcher<'run> {
	pub programs: &'run dyn Runner,
}
