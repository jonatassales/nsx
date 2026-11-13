//! The design doc's read line
//!
//! One function: it gives back the line the design doc prints once it is
//! read: its short id, then the envelope it sets.
//!
//! ## Simulation
//!
//! - `readDesignDoc`: `console › tool call: read 0005-DD · envelope cpuMillis 2000 · memoryMiB 3072 · set by the team`
//!
//! ## Rules
//!
//! - "set by the team" is the design doc's provenance: the design doc is the
//!   team's document, so whatever it sets, the team set.
//! - The line shows the envelope and not the budget. The card's row shows
//!   both.

use crate::request::design::Design;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Design {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("readDesignDoc"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"tool call: read {} · envelope cpuMillis {} · memoryMiB {} · set by the team",
					self.link.short, self.cpu_millis, self.memory_mib
				),
			},
		}
	}
}
