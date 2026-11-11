//! The citations of EPISODES
//!
//! One function: it writes into the card data how many episodes the plan
//! cites and how many of those measured resources.
//!
//! ## Simulation
//!
//! - Card: Analysis · MEASURED RESOURCES: `none cited`, from the `episodes`
//!   line
//!
//! Spec 0078.

use crate::contract::cards::Cards;
use crate::rag::retrieval::Retrieval;

impl Retrieval {
	pub fn citations(&self, cards: &mut Cards) {
		let found = cards.episodes();

		found.cited = Some(self.cited);
		found.cited_measured = Some(self.cited_measured);
	}
}
