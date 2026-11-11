//! An index of episodes that each lean on the query `[1, 0]` by a cosine

use nsx::rag::{entry::Entry, index::Index};

pub fn leaning(cosines: &[(&str, f64)]) -> Index {
	let episodes = cosines
		.iter()
		.map(|(id, cosine)| Entry {
			id: String::from(*id),
			slug: String::from("notes"),
			vector: vec![*cosine as f32, (1.0 - cosine * cosine).sqrt() as f32],
		})
		.collect();

	Index {
		version: 1,
		embedder: String::from("test"),
		dimensions: 2,
		episodes,
	}
}
