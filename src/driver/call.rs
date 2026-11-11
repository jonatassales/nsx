//! One stage call
//!
//! One start of the coding agent, for one stage that has a skill. The call
//! carries what the agent needs to do the stage's work on the model and at
//! the effort the team chose, and where the skill must write its report. Its
//! result goes to the usage record (spec 0065).
//!
//! Definition only: the `Call` type, which carries the project's root, the
//! run, the ticket, the stage, the skill text, the applied model and effort
//! and the report path. Its result is an `Ended`, in its own file. Its
//! functions are the files in `call/`.
//!
//! ## Rules
//!
//! - One call per stage, never one per skill: `implement · tdd` is one call.
//! - The model is the price-list id, such as `claude-sonnet-5-5`. The effort
//!   is `low`, `medium` or `high`.
//! - The prompt names the stage's skill text, the ticket, the effort and the
//!   instruction to write the stage's report at the report path.
//! - Tokens the agent did not report are unknown (`null`), never zero.
//! - The implement call follows the `hook` line (spec 0060). The review call
//!   follows the dev ticket's `closed` line (spec 0061).
//!
//! Spec 0080.

pub mod log;
pub mod prompt;
pub mod run;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
	pub root: PathBuf,
	pub run: String,
	pub ticket: String,
	pub stage: String,
	pub skill: String,
	pub model: String,
	pub effort: String,
	pub report: PathBuf,
}
