//! The PRD's read line
//!
//! One function: it gives back the line the PRD prints once it is read: its
//! short id, then its use case.
//!
//! ## Simulation
//!
//! - `readPrd`: `console › tool call: read 0002-PRD · use case: book-a-demo request`
//!
//! Spec 0078.

use crate::request::prd::Prd;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Prd {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("readPrd"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"tool call: read {} · use case: {}",
					self.link.short, self.use_case
				),
			},
		}
	}
}
