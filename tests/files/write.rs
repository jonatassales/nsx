//! A file is written whole or not at all

use nsx::files::write::write;

#[test]
fn a_write_creates_the_folders_it_needs() {
	let project = tempfile::tempdir().unwrap();
	let path = project.path().join(".langdrift/finops/prices.json");

	write(&path, b"{}").unwrap();

	assert_eq!(std::fs::read(&path).unwrap(), b"{}");
}

#[test]
fn a_write_replaces_the_file_and_leaves_no_draft() {
	let project = tempfile::tempdir().unwrap();
	let path = project.path().join("plan.json");

	write(&path, b"old").unwrap();
	write(&path, b"new").unwrap();

	let names: Vec<_> = std::fs::read_dir(project.path())
		.unwrap()
		.map(|entry| entry.unwrap().file_name())
		.collect();

	assert_eq!(std::fs::read(&path).unwrap(), b"new");
	assert_eq!(names, ["plan.json"]);
}

#[test]
fn a_write_onto_a_folder_is_refused_and_leaves_no_draft() {
	let project = tempfile::tempdir().unwrap();
	let path = project.path().join("taken");

	std::fs::create_dir_all(path.join("inside")).unwrap();

	let refusal = write(&path, b"new").unwrap_err();
	let names: Vec<_> = std::fs::read_dir(project.path())
		.unwrap()
		.map(|entry| entry.unwrap().file_name())
		.collect();

	assert!(
		refusal.to_string().starts_with("nsx could not write"),
		"was `{refusal}`"
	);
	assert_eq!(names, ["taken"]);
}
