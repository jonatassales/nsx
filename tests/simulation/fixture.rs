//! The simulation's project, read where it sits

use std::path::Path;

use nsx::files::root::Root;

pub fn fixture() -> Root {
	Root {
		path: Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("tests/simulation/project"),
	}
}
