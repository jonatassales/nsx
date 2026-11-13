//! The spec's read line
//!
//! One function: it gives back the line the spec prints once it is read: its
//! short id, then how many components it names and how many acceptance
//! criteria it lists.
//!
//! ## Simulation
//!
//! - `readSpec`: `console › tool call: read 0011-SPEC · 5 components named · 3 acceptance criteria`
//!
//! ## Open
//!
//! - The words for one component or one criterion. The golden shows only
//!   plurals, and the line keeps them for any count.

use crate::request::spec::Spec;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Spec {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("readSpec"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"tool call: read {} · {} components named · {} acceptance criteria",
					self.link.short,
					self.components.len(),
					self.criteria
				),
			},
		}
	}
}
