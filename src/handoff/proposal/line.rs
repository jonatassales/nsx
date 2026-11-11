//! The plan's last result line
//!
//! One function: it gives back the line the plan prints once the draft is
//! written and the intent recorded, with the draft's revision.
//!
//! ## Simulation
//!
//! - `draft`: `✓ plan       runtime.md draft, revision 1 · proposed to the console`
//!
//! ## Open
//!
//! - Whether the words "proposed to the console" stay. The owner keeps them
//!   or changes them in the simulation first.
//!
//! Spec 0079.

use crate::handoff::proposal::Proposal;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Proposal {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("draft"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("plan"),
				value: format!(
					"runtime.md draft, revision {} · proposed to the console",
					self.revision
				),
			},
		}
	}
}
