//! The resolving line
//!
//! One function: it takes the components the spec names and gives back the
//! first line of the plan act after its comment: how many they are, and what
//! the plan resolves for each.
//!
//! ## Simulation
//!
//! - `resolve`: `console › resolving 5 components named in the spec · paths, images, ports`
//! - Card: Plan · COMPONENTS, five placeholder rows from this line
//!
//! ## Rules
//!
//! - The line says "named in the spec": that is where the list comes from.
//!
//! Spec 0079.

use crate::components::inventory::Inventory;
use crate::request::named::Named;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Inventory {
	pub fn line(named: &[Named]) -> Line {
		Line {
			id: String::from("resolve"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"resolving {} components named in the spec · paths, images, ports",
					named.len()
				),
			},
		}
	}
}
