//! The Safe Seat line
//!
//! One function: it takes the verdict and gives back the line the run prints
//! when Safe Seat is met, and none when it is not: the simulation shows no
//! such line.
//!
//! ## Simulation
//!
//! - `safeSeatMet`: `✓ Safe Seat  met`
//!
//! ## Rules
//!
//! - It follows `✓ checks     4 of 4 passed` and comes before the runtime's
//!   stopped line.
//! - `decidedAt` is written when the line prints.

use crate::seat::verdict::Verdict;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Verdict {
	pub fn line(&self) -> Option<Line> {
		if !self.met {
			return None;
		}

		Some(Line {
			id: String::from("safeSeatMet"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("Safe Seat"),
				value: String::from("met"),
			},
		})
	}
}
