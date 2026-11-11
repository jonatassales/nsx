//! Running a stage call
//!
//! One function: it takes the coding agent, the driver configuration and the
//! project's root, starts the agent for the call, waits for it to end and
//! gives back how the call ended.
//!
//! ## Reads and writes
//!
//! - deletes the stage's report, when an earlier call left one
//! - starts the coding agent, through the `Agent` trait
//!
//! ## Rules
//!
//! - The report the stage's check reads is always this call's: one left at
//!   the same path is removed before the agent starts.
//! - A call that does not start is a stop. One that started gives back how it
//!   ended, whatever its exit status and whether or not it reported its
//!   tokens: the usage record keeps it first, and the stage then fails on a
//!   non-zero status or an unknown count (the stop rule). Nothing is retried.
//! - Nothing after the agent has started can fail here: the call's log is
//!   written by `log.rs`, once the usage record has kept the call.
//!
//! ## Open
//!
//! - What a failed or hung call prints and records, and whether a time limit
//!   is wanted. Until the owner decides, such a call stops the run silently.
//!
//! Spec 0080.

use std::io::ErrorKind;

use crate::driver::{
	agent::Agent, call::Call, config::Config, ended::Ended, error::Error,
	tokens::Tokens,
};
use crate::files::{inside::inside, root::Root};

impl Call {
	pub async fn run(
		&self,
		agent: &dyn Agent,
		config: &Config,
		root: &Root,
	) -> Result<Ended, Error> {
		let report = inside(root, &self.report)?;

		// A report an earlier call left would be read as this call's.
		match std::fs::remove_file(&report) {
			Ok(()) => {}
			Err(cause) if cause.kind() == ErrorKind::NotFound => {}
			Err(cause) => {
				return Err(Error::File(crate::files::error::Error::Unwritable {
					path: report,
					cause,
				}));
			}
		}

		let output = agent.start(self, config).await?;

		let printed = String::from_utf8_lossy(&output.printed);
		let tokens = Tokens::read(&printed, &config.usage);

		Ok(Ended {
			stage: self.stage.clone(),
			skill: self.skill.clone(),
			status: output.status,
			tokens,
			output: [output.printed, output.complained].concat(),
		})
	}
}
