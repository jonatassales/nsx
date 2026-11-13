//! The checks of the run's pull request
//!
//! The suite is what the forge lists for the run's pull request: each check
//! with its name and its state, in the forge's order. It decides whether the
//! run may go on to Safe Seat: only when every check passed.
//!
//! Definition only: the `Suite` type: the listed checks. The pace of the
//! poll and the number of answers the first listing waits for sit beside
//! it. Its functions are the files in `suite/`.
//!
//! ## Rules
//!
//! - Only the state PULL REQUEST shows is recorded: `pullRequest.checks` in
//!   `run.json`, with `state` `running`, then `passed` with `passed` (`4`)
//!   and `total` (`4`). It goes through the record's one writer.
//! - The names and the per-check results are read, printed and used to
//!   decide. They are not kept, because no card field shows them.
//! - The forge is asked every 15 seconds.
//!
//! ## Open
//!
//! - The poll interval is not set. Until the owner decides, it is 15 seconds.

pub mod card;
pub mod line;
pub mod list;
pub mod passed;
pub mod poll;

use std::time::Duration;

use crate::checks::check::Check;

pub const INTERVAL: Duration = Duration::from_secs(15);

pub const PATIENCE: u32 = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suite {
	pub checks: Vec<Check>,
}
