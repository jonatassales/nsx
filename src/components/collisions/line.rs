//! The cross-check's line
//!
//! One function: it gives back the line the plan prints when the cross-check
//! found no collision: the ports that are distinct and the images resolved.
//!
//! ## Simulation
//!
//! - `crossCheck`: `console › cross-checking · 5 ports distinct · 5 images resolved · no collisions`
//! - Card: Plan · the ports line,
//!   `ports 8080 · 8081 · 4222 · 6379 · 3000 · distinct · no collisions`
//!
//! ## Rules
//!
//! - The line prints only when nothing collides. A collision is a stop, and
//!   the simulation has no line for it.

use crate::components::collisions::Collisions;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Collisions {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("crossCheck"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"cross-checking · {} ports distinct · {} images resolved · no collisions",
					self.ports, self.images
				),
			},
		}
	}
}
