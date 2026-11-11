//! Settling a run
//!
//! One function: it takes the reservation, the applied model's prices and the
//! tokens the run observed, and gives back the settlement: the observed spend
//! and the released amount, or nothing when an amount is too large to
//! state.
//!
//! ## Rules
//!
//! - The observed spend uses the estimate's formula at the same list price,
//!   rounded half up.
//! - The release is the reservation minus the observed spend. It is not
//!   clamped at zero.
//! - With the simulation's fixture, Sonnet at medium effort observes 3,700K
//!   input and 240K output tokens: $9.80 observed, $1.60 released.
//!
//! ## Open
//!
//! - What RELEASED and the `ready` line say when the observed spend is above
//!   the reservation: a negative release or something else. No gate is added
//!   during the run.
//!
//! Spec 0080.

use crate::money::{
	cents::Cents, estimate::Estimate, model::Model, settlement::Settlement,
};

impl Settlement {
	pub fn settle(
		reserved: Cents,
		model: &Model,
		input_tokens: u64,
		output_tokens: u64,
	) -> Option<Self> {
		let observed = Estimate::compute(model, input_tokens, output_tokens)?;
		let released = reserved.0.checked_sub(observed.0)?;

		Some(Self {
			input_tokens,
			output_tokens,
			observed,
			released: Cents(released),
		})
	}
}
