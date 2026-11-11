//! Every line is refused, as a terminal that went away would

use nsx::session::{line::Line, sink::Sink, stop::Stop};

use crate::simulation::fake::refuser::Refuser;

impl Sink for Refuser {
	fn print(&self, line: &Line) -> Result<(), Stop> {
		Err(Stop {
			sentence: format!("the line {} was refused.", line.id),
		})
	}
}
