//! What sizing the plan and writing its draft can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"The components need {cpu_millis}m and {memory_mib} MiB, over the bound of {bound_cpu_millis}m and {bound_memory_mib} MiB the team set in {}.",
		design.display()
	)]
	Over {
		cpu_millis: i64,
		memory_mib: i64,
		bound_cpu_millis: i64,
		bound_memory_mib: i64,
		design: PathBuf,
	},
	#[error(
		"nsx validated the runtime.md draft against the schema and counted {errors} errors: {problems}."
	)]
	Invalid { errors: usize, problems: String },
	#[error("nsx could not write {} as JSON: {cause}.", path.display())]
	Unwritten {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
