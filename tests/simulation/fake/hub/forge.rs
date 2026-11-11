//! The five answers of the forge that follows a script

use nsx::checks::check::Check;
use nsx::delivery::forge::{Asked, Forge};

use crate::simulation::fake::hub::Hub;

impl Forge for Hub {
	fn start<'run>(&'run self, branch: &'run str) -> Asked<'run, ()> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("start {branch}"));

			Ok(())
		})
	}

	fn push<'run>(&'run self, branch: &'run str) -> Asked<'run, String> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("push {branch}"));

			Ok(self.commit.clone())
		})
	}

	fn open<'run>(&'run self, branch: &'run str) -> Asked<'run, (u64, String)> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("open {branch}"));

			Ok(self.opened.clone())
		})
	}

	fn checks<'run>(
		&'run self,
		pull: u64,
		commit: &'run str,
	) -> Asked<'run, Option<Vec<Check>>> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("checks {pull}"));

			// A pull request whose head is another commit has no checks of
			// the one asked about.
			if commit != self.commit {
				return Ok(None);
			}

			let mut listings = self.listings.borrow_mut();
			let listed = if listings.len() > 1 {
				listings.pop_front()
			} else {
				listings.front().cloned()
			};

			Ok(Some(listed.unwrap_or_default()))
		})
	}

	fn ready<'run>(&'run self, pull: u64) -> Asked<'run, ()> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("ready {pull}"));

			Ok(())
		})
	}
}
