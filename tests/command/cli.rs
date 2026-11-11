//! The command line: `init`, `agent plan <ticket>` and `agent run`, and
//! nothing else.

use std::path::PathBuf;

use clap::Parser;
use rstest::rstest;

use nsx::command::{agent::Agent, cli::Cli};

use crate::simulation::fake::recorder::Recorder;
use crate::simulation::fake::session::session;

#[test]
fn init_is_the_init_command() {
	let cli = Cli::try_parse_from(["nsx", "init"]).unwrap();

	assert!(matches!(cli, Cli::Init(_)), "parsed as {cli:?}");
}

#[test]
fn agent_plan_carries_the_bare_ticket_id() {
	let cli = Cli::try_parse_from([
		"nsx",
		"agent",
		"plan",
		"0031-TICKET-book-a-demo-runtime",
	])
	.unwrap();

	let Cli::Agent(Agent::Plan(plan)) = cli else {
		panic!("parsed as {cli:?}");
	};

	assert_eq!(plan.ticket, "0031-TICKET-book-a-demo-runtime");
}

#[test]
fn agent_run_takes_no_argument() {
	let cli = Cli::try_parse_from(["nsx", "agent", "run"]).unwrap();

	assert!(
		matches!(cli, Cli::Agent(Agent::Run(_))),
		"parsed as {cli:?}"
	);
}

#[rstest]
#[case::no_command(&["nsx"])]
#[case::help_command(&["nsx", "help"])]
#[case::harness(&["nsx", "harness", "envelope", "set"])]
#[case::finops(&["nsx", "finops", "budget", "set"])]
#[case::gates(&["nsx", "gates", "report"])]
#[case::agent_confirm(&["nsx", "agent", "confirm"])]
#[case::agent_connect(&["nsx", "agent", "connect"])]
#[case::plan_without_a_ticket(&["nsx", "agent", "plan"])]
#[case::plan_with_two_tickets(&["nsx", "agent", "plan", "0031", "0032"])]
#[case::run_with_an_argument(&["nsx", "agent", "run", "book-a-demo-runtime"])]
#[case::version_flag(&["nsx", "--version"])]
fn anything_else_is_refused(#[case] typed: &[&str]) {
	let parsed = Cli::try_parse_from(typed);

	assert!(parsed.is_err(), "parsed as {parsed:?}");
}

#[rstest]
#[case::init(&["nsx", "init"])]
#[tokio::test]
async fn a_command_runs_over_its_session(#[case] typed: &[&str]) {
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let cli = Cli::try_parse_from(typed).unwrap();

	let ended = cli.execute(&mut session).await;

	assert!(ended.is_ok(), "stopped with {ended:?}");
}

#[rstest]
#[case::plan(&["nsx", "agent", "plan", "0031-TICKET-book-a-demo-runtime"])]
#[case::run(&["nsx", "agent", "run"])]
#[tokio::test]
async fn an_agent_command_looks_for_the_project_before_anything_else(
	#[case] typed: &[&str],
) {
	let elsewhere = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(elsewhere.path()), &recorder);
	let cli = Cli::try_parse_from(typed).unwrap();

	let stop = cli.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains("nsx init"), "said {stop:?}");
	assert!(recorder.lines.borrow().is_empty());
}
