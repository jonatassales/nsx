//! The run's opening comment
//!
//! One function: it takes the authorized intent and gives back the first
//! line the run prints: which ticket runs, and on whose authority.
//!
//! ## Simulation
//!
//! - `scope`: `# run · 0031-TICKET-book-a-demo-runtime · the recorded intent, authorized by the team`
//!
//! ## Rules
//!
//! - The ticket is the intent's, here the simulation's ticket.
//! - The words "the team" are fixed. Only an authorized intent runs, and
//!   authorization is the team's: no actor's name is printed.
//! - The line is the same in every variant of the run.

use crate::handoff::authorization::Authorization;
use crate::session::line::Line;

impl Authorization {
	pub fn scope(&self) -> Line {
		Line::comment(
			"scope",
			&format!(
				"run · {} · the recorded intent, authorized by the team",
				self.intent.ticket
			),
		)
	}
}
