//! The dev ticket as the run's record keeps it
//!
//! One function: it takes the dev-ticket stage of `run.json` and gives back
//! the part as the run closed it, or left it.
//!
//! ## Rules
//!
//! - Safe Seat is decided from what the record says, not from what the run
//!   remembers: the state, who closed the stage and the paid calls counted.

use crate::contract::step::Step;
use crate::seat::part::Part;

impl Part {
	pub fn of(step: &Step) -> Self {
		Self {
			state: step.state.clone(),
			closed_by: step.closed_by.clone(),
			paid_calls: step.paid_calls,
		}
	}
}
