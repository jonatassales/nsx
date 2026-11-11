//! Finding the suggestion of the Analysis card
//!
//! One function: it gives back the `suggestion` section of the card data,
//! opening the Analysis card and the section when a line fills them for the
//! first time.
//!
//! Spec 0077.

use crate::contract::{cards::Cards, recommendation::Recommendation};

impl Cards {
	pub fn suggestion(&mut self) -> &mut Recommendation {
		self
			.analysis
			.get_or_insert_default()
			.suggestion
			.get_or_insert_default()
	}
}
