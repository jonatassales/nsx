//! Finding the EPISODES of the Analysis card
//!
//! One function: it gives back the `episodes` section of the card data, opening
//! the Analysis card and the section when a line fills them for the first
//! time.
//!
//! Spec 0077.

use crate::contract::{cards::Cards, retrieval::Retrieval};

impl Cards {
	pub fn episodes(&mut self) -> &mut Retrieval {
		self
			.analysis
			.get_or_insert_default()
			.episodes
			.get_or_insert_default()
	}
}
