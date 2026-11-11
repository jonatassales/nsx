//! The runner on this machine starts a real program

use nsx::process::{
	environment::Environment, error::Error, local::Local, program::Program,
	runner::Runner,
};

use crate::process::shell::shell;

#[tokio::test]
async fn what_a_program_prints_and_its_status_come_back() {
	let program = shell(
		"printf hello; printf oops >&2; exit 3",
		Environment::allow(&[]),
	);

	let output = Local.run(&program).await.unwrap();

	assert_eq!(output.printed, b"hello");
	assert_eq!(output.complained, b"oops");
	assert_eq!(output.status, Some(3));
}

#[tokio::test]
async fn a_program_does_not_inherit_the_environment_of_nsx() {
	let program = shell(
		"printf '%s' \"${CARGO_PKG_NAME:-unset}\"",
		Environment::allow(&[]),
	);

	let output = Local.run(&program).await.unwrap();

	assert_eq!(output.printed, b"unset");
}

#[tokio::test]
async fn an_argument_reaches_the_program_whole() {
	let program = Program {
		name: String::from("/bin/sh"),
		arguments: vec![
			String::from("-c"),
			String::from("printf '%s' \"$1\""),
			String::from("sh"),
			String::from("two words; echo no"),
		],
		environment: Environment::allow(&[]),
		directory: std::env::temp_dir(),
		input: Vec::new(),
	};

	let output = Local.run(&program).await.unwrap();

	assert_eq!(output.printed, b"two words; echo no");
}

#[tokio::test]
async fn a_program_that_does_not_exist_is_not_run() {
	let program = Program {
		name: String::from("nsx-no-such-program"),
		arguments: Vec::new(),
		environment: Environment::allow(&[]),
		directory: std::env::temp_dir(),
		input: Vec::new(),
	};

	let refusal = Local.run(&program).await.unwrap_err();

	assert!(matches!(refusal, Error::NotRun { .. }), "was {refusal:?}");
	assert!(
		refusal
			.to_string()
			.starts_with("nsx could not run nsx-no-such-program:"),
		"was `{refusal}`"
	);
}
