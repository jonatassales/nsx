//! The criteria's statements on the card
//!
//! One function: it writes the judge's statement of each criterion into the
//! guardrail's row of the card data.
//!
//! ## Simulation
//!
//! - Card: Analysis · the FIN line under the guardrail row, set when the
//!   `judgeGuardrail` line prints
//!
//! ## Rules
//!
//! - The row and its criteria are the guardrail's, written with its own line.
//!   The statements go to them in the guardrail's order.

use crate::contract::cards::Cards;
use crate::judgment::criteria::Criteria;

impl Criteria {
	pub fn card(&self, cards: &mut Cards) {
		let rules = cards
			.documents()
			.guardrail
			.iter_mut()
			.flat_map(|row| &mut row.criteria);

		for (rule, criterion) in rules.zip(&self.stated) {
			rule.statement = Some(criterion.statement.clone());
		}
	}
}
