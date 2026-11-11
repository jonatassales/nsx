//! What reading the harness can refuse
//!
//! The errors of this folder, each one plain sentence that names the file, so
//! that the stop rule can print it as it is.
//!
//! Definition only: the `Error` type.
//!
//! Spec 0078.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx found no criteria table in {}: a guardrail lists its criteria under ID, the rule and Checked at.",
		path.display()
	)]
	NoTable { path: PathBuf },
	#[error(
		"nsx could not read the criterion {row} in {}: a row gives an ID and is checked at plan, run or both.",
		path.display()
	)]
	Row { path: PathBuf, row: String },
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
