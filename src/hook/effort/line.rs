//! The effort hook's line
//!
//! One function: it gives back the line a run prints once the effort record is
//! written and the check on it holds.
//!
//! ## Simulation
//!
//! - `hook`: `console › hook       effort recorded · implement unblocked`
//!
//! ## Rules
//!
//! - It prints once per run, always before the `implement` line.
//! - It follows the `toTickets` line or, when Implement is ticked, the
//!   `confirmed-implement` line.
//! - It reports the effort gate, not a script's exit status.
//! - It has no card field: on the card it belongs to Implement's span of
//!   lines.

use crate::hook::effort::Effort;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Effort {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("hook"),
			kind: Kind::Output,
			shape: Shape::Labeled {
				label: String::from("hook"),
				value: String::from("effort recorded · implement unblocked"),
			},
		}
	}
}
