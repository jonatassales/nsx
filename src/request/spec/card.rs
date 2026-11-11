//! The spec's row of DOCUMENTS
//!
//! One function: it writes the spec's row into the Analysis card of the card
//! data: its short id, its file, and its two counts.
//!
//! ## Simulation
//!
//! - Card: Analysis · the spec row of DOCUMENTS, set by the `readSpec` line
//!
//! Spec 0078.

use crate::contract::{cards::Cards, specified::Specified};
use crate::request::spec::Spec;

impl Spec {
	pub fn card(&self, cards: &mut Cards) {
		cards.documents().spec = Some(Specified {
			short_id: self.link.short.clone(),
			file: self.link.name.clone(),
			components: self.components.len() as u64,
			acceptance_criteria: self.criteria,
		});
	}
}
