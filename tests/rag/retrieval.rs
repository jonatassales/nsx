//! The retrieval counts what the threshold kept and words its six lines

use std::path::PathBuf;

use nsx::files::{layout::Layout, root::Root};
use nsx::rag::{error::Error, retrieval::Retrieval};

use crate::rag::found::found;

#[test]
fn the_six_lines_read_as_the_simulation_prints_them() {
	let retrieval = found(&[0.81, 0.74, 0.66, 0.52, 0.47, 0.42], 0);
	let lines = [
		Retrieval::memory(),
		Retrieval::embedding(),
		Retrieval::search(),
		retrieval.candidates(),
		retrieval.dropped(),
		retrieval.cited().unwrap(),
	]
	.map(|line| (line.id.clone(), line.render()));

	assert_eq!(
		lines,
		[
			(
				"memory",
				"console › RAG over project episodes · a model and effort for this ticket"
			),
			(
				"embed",
				"console › embedding the ticket intent · 1 query vector"
			),
			(
				"vectorSearch",
				"console › vector search · project episodes · top 6 by cosine similarity"
			),
			(
				"candidates",
				"console › 6 candidates · similarity 0.81 to 0.42"
			),
			("drop", "console › dropped 3 below 0.55 · kept 3"),
			(
				"episodes",
				"console › 3 episodes cited · none with measured resources"
			),
		]
		.map(|(id, text)| (String::from(id), String::from(text)))
	);
}

#[test]
fn a_cited_episode_that_measured_resources_has_no_words_yet() {
	let retrieval = found(&[0.81, 0.74], 1);

	let refusal = retrieval.cited().unwrap_err();

	assert!(
		matches!(
			refusal,
			Error::Measured {
				cited: 1
			}
		),
		"was {refusal:?}"
	);
}

#[test]
fn a_project_with_no_episode_retrieves_none() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	let retrieval = Retrieval::retrieve(&root, &layout, "Book a demo").unwrap();

	assert_eq!(
		(retrieval.total, retrieval.kept, retrieval.cited),
		(0, 0, 0)
	);
	assert_eq!(retrieval.candidates().render(), "console › 0 candidates");
	assert_eq!(
		retrieval.dropped().render(),
		"console › dropped 0 below 0.55 · kept 0"
	);
}
