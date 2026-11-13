//! Marking the pull request ready with gh
//!
//! One function: it takes the pull request's number and marks the pull
//! request ready for review on the forge.
//!
//! ## Reads and writes
//!
//! - runs `gh pr ready`

use crate::delivery::{
	error::Error,
	github::{Github, PASSED},
};
use crate::process::program::Program;

impl Github<'_> {
	pub async fn ready(&self, pull: u64) -> Result<(), Error> {
		let number = pull.to_string();
		let marking =
			Program::tool("gh", &["pr", "ready", &number], &PASSED, &self.directory);

		self.programs.finish(&marking).await?;

		Ok(())
	}
}
