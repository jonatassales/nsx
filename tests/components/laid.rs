//! A project with one component folder holding the files a test names

use std::path::PathBuf;

use nsx::files::root::Root;
use tempfile::TempDir;

pub fn laid(path: &str, files: &[(&str, &str)]) -> (TempDir, Root) {
	let project = tempfile::tempdir().unwrap();
	let folder = project.path().join(path);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	std::fs::create_dir_all(&folder).unwrap();

	for (name, written) in files {
		std::fs::write(folder.join(name), written).unwrap();
	}

	(project, root)
}
