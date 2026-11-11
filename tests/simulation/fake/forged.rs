//! The forge that delivers as the simulation shows
//!
//! The branch starts, the push lands on `3f9c2e1`, the pull request is #19
//! with the simulation's title, and the four checks of that commit have
//! passed. A test can make one operation refused (`start`, `push`, `open`
//! or `ready`), one check fail, the checks never listed, or the forge never
//! answer about them.

mod answer;
mod forge;

#[derive(Debug, Clone, Copy)]
pub struct Forged {
	pub refusing: Option<&'static str>,
	pub failing: Option<&'static str>,
	pub unlisted: bool,
	pub stalling: bool,
}

pub const FORGED: Forged = Forged {
	refusing: None,
	failing: None,
	unlisted: false,
	stalling: false,
};

pub const CHECKS: [&str; 4] = ["build", "check", "test", "guardrails"];

pub const COMMIT: &str = "3f9c2e1b5a7d4c6e8f0a1b2c3d4e5f6a7b8c9d0e";

pub const TITLE: &str = "feat(demo): run book a demo as a microruntime";
