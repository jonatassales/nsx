//! The coding agent that does each stage as the simulation shows it
//!
//! It writes the stage's report where the call says, with the contract
//! samples' figures, and reports the tokens a test gives it for every call.
//! A test can make one stage fail, leave its report unwritten or its tokens
//! unsaid, put a folder where the run would write a file after that stage's
//! call, or make one stage never end.

mod start;

#[derive(Debug, Clone, Copy)]
pub struct Coder {
	pub input: u64,
	pub output: u64,
	pub failing: Option<&'static str>,
	pub silent: Option<&'static str>,
	pub unmeasured: Option<&'static str>,
	pub blocking: Option<(&'static str, &'static str)>,
	pub stalling: Option<&'static str>,
}

pub const CODER: Coder = Coder {
	input: 740_000,
	output: 48_000,
	failing: None,
	silent: None,
	unmeasured: None,
	blocking: None,
	stalling: None,
};
