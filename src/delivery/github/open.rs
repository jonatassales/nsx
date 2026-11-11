//! Opening the draft pull request with gh
//!
//! One function: it takes the branch's name and gives back the number and
//! the title of the branch's pull request: the open one an earlier run
//! left, or a draft it opens now.
//!
//! ## Reads and writes
//!
//! - runs `gh pr view`, to find the branch's pull request
//! - runs `gh pr ready --undo`, for a reused pull request that is not a
//!   draft
//! - runs `gh pr create`, as a draft titled with the branch's first commit
//!
//! ## Rules
//!
//! - gh ends with another status for a branch that has no pull request:
//!   that is an answer, not a failure. A pull request that is closed or
//!   merged is not reused.
//! - An open pull request that is not a draft is made one again
//!   (`gh pr ready --undo`): it is marked ready only at the end of the run.
//! - What gh says is read strictly: a number and a title, or a stop.
//!
//! Spec 0081.

use serde_json::Value;

use crate::delivery::{
	error::Error,
	github::{Github, PASSED},
};
use crate::process::{output::Output, program::Program};

const ABOUT: &str = "pull request";

impl Github<'_> {
	pub async fn open(&self, branch: &str) -> Result<(u64, String), Error> {
		// What gh said of one pull request: its number, its title, whether
		// it is open and whether it is a draft.
		fn said(output: &Output) -> Result<(u64, String, bool, bool), Error> {
			let lacking = |lacking: &'static str| Error::Lacking {
				about: ABOUT,
				lacking,
			};
			let viewed =
				serde_json::from_slice::<Value>(&output.printed).map_err(|cause| {
					Error::Answer {
						about: ABOUT,
						cause,
					}
				})?;
			let number = viewed
				.get("number")
				.and_then(Value::as_u64)
				.ok_or_else(|| lacking("a number"))?;
			let title = viewed
				.get("title")
				.and_then(Value::as_str)
				.ok_or_else(|| lacking("a title"))?;
			let state = viewed
				.get("state")
				.and_then(Value::as_str)
				.ok_or_else(|| lacking("a state"))?;

			let draft = viewed
				.get("isDraft")
				.and_then(Value::as_bool)
				.ok_or_else(|| lacking("whether it is a draft"))?;

			Ok((number, String::from(title), state == "OPEN", draft))
		}

		let viewing = Program::tool(
			"gh",
			&["pr", "view", branch, "--json", "number,title,state,isDraft"],
			&PASSED,
			&self.directory,
		);
		let found = self.programs.run(&viewing).await?;

		let reused = match found.status {
			Some(0) => {
				Some(said(&found)?).filter(|(_number, _title, open, _draft)| *open)
			}
			_ => None,
		};

		if let Some((number, title, _open, draft)) = reused {
			// A pull request someone marked ready goes back to a draft: it
			// is ready again only once checked and met.
			if !draft {
				let undoing = Program::tool(
					"gh",
					&["pr", "ready", branch, "--undo"],
					&PASSED,
					&self.directory,
				);

				self.programs.finish(&undoing).await?;
			}

			return Ok((number, title));
		}

		let creating = Program::tool(
			"gh",
			&["pr", "create", "--draft", "--fill-first", "--head", branch],
			&PASSED,
			&self.directory,
		);

		self.programs.finish(&creating).await?;

		let opened = self.programs.finish(&viewing).await?;
		let (number, title, _open, _draft) = said(&opened)?;

		Ok((number, title))
	}
}
