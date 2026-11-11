//! Building the spend table
//!
//! One function: it takes the price list, the forecast and the budget, and
//! gives back the spend table: every model and effort pair with its expected
//! tokens, its estimate, its reservation and whether it is over the budget.
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
//! - The rows follow the three models nsx reads, then low, medium and high.
//!   A price list without one of the models, or a forecast without one of
//!   the efforts, stops the command, and so does an amount too large to
//!   state in cents.
//!
//! Spec 0079.

use crate::contract::fence::EFFORTS;
use crate::money::{
	budget::Budget,
	error::Error,
	estimate::Estimate,
	forecast::Forecast,
	pair::Pair,
	price::{MODELS, Price},
	reservation::Reservation,
	spend::{MARGIN, Spend},
};

impl Spend {
	pub fn build(
		prices: &Price,
		forecast: &Forecast,
		budget: &Budget,
	) -> Result<Self, Error> {
		let mut pairs = Vec::new();

		for id in MODELS {
			let listed = prices.models.iter().find(|model| model.id == id);
			let Some(model) = listed else {
				return Err(Error::Model {
					id: String::from(id),
				});
			};

			for effort in EFFORTS {
				let Some(tokens) = forecast.by_effort.get(effort) else {
					return Err(Error::Effort {
						effort: String::from(effort),
					});
				};
				let unpriced = || Error::Unpriced {
					model: String::from(id),
					effort: String::from(effort),
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
					model: String::from(id),
					effort: String::from(effort),
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
