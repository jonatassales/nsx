//! Printing one line
//!
//! One function: it takes a line and writes it to standard output, ended by
//! a newline. It gives back a stop when standard output cannot be written.
//!
//! ## Reads and writes
//!
//! - writes the line to standard output
//!
//! ## Rules
//!
//! - When standard output is a terminal, the line takes the tone of its
//!   kind: a comment dimmed, a success line green, a warning yellow, an
//!   output line the default color.
//! - Otherwise the bytes are the plain text, with no escape sequence.
//! - A line that carries a control character, a line break or an escape among
//!   them, is not printed: it is a stop, whatever it was read from.

use std::io::Write;

use crate::session::{
	kind::Kind, line::Line, sink::Sink, stop::Stop, terminal::Terminal,
};

const RESET: &str = "\x1b[0m";

impl Sink for Terminal {
	fn print(&self, line: &Line) -> Result<(), Stop> {
		let text = line.render();

		// What a document or an answer says must never pass for the
		// terminal's own: a line is one line of text.
		if text.chars().any(char::is_control) {
			return Err(Stop {
				sentence: format!(
					"nsx will not print the line {}: it carries a control character.",
					line.id
				),
			});
		}

		let tone = match line.kind {
			Kind::Comment => "\x1b[2m",
			Kind::Success => "\x1b[32m",
			Kind::Warning => "\x1b[33m",
			Kind::Output => "",
		};
		let plain = !self.toned || tone.is_empty();
		let bytes = if plain {
			format!("{text}\n")
		} else {
			format!("{tone}{text}{RESET}\n")
		};

		std::io::stdout()
			.lock()
			.write_all(bytes.as_bytes())
			.map_err(|failure| Stop {
				sentence: format!("nsx could not write to standard output: {failure}."),
			})
	}
}
