//! A copy of the fixture project, in a folder of its own

use std::path::Path;

use tempfile::TempDir;

pub fn project() -> TempDir {
	fn copy(from: &Path, to: &Path) {
		std::fs::create_dir_all(to).unwrap();

		for entry in std::fs::read_dir(from).unwrap() {
			let entry = entry.unwrap();
			let target = to.join(entry.file_name());

			if entry.file_type().unwrap().is_dir() {
				copy(&entry.path(), &target);

				continue;
			}

			std::fs::copy(entry.path(), target).unwrap();
		}
	}

	let fixture =
		Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/simulation/project");
	let project = tempfile::tempdir().unwrap();

	copy(&fixture, project.path());

	project
}
