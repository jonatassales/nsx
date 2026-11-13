//! What the effort hook can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type.

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx will not start implement: the run has recorded no effort yet.")]
	Unrecorded,
	#[error(
		"nsx will not start implement: the recorded effort, {recorded}, is not the applied choice, {applied}."
	)]
	Mismatch { recorded: String, applied: String },
}
