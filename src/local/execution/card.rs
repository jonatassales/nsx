//! The runtime, stopped, in the run's record
//!
//! One function: it takes a settled execution ledger and the run's record,
//! and marks the record's runtime stopped, with what was removed and nothing
//! left.
//!
//! ## Simulation
//!
//! - Card: Workflow · the runtime, `stopped` from the `runtimeStopped` line
//!
//! ## Rules
//!
//! - A ledger that is not settled changes nothing: the record says
//!   `stopped` only once nothing is left.
//! - A record with no runtime section stays without one: the run never said
//!   the runtime was ready.
//! - The record is marked on a silent take-down too, so that it never says
//!   `ready` of a runtime that is gone.

use crate::contract::{hosted::STOPPED, run::Run};
use crate::local::execution::Execution;

impl Execution {
	pub fn card(&self, run: &mut Run) {
		if !self.settled {
			return;
		}

		let Some(runtime) = run.runtime.as_mut() else {
			return;
		};

		runtime.state = String::from(STOPPED);
		runtime.removed = Some(self.removed.len() as u64);
		runtime.left = Some(0);
	}
}
