//! An empty project, with its root and the paths under `.langdrift/`

use std::path::PathBuf;

use nsx::files::{layout::Layout, root::Root};

pub fn rooted() -> (tempfile::TempDir, Root, Layout) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	std::fs::create_dir_all(&layout.langdrift).unwrap();

	(project, root, layout)
}
