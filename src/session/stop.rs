//! The stop rule
//!
//! The single rule every folder follows. A failure, a refusal or an
//! interruption that the simulation does not show is a stop: the command
//! ends without printing a line the simulation does not have.
//!
//! Definition only: the `Stop` type, a failure from any folder, as the one
//! plain sentence that names its cause. Its functions are the files in
//! `stop/`.
//!
//! ## Rules
//!
//! - The command prints no further session line, and no warning or error
//!   line of its own. The lines already printed stay.
//! - A plan that stops writes no draft and no recorded intent. Its card data
//!   stays as far as it got.
//! - If the run started the runtime, it is taken down without a line.
//! - The records reflect what happened, never more: a paid call is never
//!   left out of the usage record, a stage keeps the state it reached in the
//!   run's record, and money settles as spec 0056 says.
//! - Nothing is retried and nothing resumes inside the command.
//! - One plain sentence naming the cause goes to standard error. It names no
//!   secret. Standard error is not the session: standard output stays a
//!   prefix of the simulation.
//! - The command exits with status 1.
//! - A command that ends on its last simulation line exits 0. The
//!   over-budget pause is a shown ending and exits 0. An interrupt (Ctrl-C)
//!   is a stop.
//!
//! Spec 0077.

pub mod exit;
pub mod from;
pub mod interrupted;
pub mod usage;

#[derive(Debug)]
pub struct Stop {
	pub sentence: String,
}
