//! The real nsx binary, started in a project

use std::path::Path;

use assert_cmd::Command;

pub fn nsx(project: &Path) -> Command {
	let mut command = Command::cargo_bin("nsx").unwrap();

	command.current_dir(project);

	command
}
