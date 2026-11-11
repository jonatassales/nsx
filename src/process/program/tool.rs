//! A tool of the machine, ready to run
//!
//! One function: it takes a tool's name, its arguments, the names of the
//! variables it may see beside the minimal base and the folder it starts
//! in, and gives back the program, with nothing to read.
//!
//! ## Rules
//!
//! - The tool sees the minimal base and the variables named, and no other
//!   (ADR 0002).
//!
//! Shared: used by `local` and `delivery`.

use std::path::Path;

use crate::process::{environment::Environment, program::Program};

impl Program {
	pub fn tool(
		name: &str,
		arguments: &[&str],
		passed: &[&str],
		directory: &Path,
	) -> Self {
		Self {
			name: String::from(name),
			arguments: arguments.iter().map(|text| String::from(*text)).collect(),
			environment: Environment::allow(passed),
			directory: directory.to_path_buf(),
			input: Vec::new(),
		}
	}
}
