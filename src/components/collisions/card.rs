//! The cross-check on the Plan card
//!
//! One function: it takes the cross-checked components and puts the
//! cross-check into the COMPONENTS of the plan's cards: each row's image, the
//! ports that are distinct, the images resolved and the collisions.
//!
//! ## Simulation
//!
//! - Card: Plan · the ports line,
//!   `ports 8080 · 8081 · 4222 · 6379 · 3000 · distinct · no collisions`
//!
//! ## Rules
//!
//! - The rows are in the spec's order, as the components are: each row takes
//!   the image of the component at its place.
//! - The card's ports are the rows' ports. This function adds the counts.
//! - This function writes no file: the one writer in `handoff/` does.

use crate::components::{collisions::Collisions, component::Component};
use crate::contract::cards::Cards;

impl Collisions {
	pub fn card(&self, components: &[Component], cards: &mut Cards) {
		let section = cards.components();
		let rows = section.rows.get_or_insert_default();

		for (row, component) in rows.iter_mut().zip(components) {
			row.image = component.image.as_ref().map(|image| image.0.clone());
		}

		section.ports_distinct = Some(self.ports as u64);
		section.images_resolved = Some(self.images as u64);
		section.collisions = Some(self.collisions as u64);
	}
}
