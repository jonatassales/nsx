//! What the dev ticket can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type.
//!
//! Spec 0080.

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"Nobody said the dev ticket was written, so the run stops there: enter a line when the reserved part is done."
	)]
	Unanswered,
	#[error(
		"The run paid for {calls} stage calls while the dev ticket waited: the person's part was not written without an agent."
	)]
	Paid { calls: usize },
	#[error(
		"The usage record lost calls while the dev ticket waited: it held {before} of this run's and now holds {after}."
	)]
	Lost { before: usize, after: usize },
	#[error(
		"Safe Seat is not met: the dev ticket was not closed by the person with no paid call, so the pull request stays a draft."
	)]
	Unmet,
	#[error("nsx could not read the answer for the dev ticket: {cause}.")]
	Unread { cause: std::io::Error },
}
