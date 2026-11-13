//! The first listing of the checks
//!
//! One function: it takes the forge, the run's pull request, the commit the
//! run pushed and the pace of the poll, and asks the forge until it lists at
//! least one check of that commit. It gives back the suite as first listed.
//!
//! ## Reads and writes
//!
//! - calls the forge, which reads each check's name and state with `gh`
//!
//! ## Rules
//!
//! - The `checks` line prints from this answer, with every listed name.
//! - A pull request that lists no check after twenty answers is a stop: the
//!   run has nothing to follow, and it does not wait forever. When the pull
//!   request was still not at the pushed commit by then, the stop says that
//!   instead.
//! - A forge answer that cannot be read is a stop under the stop rule.
//!
//! ## Open
//!
//! - A pull request with no checks. The owner may prefer `0 of 0` to the
//!   stop, or a longer wait than twenty answers.

use std::time::Duration;

use crate::checks::{
	error::Error,
	suite::{PATIENCE, Suite},
};
use crate::delivery::forge::Forge;

impl Suite {
	pub async fn list(
		forge: &dyn Forge,
		pull: u64,
		commit: &str,
		interval: Duration,
	) -> Result<Self, Error> {
		let mut elsewhere = false;

		for _answer in 0..PATIENCE {
			let answered = forge.checks(pull, commit).await?;
			let listed = answered.as_ref().is_some_and(|checks| !checks.is_empty());

			elsewhere = answered.is_none();

			if let Some(checks) = answered.filter(|_checks| listed) {
				return Ok(Self {
					checks,
				});
			}

			tokio::time::sleep(interval).await;
		}

		if elsewhere {
			return Err(Error::Elsewhere {
				pull,
			});
		}

		Err(Error::Unlisted {
			pull,
			answers: PATIENCE,
		})
	}
}
