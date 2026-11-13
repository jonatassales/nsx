//! Opening the pull request
//!
//! One function: it takes the forge and the pushed branch, opens a draft
//! pull request from it to the repository's default branch, and gives back
//! the pull request with its number and its title.
//!
//! ## Reads and writes
//!
//! - calls the forge, which runs `gh` to open the draft pull request
//!
//! ## Rules
//!
//! - It runs after the pushed line has printed.
//! - The title is the subject of the branch's first commit.
//! - When the branch already has an open pull request from an earlier run, it
//!   uses that one.
//! - A pull request that cannot be opened, or that comes back with no number
//!   or no title, stops the run before the pr line, with the branch pushed
//!   (the stop rule).
//! - Once the pr line has printed, the number goes to `checks`.

use crate::delivery::{
	branch::Branch, error::Error, forge::Forge, pull::Pull, readiness::Readiness,
};

impl Pull {
	pub async fn open(forge: &dyn Forge, branch: &Branch) -> Result<Self, Error> {
		let (number, title) = forge.open(&branch.name).await?;
		let named = number > 0 && !title.trim().is_empty();

		if !named {
			return Err(Error::Unnamed);
		}

		Ok(Self {
			number,
			title,
			state: Readiness::Draft,
		})
	}
}
