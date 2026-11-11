//! The tokens a call used, read from what the coding agent printed

use nsx::driver::{paths::Paths, tokens::Tokens};
use rstest::rstest;

#[rstest]
#[case::both_counts(
	r#"{"usage": {"input_tokens": 740000, "output_tokens": 48000}}"#,
	Some(740_000),
	Some(48_000)
)]
#[case::text_then_the_report_on_the_last_line(
	"working...\ndone\n{\"usage\": {\"input_tokens\": 12, \"output_tokens\": 3}}\n",
	Some(12),
	Some(3)
)]
#[case::a_count_of_zero_is_known(
	r#"{"usage": {"input_tokens": 0, "output_tokens": 7}}"#,
	Some(0),
	Some(7)
)]
#[case::one_count_missing(r#"{"usage": {"input_tokens": 12}}"#, Some(12), None)]
#[case::no_usage_at_all(r#"{"result": "ok"}"#, None, None)]
#[case::a_count_that_is_text(
	r#"{"usage": {"input_tokens": "12", "output_tokens": 3}}"#,
	None,
	Some(3)
)]
#[case::a_negative_count(
	r#"{"usage": {"input_tokens": -5, "output_tokens": 3}}"#,
	None,
	Some(3)
)]
#[case::a_count_that_is_not_whole(
	r#"{"usage": {"input_tokens": 1.5, "output_tokens": 3}}"#,
	None,
	Some(3)
)]
#[case::output_that_is_not_json("the agent crashed", None, None)]
#[case::no_output("", None, None)]
fn tokens_are_read_at_the_configured_paths_and_unknown_is_never_zero(
	#[case] printed: &str,
	#[case] input: Option<u64>,
	#[case] output: Option<u64>,
) {
	let paths = Paths {
		input_tokens: String::from("usage.input_tokens"),
		output_tokens: String::from("usage.output_tokens"),
	};

	let tokens = Tokens::read(printed, &paths);

	assert_eq!((tokens.input, tokens.output), (input, output));
}

#[test]
fn a_path_may_lead_anywhere_in_the_output() {
	let paths = Paths {
		input_tokens: String::from("result.tokens.in"),
		output_tokens: String::from("out"),
	};
	let printed = r#"{"result": {"tokens": {"in": 9}}, "out": 4}"#;

	let tokens = Tokens::read(printed, &paths);

	assert_eq!((tokens.input, tokens.output), (Some(9), Some(4)));
}
