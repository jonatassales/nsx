//! Closing the dev ticket
//!
//! One function: it takes the waiting stage once the person has said they are
//! done and the number of stage calls the run made meanwhile, and gives the
//! stage back closed: `done`, with `closedBy: person` and that count.
//!
//! ## Rules
//!
//! - No stage-driver call may happen between the stage's two lines: the run's
//!   script makes none, and the count it measured, zero, is what is
//!   recorded. `verdict/reach.rs` reads it.
//! - `code-review` runs only after the close (spec 0059).
//!
//! Spec 0080.

use crate::contract::step::DONE;
use crate::seat::part::{PERSON, Part};

impl Part {
	pub fn close(self, paid_calls: usize) -> Self {
		Self {
			state: String::from(DONE),
			closed_by: Some(String::from(PERSON)),
			paid_calls: Some(paid_calls as u64),
		}
	}
}
