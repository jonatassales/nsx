//! The forge
//!
//! Where the team reviews: the repository and the service that holds the
//! pull request. The run starts its branch there, pushes it, opens a draft
//! pull request, reads the pull request's checks and marks it ready at the
//! end.
//!
//! Definition only: the trait `Forge` and its operations, so that tests can
//! replace it with a fake, and `Asked`, the answer each gives.
//!
//! ## Rules
//!
//! - It has no merge operation: nothing in nsx merges.
//! - It is the one port spec 0063 asks for, with its four operations: start
//!   the branch, push, open a draft pull request, mark it ready. Spec 0064
//!   extends it with the read of the checks.
//! - No operation commits.
//! - Pushing gives back the pushed head's hash, whole, and opening gives
//!   back the pull request's number and title.
//! - The checks asked for are those of one commit of the pull request, the
//!   one the run pushed. A pull request that is not at that commit has no
//!   checks of it to give: the answer is nothing, which is not an empty
//!   list.
//!
//! Spec 0081.

use std::future::Future;
use std::pin::Pin;

use crate::checks::check::Check;
use crate::delivery::error::Error;

pub type Asked<'run, Answer> =
	Pin<Box<dyn Future<Output = Result<Answer, Error>> + 'run>>;

pub trait Forge {
	fn start<'run>(&'run self, branch: &'run str) -> Asked<'run, ()>;

	fn push<'run>(&'run self, branch: &'run str) -> Asked<'run, String>;

	fn open<'run>(&'run self, branch: &'run str) -> Asked<'run, (u64, String)>;

	fn checks<'run>(
		&'run self,
		pull: u64,
		commit: &'run str,
	) -> Asked<'run, Option<Vec<Check>>>;

	fn ready<'run>(&'run self, pull: u64) -> Asked<'run, ()>;
}
