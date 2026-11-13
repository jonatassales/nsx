//! The placeholder rows of the Plan card
//!
//! One function: it takes the components the spec names and puts one row per
//! path into the COMPONENTS of the plan's cards, each with its path alone.
//!
//! ## Simulation
//!
//! - Card: Plan · COMPONENTS, five placeholder rows from the `resolve` line
//!
//! ## Rules
//!
//! - The rows are in the spec's order. Each is filled when its own row line
//!   prints.
//! - This function writes no file: the one writer in `handoff/` does.

use crate::components::inventory::Inventory;
use crate::contract::{cards::Cards, resolved::Resolved};
use crate::request::named::Named;

impl Inventory {
	pub fn card(named: &[Named], cards: &mut Cards) {
		let rows = named
			.iter()
			.map(|component| Resolved {
				path: component.path.clone(),
				..Resolved::default()
			})
			.collect();

		cards.components().rows = Some(rows);
	}
}
