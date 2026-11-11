//! A tool of the machine: how it is built, fed and held to ending well

use std::path::Path;

use nsx::process::{
	environment::Environment, error::Error, local::Local, program::Program,
	runner::Runner,
};
use rstest::rstest;

use crate::process::{said::said, shell::shell};
use crate::simulation::fake::programs::Programs;

#[test]
fn a_tool_is_its_name_its_arguments_whole_and_nothing_to_read() {
	let tool = Program::tool(
		"git",
		&["push", "--set-upstream", "origin", "feat/a b"],
		&["NSX_TEST_NEVER_SET"],
		Path::new("/project"),
	);

	assert_eq!(tool.name, "git");
	assert_eq!(
		tool.arguments,
		["push", "--set-upstream", "origin", "feat/a b"]
	);
	assert_eq!(tool.directory, Path::new("/project"));
	assert!(tool.input.is_empty());
	assert!(
		!tool
			.environment
			.variables
			.contains_key("NSX_TEST_NEVER_SET")
	);
}

#[tokio::test]
async fn what_a_program_is_given_reaches_its_standard_input() {
	let program = Program {
		input: b"one line\nand another".to_vec(),
		..shell("cat", Environment::allow(&[]))
	};

	let output = Local.run(&program).await.unwrap();

	assert_eq!(output.printed, b"one line\nand another");
	assert_eq!(output.status, Some(0));
}

#[tokio::test]
async fn a_program_given_nothing_reads_nothing() {
	let program = shell("cat; printf done", Environment::allow(&[]));

	let output = Local.run(&program).await.unwrap();

	assert_eq!(output.printed, b"done");
}

#[tokio::test]
async fn a_tool_that_ends_well_gives_back_what_it_printed() {
	let programs = Programs {
		printed: String::from("3f9c2e1\n"),
		..Programs::default()
	};
	let runner: &dyn Runner = &programs;
	let tool = Program::tool("git", &["rev-parse"], &[], Path::new("/project"));

	let output = runner.finish(&tool).await.unwrap();

	assert_eq!(output.printed, b"3f9c2e1\n");
}

#[rstest]
#[case::with_an_argument(&["push", "origin"], "git push ended with the exit status 128.")]
#[case::with_none(&[], "git ended with the exit status 128.")]
fn a_tool_that_ends_badly_is_named_with_its_status_and_not_its_words(
	#[case] arguments: &[&str],
	#[case] sentence: &str,
) {
	let tool = Program::tool("git", arguments, &[], Path::new("/project"));
	let output = said(128, "remote: https://token@host/repository refused");

	let refusal = output.ended(&tool).unwrap_err();

	assert!(matches!(refusal, Error::Failed { .. }), "was {refusal:?}");
	assert_eq!(refusal.to_string(), sentence);
}

#[test]
fn a_tool_ended_by_a_signal_is_its_own_sentence() {
	let tool = Program::tool("kubectl", &["apply"], &[], Path::new("/project"));
	let output = nsx::process::output::Output {
		status: None,
		..said(0, "")
	};

	let refusal = output.ended(&tool).unwrap_err();

	assert_eq!(refusal.to_string(), "kubectl apply was ended by a signal.");
}
