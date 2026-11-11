//! The coding agent
//!
//! The team's own coding agent: the program that does a stage's work. It
//! lives outside nsx, so the run reaches it through one seam.
//!
//! Definition only: the trait `Agent`, through which a stage call starts the
//! coding agent and learns how it ended: its exit status and what it
//! printed. Tests replace it with a fake.
//!
//! ## Rules
//!
//! - The agent is a trusted process, started directly and never through a
//!   shell command string.
//! - What the agent prints never enters evidence.
//! - How the agent maps the effort onto its own reasoning setting is its own
//!   configuration. nsx guarantees only that the applied value reaches it.
//!
//! Spec 0080.

use std::future::Future;
use std::pin::Pin;

use crate::driver::{call::Call, config::Config};
use crate::process::{error::Error, output::Output};

pub trait Agent {
	fn start<'call>(
		&'call self,
		call: &'call Call,
		config: &'call Config,
	) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + 'call>>;
}
