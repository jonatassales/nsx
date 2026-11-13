//! Pushing the branch
//!
//! One function: it takes the branch and the forge, pushes the branch to
//! `origin` with its upstream set, and gives back the branch with the pushed
//! head's hash.
//!
//! ## Reads and writes
//!
//! - calls the forge, which runs `git push` of the branch to `origin`, with
//!   its upstream set
//!
//! ## Rules
//!
//! - It runs after the `completed` line has printed.
//! - The hash is the head's whole hash: forty hexadecimal characters or
//!   more, in lowercase. Anything else the forge gives back is no hash, and
//!   a stop.
//! - A refused push, or a missing `origin`, stops the run before the pushed
//!   line (the stop rule).
//!
//! ## Open
//!
//! - Work the stages or the person left uncommitted. The push carries only
//!   the committed head. Until the owner decides, nsx commits nothing itself.

use crate::delivery::{branch::Branch, error::Error, forge::Forge};

const WHOLE: usize = 40;

impl Branch {
	pub async fn push(self, forge: &dyn Forge) -> Result<Self, Error> {
		let commit = forge.push(&self.name).await?;
		let digits = commit
			.chars()
			.all(|digit| digit.is_ascii_digit() || ('a'..='f').contains(&digit));
		let hashed = commit.len() >= WHOLE && digits;

		if !hashed {
			return Err(Error::Unhashed);
		}

		Ok(Self {
			commit: Some(commit),
			..self
		})
	}
}
