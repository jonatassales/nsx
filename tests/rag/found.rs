//! A retrieval whose candidates have the similarities a test gives

use nsx::rag::{
	candidate::Candidate, retrieval::Retrieval, threshold::THRESHOLD,
};

pub fn found(similarities: &[f64], cited_measured: u64) -> Retrieval {
	let candidates = similarities
		.iter()
		.map(|similarity| Candidate {
			id: String::from("ep-0001"),
			slug: String::from("notes"),
			similarity: *similarity,
			kept: *similarity >= THRESHOLD.0,
			reason: None,
		})
		.collect::<Vec<_>>();
	let total = candidates.len() as u64;
	let kept =
		candidates.iter().filter(|candidate| candidate.kept).count() as u64;

	Retrieval {
		threshold: THRESHOLD,
		candidates,
		records: Vec::new(),
		kept,
		dropped: total - kept,
		total,
		cited: kept,
		cited_measured,
	}
}
