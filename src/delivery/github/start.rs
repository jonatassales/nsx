//! Starting the branch with git
//!
//! One function: it takes the branch's name and starts the branch from the
//! checked-out commit, or checks it out when the repository already has it.
//!
//! ## Reads and writes
//!
//! - runs `git rev-parse`, to know whether the branch exists
//! - runs `git checkout`, with `-b` for a new branch

use crate::delivery::{
	error::Error,
	github::{Github, PASSED},
};
use crate::process::program::Program;

impl Github<'_> {
	pub async fn start(&self, branch: &str) -> Result<(), Error> {
		let named = format!("refs/heads/{branch}");
		let asking = Program::tool(
			"git",
			&["rev-parse", "--verify", "--quiet", &named],
			&PASSED,
			&self.directory,
		);
		// git ends with another status for a branch it does not have: that
		// is an answer, not a failure.
		let known = self.programs.run(&asking).await?.status == Some(0);
		let arguments: &[&str] = if known {
			&["checkout", branch]
		} else {
			&["checkout", "-b", branch]
		};
		let checking = Program::tool("git", arguments, &PASSED, &self.directory);

		self.programs.finish(&checking).await?;

		Ok(())
	}
}
