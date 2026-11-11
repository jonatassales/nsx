//! What the acts of a run share
//!
//! What every stage call of one run needs and adds to, and what the delivery
//! takes over from the stages: how the coding agent is started, the spec the
//! ticket links, the usage record as it fills, the effort record once the
//! hook has written it, and the branch the stages work on.
//!
//! Definition only: the `Work` type: the driver configuration, the linked
//! spec, the usage record with the file that keeps it, the effort record,
//! absent until Implement starts, and the run's branch.
//!
//! Spec 0077.

use std::path::PathBuf;

use crate::contract::linked::Linked;
use crate::delivery::branch::Branch;
use crate::driver::config::Config;
use crate::evidence::usage::Usage;
use crate::hook::effort::Effort;

#[derive(Debug, Clone)]
pub struct Work {
	pub config: Config,
	pub spec: Linked,
	pub usage: Usage,
	pub file: PathBuf,
	pub effort: Option<Effort>,
	pub branch: Branch,
}
