//! What reading the workflow can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is. It names the file it could not read, or the stage
//! a person did not confirm.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx could not read the workflow {}: {cause}.", path.display())]
	Workflow {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error("nsx could not read the workflow {}: {rule}.", path.display())]
	Rule { path: PathBuf, rule: String },
	#[error(
		"The owner did not confirm {stage}, so the run stops there: answer yes to go on."
	)]
	Declined { stage: String },
	#[error("nsx could not read the answer for {stage}: {cause}.")]
	Unanswered {
		stage: String,
		cause: std::io::Error,
	},
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
