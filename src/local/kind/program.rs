//! A cluster tool to start
//!
//! One function: it takes a tool's name and its arguments, and gives back
//! the program to run, with nsx's tools folder first on its `PATH`.
//!
//! ## Rules
//!
//! - Every program the cluster starts, kind, kubectl, Docker and what
//!   starts Docker, starts this way, so that nsx's own pinned copies are
//!   found before any the machine has, by nsx and by the tools that start
//!   one another.
//! - A machine that gives nsx no `PATH` leaves the tools folder alone on
//!   it.
//! - The rest of the environment is the small set the cluster tools are
//!   given.

use crate::local::{
	error::Error,
	kind::{Kind, PASSED},
};
use crate::process::program::Program;

impl Kind<'_> {
	pub fn program(
		&self,
		name: &str,
		arguments: &[&str],
	) -> Result<Program, Error> {
		let folder = self.tools()?;
		let mut program = Program::tool(name, arguments, &PASSED, &self.directory);
		let machine = program
			.environment
			.variables
			.remove("PATH")
			.unwrap_or_default();
		let leading = std::env::join_paths(
			std::iter::once(folder.clone()).chain(std::env::split_paths(&machine)),
		)
		.map_err(|cause| Error::Unled {
			path: folder.clone(),
			cause,
		})?;
		let spoken = leading.to_str().map(String::from).ok_or(Error::Unspoken {
			path: folder,
		})?;

		program
			.environment
			.variables
			.insert(String::from("PATH"), spoken);

		Ok(program)
	}
}
