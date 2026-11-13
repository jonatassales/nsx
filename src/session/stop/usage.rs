//! A command line nsx does not accept, as a stop
//!
//! One function: it takes the parser's refusal and gives back the stop that
//! says, in one sentence, what was wrong with what the person typed.
//!
//! ## Rules
//!
//! - nsx has three commands. Anything else a person types is refused here,
//!   before any line prints.
//! - The ticket is the one argument nsx has, so a missing argument is a
//!   missing ticket.
//! - The parser's own explanation runs over several lines; only its first
//!   line, the cause, is kept.

use clap::error::ErrorKind;

use crate::session::stop::Stop;

const COMMANDS: &str = "init, agent plan <ticket> or agent run";
const TICKET: &str = "nsx agent plan needs the ticket's bare id, as in 0031-TICKET-book-a-demo-runtime.";

impl Stop {
	pub fn usage(refusal: &clap::Error) -> Self {
		let names_no_command = matches!(
			refusal.kind(),
			ErrorKind::MissingSubcommand
				| ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
		);

		if names_no_command {
			return Self {
				sentence: format!("nsx needs a command: {COMMANDS}."),
			};
		}

		if refusal.kind() == ErrorKind::MissingRequiredArgument {
			return Self {
				sentence: String::from(TICKET),
			};
		}

		let explanation = refusal.to_string();
		let cause = explanation
			.lines()
			.next()
			.unwrap_or_default()
			.trim_start_matches("error: ");

		Self {
			sentence: format!("nsx does not accept that: {cause}."),
		}
	}
}
