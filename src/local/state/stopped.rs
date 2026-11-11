//! The runtime that is stopped
//!
//! One function: it takes the ready runtime and the settled execution
//! ledger, and gives back the runtime's state, stopped, with the number of
//! Deployments removed.
//!
//! ## Rules
//!
//! - The removed count is what the execution ledger records as removed.
//! - It is asked only once the ledger is settled: a state is stopped only
//!   when nothing is left.
//!
//! Spec 0081.

use crate::local::{execution::Execution, state::State};

impl State {
	pub fn stopped(self, execution: &Execution) -> Self {
		Self {
			removed: Some(execution.removed.len() as u64),
			..self
		}
	}
}
