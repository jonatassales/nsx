//! The runtime section of the run's record
//!
//! One function: it takes the ready runtime's state and the run's record,
//! and fills the record's runtime section, as ready.
//!
//! ## Simulation
//!
//! - Card: Workflow · the runtime, `ready` from the `runtimeReady` line
//!
//! ## Rules
//!
//! - The run asks here when the runtime is ready. `removed` and `left` are
//!   absent until the runtime is stopped, which the execution ledger marks
//!   (`execution/card.rs`).

use crate::contract::{
	hosted::{Hosted, READY},
	run::Run,
};
use crate::local::state::State;

impl State {
	pub fn card(&self, run: &mut Run) {
		run.runtime = Some(Hosted {
			revision: self.revision,
			deployments: self.deployments,
			cpu_millis: self.cpu_millis,
			memory_mib: self.memory_mib,
			state: String::from(READY),
			removed: None,
			left: None,
		});
	}
}
