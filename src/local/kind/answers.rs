//! Whether Docker answers
//!
//! One function: it asks Docker for its state and says whether it answered.
//!
//! ## Reads and writes
//!
//! - runs `docker info`
//!
//! ## Rules
//!
//! - A Docker that is installed and stopped does not answer: that is an
//!   answer, not a failure.
//! - No Docker to run at all is a stop, with a sentence that says how this
//!   system gets one.

use crate::local::{error::Error, kind::Kind, system::System};

impl Kind<'_> {
	pub async fn answers(&self) -> Result<bool, Error> {
		let asking = self.program("docker", &["info"])?;

		match self.programs.run(&asking).await {
			Ok(output) => Ok(output.status == Some(0)),
			Err(_missing) => Err(match self.system {
				System::Mac => Error::RuntimelessMac,
				System::Linux => Error::RuntimelessLinux,
			}),
		}
	}
}
