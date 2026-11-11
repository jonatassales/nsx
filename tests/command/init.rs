//! `nsx init` lays down `.langdrift/`, prints nothing, and is safe to repeat

use std::path::PathBuf;

use assert_cmd::Command;
use serde_json::Value;

use nsx::command::init::Init;

use crate::command::{laid::laid, tree::tree};
use crate::simulation::fake::recorder::Recorder;
use crate::simulation::fake::session::session;

const DECLARED: [&str; 16] = [
	"finops/forecasts.json",
	"finops/prices.json",
	"harness/guardrails/0001-finops.gr.md",
	"harness/workflow.json",
	"skills/code-review/SKILL.md",
	"skills/grilling/SKILL.md",
	"skills/implement/SKILL.md",
	"skills/tdd/SKILL.md",
	"skills/tdd/mocking.guide.md",
	"skills/tdd/tests.guide.md",
	"skills/to-spec/SKILL.md",
	"skills/to-tickets/SKILL.md",
	"templates/dd.md",
	"templates/prd.md",
	"templates/spec.md",
	"templates/ticket.md",
];

#[tokio::test]
async fn init_lays_down_the_declared_files_and_no_other() {
	let project = tempfile::tempdir().unwrap();

	laid(project.path()).await;

	let laid = tree(&project.path().join(".langdrift"));

	assert_eq!(laid.keys().collect::<Vec<_>>(), DECLARED);
	assert_eq!(tree(project.path()).len(), DECLARED.len());
}

#[tokio::test]
async fn init_prints_nothing() {
	let project = tempfile::tempdir().unwrap();

	let recorder = laid(project.path()).await;

	assert!(recorder.lines.borrow().is_empty());
	assert!(recorder.title.borrow().is_none());
}

#[tokio::test]
async fn a_second_run_changes_no_byte() {
	let project = tempfile::tempdir().unwrap();

	laid(project.path()).await;

	let first = tree(project.path());

	laid(project.path()).await;

	assert_eq!(tree(project.path()), first);
}

#[tokio::test]
async fn a_file_the_team_shaped_is_kept_byte_for_byte() {
	let project = tempfile::tempdir().unwrap();
	let langdrift = project.path().join(".langdrift");
	let prices = langdrift.join("finops").join("prices.json");
	let workflow = langdrift.join("harness").join("workflow.json");

	laid(project.path()).await;

	std::fs::write(&prices, "{\"models\": []}\n").unwrap();
	std::fs::remove_file(&workflow).unwrap();

	laid(project.path()).await;

	assert_eq!(std::fs::read(&prices).unwrap(), b"{\"models\": []}\n");
	assert!(workflow.is_file());
	assert_eq!(tree(&langdrift).keys().collect::<Vec<_>>(), DECLARED);
}

#[tokio::test]
async fn the_price_list_holds_the_three_models_read_on_2026_10_06() {
	let project = tempfile::tempdir().unwrap();

	laid(project.path()).await;

	let laid = tree(&project.path().join(".langdrift"));
	let prices: Value =
		serde_json::from_slice(&laid["finops/prices.json"]).unwrap();
	let models = prices["models"]
		.as_array()
		.unwrap()
		.iter()
		.map(|model| {
			(
				model["id"].as_str().unwrap(),
				model["shortName"].as_str().unwrap(),
				model["inputCents"].as_u64().unwrap(),
				model["outputCents"].as_u64().unwrap(),
			)
		})
		.collect::<Vec<_>>();

	assert_eq!(prices["readOn"], "2026-10-06");
	assert_eq!(prices["cache"], "uncached");
	assert_eq!(prices["currency"], "USD");
	assert_eq!(
		models,
		[
			("claude-opus-5-5", "Opus 5.5", 400, 2000),
			("claude-sonnet-5-5", "Sonnet 5.5", 200, 1000),
			("claude-haiku-5-5", "Haiku 5.5", 10, 50),
		]
	);
}

#[tokio::test]
async fn the_forecasts_hold_the_expected_tokens_per_effort() {
	let project = tempfile::tempdir().unwrap();

	laid(project.path()).await;

	let laid = tree(&project.path().join(".langdrift"));
	let forecasts: Value =
		serde_json::from_slice(&laid["finops/forecasts.json"]).unwrap();
	let figures = ["low", "medium", "high"].map(|effort| {
		let forecast = &forecasts["byEffort"][effort];

		(
			forecast["input"].as_u64().unwrap(),
			forecast["output"].as_u64().unwrap(),
		)
	});

	assert_eq!(forecasts["unit"], "thousands-of-tokens");
	assert_eq!(figures, [(2000, 120), (3500, 250), (6000, 500)]);
}

#[tokio::test]
async fn the_workflow_holds_the_seven_stages_and_the_default_choice() {
	let project = tempfile::tempdir().unwrap();

	laid(project.path()).await;

	let laid = tree(&project.path().join(".langdrift"));
	let workflow: Value =
		serde_json::from_slice(&laid["harness/workflow.json"]).unwrap();
	let stages = workflow["stages"]
		.as_array()
		.unwrap()
		.iter()
		.map(|stage| {
			(
				stage["id"].as_str().unwrap(),
				stage["skill"].as_str(),
				stage["person"].as_str().unwrap(),
			)
		})
		.collect::<Vec<_>>();
	let defaults = &workflow["defaults"];

	assert_eq!(
		stages,
		[
			("planning", Some("grilling"), "required"),
			("spec", Some("to-spec"), "optional"),
			("tickets", Some("to-tickets"), "optional"),
			("implement", Some("implement · tdd"), "optional"),
			("dev-ticket", None, "writes"),
			("review", Some("code-review"), "optional"),
			("merge", None, "owner"),
		]
	);
	assert_eq!(defaults["model"], "claude-sonnet-5-5");
	assert_eq!(defaults["effort"], "medium");
	assert_eq!(
		defaults["human"],
		serde_json::json!({
			"planning": true,
			"spec": false,
			"tickets": false,
			"implement": false,
			"review": true,
		})
	);
}

#[cfg(unix)]
#[tokio::test]
async fn a_langdrift_folder_that_is_a_link_is_refused() {
	let project = tempfile::tempdir().unwrap();
	let elsewhere = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);

	std::os::unix::fs::symlink(
		elsewhere.path(),
		project.path().join(".langdrift"),
	)
	.unwrap();

	let stop = Init.execute(&mut session).await.unwrap_err();

	assert!(stop.sentence.contains("symbolic link"), "said {stop:?}");
	assert!(tree(elsewhere.path()).is_empty());
}

#[test]
fn typed_in_a_project_it_prints_nothing_and_exits_0() {
	let project = tempfile::tempdir().unwrap();

	Command::cargo_bin("nsx")
		.unwrap()
		.arg("init")
		.current_dir(project.path())
		.assert()
		.success()
		.stdout("")
		.stderr("");

	assert!(
		project
			.path()
			.join(".langdrift/finops/prices.json")
			.is_file()
	);
	assert!(
		!project
			.path()
			.join(".langdrift/harness/driver.json")
			.exists()
	);
}
