//! The reports of a runtime the console has not named yet, kept in a project

use std::path::PathBuf;

use nsx::console::report::Report;
use nsx::contract::status::Status;

pub fn opened(project: &tempfile::TempDir) -> Report {
	Report {
		kept: PathBuf::from(project.path()).join("runtime.id"),
		runtime: None,
		status: Status::Workflow,
		unavailable: false,
	}
}
