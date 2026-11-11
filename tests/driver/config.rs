//! `driver.json`: how the team's coding agent is started

use std::path::PathBuf;

use nsx::driver::{config::Config, error::Error};
use nsx::files::{layout::Layout, root::Root};
use rstest::rstest;

const WRITTEN: &str = r#"{"executable": "coding-agent", "args": ["--model", "{{model}}", "--effort={{effort}}", "{{prompt}}"], "passEnv": ["AGENT_KEY"], "usage": {"inputTokens": "usage.input_tokens", "outputTokens": "usage.output_tokens"}}"#;

#[test]
fn the_configuration_reads_as_the_team_wrote_it() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	std::fs::create_dir_all(layout.driver.parent().unwrap()).unwrap();
	std::fs::write(&layout.driver, WRITTEN).unwrap();

	let config = Config::load(&root, &layout).unwrap();

	assert_eq!(config.executable, "coding-agent");
	assert_eq!(config.args.len(), 4);
	assert_eq!(config.pass_env, ["AGENT_KEY"]);
	assert_eq!(config.usage.input_tokens, "usage.input_tokens");
	assert_eq!(config.usage.output_tokens, "usage.output_tokens");
}

#[test]
fn a_project_without_a_driver_configuration_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal = Config::load(&root, &Layout::under(&root)).unwrap_err();

	assert!(matches!(refusal, Error::Missing { .. }), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("driver.json"),
		"said {refusal}"
	);
}

#[rstest]
#[case::no_model("{{model}}", "sonnet")]
#[case::no_effort("{{effort}}", "medium")]
fn arguments_that_do_not_carry_the_choice_are_refused(
	#[case] placeholder: &str,
	#[case] fixed: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	std::fs::create_dir_all(layout.driver.parent().unwrap()).unwrap();
	std::fs::write(&layout.driver, WRITTEN.replace(placeholder, fixed)).unwrap();

	let refusal = Config::load(&root, &layout).unwrap_err();

	assert!(
		matches!(refusal, Error::Arguments { .. }),
		"was {refusal:?}"
	);
	assert!(
		refusal
			.to_string()
			.contains("its args must carry {{model}} and {{effort}}"),
		"said {refusal}"
	);
}

#[test]
fn a_field_the_configuration_does_not_define_is_refused() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = WRITTEN.replace(r#""passEnv""#, r#""shell": true, "passEnv""#);

	std::fs::create_dir_all(layout.driver.parent().unwrap()).unwrap();
	std::fs::write(&layout.driver, written).unwrap();

	let refusal = Config::load(&root, &layout).unwrap_err();

	assert!(matches!(refusal, Error::Config { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains("shell"), "said {refusal}");
}
