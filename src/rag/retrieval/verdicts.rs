//! The verdicts of EPISODES
//!
//! One function: it writes each candidate's verdict and reason into its row
//! of the card data, and how many were kept and dropped.
//!
//! ## Simulation
//!
//! - Card: Analysis · EPISODES rows: from the `drop` line each row also
//!   carries its verdict, as in `ep-0351 signup-redesign-preview 0.66 kept`
//!   and `ep-0240 batch-export-worker 0.47 dropped · other runtime`
//!
//! ## Rules
//!
//! - The verdict is the threshold's; the reason is the judge's.
//!
//! Spec 0078.

use crate::contract::cards::Cards;
use crate::contract::episode::VERDICTS;
use crate::rag::retrieval::Retrieval;

impl Retrieval {
	pub fn verdicts(&self, cards: &mut Cards) {
		let [kept, dropped] = VERDICTS;
		let found = cards.episodes();
		let rows = found.candidates.iter_mut().flatten();

		for (row, candidate) in rows.zip(&self.candidates) {
			let verdict = if candidate.kept { kept } else { dropped };

			row.verdict = Some(String::from(verdict));
			row.reason = candidate.reason.clone();
		}

		found.kept = Some(self.kept);
		found.dropped = Some(self.dropped);
	}
}
