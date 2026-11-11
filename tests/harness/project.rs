//! A project whose guardrail holds the text a test gives

use tempfile::TempDir;

pub fn project(guardrail: &str) -> TempDir {
	let project = tempfile::tempdir().unwrap();
	let folder = project.path().join(".langdrift/harness/guardrails");

	std::fs::create_dir_all(&folder).unwrap();
	std::fs::write(folder.join("0001-finops.gr.md"), guardrail).unwrap();

	project
}
