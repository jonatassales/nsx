//! A project made of the files a test names, each by its path and its text

use tempfile::TempDir;

pub fn project(files: &[(&str, &str)]) -> TempDir {
	let project = tempfile::tempdir().unwrap();

	std::fs::create_dir(project.path().join(".langdrift")).unwrap();

	for (name, text) in files {
		let path = project.path().join(name);

		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
		std::fs::write(path, text).unwrap();
	}

	project
}
