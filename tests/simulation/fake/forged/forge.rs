//! The five answers of the forge that delivers as the simulation shows

use nsx::checks::{check::Check, standing::Standing};
use nsx::delivery::forge::{Asked, Forge};

use crate::simulation::fake::forged::{CHECKS, COMMIT, Forged, TITLE};

impl Forge for Forged {
	fn start<'run>(&'run self, _branch: &'run str) -> Asked<'run, ()> {
		Box::pin(async { self.answer("start", "git checkout", ()) })
	}

	fn push<'run>(&'run self, _branch: &'run str) -> Asked<'run, String> {
		Box::pin(async { self.answer("push", "git push", String::from(COMMIT)) })
	}

	fn open<'run>(&'run self, _branch: &'run str) -> Asked<'run, (u64, String)> {
		Box::pin(async { self.answer("open", "gh pr", (19, String::from(TITLE))) })
	}

	fn checks<'run>(
		&'run self,
		_pull: u64,
		commit: &'run str,
	) -> Asked<'run, Option<Vec<Check>>> {
		Box::pin(async move {
			if self.stalling {
				std::future::pending::<()>().await;
			}

			// The pull request is at the commit this forge pushed, and no
			// other.
			if commit != COMMIT {
				return Ok(None);
			}

			if self.unlisted {
				return Ok(Some(Vec::new()));
			}

			let checks = CHECKS
				.iter()
				.map(|name| Check {
					name: String::from(*name),
					standing: if self.failing == Some(*name) {
						Standing::Failed
					} else {
						Standing::Passed
					},
				})
				.collect();

			Ok(Some(checks))
		})
	}

	fn ready<'run>(&'run self, _pull: u64) -> Asked<'run, ()> {
		Box::pin(async { self.answer("ready", "gh pr", ()) })
	}
}
