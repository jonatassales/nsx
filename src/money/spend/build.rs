//! Building the spend table
//!
//! One function: it takes the candidates, the forecast and the budget, and
//! gives back the spend table: every candidate model at each of its efforts,
//! with its expected tokens, its estimate, its reservation and whether it is
//! over the budget.
//!
//! ## Rules
//!
//! - The table is built before the plan's first line prints. The design doc's
//!   budget is read that early.
//! - The table carries the budget with its source, the design doc's path, and
//!   the margin, 20.
//! - The table goes into the `workflow` section of `plan.json` through the one
//!   writer in `handoff/`. This function writes no file.
//! - The estimate is `Estimate::compute`'s, on the forecast's thousands of
//!   tokens, and the reservation `Reservation::total`'s: each formula is
//!   written once.
//! - A pair is over the budget when its reservation exceeds the budget. A
//!   reservation equal to the budget is not over it.
//! - The rows follow the candidates: the models in the order of the three
//!   nsx reads, each at its efforts from low to high. A forecast without one
//!   of those efforts stops the command, and so does an amount too large to
//!   state in cents.

use crate::money::{
	budget::Budget,
	candidates::Candidates,
	error::Error,
	estimate::Estimate,
	forecast::Forecast,
	pair::Pair,
	reservation::Reservation,
	spend::{MARGIN, Spend},
};

impl Spend {
	pub fn build(
		candidates: &Candidates,
		forecast: &Forecast,
		budget: &Budget,
	) -> Result<Self, Error> {
		let mut pairs = Vec::new();

		for candidate in &candidates.models {
			let model = &candidate.model;

			for effort in &candidate.efforts {
				let Some(tokens) = forecast.by_effort.get(effort.as_str()) else {
					return Err(Error::Effort {
						effort: effort.clone(),
					});
				};
				let unpriced = || Error::Unpriced {
					model: model.id.clone(),
					effort: effort.clone(),
				};
				let estimate = Estimate::compute(
					model,
					u64::from(tokens.input) * 1000,
					u64::from(tokens.output) * 1000,
				)
				.ok_or_else(unpriced)?;
				let reserved =
					Reservation::total(estimate).ok_or_else(unpriced)?.amount;

				pairs.push(Pair {
					model: model.id.clone(),
					effort: effort.clone(),
					tokens: *tokens,
					estimate,
					reserved,
					over: reserved > budget.amount,
				});
			}
		}

		Ok(Self {
			budget: budget.clone(),
			margin: MARGIN,
			pairs,
		})
	}
}
