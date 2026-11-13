//! The runtime's two lines
//!
//! One function: it gives back the line for the runtime's state. Ready: the
//! runtime's id, the applied revision, the number of Deployments started and
//! the sum of their shares. Stopped: the number of Deployments removed, and
//! that nothing is left.
//!
//! ## Simulation
//!
//! - `runtimeReady`: `runtime    book-a-demo-runtime · revision 1 · 5 Deployments ready · 1450m · 2304 MiB`
//! - `runtimeStopped`: `runtime    stopped · 5 Deployments removed · nothing left`
//!
//! ## Rules
//!
//! - `runtimeReady` prints after `codeReview` and before `completed`.
//!   `runtimeStopped` prints after `safeSeatMet` and before `ready`.
//! - The runtime's id and the revision are the applied intent's. The figures
//!   are the sum of the started Deployments' shares, in plain integers.
//! - The removed count is what the execution ledger records as removed.
//!   "nothing left" prints only when the space's absence is confirmed: a
//!   state is stopped only then.
//! - Both are output lines, the same with Review ticked or not and with any
//!   stages checked. Neither prints when the gate pauses the run.
//! - No other runtime line exists: none for a bring-up, a timeout or an
//!   unknown state.

use crate::local::state::State;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl State {
	pub fn line(&self) -> Line {
		let (id, value) = match self.removed {
			Some(removed) => (
				"runtimeStopped",
				format!("stopped · {removed} Deployments removed · nothing left"),
			),
			None => (
				"runtimeReady",
				format!(
					"{} · revision {} · {} Deployments ready · {}m · {} MiB",
					self.runtime,
					self.revision,
					self.deployments,
					self.cpu_millis,
					self.memory_mib
				),
			),
		};

		Line {
			id: String::from(id),
			kind: Kind::Output,
			shape: Shape::Status {
				label: String::from("runtime"),
				value,
			},
		}
	}
}
