//! Pushing the branch with git
//!
//! One function: it takes the branch's name, pushes the branch to `origin`
//! with its upstream set, and gives back the pushed head's hash, whole.
//!
//! ## Reads and writes
//!
//! - runs `git push` of the branch to `origin`, with its upstream set
//! - runs `git rev-parse`, for the head's hash
//!
//! Spec 0081.

use crate::delivery::{
	error::Error,
	github::{Github, PASSED},
};
use crate::process::program::Program;

impl Github<'_> {
	pub async fn push(&self, branch: &str) -> Result<String, Error> {
		let pushing = Program::tool(
			"git",
			&["push", "--set-upstream", "origin", branch],
			&PASSED,
			&self.directory,
		);

		self.programs.finish(&pushing).await?;

		let named = format!("refs/heads/{branch}");
		let asking = Program::tool(
			"git",
			&["rev-parse", "--verify", &named],
			&PASSED,
			&self.directory,
		);
		let output = self.programs.finish(&asking).await?;
		let commit = String::from_utf8_lossy(&output.printed);

		Ok(String::from(commit.trim()))
	}
}
