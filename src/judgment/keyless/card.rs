//! The Analysis card of a plan without a key
//!
//! One function: it records on the card data that the judge is unavailable
//! because the team has no Anthropic key.
//!
//! ## Simulation
//!
//! - Card: Analysis · JUDGE `no key · Settings → AI Models`, and the header
//!   `judge unavailable · no Anthropic key`, which the receiver words

use crate::contract::{
	availability::{Availability, MODEL_KEY_MISSING},
	cards::Cards,
};
use crate::judgment::keyless::Keyless;

impl Keyless {
	pub fn card(&self, cards: &mut Cards) {
		cards.analysis.get_or_insert_default().judge = Some(Availability {
			unavailable: String::from(MODEL_KEY_MISSING),
		});
	}
}
