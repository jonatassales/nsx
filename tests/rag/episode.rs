//! The episode a completed run leaves, and the index that gains it

use nsx::rag::{
	embedder::Embedder, episode::Episode, error::Error, index::Index,
	retrieval::Retrieval,
};
use rstest::rstest;

use crate::rag::{
	copied::copied,
	finished::{INTENT, finished},
};

#[test]
fn a_run_leaves_one_episode_under_the_next_free_number() {
	let (_project, root, layout) = copied();

	let written = finished().write(&root, &layout, &Embedder).unwrap();
	let file = layout.episodes.join("ep-0413-notes-runtime.json");
	let read = Episode::load(&root, &file).unwrap();

	assert_eq!(written.id, "ep-0413");
	assert_eq!(read, written);
	assert_eq!(read.ticket, "0032-TICKET-notes-runtime");
	assert_eq!(read.measured, None);
}

#[test]
fn the_first_episode_of_a_project_is_ep_0001() {
	let project = tempfile::tempdir().unwrap();
	let root = nsx::files::root::Root {
		path: std::path::PathBuf::from(project.path()),
	};
	let layout = nsx::files::layout::Layout::under(&root);

	let written = finished().write(&root, &layout, &Embedder).unwrap();

	assert_eq!(written.id, "ep-0001");
	assert!(layout.episodes.join("ep-0001-notes-runtime.json").is_file());
}

#[test]
fn the_index_gains_the_episode_without_a_rebuild() {
	let (_project, root, layout) = copied();
	let before = Index::rebuild(&root, &layout, &Embedder).unwrap();

	let written = finished().write(&root, &layout, &Embedder).unwrap();
	let after = Index::load(&root, &layout).unwrap().unwrap();
	let (added, kept) = after.episodes.split_last().unwrap();

	assert_eq!(kept, before.episodes.as_slice());
	assert_eq!(added.id, written.id);
	assert_eq!(added.slug, "notes-runtime");
	assert_eq!(added.vector, Embedder.embed(INTENT));
}

#[test]
fn a_rebuild_from_the_episodes_gives_the_same_index() {
	let (_project, root, layout) = copied();

	Index::rebuild(&root, &layout, &Embedder).unwrap();
	finished().write(&root, &layout, &Embedder).unwrap();

	let gained = Index::load(&root, &layout).unwrap().unwrap();
	let rebuilt = Index::rebuild(&root, &layout, &Embedder).unwrap();

	assert_eq!(rebuilt, gained);
}

#[test]
fn a_project_with_no_index_gets_one_that_holds_the_new_episode() {
	let (_project, root, layout) = copied();

	let written = finished().write(&root, &layout, &Embedder).unwrap();
	let index = Index::load(&root, &layout).unwrap().unwrap();

	assert_eq!(index.episodes.len(), 7);
	assert!(index.episodes.iter().any(|entry| entry.id == written.id));
}

#[test]
fn a_plan_after_the_run_cites_the_new_episode_when_it_is_similar_enough() {
	let (_project, root, layout) = copied();
	let written = finished().write(&root, &layout, &Embedder).unwrap();

	let retrieval = Retrieval::retrieve(&root, &layout, INTENT).unwrap();
	let first = &retrieval.candidates[0];

	assert_eq!(first.id, written.id);
	assert_eq!(first.similarity, 1.0);
	assert!(first.kept);
	assert!(
		retrieval
			.records
			.iter()
			.any(|record| record.id == written.id)
	);
}

#[test]
fn a_number_past_four_digits_keeps_them_all() {
	let (_project, root, layout) = copied();

	let old = layout.episodes.join("ep-0412-checkout-preview-sizing.json");
	let record = std::fs::read_to_string(&old).unwrap();

	std::fs::write(
		layout.episodes.join("ep-9999-checkout-preview-sizing.json"),
		record.replace("ep-0412", "ep-9999"),
	)
	.unwrap();
	std::fs::remove_file(old).unwrap();

	let first = finished().write(&root, &layout, &Embedder).unwrap();
	let second = finished().write(&root, &layout, &Embedder).unwrap();

	assert_eq!(first.id, "ep-10000");
	assert_eq!(second.id, "ep-10001");
	assert_eq!(
		Index::load(&root, &layout).unwrap().unwrap().episodes.len(),
		8
	);
}

#[rstest]
#[case::a_record("ep-0412-checkout-preview-sizing.json", Some("ep-0412"))]
#[case::past_four_digits("ep-10001-notes-runtime.json", Some("ep-10001"))]
#[case::too_few_digits("ep-12-notes.json", None)]
#[case::no_slug("ep-0412.json", None)]
#[case::another_file("notes.json", None)]
#[case::another_kind("ep-0412-notes.txt", None)]
fn a_records_name_says_the_id_it_holds(
	#[case] name: &str,
	#[case] id: Option<&str>,
) {
	assert_eq!(Episode::filed(name), id);
	assert_eq!(
		Episode::filed(name).and_then(Episode::number).is_some(),
		id.is_some()
	);
}

#[test]
fn a_record_whose_name_does_not_say_its_id_stops_the_rebuild() {
	let (_project, root, layout) = copied();

	std::fs::rename(
		layout.episodes.join("ep-0412-checkout-preview-sizing.json"),
		layout.episodes.join("ep-0500-checkout-preview-sizing.json"),
	)
	.unwrap();

	let refusal = Index::rebuild(&root, &layout, &Embedder).unwrap_err();

	assert!(matches!(refusal, Error::Misnamed { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("holds the episode ep-0412"),
		"said {refusal}"
	);
}

#[test]
fn a_slug_that_is_no_label_is_refused_and_nothing_is_written() {
	let (project, root, layout) = copied();
	let episode = Episode {
		slug: String::from("x/../../escaped"),
		..finished()
	};

	let refusal = episode.write(&root, &layout, &Embedder).unwrap_err();
	let records = std::fs::read_dir(&layout.episodes).unwrap().count();

	assert!(
		refusal.to_string().contains("lowercase label"),
		"said {refusal}"
	);
	assert_eq!(records, 6);
	assert!(!project.path().join("escaped.json").exists());
}

#[test]
fn an_index_out_of_step_with_the_records_is_built_again() {
	let (_project, root, layout) = copied();

	Index::rebuild(&root, &layout, &Embedder).unwrap();

	let record = finished().write(&root, &layout, &Embedder).unwrap();
	let mut index = Index::load(&root, &layout).unwrap().unwrap();

	// The index as it would stand had its write failed after the record's.
	index.episodes.pop();
	index.write(&root, &layout).unwrap();

	let stale = Index::load(&root, &layout).unwrap();
	let retrieval = Retrieval::retrieve(&root, &layout, INTENT).unwrap();

	assert!(stale.is_none());
	assert_eq!(retrieval.candidates[0].id, record.id);
}
