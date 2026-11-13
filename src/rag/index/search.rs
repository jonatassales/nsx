//! Searching the index
//!
//! One function: it takes the query vector and gives back the six episodes
//! with the highest cosine similarity to the query, each with its similarity
//! at two decimals and the threshold's verdict.
//!
//! ## Rules
//!
//! - The order is descending similarity, ties broken by ascending id.
//! - Cosine similarity is the only measure: no structured score takes part.

use crate::rag::{candidate::Candidate, index::Index, threshold::THRESHOLD};

const TOP: usize = 6;

impl Index {
	pub fn search(&self, query: &[f32]) -> Vec<Candidate> {
		// Between -1 and 1 for unit vectors. A vector of no length points
		// nowhere, so it is similar to nothing.
		fn cosine(left: &[f32], right: &[f32]) -> f64 {
			let wide = |cell: &f32| f64::from(*cell);
			let dot = left
				.iter()
				.zip(right)
				.map(|(ours, theirs)| wide(ours) * wide(theirs))
				.sum::<f64>();
			let length = |vector: &[f32]| {
				vector.iter().map(|cell| wide(cell).powi(2)).sum::<f64>()
			};
			let lengths = length(left).sqrt() * length(right).sqrt();
			let directionless = lengths == 0.0;

			if directionless {
				return 0.0;
			}

			dot / lengths
		}

		let mut ranked = self
			.episodes
			.iter()
			.map(|entry| (cosine(query, &entry.vector), entry))
			.collect::<Vec<_>>();

		ranked.sort_by(|(left, ours), (right, theirs)| {
			right.total_cmp(left).then_with(|| ours.id.cmp(&theirs.id))
		});

		ranked
			.into_iter()
			.take(TOP)
			.map(|(cosine, entry)| {
				// Half away from zero, to two decimals; adding zero keeps a
				// rounded -0.00 from printing with its sign.
				let similarity = (cosine * 100.0).round() / 100.0 + 0.0;

				Candidate {
					id: entry.id.clone(),
					slug: entry.slug.clone(),
					similarity,
					kept: similarity >= THRESHOLD.0,
					reason: None,
				}
			})
			.collect()
	}
}
