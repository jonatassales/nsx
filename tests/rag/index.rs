//! The index is rebuilt only when missing or built by another embedder

use std::path::PathBuf;

use nsx::files::{layout::Layout, root::Root};
use nsx::rag::{
	embedder::{DIMENSIONS, Embedder, ID},
	entry::Entry,
	error::Error,
	index::Index,
};

use crate::rag::copied::copied;

#[test]
fn a_project_with_no_index_has_none_to_search() {
	let (_project, root, layout) = copied();

	assert_eq!(Index::load(&root, &layout).unwrap(), None);
}

#[test]
fn a_rebuild_embeds_every_episode_and_writes_the_index() {
	let (_project, root, layout) = copied();

	let index = Index::rebuild(&root, &layout, &Embedder).unwrap();
	let ids = index
		.episodes
		.iter()
		.map(|entry| entry.id.as_str())
		.collect::<Vec<_>>();

	assert_eq!(
		ids,
		[
			"ep-0188", "ep-0240", "ep-0297", "ep-0351", "ep-0388", "ep-0412"
		]
	);
	assert_eq!(
		(index.embedder.as_str(), index.dimensions),
		(ID, DIMENSIONS)
	);
	assert!(
		index
			.episodes
			.iter()
			.all(|entry| entry.vector.len() == DIMENSIONS)
	);
	assert_eq!(Index::load(&root, &layout).unwrap(), Some(index));
}

#[test]
fn an_index_built_by_another_embedder_is_not_searched() {
	let (_project, root, layout) = copied();
	let mut index = Index::rebuild(&root, &layout, &Embedder).unwrap();

	index.embedder = String::from("another-embedder-2");

	std::fs::write(&layout.index, serde_json::to_vec(&index).unwrap()).unwrap();

	assert_eq!(Index::load(&root, &layout).unwrap(), None);
}

#[test]
fn a_project_with_no_episode_gets_an_empty_index() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	let index = Index::rebuild(&root, &layout, &Embedder).unwrap();

	assert!(index.episodes.is_empty());
	assert!(layout.index.is_file());
}

#[test]
fn an_index_that_does_not_read_is_a_stop() {
	let (_project, root, layout) = copied();

	std::fs::create_dir_all(layout.index.parent().unwrap()).unwrap();
	std::fs::write(&layout.index, "{\"version\": 1}").unwrap();

	assert!(Index::load(&root, &layout).is_err());
}

#[test]
fn a_vector_that_is_not_the_embedders_width_is_a_stop() {
	let (_project, root, layout) = copied();
	let mut index = Index::rebuild(&root, &layout, &Embedder).unwrap();

	index.episodes[2].vector.truncate(3);

	std::fs::write(&layout.index, serde_json::to_vec(&index).unwrap()).unwrap();

	let refusal = Index::load(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::Width { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains(&index.episodes[2].id),
		"said {refusal}"
	);
}

#[test]
fn an_index_of_many_episodes_is_read_whole() {
	let (_project, root, layout) = copied();
	let mut index = Index::rebuild(&root, &layout, &Embedder).unwrap();
	let entry = index.episodes[0].clone();

	std::fs::remove_dir_all(&layout.episodes).unwrap();
	std::fs::create_dir_all(&layout.episodes).unwrap();

	index.episodes = (1000..1400)
		.map(|number| Entry {
			id: format!("ep-{number}"),
			..entry.clone()
		})
		.collect();

	for entry in &index.episodes {
		let record = layout.episodes.join(format!("{}-many.json", entry.id));

		std::fs::write(record, "{}").unwrap();
	}

	let written = serde_json::to_vec(&index).unwrap();

	std::fs::write(&layout.index, &written).unwrap();

	let read = Index::load(&root, &layout).unwrap().unwrap();

	assert!(written.len() > 1 << 20, "{} bytes", written.len());
	assert_eq!(read.episodes.len(), 400);
}

#[test]
fn an_episode_listed_twice_puts_the_index_out_of_step() {
	let (_project, root, layout) = copied();
	let mut index = Index::rebuild(&root, &layout, &Embedder).unwrap();
	let again = index.episodes[0].clone();

	index.episodes.push(again);

	std::fs::write(&layout.index, serde_json::to_vec(&index).unwrap()).unwrap();

	assert_eq!(Index::load(&root, &layout).unwrap(), None);
}
