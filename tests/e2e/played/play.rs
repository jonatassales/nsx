//! Planning a generated ticket: the fixture renamed, connected and planned

use crate::e2e::{
	generated::Generated,
	played::{Played, nsx::nsx},
};
use crate::simulation::project::project;

// The only place the fixture names its ticket.
const FIXTURE: &str = ".agents/0031-TICKET-book-a-demo-runtime.ticket.md";

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
