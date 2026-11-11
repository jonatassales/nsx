//! The counts of DOCUMENTS
//!
//! One function: it writes the two counts into the Analysis card of the card
//! data: what was read, for the terminal, and the documents the ticket
//! links, for the card's header.
//!
//! ## Simulation
//!
//! - Card: Analysis · DOCUMENTS header, `4 documents read`, once the `reads`
//!   line prints
//!
//! Spec 0078.

use crate::contract::cards::Cards;
use crate::request::reading::Reading;

impl Reading {
	pub fn card(&self, cards: &mut Cards) {
		let documents = cards.documents();

		documents.read_count = Some(self.read);
		documents.documents_read = Some(self.linked);
	}
}
