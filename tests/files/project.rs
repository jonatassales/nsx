//! An empty folder that is a project's root

use nsx::files::root::Root;

pub fn project() -> (tempfile::TempDir, Root) {
	let folder = tempfile::tempdir().unwrap();
	let root = Root {
		path: folder.path().to_path_buf(),
	};

	(folder, root)
}
