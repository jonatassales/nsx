//! A project of its own, planned by the real nsx
//!
//! A copy of the fixture project whose ticket carries the generated number
//! and slug, connected to the console the environment names, with what
//! `nsx agent plan` printed and how it ended.

use std::path::Path;
use std::process::Output;

use assert_cmd::Command;
use tempfile::TempDir;

use crate::e2e::generated::Generated;
use crate::simulation::project::project;

const FIXTURE: &str = ".agents/0031-TICKET-book-a-demo-runtime.ticket.md";

pub struct Played {
	pub project: TempDir,
	pub plan: Output,
}

fn nsx(project: &Path) -> Command {
	let mut command = Command::cargo_bin("nsx").unwrap();

	command.current_dir(project);

	command
}

pub fn played(ticket: &Generated, console: &str, secret: &str) -> Played {
	let project = project();
	let renamed = format!(".agents/{}.ticket.md", ticket.id());

	std::fs::rename(project.path().join(FIXTURE), project.path().join(renamed))
		.unwrap();

	nsx(project.path())
		.arg("init")
		.env("MCP_CONSOLE_URL", console)
		.env("LANGDRIFT_SECRET", secret)
		.assert()
		.success();

	let plan = nsx(project.path())
		.args(["agent", "plan", &ticket.id()])
		.output()
		.unwrap();

	Played {
		project,
		plan,
	}
}
