//! One stage call: the agent started with the run's choice, its log kept

use std::path::PathBuf;

use nsx::driver::{
	agent::Agent, call::Call, config::Config, launcher::Launcher,
};
use nsx::files::root::Root;

use crate::driver::{
	called::{REPORT, called},
	configured::configured,
};
use crate::simulation::fake::{
	coder::{CODER, Coder},
	programs::Programs,
};

#[tokio::test]
async fn the_agent_is_started_directly_with_each_placeholder_filled_in_place() {
	let project = tempfile::tempdir().unwrap();
	let programs = Programs::default();
	let launcher = Launcher {
		programs: &programs,
	};

	launcher
		.start(&called(project.path()), &configured())
		.await
		.unwrap();

	let started = programs.started.borrow();
	let program = &started[0];
	let given = |name: &str| program.environment.variables.get(name).cloned();

	assert_eq!(program.name, "coding-agent");
	assert_eq!(program.directory, project.path());
	assert_eq!(program.arguments.len(), 5);
	assert_eq!(program.arguments[0], "--model=claude-sonnet-5-5");
	assert_eq!(program.arguments[2], "medium");
	assert_eq!(
		program.arguments[3],
		"implement of 0007-TICKET-notes in notes-0001"
	);
	assert!(program.arguments[4].contains("implement · tdd"));
	assert!(program.arguments[4].contains(REPORT));
	assert_eq!(
		given("LANGDRIFT_MODEL").as_deref(),
		Some("claude-sonnet-5-5")
	);
	assert_eq!(given("LANGDRIFT_EFFORT").as_deref(), Some("medium"));
	assert_eq!(given("LANGDRIFT_STAGE").as_deref(), Some("implement"));
	assert_eq!(given("LANGDRIFT_RUN").as_deref(), Some("notes-0001"));
	assert_eq!(given("LANGDRIFT_REPORT").as_deref(), Some(REPORT));
	assert_eq!(given("NSX_TEST_NEVER_SET"), None);
}

#[tokio::test]
async fn a_call_gives_back_its_tokens_and_its_log_is_kept_privately() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let call = called(project.path());

	let ended = call.run(&CODER, &configured(), &root).await.unwrap();
	let log = project
		.path()
		.join(".langdrift/agent/state/runs/notes-0001/stages/implement.log");

	call.log(&root, &ended).unwrap();

	assert_eq!(ended.stage, "implement");
	assert_eq!(ended.skill, "implement · tdd");
	assert_eq!(ended.status, Some(0));
	assert_eq!(
		(ended.tokens.input, ended.tokens.output),
		(Some(740_000), Some(48_000))
	);
	assert!(
		std::fs::read_to_string(log)
			.unwrap()
			.contains("input_tokens")
	);
}

#[tokio::test]
async fn a_report_an_earlier_call_left_is_never_read_as_this_calls() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let call = called(project.path());
	let report = project.path().join(REPORT);
	let silent = Coder {
		silent: Some("implement"),
		..CODER
	};

	std::fs::create_dir_all(report.parent().unwrap()).unwrap();
	std::fs::write(&report, "{}").unwrap();

	call.run(&silent, &configured(), &root).await.unwrap();

	assert!(!report.exists());
}

#[tokio::test]
async fn a_value_that_looks_like_a_placeholder_is_not_filled_again() {
	let project = tempfile::tempdir().unwrap();
	let programs = Programs::default();
	let launcher = Launcher {
		programs: &programs,
	};
	let call = Call {
		ticket: String::from("T-{{effort}}"),
		skill: String::from("grilling {{model}} {{run}}"),
		..called(project.path())
	};

	launcher.start(&call, &configured()).await.unwrap();

	let started = programs.started.borrow();
	let arguments = &started[0].arguments;

	assert_eq!(arguments[3], "implement of T-{{effort}} in notes-0001");
	assert!(arguments[4].contains("grilling {{model}} {{run}}"));
	assert!(arguments[4].contains("T-{{effort}}"));
}

#[tokio::test]
async fn a_brace_before_a_placeholder_does_not_hide_it() {
	let project = tempfile::tempdir().unwrap();
	let programs = Programs::default();
	let launcher = Launcher {
		programs: &programs,
	};
	let config = Config {
		args: vec![
			String::from("{{{model}}}"),
			String::from("{ {{effort}} {{unknown}} {{"),
		],
		..configured()
	};

	launcher
		.start(&called(project.path()), &config)
		.await
		.unwrap();

	let started = programs.started.borrow();

	assert_eq!(started[0].arguments[0], "{claude-sonnet-5-5}");
	assert_eq!(started[0].arguments[1], "{ medium {{unknown}} {{");
}
