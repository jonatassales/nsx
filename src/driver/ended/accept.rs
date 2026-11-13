//! Accepting how a stage call ended
//!
//! One function: it says whether the call may give its stage a line, or the
//! reason the stage fails.
//!
//! ## Rules
//!
//! - The call must have ended with a zero exit status, and must have
//!   reported both token counts: a stage whose spend is unknown prints no
//!   line.
//! - The run asks after the usage record has kept the call, so a refused
//!   call is still on record.

use crate::driver::{ended::Ended, error::Error};

impl Ended {
	pub fn accept(&self) -> Result<(), Error> {
		let stage = || self.stage.clone();
		let measured = self.tokens.input.is_some() && self.tokens.output.is_some();

		match self.status {
			Some(0) => {}
			Some(status) => {
				return Err(Error::Failed {
					stage: stage(),
					status,
				});
			}
			None => {
				return Err(Error::Killed {
					stage: stage(),
				});
			}
		}

		if !measured {
			return Err(Error::Unmeasured {
				stage: stage(),
			});
		}

		Ok(())
	}
}
