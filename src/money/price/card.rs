//! The price on the Plan card
//!
//! One function: it puts the runtime's price source into the plan's cards.
//! The runtime has none, so the source is `null`.
//!
//! ## Rules
//!
//! - An unknown price is `null`, never zero.
//! - This function writes no file: the one writer in `handoff/` does.
//!
//! Spec 0079.

use crate::contract::{cards::Cards, source::Source};
use crate::money::price::Price;

impl Price {
	pub fn card(&self, cards: &mut Cards) {
		cards.plan.get_or_insert_default().price = Some(Source {
			source: None,
		});
	}
}
