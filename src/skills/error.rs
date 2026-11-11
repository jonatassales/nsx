//! What a stage's report can be refused for
//!
//! The errors of this folder, each one plain sentence that names the stage,
//! so that the stop rule can print it as it is.
//!
//! Definition only: the `Error` type.
//!
//! Spec 0080.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"The {stage} stage left no report at {}: its skill ends by writing one.",
		path.display()
	)]
	Missing { stage: String, path: PathBuf },
	#[error("nsx could not read the {stage} stage's report: {cause}.")]
	Unread {
		stage: String,
		cause: crate::contract::error::Error,
	},
	#[error("nsx refuses the {stage} stage's report: {field} {rule}.")]
	Refused {
		stage: String,
		field: &'static str,
		rule: String,
	},
	#[error(
		"The plan's cards link no spec for the ticket: plan the ticket again."
	)]
	Unlinked,
	#[error("nsx knows no report for the stage {stage}.")]
	Unknown { stage: String },
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
