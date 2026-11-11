//! The suggestion on the card
//!
//! One function: it writes the suggestion into the Analysis card of the card
//! data: the model, the effort, the reason against each effort and each model
//! not chosen, and that it is a recommendation.
//!
//! ## Simulation
//!
//! - Card: Analysis · SUGGESTED MODEL and SUGGESTED EFFORT: their values, from
//!   the `suggest` line
//! - Card: Analysis · `recommendation · the team authorizes`
//!
//! ## Rules
//!
//! - Nothing of the suggestion is written before the `suggest` line: the card
//!   shows placeholders until then.
//!
//! Spec 0078.

use crate::contract::{cards::Cards, recommendation::BY};
use crate::judgment::suggestion::Suggestion;

impl Suggestion {
	pub fn card(&self, cards: &mut Cards) {
		let advised = cards.suggestion();

		advised.model = Some(self.model.clone());
		advised.effort = Some(self.effort.clone());
		advised.effort_reason = Some(self.effort_reason.clone());
		advised.model_reason = Some(self.model_reason.clone());
		advised.by = Some(String::from(BY));
	}
}
