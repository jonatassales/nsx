//! The guardrail's row of DOCUMENTS
//!
//! One function: it writes the guardrail's row into the Analysis card of the
//! card data: its short id, its file, every criterion's id in table order,
//! the criteria the plan step answers to, and its enforcement.
//!
//! ## Simulation
//!
//! - Card: Analysis · the guardrail document row, set when the `guardrail`
//!   line prints. The row lists every criterion; the line lists only the plan
//!   step's.
//!
//! ## Rules
//!
//! - The enforcement is always review: nsx enforces no criterion.
//! - Each criterion's statement is the judge's, and arrives with its line.

use crate::contract::{
	cards::Cards,
	guardrail::{ENFORCEMENT, Guardrail as Row},
	rule::Rule,
};
use crate::harness::guardrail::Guardrail;

impl Guardrail {
	pub fn card(&self, cards: &mut Cards) {
		let criteria = self
			.criteria
			.iter()
			.map(|criterion| Rule {
				id: criterion.id.clone(),
				statement: None,
			})
			.collect();
		let plan_criteria = self
			.criteria
			.iter()
			.filter(|criterion| criterion.plan)
			.map(|criterion| criterion.id.clone())
			.collect();

		cards.documents().guardrail = Some(Row {
			short_id: self.short.clone(),
			file: self.name.clone(),
			criteria,
			plan_criteria,
			enforcement: String::from(ENFORCEMENT),
		});
	}
}
