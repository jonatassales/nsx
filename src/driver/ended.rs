//! How a stage call ended
//!
//! The result of one start of the coding agent: what the usage record keeps
//! and what decides whether the stage's line may print.
//!
//! Definition only: the `Ended` type, which carries the stage, the skill
//! text, the exit status, the tokens the call used and everything the agent
//! printed, for the call's log. Its function is the file in `ended/`.
//!
//! ## Rules
//!
//! - The exit status is unknown when the agent was ended by a signal.
//! - Tokens the agent did not report are unknown, never zero.
//!
//! Spec 0080.

pub mod accept;

use crate::driver::tokens::Tokens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ended {
	pub stage: String,
	pub skill: String,
	pub status: Option<i32>,
	pub tokens: Tokens,
	pub output: Vec<u8>,
}
