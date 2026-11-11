//! The spend table on the Workflow card
//!
//! One function: it puts the spend table into the `workflow` section of the
//! plan's cards: the budget with its source, the margin and the nine rows.
//!
//! ## Simulation
//!
//! - Card: Workflow · the header, `{short name} · {effort} · {estimate} of {budget}`
//! - Card: Workflow · ESTIMATED SPEND, and its over-budget form
//!
//! ## Rules
//!
//! - The section is there before the plan's first line.
//! - The budget's source is the design doc's path from the project's root.
//! - This function writes no file: the one writer in `handoff/` does.
//!
//! Spec 0079.

use crate::contract::{cards::Cards, pair::Pair as Row, workflow::Workflow};
use crate::money::spend::Spend;

impl Spend {
	pub fn card(&self, cards: &mut Cards) {
		let spend = self
			.pairs
			.iter()
			.map(|pair| Row {
				model: pair.model.clone(),
				effort: pair.effort.clone(),
				input_thousands: i64::from(pair.tokens.input),
				output_thousands: i64::from(pair.tokens.output),
				estimate_cents: pair.estimate.0,
				reserved_cents: pair.reserved.0,
				over_budget: pair.over,
			})
			.collect();

		cards.workflow = Some(Workflow {
			budget_cents: self.budget.amount.0,
			budget_source: self.budget.source.to_string_lossy().into_owned(),
			margin_percent: self.margin,
			spend,
		});
	}
}
