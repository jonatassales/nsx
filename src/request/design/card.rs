//! The design doc's row of DOCUMENTS
//!
//! One function: it writes the design doc's row into the Analysis card of the
//! card data: its short id, its file, the envelope and the budget per run.
//!
//! ## Simulation
//!
//! - Card: Analysis · the design doc row of DOCUMENTS, set by the
//!   `readDesignDoc` line

use crate::contract::{
	cards::Cards, design::Design as Row, envelope::Envelope,
};
use crate::request::design::Design;

impl Design {
	pub fn card(&self, cards: &mut Cards) {
		cards.documents().design_doc = Some(Row {
			short_id: self.link.short.clone(),
			file: self.link.name.clone(),
			envelope: Envelope {
				cpu_millis: self.cpu_millis,
				memory_mib: self.memory_mib,
			},
			budget_per_run_cents: self.budget.0,
		});
	}
}
