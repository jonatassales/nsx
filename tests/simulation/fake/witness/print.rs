//! Keeping what `plan.json` held when a line printed

use nsx::session::{line::Line, sink::Sink, stop::Stop};
use serde_json::Value;

use crate::simulation::fake::witness::Witness;

impl Sink for Witness {
	fn print(&self, line: &Line) -> Result<(), Stop> {
		// Before the plan's first write there is no file to look at.
		let cards = if self.file.is_file() {
			let written = std::fs::read_to_string(&self.file)?;

			serde_json::from_str(&written)?
		} else {
			Value::Null
		};

		self.seen.borrow_mut().push((line.id.clone(), cards));

		Ok(())
	}
}
