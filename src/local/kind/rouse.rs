//! Starting a stopped Docker
//!
//! One function: it starts the Docker this machine has, as its system
//! allows.
//!
//! ## Reads and writes
//!
//! - on a Mac, runs `open -g -a Docker`, which starts Docker Desktop in the
//!   background
//!
//! ## Rules
//!
//! - On Linux, starting the daemon needs root: it is a stop with a sentence
//!   that gives the command, and nsx asks for no password.
//! - On a Mac with a Docker client and no Docker Desktop to start, it is a
//!   stop that says a container runtime is missing.

use crate::local::{error::Error, kind::Kind, system::System};

impl Kind<'_> {
	pub async fn rouse(&self) -> Result<(), Error> {
		if self.system == System::Linux {
			return Err(Error::Stopped);
		}

		let opening = self.program("open", &["-g", "-a", "Docker"])?;
		let opened = self.programs.run(&opening).await?;

		if opened.status != Some(0) {
			return Err(Error::RuntimelessMac);
		}

		Ok(())
	}
}
