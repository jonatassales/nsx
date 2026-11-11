//! The stop rule, seen from outside: what reaches standard output, what
//! reaches standard error and how the command exits

use assert_cmd::Command;
use rstest::rstest;

#[rstest]
#[case::no_command(&[], "nsx needs a command: init, agent plan <ticket> or agent run.")]
#[case::unknown_command(
	&["harness"],
	"nsx does not accept that: unrecognized subcommand 'harness'."
)]
#[case::plan_without_a_ticket(
	&["agent", "plan"],
	"nsx agent plan needs the ticket's bare id, as in 0031-TICKET-book-a-demo-runtime."
)]
#[case::plan_with_two_tickets(
	&["agent", "plan", "0031", "0032"],
	"nsx does not accept that: unexpected argument '0032' found."
)]
#[case::removed_agent_command(
	&["agent", "confirm"],
	"nsx does not accept that: unrecognized subcommand 'confirm'."
)]
fn a_refused_command_line_is_one_sentence_and_exit_1(
	#[case] typed: &[&str],
	#[case] sentence: &str,
) {
	let project = tempfile::tempdir().unwrap();

	let output = Command::cargo_bin("nsx")
		.unwrap()
		.current_dir(project.path())
		.args(typed)
		.output()
		.unwrap();

	assert_eq!(output.status.code(), Some(1));
	assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
	assert_eq!(
		String::from_utf8(output.stderr).unwrap(),
		format!("{sentence}\n")
	);
}

#[test]
fn help_is_not_a_stop() {
	let project = tempfile::tempdir().unwrap();

	let output = Command::cargo_bin("nsx")
		.unwrap()
		.current_dir(project.path())
		.arg("--help")
		.output()
		.unwrap();

	assert_eq!(output.status.code(), Some(0));
	assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
}
