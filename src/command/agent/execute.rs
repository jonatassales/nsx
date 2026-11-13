//! Running the plan or the run
//!
//! One function: it takes the `agent` command a person typed and runs the
//! plan or the run. It gives back how that command ended: on its last
//! simulation line, or stopped.

use crate::command::{agent::Agent, session::Session};
use crate::session::stop::Stop;

impl Agent {
	pub async fn execute(self, session: &mut Session<'_>) -> Result<(), Stop> {
		match self {
			Self::Plan(plan) => plan.execute(session).await,
			Self::Run(run) => run.execute(session).await,
		}
	}
}
