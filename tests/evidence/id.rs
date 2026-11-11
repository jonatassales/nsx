//! The run's id: the runtime and the next number its ledger has not used

use std::path::PathBuf;

use nsx::evidence::id::Id;
use nsx::files::{layout::Layout, root::Root};
use rstest::rstest;

#[rstest]
#[case::the_first_run(&[], "notes-0001")]
#[case::the_run_after_the_highest(&["notes-0001", "notes-0007"], "notes-0008")]
#[case::past_four_digits(&["notes-9999"], "notes-10000")]
#[case::after_a_five_digit_run(&["notes-10000"], "notes-10001")]
#[case::another_runtime_that_starts_alike(&["notes-2-0007"], "notes-0001")]
#[case::a_name_with_too_few_digits(&["notes-7"], "notes-0001")]
fn a_run_takes_the_number_after_the_highest_of_its_runtime(
	#[case] kept: &[&str],
	#[case] next: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	std::fs::create_dir_all(&layout.ledger).unwrap();

	for run in kept {
		std::fs::write(layout.ledger.join(format!("{run}.ledger.json")), "{}")
			.unwrap();
	}

	let id = Id::mint(&root, &layout, "notes").unwrap();

	assert_eq!(id.text(), next);
}

#[test]
fn a_folder_named_like_a_ledger_entry_is_no_run() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	std::fs::create_dir_all(layout.ledger.join("notes-0042.ledger.json"))
		.unwrap();

	let id = Id::mint(&root, &layout, "notes").unwrap();

	assert_eq!(id.text(), "notes-0001");
}

#[test]
fn a_run_whose_ledger_entry_is_not_in_this_checkout_still_keeps_its_id() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	std::fs::create_dir_all(layout.runs.join("notes-0003").join("stages"))
		.unwrap();
	std::fs::create_dir_all(&layout.ledger).unwrap();
	std::fs::write(layout.ledger.join("notes-0001.ledger.json"), "{}").unwrap();

	let id = Id::mint(&root, &layout, "notes").unwrap();

	assert_eq!(id.text(), "notes-0004");
}
