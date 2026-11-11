//! Reading the pull request's checks with gh
//!
//! One function: it takes the pull request's number and the commit the run
//! pushed, and gives back that commit's checks as the forge lists them, each
//! with its name and where it stands.
//!
//! ## Reads and writes
//!
//! - runs `gh pr view`, asking for the pull request's head commit
//! - runs `gh pr checks`, asking for each check's name and bucket
//!
//! ## Rules
//!
//! - gh lists the checks of the pull request's head. While that head is not
//!   the commit the run pushed, the answer is nothing: either the forge has
//!   not caught up with the push, or someone pushed after the run. Another
//!   commit's checks are never read as this one's.
//! - gh ends with another status while a check is pending or has failed: the
//!   list it printed is still the answer. With no list, a pull request that
//!   has no check yet gives none, and anything else is a failure.
//! - `pass` is passed and `pending` is running. `fail`, `cancel` and
//!   `skipping` did not pass. Any other word is a stop: nsx never guesses a
//!   check's state.
//!
//! Spec 0081.

use serde_json::Value;

use crate::checks::{check::Check, standing::Standing};
use crate::delivery::{
	error::Error,
	github::{Github, PASSED},
};
use crate::process::program::Program;

const ABOUT: &str = "checks";

impl Github<'_> {
	pub async fn checks(
		&self,
		pull: u64,
		commit: &str,
	) -> Result<Option<Vec<Check>>, Error> {
		let number = pull.to_string();
		let lacking = |lacking: &'static str| Error::Lacking {
			about: ABOUT,
			lacking,
		};
		let viewing = Program::tool(
			"gh",
			&["pr", "view", &number, "--json", "headRefOid"],
			&PASSED,
			&self.directory,
		);
		let viewed = self.programs.finish(&viewing).await?;
		let head =
			serde_json::from_slice::<Value>(&viewed.printed).map_err(|cause| {
				Error::Answer {
					about: ABOUT,
					cause,
				}
			})?;
		let head = head
			.get("headRefOid")
			.and_then(Value::as_str)
			.ok_or_else(|| lacking("the pull request's head"))?;

		if head != commit {
			return Ok(None);
		}

		let listing = Program::tool(
			"gh",
			&["pr", "checks", &number, "--json", "name,bucket"],
			&PASSED,
			&self.directory,
		);
		let output = self.programs.run(&listing).await?;
		let silent = output.printed.iter().all(u8::is_ascii_whitespace);
		let unreported = String::from_utf8_lossy(&output.complained)
			.contains("no checks reported");
		let unchecked = silent && unreported;

		if unchecked {
			return Ok(Some(Vec::new()));
		}

		// gh said nothing to read: its exit status is the failure.
		let output = if silent {
			output.ended(&listing)?
		} else {
			output
		};
		let listed = serde_json::from_slice::<Vec<Value>>(&output.printed)
			.map_err(|cause| Error::Answer {
				about: ABOUT,
				cause,
			})?;

		let checks = listed
			.iter()
			.map(|check| {
				let name = check
					.get("name")
					.and_then(Value::as_str)
					.ok_or_else(|| lacking("a name"))?;
				let bucket = check
					.get("bucket")
					.and_then(Value::as_str)
					.ok_or_else(|| lacking("a state"))?;
				let standing = match bucket {
					"pass" => Standing::Passed,
					"pending" => Standing::Running,
					"fail" | "cancel" | "skipping" => Standing::Failed,
					_ => return Err(lacking("a state nsx knows")),
				};

				Ok(Check {
					name: String::from(name),
					standing,
				})
			})
			.collect::<Result<Vec<_>, Error>>()?;

		Ok(Some(checks))
	}
}
