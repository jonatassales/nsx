//! Finding the DOCUMENTS of the Analysis card
//!
//! One function: it gives back the `documents` section of the card data,
//! opening the Analysis card and the section when a line fills them for the
//! first time.

use crate::contract::{cards::Cards, documents::Documents};

impl Cards {
	pub fn documents(&mut self) -> &mut Documents {
		self
			.analysis
			.get_or_insert_default()
			.documents
			.get_or_insert_default()
	}
}
