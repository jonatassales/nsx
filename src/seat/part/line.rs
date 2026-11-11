//! The dev ticket's lines
//!
//! One function: it gives back the line the dev-ticket stage prints in the
//! state it is in: the `waiting` line when the run hands the reserved part
//! to the person, or the `closed` line once the stage is done.
//!
//! ## Simulation
//!
//! - `devTicketWaiting`: `console › waiting    dev-ticket · the person writes the reserved part`
//! - `devTicketClosed`: `✓ closed     dev-ticket · written without an agent · no paid call`
//!
//! ## Rules
//!
//! - The two lines follow the `implement` line and come before the
//!   `code-review` line (spec 0059).
//! - They print in every run that reaches them. No choice adds or removes
//!   them.
//!
//! Spec 0080.

use crate::contract::step::DONE;
use crate::seat::part::Part;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Part {
	pub fn line(&self) -> Line {
		if self.state == DONE {
			return Line {
				id: String::from("devTicketClosed"),
				kind: Kind::Success,
				shape: Shape::Result {
					label: String::from("closed"),
					value: String::from(
						"dev-ticket · written without an agent · no paid call",
					),
				},
			};
		}

		Line {
			id: String::from("devTicketWaiting"),
			kind: Kind::Output,
			shape: Shape::Labeled {
				label: String::from("waiting"),
				value: String::from("dev-ticket · the person writes the reserved part"),
			},
		}
	}
}
