//! Keeping one line
//!
//! One function: it takes a line and keeps it after the lines it already
//! holds. It never stops.

use nsx::session::{line::Line, sink::Sink, stop::Stop};

use crate::simulation::fake::recorder::Recorder;

impl Sink for Recorder {
	fn print(&self, line: &Line) -> Result<(), Stop> {
		self.lines.borrow_mut().push(line.clone());

		Ok(())
	}
}
