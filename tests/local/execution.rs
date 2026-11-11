//! The execution ledger: written before each creation, and found again

use std::path::PathBuf;

use nsx::contract::label::label;
use nsx::files::{private::private, root::Root};
use nsx::local::{error::Error, execution::Execution};
use rstest::rstest;

use crate::local::{deployed::deployed, rooted::rooted};
use crate::simulation::fake::{host::HOST, machine::Machine};

const LEDGER: &str = ".langdrift/agent/state/runs/notes-0001/execution.json";

#[tokio::test]
async fn a_deployment_is_written_down_before_it_is_created() {
	let (project, root, layout) = rooted();
	let machine = Machine {
		failing: Some("a"),
		..Machine::default()
	};
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();

	execution
		.up(&root, &layout, &machine, &[deployed("a"), deployed("b")])
		.await
		.unwrap_err();

	let file = project.path().join(LEDGER);
	let kept = Execution::load(&root, &file).unwrap().unwrap();

	assert_eq!(kept.deployments, ["a"]);
	assert!(kept.removed.is_empty());
	assert!(!kept.settled);
	assert!(private(&file).unwrap());
}

#[test]
fn the_space_is_written_down_before_the_cluster_is_asked() {
	let (project, root, layout) = rooted();

	let execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();
	let kept = Execution::load(&root, &project.path().join(LEDGER))
		.unwrap()
		.unwrap();

	assert_eq!(kept, execution);
	assert_eq!(kept.space, Execution::space(&root, "notes-0001"));
}

#[test]
fn a_space_is_the_runs_own_in_this_project_and_a_label() {
	let (_project, root, _layout) = rooted();
	let (_other, elsewhere, _paths) = rooted();
	let long = format!("{}-0001", "runtime".repeat(9));

	let space = Execution::space(&root, "notes-0001");
	let cut = Execution::space(&root, &long);

	assert!(space.starts_with("notes-0001-"), "was {space}");
	assert_eq!(space.len(), "notes-0001-".len() + 8);
	assert_eq!(space, Execution::space(&root, "notes-0001"));
	assert_ne!(space, Execution::space(&elsewhere, "notes-0001"));
	assert_ne!(space, Execution::space(&root, "notes-0002"));
	assert!(label(&space) && label(&cut), "were {space} and {cut}");
	assert!(cut.len() <= 59, "was {cut}");
}

#[test]
fn a_run_id_that_ends_its_cut_on_a_hyphen_still_makes_a_label() {
	let root = Root {
		path: PathBuf::from("/project"),
	};
	let run = format!("{}-0001", "a".repeat(49));

	let space = Execution::space(&root, &run);

	assert!(label(&space), "was {space}");
	assert!(!space.contains("--"), "was {space}");
}

#[tokio::test]
async fn only_the_unsettled_runs_of_the_runtime_are_left_oldest_first() {
	let (_project, root, layout) = rooted();
	let mut settled =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();
	let later = Execution::open(&root, &layout, "notes-0003", "notes").unwrap();
	let earlier = Execution::open(&root, &layout, "notes-0002", "notes").unwrap();

	std::fs::create_dir_all(layout.runs.join("notes-0004").join("stages"))
		.unwrap();
	settled.cleanup(&root, &layout, &HOST).await.unwrap();

	let left = Execution::left(&root, &layout, "notes").unwrap();

	assert_eq!(left, [earlier, later]);
}

#[test]
fn what_is_not_a_run_of_the_runtime_is_not_read() {
	let (_project, root, layout) = rooted();
	let other = layout.runs.join("other-0001");
	let alike = layout.runs.join("notes-archive");

	std::fs::create_dir_all(&other).unwrap();
	std::fs::create_dir_all(&alike).unwrap();
	std::fs::write(other.join("execution.json"), "{").unwrap();
	std::fs::write(alike.join("execution.json"), "{").unwrap();
	std::fs::write(layout.runs.join(".DS_Store"), "x").unwrap();
	std::fs::write(layout.runs.join("notes-0009"), "a file, not a run").unwrap();

	let left = Execution::left(&root, &layout, "notes").unwrap();

	assert!(left.is_empty());
}

#[test]
fn a_project_with_no_run_has_nothing_left() {
	let (_project, root, layout) = rooted();

	assert!(Execution::left(&root, &layout, "notes").unwrap().is_empty());
}

#[test]
fn a_project_that_moved_still_finds_what_it_left_under_its_old_folder() {
	let (_project, root, layout) = rooted();
	let (_other, elsewhere, _paths) = rooted();
	let ledger = Execution::open(&root, &layout, "notes-0001", "notes").unwrap();
	let moved = Execution {
		space: Execution::space(&elsewhere, "notes-0001"),
		..ledger
	};

	moved.write(&root, &layout).unwrap();

	let left = Execution::left(&root, &layout, "notes").unwrap();

	assert_ne!(moved.space, Execution::space(&root, "notes-0001"));
	assert_eq!(left, [moved]);
}

#[tokio::test]
async fn a_settled_ledger_is_never_a_reason_to_stop() {
	let (_project, root, layout) = rooted();
	let mut ledger =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();

	ledger.cleanup(&root, &layout, &HOST).await.unwrap();

	let odd = Execution {
		space: String::from("kube-system"),
		..ledger
	};

	odd.write(&root, &layout).unwrap();

	assert!(Execution::left(&root, &layout, "notes").unwrap().is_empty());
}

#[rstest]
#[case::another_namespace("kube-system")]
#[case::another_runs_space("notes-0002-0a1b2c3d")]
#[case::a_mark_that_is_no_digest("notes-0001-zzzzzzzz")]
#[case::a_mark_too_short("notes-0001-0a1b2c")]
#[case::the_run_id_alone("notes-0001")]
fn a_ledger_whose_space_is_not_its_runs_is_refused(#[case] space: &str) {
	let (_project, root, layout) = rooted();
	let ledger = Execution::open(&root, &layout, "notes-0001", "notes").unwrap();
	let edited = Execution {
		space: String::from(space),
		..ledger
	};

	edited.write(&root, &layout).unwrap();

	let refusal = Execution::left(&root, &layout, "notes").unwrap_err();

	assert!(
		matches!(refusal, Error::Misplaced { .. }),
		"was {refusal:?}"
	);
}

#[test]
fn a_ledger_that_does_not_read_is_a_stop() {
	let (project, root, layout) = rooted();
	let file = project.path().join(LEDGER);

	std::fs::create_dir_all(file.parent().unwrap()).unwrap();
	std::fs::write(&file, "{").unwrap();

	let refusal = Execution::left(&root, &layout, "notes").unwrap_err();

	assert!(matches!(refusal, Error::Unread { .. }), "was {refusal:?}");
}

#[tokio::test]
async fn a_settled_ledger_marks_the_records_runtime_stopped() {
	let (_project, root, layout) = rooted();
	let mut run = crate::evidence::recorded::recorded();
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();

	execution
		.up(&root, &layout, &HOST, &[deployed("a"), deployed("b")])
		.await
		.unwrap();
	nsx::local::state::State::of(
		&crate::local::applied::applied(),
		&[deployed("a"), deployed("b")],
	)
	.card(&mut run);

	execution.card(&mut run);

	let ready = run.runtime.clone().unwrap();

	execution.cleanup(&root, &layout, &HOST).await.unwrap();
	execution.card(&mut run);

	let stopped = run.runtime.unwrap();

	assert_eq!((ready.state.as_str(), ready.removed), ("ready", None));
	assert_eq!(stopped.state, "stopped");
	assert_eq!((stopped.removed, stopped.left), (Some(2), Some(0)));
}
