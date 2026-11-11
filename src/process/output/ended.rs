//! Whether a program ended well
//!
//! One function: it takes a program's output and the program, and gives the
//! output back when the program ended with the exit status 0, or the
//! failure that names the program.
//!
//! ## Rules
//!
//! - Any other exit status, or an end by a signal, is the caller's stop.
//! - The failure names the program and its first argument, such as
//!   `git push`, and its exit status. It never quotes what the program
//!   printed: a tool's output can hold an address with a token in it.
//!
//! Shared: used by `local` and `delivery`.

use crate::process::{error::Error, output::Output, program::Program};

impl Output {
	pub fn ended(self, program: &Program) -> Result<Self, Error> {
		let command = || {
			let asked = program.arguments.first().map(String::as_str);

			[Some(program.name.as_str()), asked]
				.into_iter()
				.flatten()
				.collect::<Vec<_>>()
				.join(" ")
		};

		match self.status {
			Some(0) => Ok(self),
			Some(status) => Err(Error::Failed {
				command: command(),
				status,
			}),
			None => Err(Error::Killed {
				command: command(),
			}),
		}
	}
}
