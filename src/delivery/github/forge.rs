//! git and gh, as the forge the run asks
//!
//! `Github` is `Forge` as `git` and `gh` serve it: each operation of the
//! trait is the file of the same name in this folder.
//!
//! Spec 0081.

use crate::checks::check::Check;
use crate::delivery::{
	forge::{Asked, Forge},
	github::Github,
};

impl Forge for Github<'_> {
	fn start<'run>(&'run self, branch: &'run str) -> Asked<'run, ()> {
		Box::pin(Github::start(self, branch))
	}

	fn push<'run>(&'run self, branch: &'run str) -> Asked<'run, String> {
		Box::pin(Github::push(self, branch))
	}

	fn open<'run>(&'run self, branch: &'run str) -> Asked<'run, (u64, String)> {
		Box::pin(Github::open(self, branch))
	}

	fn checks<'run>(
		&'run self,
		pull: u64,
		commit: &'run str,
	) -> Asked<'run, Option<Vec<Check>>> {
		Box::pin(Github::checks(self, pull, commit))
	}

	fn ready<'run>(&'run self, pull: u64) -> Asked<'run, ()> {
		Box::pin(Github::ready(self, pull))
	}
}
