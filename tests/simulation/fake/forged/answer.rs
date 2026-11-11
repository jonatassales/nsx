//! The answer to one operation: refused when the test said so, with the
//! sentence the real tool's failure would give

use nsx::delivery::error::Error;
use nsx::process::error::Error as Ended;

use crate::simulation::fake::forged::Forged;

impl Forged {
	pub fn answer<Answer>(
		&self,
		operation: &'static str,
		command: &'static str,
		answer: Answer,
	) -> Result<Answer, Error> {
		if self.refusing == Some(operation) {
			return Err(Error::Program(Ended::Failed {
				command: String::from(command),
				status: 1,
			}));
		}

		Ok(answer)
	}
}
