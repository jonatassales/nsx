//! The fixture's episode index, built so the six cosines are the simulation's
//!
//! As spec 0052 has it: the index holds six vectors built from the fixture
//! ticket's own query vector, so that the real embedder and the real search
//! give 0.81, 0.74, 0.66, 0.52, 0.47 and 0.42. Each vector leans on the query
//! by its similarity and, for the rest, on a direction the query does not
//! have.

use std::path::Path;

use nsx::files::{layout::Layout, root::Root};
use nsx::rag::{
	embedder::{DIMENSIONS, Embedder, ID},
	entry::Entry,
	index::{Index, VERSION},
};
use nsx::request::ticket::Ticket;

const EPISODES: [(&str, &str, f64); 6] = [
	("ep-0412", "checkout-preview-sizing", 0.81),
	("ep-0388", "notify-api-rollout", 0.74),
	("ep-0351", "signup-redesign-preview", 0.66),
	("ep-0297", "docs-search-index", 0.52),
	("ep-0240", "batch-export-worker", 0.47),
	("ep-0188", "storefront-cache-warmup", 0.42),
];

pub fn indexed(project: &Path) {
	let root = Root {
		path: project.to_path_buf(),
	};
	let layout = Layout::under(&root);
	let id = "0031-TICKET-book-a-demo-runtime";
	let found = Ticket::resolve(&root, id).unwrap();
	let written = std::fs::read_to_string(project.join(&found)).unwrap();
	let ticket = Ticket::parse(id, &found, &written).unwrap();
	let query = Embedder
		.embed(&ticket.intent(&written))
		.into_iter()
		.map(f64::from)
		.collect::<Vec<_>>();

	// The direction the query does not have: one axis, less what the query
	// holds of it, at unit length.
	let axis = query.iter().position(|cell| *cell == 0.0).unwrap();
	let mut apart = query
		.iter()
		.map(|cell| -query[axis] * cell)
		.collect::<Vec<_>>();

	apart[axis] += 1.0;

	let length = apart.iter().map(|cell| cell * cell).sum::<f64>().sqrt();
	let episodes = EPISODES
		.iter()
		.map(|(id, slug, similarity)| {
			let rest = (1.0 - similarity * similarity).sqrt();
			let vector = query
				.iter()
				.zip(&apart)
				.map(|(along, across)| {
					(similarity * along + rest * across / length) as f32
				})
				.collect();

			Entry {
				id: String::from(*id),
				slug: String::from(*slug),
				vector,
			}
		})
		.collect();
	let index = Index {
		version: VERSION,
		embedder: String::from(ID),
		dimensions: DIMENSIONS,
		episodes,
	};

	std::fs::create_dir_all(layout.index.parent().unwrap()).unwrap();
	std::fs::write(&layout.index, serde_json::to_vec(&index).unwrap()).unwrap();
}
