//! A path a document names stays inside the project

use std::path::Path;

use rstest::rstest;

use nsx::files::{error::Error, within::within};

use crate::files::project::project;

#[rstest]
#[case::a_file(".agents/0031-TICKET.ticket.md")]
#[case::a_file_named_from_here("./.agents/0031-TICKET.ticket.md")]
#[case::a_file_that_does_not_exist_yet("services/new-api/Cargo.toml")]
fn a_path_inside_the_project_is_accepted(#[case] named: &str) {
	let (_folder, root) = project();

	let inside = within(&root, Path::new(named)).unwrap();

	assert!(inside.starts_with(&root.path), "was {inside:?}");
	assert!(inside.ends_with(named.trim_start_matches("./")));
}

#[rstest]
#[case::climbing("../outside.md")]
#[case::climbing_from_inside(".agents/../../outside.md")]
#[case::absolute("/etc/hosts")]
fn a_path_that_leaves_the_project_is_refused(#[case] named: &str) {
	let (_folder, root) = project();

	let refusal = within(&root, Path::new(named)).unwrap_err();

	assert!(matches!(refusal, Error::Outside { .. }), "was {refusal:?}");
}

#[test]
fn a_symbolic_link_is_refused_even_when_it_points_inside() {
	let (_folder, root) = project();
	let target = root.path.join("real.md");

	std::fs::write(&target, "real").unwrap();
	std::os::unix::fs::symlink(&target, root.path.join("alias.md")).unwrap();

	let refusal = within(&root, Path::new("alias.md")).unwrap_err();

	assert!(matches!(refusal, Error::Linked { .. }), "was {refusal:?}");
}

#[test]
fn a_folder_that_is_a_symbolic_link_out_is_refused() {
	let (_folder, root) = project();
	let outside = tempfile::tempdir().unwrap();

	std::os::unix::fs::symlink(outside.path(), root.path.join("docs")).unwrap();

	let refusal = within(&root, Path::new("docs/secret.md")).unwrap_err();

	assert!(matches!(refusal, Error::Linked { .. }), "was {refusal:?}");
}
