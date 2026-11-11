//! Marking the pull request ready
//!
//! One function: it takes the draft pull request, the forge, the checks,
//! Safe Seat's verdict and the Review tick, marks the pull request ready on
//! the forge, and gives it back in the state `ready for review` or
//! `ready to merge`.
//!
//! ## Reads and writes
//!
//! - calls the forge, which runs `gh` to mark the pull request ready
//!
//! ## Rules
//!
//! - Ready means checked and met: the forge is asked only when every check
//!   passed and Safe Seat is met. Otherwise the pull request stays a draft
//!   and the run stops, with the forge not asked.
//! - It runs after the runtime's stopped line has printed (spec 0062).
//! - The state follows spec 0057's `reviewed` and nothing else.
//! - A pull request that cannot be marked ready stops the run before the
//!   ready line, and it stays a draft (the stop rule).
//! - Marking ready is the last thing nsx does to the pull request. It never
//!   merges.
//!
//! Spec 0081.

use crate::checks::suite::Suite;
use crate::delivery::{
	error::Error, forge::Forge, pull::Pull, readiness::Readiness,
};
use crate::seat::verdict::Verdict;

impl Pull {
	pub async fn ready(
		self,
		forge: &dyn Forge,
		suite: &Suite,
		verdict: &Verdict,
		reviewed: bool,
	) -> Result<Self, Error> {
		if !suite.passed() {
			return Err(Error::Draft {
				pull: self.number,
				reason: "its checks have not all passed",
			});
		}

		if !verdict.met {
			return Err(Error::Draft {
				pull: self.number,
				reason: "Safe Seat is not met",
			});
		}

		forge.ready(self.number).await?;

		let state = if reviewed {
			Readiness::Review
		} else {
			Readiness::Merge
		};

		Ok(Self {
			state,
			..self
		})
	}
}
