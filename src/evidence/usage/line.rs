//! The completed line
//!
//! One function: it takes the observed spend and the reservation, both
//! `money`'s, and gives back the line that closes the implementation.
//!
//! ## Simulation
//!
//! - `completed`: `✓ completed  implementation · $9.80 observed of $11.40 reserved · usage record`
//!
//! ## Rules
//!
//! - The two amounts are for the applied model and effort (spec 0056). This
//!   file prices nothing.
//! - A run whose usage is unknown stops before this line: it never prints a
//!   figure that was not measured.
//! - It prints after the runtime's ready line and before the pushed line. It
//!   does not print when the gate paused.
//!
//! Spec 0080.

use crate::evidence::usage::Usage;
use crate::money::cents::Cents;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Usage {
	pub fn line(observed: Cents, reserved: Cents) -> Line {
		Line {
			id: String::from("completed"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("completed"),
				value: format!(
					"implementation · {} observed of {} reserved · usage record",
					observed.format(),
					reserved.format()
				),
			},
		}
	}
}
