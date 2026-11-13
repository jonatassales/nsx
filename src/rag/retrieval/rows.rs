//! The rows of EPISODES
//!
//! One function: it writes the candidates into the EPISODES of the card data,
//! in descending similarity, each with its id, its slug and its similarity,
//! and how many there are.
//!
//! ## Simulation
//!
//! - Card: Analysis · EPISODES rows: from the `candidates` line each row
//!   carries its id, its slug and its similarity

use crate::contract::candidate::Candidate as Row;
use crate::contract::cards::Cards;
use crate::rag::retrieval::Retrieval;

impl Retrieval {
	pub fn rows(&self, cards: &mut Cards) {
		let found = cards.episodes();

		found.total = Some(self.total);
		found.candidates = Some(
			self
				.candidates
				.iter()
				.map(|candidate| Row {
					id: candidate.id.clone(),
					slug: candidate.slug.clone(),
					similarity: candidate.similarity,
					verdict: None,
					reason: None,
				})
				.collect(),
		);
	}
}
