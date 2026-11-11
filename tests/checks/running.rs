//! The simulation's four checks, as the forge first lists them: running

use nsx::checks::{standing::Standing, suite::Suite};

use crate::checks::listed::listed;

pub const NAMES: [&str; 4] = ["build", "check", "test", "guardrails"];

pub fn running() -> Suite {
	Suite {
		checks: listed(&NAMES.map(|name| (name, Standing::Running))),
	}
}
