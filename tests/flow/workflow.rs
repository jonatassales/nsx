//! The workflow is read as `nsx init` laid it down

use std::path::PathBuf;

use nsx::command::init::Init;
use nsx::files::{layout::Layout, root::Root};
use nsx::flow::{error::Error, workflow::Workflow};
use rstest::rstest;

use crate::money::listed::listed;
use crate::simulation::fake::recorder::Recorder;
use crate::simulation::fake::session::session;

#[tokio::test]
async fn the_workflow_reads_as_init_laid_it_down() {
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	Init.execute(&mut session).await.unwrap();

	let workflow =
		Workflow::load(&root, &Layout::under(&root), &listed().0).unwrap();
	let stages = workflow
		.stages
		.iter()
		.map(|stage| stage.id.as_str())
		.collect::<Vec<_>>();
	let defaults = workflow.defaults;

	assert_eq!(
		stages,
		[
			"planning",
			"spec",
			"tickets",
			"implement",
			"dev-ticket",
			"review",
			"merge",
		]
	);
	assert_eq!(workflow.stages[3].skill.as_deref(), Some("implement · tdd"));
	assert_eq!(workflow.stages[4].skill, None);
	assert_eq!(
		(defaults.model.as_str(), defaults.effort.as_str()),
		("claude-sonnet-5-5", "medium")
	);
	assert!(defaults.human.planning && defaults.human.review);
	assert!(!defaults.human.spec);
}

#[test]
fn a_project_without_a_workflow_is_a_stop() {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};

	let refusal =
		Workflow::load(&root, &Layout::under(&root), &listed().0).unwrap_err();

	assert!(matches!(refusal, Error::File(_)), "was {refusal:?}");
	assert!(
		refusal.to_string().contains("workflow.json"),
		"said {refusal}"
	);
}

#[rstest]
#[case::a_field_it_does_not_define(
	r#""order": "fixed""#,
	"medium",
	"claude-sonnet-5-5",
	"order"
)]
fn a_workflow_that_cannot_be_read_as_written_is_a_stop(
	#[case] extra: &str,
	#[case] effort: &str,
	#[case] model: &str,
	#[case] named: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);
	let written = format!(
		r#"{{{extra}, "stages": [], "defaults": {{"model": "{model}", "effort": "{effort}", "human": {{"planning": true, "spec": false, "tickets": false, "implement": false, "review": true}}}}}}"#
	);

	std::fs::create_dir_all(layout.workflow.parent().unwrap()).unwrap();
	std::fs::write(&layout.workflow, written).unwrap();

	let refusal = Workflow::load(&root, &layout, &listed().0).unwrap_err();

	assert!(refusal.to_string().contains(named), "said {refusal}");
}

#[rstest]
#[case::an_effort_outside_the_three(
	"\"medium\"",
	"\"extreme\"",
	"default effort must be one of low, medium, high"
)]
#[case::a_model_the_price_list_lacks(
	"claude-sonnet-5-5",
	"claude-mini-1",
	"default model claude-mini-1 is not in the price list"
)]
#[case::a_stage_missing(
	"\"id\": \"merge\"",
	"\"id\": \"ship\"",
	"stages must be planning, spec"
)]
#[case::a_stage_without_its_skill(
	"\"skill\": \"implement · tdd\"",
	"\"skill\": null",
	"each stage must keep the skill nsx plays it with"
)]
#[case::a_stage_with_another_skill(
	"\"skill\": \"grilling\"",
	"\"skill\": \"vibes\"",
	"each stage must keep the skill nsx plays it with"
)]
#[tokio::test]
async fn a_workflow_that_breaks_a_rule_is_a_stop(
	#[case] from: &str,
	#[case] to: &str,
	#[case] named: &str,
) {
	let project = tempfile::tempdir().unwrap();
	let recorder = Recorder::default();
	let mut session = session(PathBuf::from(project.path()), &recorder);
	let root = Root {
		path: PathBuf::from(project.path()),
	};
	let layout = Layout::under(&root);

	Init.execute(&mut session).await.unwrap();

	let written = std::fs::read_to_string(&layout.workflow).unwrap();

	std::fs::write(&layout.workflow, written.replace(from, to)).unwrap();

	let refusal = Workflow::load(&root, &layout, &listed().0).unwrap_err();

	assert!(matches!(refusal, Error::Rule { .. }), "was {refusal:?}");
	assert!(refusal.to_string().contains(named), "said {refusal}");
}
