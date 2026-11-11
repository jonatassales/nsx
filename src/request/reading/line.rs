//! The line that closes the reads
//!
//! One function: it gives back the success line that counts the documents read
//! and says that nothing outside the ticket's links was read.
//!
//! ## Simulation
//!
//! - `reads`: `✓ read       4 documents · nothing outside the ticket's links`
//! - Card: Analysis · DOCUMENTS header, `4 documents read`, once this line
//!   prints
//!
//! Spec 0078.

use crate::request::reading::Reading;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Reading {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("reads"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("read"),
				value: format!(
					"{} documents · nothing outside the ticket's links",
					self.read
				),
			},
		}
	}
}
