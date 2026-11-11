//! git and gh as a test reaches them: over the runner that starts nothing

use std::path::PathBuf;

use nsx::delivery::github::Github;

use crate::simulation::fake::programs::Programs;

pub fn reached(programs: &Programs) -> Github<'_> {
	Github {
		programs,
		directory: PathBuf::from("/project"),
	}
}
