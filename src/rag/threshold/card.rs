//! The threshold on the card
//!
//! One function: it writes the threshold into the EPISODES of the card data.
//!
//! ## Simulation
//!
//! - Card: Analysis · EPISODES header: `threshold 0.55`, shown with the card
//!
//! ## Rules
//!
//! - It is written before the plan's first line: the header shows the
//!   threshold from the moment the card appears.
//!
//! Spec 0078.

use crate::contract::cards::Cards;
use crate::rag::threshold::Threshold;

impl Threshold {
	pub fn card(&self, cards: &mut Cards) {
		cards.episodes().threshold = Some(self.0);
	}
}
