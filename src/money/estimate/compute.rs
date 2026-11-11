//! Computing the estimate
//!
//! One function: it takes a model's prices and the input and output tokens,
//! as whole tokens, and gives back their price in whole cents, or nothing
//! when the amount is too large to state.
//!
//! ## Rules
//!
//! - Spend is the input tokens times the input price plus the output tokens
//!   times the output price, divided by a million tokens, rounded half up.
//! - Half up: 47.5 cents is 48 cents.
//! - The same formula, with the same rounding, prices the observed tokens, so
//!   the estimate and the observed spend compare like for like.
//!
//! Spec 0080.

use crate::money::{cents::Cents, model::Model};

const MILLION: i128 = 1_000_000;

impl crate::money::estimate::Estimate {
	pub fn compute(model: &Model, input: u64, output: u64) -> Option<Cents> {
		// Wide enough that no count times a price overflows.
		let spent = i128::from(input) * i128::from(model.input_cents.0)
			+ i128::from(output) * i128::from(model.output_cents.0);
		let cents = (spent + MILLION / 2).div_euclid(MILLION);

		i64::try_from(cents).ok().map(Cents)
	}
}
