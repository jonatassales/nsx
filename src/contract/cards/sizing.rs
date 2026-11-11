//! The sizing of the Plan card
//!
//! One function: it gives back the `sizing` section of the Plan card, to
//! fill, and makes the card and the section first when they are not there.
//!
//! Spec 0077.

use crate::contract::{cards::Cards, sizing::Sizing};

impl Cards {
	pub fn sizing(&mut self) -> &mut Sizing {
		self
			.plan
			.get_or_insert_default()
			.sizing
			.get_or_insert_default()
	}
}
