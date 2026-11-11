//! Whether the cluster can be reached at all

use nsx::local::error::Error;

use crate::simulation::fake::machine::Machine;

impl Machine {
	pub fn reach(&self, command: &'static str) -> Result<(), Error> {
		if self.unreachable {
			return Err(Self::refusal(command));
		}

		Ok(())
	}
}
