//! What can go wrong with a program
//!
//! Each failure of this folder, worded as the one plain sentence the stop
//! rule sends to standard error.
//!
//! Definition only: the `Error` type: a program that could not be started
//! or waited for, and a tool that had to end well and ended with another
//! status, or by a signal.
//!
//! ## Rules
//!
//! - A sentence names the program and the cause. For a tool that ended
//!   badly it also names the tool's first argument, which nsx chose itself
//!   (`git push`): never its other arguments, its environment or what it
//!   printed, which may hold a secret.
//! - A program that ran and failed is a failure only where its caller asked
//!   for it to end well (`runner/finish.rs`). Otherwise the caller reads the
//!   output's status.
//!
//! Shared: used by `driver`, `local`, `delivery` and `console`.

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx could not run {program}: {cause}.")]
	NotRun {
		program: String,
		cause: std::io::Error,
	},
	#[error("{command} ended with the exit status {status}.")]
	Failed { command: String, status: i32 },
	#[error("{command} was ended by a signal.")]
	Killed { command: String },
}
