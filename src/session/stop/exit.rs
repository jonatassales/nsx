//! Ending a stopped command
//!
//! One function: it takes a stop, writes its one plain sentence to standard
//! error and gives back exit status 1, which the binary ends with.
//!
//! ## Reads and writes
//!
//! - writes one sentence to standard error
//!
//! ## Rules
//!
//! - The sentence names the cause and no secret.
//! - A control character in the sentence is written as its escape.
//! - Nothing more goes to standard output, which stays a prefix of the
//!   simulation.
//! - The over-budget pause never comes here: it is a shown ending and exits
//!   0.

use std::io::Write;
use std::process::ExitCode;

use crate::session::stop::Stop;

impl Stop {
	pub fn exit(&self) -> ExitCode {
		// A sentence may quote what a file or a command line said: a control
		// character in it is shown as its escape, never sent to the terminal.
		let shown = self
			.sentence
			.chars()
			.map(|character| {
				if character.is_control() {
					character.escape_default().to_string()
				} else {
					String::from(character)
				}
			})
			.collect::<String>();

		// NOTE: a standard error that cannot be written leaves nowhere to
		// say so; the exit status still tells the stop.
		let _unwritten = writeln!(std::io::stderr().lock(), "{shown}");

		ExitCode::FAILURE
	}
}
