//! What following the checks can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type.

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"The check {name} did not pass, so the run stops: the pull request stays a draft."
	)]
	Failed { name: String },
	#[error(
		"The pull request #{pull} listed no check after {answers} answers from the forge: nsx has nothing to follow."
	)]
	Unlisted { pull: u64, answers: u32 },
	#[error(
		"The pull request #{pull} is not at the commit the run pushed, so the run has no checks of its own to follow: its head moved."
	)]
	Elsewhere { pull: u64 },
	#[error(transparent)]
	Forge(#[from] crate::delivery::error::Error),
}
