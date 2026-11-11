//! A project that holds a copy of the fixture's six episode records

use std::path::{Path, PathBuf};

use nsx::files::{layout::Layout, root::Root};

pub fn copied() -> (tempfile::TempDir, Root, Layout) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let records = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("tests/simulation/project/.langdrift/microruntimes/episodes");

	std::fs::create_dir_all(&layout.episodes).unwrap();

	for record in std::fs::read_dir(records).unwrap() {
		let record = record.unwrap();

		std::fs::copy(record.path(), layout.episodes.join(record.file_name()))
			.unwrap();
	}

	(project, root, layout)
}
