//! The COMPONENTS of the Plan card
//!
//! One function: it gives back the `components` section of the Plan card, to
//! fill, and makes the card and the section first when they are not there.

use crate::contract::{cards::Cards, components::Components};

impl Cards {
	pub fn components(&mut self) -> &mut Components {
		self
			.plan
			.get_or_insert_default()
			.components
			.get_or_insert_default()
	}
}
