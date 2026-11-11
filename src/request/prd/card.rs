//! The PRD's row of DOCUMENTS
//!
//! One function: it writes the PRD's row into the Analysis card of the card
//! data: its short id, its file and its use case.
//!
//! ## Simulation
//!
//! - Card: Analysis · the PRD row of DOCUMENTS, set by the `readPrd` line
//!
//! Spec 0078.

use crate::contract::{cards::Cards, product::Product};
use crate::request::prd::Prd;

impl Prd {
	pub fn card(&self, cards: &mut Cards) {
		cards.documents().prd = Some(Product {
			short_id: self.link.short.clone(),
			file: self.link.name.clone(),
			use_case: self.use_case.clone(),
		});
	}
}
