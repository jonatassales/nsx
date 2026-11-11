//! Titling the window
//!
//! One function: it takes the title and writes the standard terminal title
//! sequence, with the command's name, an em dash and the project's folder.
//! It gives nothing back.
//!
//! ## Simulation
//!
//! - The window title: `nsx — project`
//!
//! ## Reads and writes
//!
//! - writes the title sequence to standard output, only when it is a
//!   terminal
//!
//! ## Rules
//!
//! - Piped output carries no escape sequence, so nothing is written there.
//! - A folder name that carries a control character is never written into
//!   the sequence: it is a stop.
//!
//! Spec 0077.

use std::io::Write;

use crate::session::{
	stop::Stop, terminal::Terminal, title::Title, window::Window,
};

impl Window for Terminal {
	fn title(&self, title: &Title) -> Result<(), Stop> {
		if !self.toned {
			return Ok(());
		}

		if title.project.chars().any(char::is_control) {
			return Err(Stop {
				sentence: String::from(
					"nsx will not title the window: the project folder's name carries a control character.",
				),
			});
		}

		let sequence = format!("\x1b]0;nsx — {}\x07", title.project);

		std::io::stdout()
			.lock()
			.write_all(sequence.as_bytes())
			.map_err(|failure| Stop {
				sentence: format!("nsx could not write to standard output: {failure}."),
			})
	}
}
