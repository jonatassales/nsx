//! Polling the checks
//!
//! One function: it takes the suite as first listed, the forge, the run's
//! pull request, the commit the run pushed and the pace of the poll, and
//! asks the forge until every listed check has passed. It gives back the
//! suite as the forge last listed it, or the stop when a check did not pass.
//!
//! ## Reads and writes
//!
//! - calls the forge, which reads each check's name and state with `gh`
//!
//! ## Rules
//!
//! - The wait has no deadline. It ends when the checks end or the person
//!   interrupts.
//! - A check that completes as failed, cancelled or skipped is a stop under
//!   the stop rule, as soon as the forge says so: no line, the runtime taken
//!   down, the pull request left a draft.
//! - An interrupt, or a forge answer that cannot be read, is a stop under the
//!   same rule.
//! - An answer that lists no check any more decides nothing: the forge is
//!   asked again.
//! - A pull request that left the pushed commit, because someone pushed
//!   after the run, has no checks of that commit to follow: a stop, said as
//!   it is, never an endless wait.
//!
//! ## Open
//!
//! - Checks a workflow starts only after others pass are not listed until
//!   then. The poll ends on the first answer in which every listed check
//!   passed, so such a later check is not waited for. The forge gives nsx
//!   no way to know one is coming.
//! - A check that fails, is cancelled or is skipped: the stop is provisional.
//!   The owner may prefer the run to report and go on, or the coding agent to
//!   fix and push again.

use std::time::Duration;

use crate::checks::{error::Error, standing::Standing, suite::Suite};
use crate::delivery::forge::Forge;

impl Suite {
	pub async fn poll(
		mut self,
		forge: &dyn Forge,
		pull: u64,
		commit: &str,
		interval: Duration,
	) -> Result<Self, Error> {
		loop {
			let failed = self
				.checks
				.iter()
				.find(|check| check.standing == Standing::Failed);

			if let Some(check) = failed {
				return Err(Error::Failed {
					name: check.name.clone(),
				});
			}

			if self.passed() {
				return Ok(self);
			}

			tokio::time::sleep(interval).await;

			self.checks =
				forge.checks(pull, commit).await?.ok_or(Error::Elsewhere {
					pull,
				})?;
		}
	}
}
