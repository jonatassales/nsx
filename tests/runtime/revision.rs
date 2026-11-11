//! The revision: 1 for a first plan, the earlier draft's plus one after it

use std::path::PathBuf;

use nsx::files::{error::Error, root::Root};
use nsx::runtime::revision::Revision;
use rstest::rstest;

const FILE: &str = ".langdrift/microruntimes/runtimes/notes/runtime.md";

#[test]
fn a_runtime_with_no_earlier_draft_is_at_revision_one() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let revision = Revision::next(&root, &project.path().join(FILE)).unwrap();

	assert_eq!(revision, Revision(1));
}

#[rstest]
#[case::the_revision_after_the_first(
	"# notes\n\n```langdrift\n{\"version\": 3, \"revision\": 1}\n```\n",
	2
)]
#[case::the_revision_after_a_later_one(
	"# notes\n\n```langdrift\n{\"revision\": 41}\n```\n\n## Sizing\n",
	42
)]
#[case::an_older_version_read_for_its_revision_alone(
	"```langdrift\n{\"version\": 2, \"revision\": 7, \"ceiling\": {}}\n```\n",
	8
)]
#[case::a_block_that_is_not_json("```langdrift\nrevision: 7\n```\n", 1)]
#[case::a_draft_with_no_block("# notes\n\nrevision 7\n", 1)]
#[case::a_block_never_closed("```langdrift\n{\"revision\": 7}", 1)]
#[case::a_revision_that_is_no_revision(
	"```langdrift\n{\"revision\": 0}\n```\n",
	1
)]
#[case::a_revision_written_as_text(
	"```langdrift\n{\"revision\": \"7\"}\n```\n",
	1
)]
#[case::a_revision_with_no_next(
	"```langdrift\n{\"revision\": 9223372036854775807}\n```\n",
	1
)]
fn the_next_revision_follows_the_earlier_draft(
	#[case] earlier: &str,
	#[case] next: i64,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);

	std::fs::create_dir_all(file.parent().unwrap()).unwrap();
	std::fs::write(&file, earlier).unwrap();

	let revision = Revision::next(&root, &file).unwrap();

	assert_eq!(revision, Revision(next));
}

#[test]
fn an_earlier_draft_that_is_a_folder_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);

	std::fs::create_dir_all(&file).unwrap();

	let refusal = Revision::next(&root, &file).unwrap_err();

	assert!(matches!(refusal, Error::NotFile { .. }), "was {refusal:?}");
}
