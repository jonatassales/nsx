//! The usage record: what each paid call used, and the run's totals

use std::path::PathBuf;

use nsx::contract::decode::decode;
use nsx::driver::{ended::Ended, tokens::Tokens};
use nsx::evidence::usage::Usage;
use nsx::files::root::Root;
use nsx::money::cents::Cents;
use nsx::session::kind::Kind;

const FILE: &str = ".langdrift/microruntimes/runtimes/notes/usage.json";

#[test]
fn each_call_is_kept_in_order_and_the_totals_are_summed() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);
	let call = |stage: &str, input: u64, output: u64| Ended {
		stage: String::from(stage),
		skill: String::from("grilling"),
		status: Some(0),
		tokens: Tokens {
			input: Some(input),
			output: Some(output),
		},
		output: Vec::new(),
	};
	let mut usage = Usage {
		run: String::from("notes-0001"),
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		calls: Vec::new(),
		input_tokens: None,
		output_tokens: None,
	};

	usage
		.write(&root, &file, &call("planning", 700, 40))
		.unwrap();
	usage.write(&root, &file, &call("spec", 300, 8)).unwrap();

	let kept =
		decode::<Vec<Usage>>(&std::fs::read_to_string(&file).unwrap()).unwrap();
	let stages = kept[0]
		.calls
		.iter()
		.map(|used| used.stage.as_str())
		.collect::<Vec<_>>();

	assert_eq!(kept.len(), 1);
	assert_eq!(kept[0], usage);
	assert_eq!(stages, ["planning", "spec"]);
	assert_eq!(
		(usage.input_tokens, usage.output_tokens),
		(Some(1000), Some(48))
	);
}

#[test]
fn a_call_that_reported_no_tokens_is_kept_and_makes_the_totals_unknown() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);
	let measured = Ended {
		stage: String::from("planning"),
		skill: String::from("grilling"),
		status: Some(0),
		tokens: Tokens {
			input: Some(700),
			output: Some(40),
		},
		output: Vec::new(),
	};
	let unmeasured = Ended {
		stage: String::from("spec"),
		skill: String::from("to-spec"),
		status: Some(1),
		tokens: Tokens {
			input: None,
			output: Some(8),
		},
		output: Vec::new(),
	};
	let mut usage = Usage {
		run: String::from("notes-0001"),
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		calls: Vec::new(),
		input_tokens: None,
		output_tokens: None,
	};

	usage.write(&root, &file, &measured).unwrap();
	usage.write(&root, &file, &unmeasured).unwrap();

	assert_eq!(usage.calls.len(), 2);
	assert_eq!(usage.calls[1].input_tokens, None);
	assert_eq!((usage.input_tokens, usage.output_tokens), (None, Some(48)));
}

#[test]
fn a_later_run_of_the_runtime_is_appended_beside_the_earlier_one() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let file = project.path().join(FILE);
	let call = Ended {
		stage: String::from("planning"),
		skill: String::from("grilling"),
		status: Some(0),
		tokens: Tokens {
			input: Some(700),
			output: Some(40),
		},
		output: Vec::new(),
	};
	let entry = |run: &str| Usage {
		run: String::from(run),
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		calls: Vec::new(),
		input_tokens: None,
		output_tokens: None,
	};

	entry("notes-0001").write(&root, &file, &call).unwrap();
	entry("notes-0002").write(&root, &file, &call).unwrap();

	let kept =
		decode::<Vec<Usage>>(&std::fs::read_to_string(&file).unwrap()).unwrap();
	let runs = kept
		.iter()
		.map(|usage| usage.run.as_str())
		.collect::<Vec<_>>();

	assert_eq!(runs, ["notes-0001", "notes-0002"]);
}

#[test]
fn the_completed_line_states_what_was_observed_of_what_was_reserved() {
	let line = Usage::line(Cents(980), Cents(1140));

	assert_eq!((line.id.as_str(), line.kind), ("completed", Kind::Success));
	assert_eq!(
		line.render(),
		"✓ completed  implementation · $9.80 observed of $11.40 reserved · usage record"
	);
}
