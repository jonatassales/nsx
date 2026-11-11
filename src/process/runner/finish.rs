//! Running a program that must end well
//!
//! One function: it takes a program, runs it and gives back its output when
//! it ended with the exit status 0, or the failure that names the program.
//!
//! ## Reads and writes
//!
//! - runs the program it is given
//!
//! ## Rules
//!
//! - Any other exit status, or an end by a signal, is the caller's stop
//!   (`output/ended.rs`).
//!
//! Shared: used by `local` and `delivery`.

use crate::process::{
	error::Error, output::Output, program::Program, runner::Runner,
};

impl dyn Runner + '_ {
	pub async fn finish(&self, program: &Program) -> Result<Output, Error> {
		self.run(program).await?.ended(program)
	}
}
