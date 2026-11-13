//! Opening the run's money record
//!
//! One function: it takes the run's id, the plan's Workflow card and the
//! applied model and effort, and gives back the run's money entry as the
//! plan priced it: the expected tokens, the estimate, the reservation, the
//! budget and the margin, with the gate undecided and nothing observed.
//!
//! ## Rules
//!
//! - The figures are the row of the plan's spend table for the applied
//!   pair: nothing is priced again.
//! - A plan with no Workflow card, or no row for the pair, stops the run
//!   before its first line.
//! - `plan.json` is a file anyone can edit, so its figures are checked: a
//!   negative amount, or a token count that is negative or too large, stops
//!   the run before its first line. No such figure is ever printed.

use crate::contract::workflow::Workflow;
use crate::money::{error::Error, ledger::Ledger};

impl Ledger {
	pub fn open(
		run: String,
		workflow: Option<&Workflow>,
		model: &str,
		effort: &str,
	) -> Result<Self, Error> {
		let unplanned = || Error::Unplanned {
			model: String::from(model),
			effort: String::from(effort),
		};
		let unusable = || Error::Planned {
			model: String::from(model),
			effort: String::from(effort),
		};
		let workflow = workflow.ok_or_else(unplanned)?;
		let pair = workflow.pair(model, effort).ok_or_else(unplanned)?;
		let thousands =
			|count: i64| u32::try_from(count).map_err(|_range| unusable());
		let amounts = [
			pair.estimate_cents,
			pair.reserved_cents,
			workflow.budget_cents,
			workflow.margin_percent,
		];

		if amounts.iter().any(|amount| *amount < 0) {
			return Err(unusable());
		}

		Ok(Self {
			run,
			model: pair.model.clone(),
			effort: pair.effort.clone(),
			input_thousands: thousands(pair.input_thousands)?,
			output_thousands: thousands(pair.output_thousands)?,
			estimate_cents: pair.estimate_cents,
			budget_cents: workflow.budget_cents,
			budget_source: workflow.budget_source.clone(),
			margin_percent: workflow.margin_percent,
			reserved_cents: pair.reserved_cents,
			gate: None,
			observed_input_tokens: None,
			observed_output_tokens: None,
			observed_cents: None,
			released_cents: None,
		})
	}
}
