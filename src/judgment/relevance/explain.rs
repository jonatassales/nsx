//! Giving each candidate its reason
//!
//! One function: it takes the retrieval and writes, on each of its
//! candidates, the reason the judge gave for it.
//!
//! ## Rules
//!
//! - The verdict stays the threshold's. Only the reason is the judge's.

use crate::judgment::relevance::Relevance;
use crate::rag::retrieval::Retrieval;

impl Relevance {
	pub fn explain(&self, retrieval: &mut Retrieval) {
		for candidate in &mut retrieval.candidates {
			candidate.reason = self
				.judged
				.iter()
				.find(|judged| judged.id == candidate.id)
				.map(|judged| judged.reason.clone());
		}
	}
}
