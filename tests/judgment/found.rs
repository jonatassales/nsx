//! The simulation's retrieval: six candidates, three kept

use nsx::rag::{
	candidate::Candidate, retrieval::Retrieval, threshold::THRESHOLD,
};

pub fn found() -> Retrieval {
	let candidates = [
		("ep-0412", 0.81),
		("ep-0388", 0.74),
		("ep-0351", 0.66),
		("ep-0297", 0.52),
		("ep-0240", 0.47),
		("ep-0188", 0.42),
	]
	.map(|(id, similarity)| Candidate {
		id: String::from(id),
		slug: String::from("notes"),
		similarity,
		kept: similarity >= THRESHOLD.0,
		reason: None,
	});

	Retrieval {
		threshold: THRESHOLD,
		candidates: Vec::from(candidates),
		records: Vec::new(),
		kept: 3,
		dropped: 3,
		total: 6,
		cited: 3,
		cited_measured: 0,
	}
}
