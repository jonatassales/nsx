//! The failure a tool of the cluster would give

use nsx::local::error::Error;
use nsx::process::error::Error as Ended;

use crate::simulation::fake::machine::Machine;

impl Machine {
	pub fn refusal(command: &'static str) -> Error {
		Error::Program(Ended::Failed {
			command: String::from(command),
			status: 1,
		})
	}
}
