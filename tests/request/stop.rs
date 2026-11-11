//! A plan that cannot read its ticket or a document stops with one sentence

use assert_cmd::Command;

use crate::request::project::project;

const HEADER: &str = "# Book a demo\n\n**Status:** ready-for-agent\n\n";

const FOUR: &str = "- [prd](./0002-PRD-notes.prd.md)\n- [dd](./0005-DD-notes.dd.md)\n- [spec](./0011-SPEC-notes.spec.md)\n- [gr](/.langdrift/harness/guardrails/0001-finops.gr.md)\n";

const DESIGN: &str = "# Notes\n\n**cpuMillis:** 2000\n\n**memoryMiB:** 3072\n\n**Budget per run:** $25.00\n";

#[test]
fn an_unknown_ticket_stops_the_plan_before_its_first_line() {
	let ticket = format!("{HEADER}{FOUR}");
	let project = project(&[(".agents/0007-TICKET-notes.ticket.md", &ticket)]);

	let output = Command::cargo_bin("nsx")
		.unwrap()
		.args(["agent", "plan", "0099-TICKET-nothing"])
		.current_dir(project.path())
		.output()
		.unwrap();

	assert_eq!(output.status.code(), Some(1));
	assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
	assert_eq!(
		String::from_utf8(output.stderr).unwrap(),
		"nsx found no ticket 0099-TICKET-nothing: no file named 0099-TICKET-nothing.ticket.md is under .agents/ or .claude/.\n"
	);
}

#[test]
fn a_link_the_plan_does_not_read_stops_it_before_its_first_line() {
	let ticket = format!("{HEADER}{FOUR}- [notes](./notes.md)\n");
	let project = project(&[(".agents/0007-TICKET-notes.ticket.md", &ticket)]);

	let output = Command::cargo_bin("nsx")
		.unwrap()
		.args(["agent", "plan", "0007-TICKET-notes"])
		.current_dir(project.path())
		.output()
		.unwrap();

	assert_eq!(output.status.code(), Some(1));
	assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
	assert_eq!(
		String::from_utf8(output.stderr).unwrap(),
		"nsx does not read the link ./notes.md: a ticket links one .prd.md, one .dd.md, one .spec.md and one .gr.md.\n"
	);
}

#[test]
fn a_document_without_its_field_stops_the_plan_after_the_lines_already_printed()
{
	let ticket = format!("{HEADER}{FOUR}");
	let project = project(&[
		(".agents/0007-TICKET-notes.ticket.md", &ticket),
		(".agents/0005-DD-notes.dd.md", DESIGN),
		(
			".agents/0002-PRD-notes.prd.md",
			"# Notes\n\nNo use case is stated.\n",
		),
	]);

	Command::cargo_bin("nsx")
		.unwrap()
		.arg("init")
		.current_dir(project.path())
		.assert()
		.success();

	let output = Command::cargo_bin("nsx")
		.unwrap()
		.args(["agent", "plan", "0007-TICKET-notes"])
		.current_dir(project.path())
		.output()
		.unwrap();

	assert_eq!(output.status.code(), Some(1));
	assert_eq!(
		String::from_utf8(output.stdout).unwrap(),
		"# plan only · nothing is built, started or paid for in this step\nconsole › tool call: read 0007-TICKET-notes\nconsole › ready-for-agent · links 0002-PRD · 0005-DD · 0011-SPEC · guardrail 0001-finops\n# analysis · reading the linked documents, then the project's episodes\n"
	);
	assert_eq!(
		String::from_utf8(output.stderr).unwrap(),
		"nsx found no **Use case:** line in .agents/0002-PRD-notes.prd.md.\n"
	);
}
