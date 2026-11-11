//! The project's root is the nearest folder upward that holds `.langdrift/`

use nsx::files::{error::Error, layout::Layout, root::Root};

#[test]
fn the_root_is_found_from_a_folder_inside_the_project() {
	let project = tempfile::tempdir().unwrap();
	let nested = project.path().join("services").join("signup-api");

	std::fs::create_dir_all(project.path().join(".langdrift")).unwrap();
	std::fs::create_dir_all(&nested).unwrap();

	let root = Root::find(&nested).unwrap();

	assert_eq!(root.path, project.path());
}

#[test]
fn a_folder_outside_any_project_has_no_root() {
	let elsewhere = tempfile::tempdir().unwrap();

	let refusal = Root::find(elsewhere.path()).unwrap_err();

	assert!(matches!(refusal, Error::NoRoot { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().starts_with("nsx found no project:"),
		"was `{refusal}`"
	);
}

#[test]
fn the_layout_names_the_paths_under_langdrift() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: project.path().to_path_buf(),
	};

	let layout = Layout::under(&root);

	assert_eq!(
		layout.prices,
		project.path().join(".langdrift/finops/prices.json")
	);
	assert_eq!(
		layout.guardrails,
		project.path().join(".langdrift/harness/guardrails")
	);
	assert_eq!(
		layout.index,
		project.path().join(".langdrift/agent/state/rag/index.json")
	);
}
