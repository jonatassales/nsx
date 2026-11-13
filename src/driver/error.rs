//! What a stage call can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx found no driver configuration at {}: the team writes it to say how its coding agent is started.",
		path.display()
	)]
	Missing { path: PathBuf },
	#[error("nsx could not read the driver configuration {}: {cause}.", path.display())]
	Config {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error(
		"nsx will not use the driver configuration {}: its args must carry {{{{model}}}} and {{{{effort}}}}.",
		path.display()
	)]
	Arguments { path: PathBuf },
	#[error("The coding agent ended {stage} with the exit status {status}.")]
	Failed { stage: String, status: i32 },
	#[error("The coding agent was ended by a signal during {stage}.")]
	Killed { stage: String },
	#[error(
		"The coding agent reported no token usage for {stage}: nsx never prints a spend it did not measure."
	)]
	Unmeasured { stage: String },
	#[error(transparent)]
	Program(#[from] crate::process::error::Error),
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
