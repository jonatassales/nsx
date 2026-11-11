//! How a call ended, and whether that gives its stage a line

use nsx::driver::{ended::Ended, tokens::Tokens};
use rstest::rstest;

#[test]
fn a_call_that_ended_well_and_said_what_it_used_is_accepted() {
	let ended = Ended {
		stage: String::from("spec"),
		skill: String::from("to-spec"),
		status: Some(0),
		tokens: Tokens {
			input: Some(0),
			output: Some(12),
		},
		output: Vec::new(),
	};

	ended.accept().unwrap();
}

#[rstest]
#[case::a_non_zero_exit(
	Some(2),
	Some(1),
	Some(1),
	"The coding agent ended spec with the exit status 2."
)]
#[case::an_end_by_a_signal(
	None,
	Some(1),
	Some(1),
	"The coding agent was ended by a signal during spec."
)]
#[case::no_input_count(
	Some(0),
	None,
	Some(1),
	"The coding agent reported no token usage for spec: nsx never prints a spend it did not measure."
)]
#[case::no_output_count(
	Some(0),
	Some(1),
	None,
	"The coding agent reported no token usage for spec: nsx never prints a spend it did not measure."
)]
fn any_other_end_fails_the_stage(
	#[case] status: Option<i32>,
	#[case] input: Option<u64>,
	#[case] output: Option<u64>,
	#[case] sentence: &str,
) {
	let ended = Ended {
		stage: String::from("spec"),
		skill: String::from("to-spec"),
		status,
		tokens: Tokens {
			input,
			output,
		},
		output: Vec::new(),
	};

	let refusal = ended.accept().unwrap_err();

	assert_eq!(refusal.to_string(), sentence);
}
