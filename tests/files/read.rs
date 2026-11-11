//! A file is read up to a limit

use nsx::files::{error::Error, read::read};

#[test]
fn a_file_within_the_limit_is_read_whole() {
	let project = tempfile::tempdir().unwrap();
	let path = project.path().join("ticket.md");

	std::fs::write(&path, "# 0031").unwrap();

	assert_eq!(read(&path, 6).unwrap(), b"# 0031");
}

#[test]
fn a_file_over_the_limit_is_refused() {
	let project = tempfile::tempdir().unwrap();
	let path = project.path().join("ticket.md");

	std::fs::write(&path, "# 0031").unwrap();

	let refusal = read(&path, 5).unwrap_err();

	assert!(matches!(refusal, Error::TooLarge { .. }), "was {refusal:?}");
}

#[test]
fn a_missing_file_is_refused() {
	let project = tempfile::tempdir().unwrap();

	let refusal = read(&project.path().join("ticket.md"), 64).unwrap_err();

	assert!(
		matches!(refusal, Error::Unreadable { .. }),
		"was {refusal:?}"
	);
}

#[test]
fn a_folder_is_refused() {
	let project = tempfile::tempdir().unwrap();

	let refusal = read(project.path(), 1024).unwrap_err();

	assert!(matches!(refusal, Error::NotFile { .. }), "was {refusal:?}");
}

#[cfg(unix)]
#[test]
fn a_pipe_is_refused_before_a_byte_is_asked_of_it() {
	let project = tempfile::tempdir().unwrap();
	let pipe = project.path().join("notes.prd.md");
	let made = std::process::Command::new("mkfifo")
		.arg(&pipe)
		.status()
		.unwrap();

	assert!(made.success());

	let refusal = read(&pipe, 1024).unwrap_err();

	assert!(matches!(refusal, Error::NotFile { .. }), "was {refusal:?}");
}
