//! What keeping a run's records can refuse
//!
//! The errors of this folder, each one plain sentence that names the file, so
//! that the stop rule can print it as it is.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx could not write {} as JSON: {cause}.", path.display())]
	Unwritten {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error("nsx could not read {}: {cause}.", path.display())]
	Unread {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error(
		"The run's usage is not known: nsx never prints a spend it did not measure."
	)]
	Unmeasured,
	#[error("nsx has no run id left for the runtime {runtime}.")]
	Exhausted { runtime: String },
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
