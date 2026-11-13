//! The validation on the Plan card
//!
//! One function: it takes the count the draft's validation gave and puts it
//! into the sizing of the plan's cards, with the schema it was counted
//! against.
//!
//! ## Simulation
//!
//! - Card: Plan · VALIDATION, `schema · 0 errors`
//!
//! ## Rules
//!
//! - The schema is named by its `$id`.
//! - This function writes no file: the one writer in `handoff/` does.

use crate::contract::{cards::Cards, fence::SCHEMA, validation::Validation};
use crate::runtime::draft::Draft;

impl Draft {
	pub fn card(errors: usize, cards: &mut Cards) {
		cards.sizing().validation = Some(Validation {
			schema: String::from(SCHEMA),
			errors: errors as u64,
		});
	}
}
