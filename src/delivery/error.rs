//! What the delivery can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type.
//!
//! ## Rules
//!
//! - No sentence quotes what `git` or `gh` printed: a remote's address can
//!   hold a token.

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx cannot name the run's branch: the ticket {ticket} has no number."
	)]
	Numberless { ticket: String },
	#[error(
		"The plan's cards hold no ticket, so the run has no spec to follow and no branch to name: plan the ticket again."
	)]
	Unplanned,
	#[error("git gave no hash for the pushed head.")]
	Unhashed,
	#[error("nsx could not read what gh said about the {about}: {cause}.")]
	Answer {
		about: &'static str,
		cause: serde_json::Error,
	},
	#[error("gh said nothing nsx can use about the {about}: it lacks {lacking}.")]
	Lacking {
		about: &'static str,
		lacking: &'static str,
	},
	#[error("gh opened no pull request nsx can name: it has no number or title.")]
	Unnamed,
	#[error("The pull request #{pull} stays a draft: {reason}.")]
	Draft { pull: u64, reason: &'static str },
	#[error(transparent)]
	Program(#[from] crate::process::error::Error),
}
