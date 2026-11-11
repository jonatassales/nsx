//! Running the typed command
//!
//! One function: it takes the parsed command line and runs the one command it
//! names, `nsx init` or one of the `agent` family. It gives back how that
//! command ended: on its last simulation line, or stopped.
//!
//! ## Rules
//!
//! - A command that ends on its last simulation line exits 0. A stop exits 1
//!   (`session/stop.rs`).
//! - The typed command is never printed.
//!
//! Spec 0077.

use crate::command::{cli::Cli, session::Session};
use crate::session::stop::Stop;

impl Cli {
	pub async fn execute(self, session: &mut Session<'_>) -> Result<(), Stop> {
		match self {
			Self::Init(init) => init.execute(session).await,
			Self::Agent(agent) => agent.execute(session).await,
		}
	}
}
