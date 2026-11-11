//! Cleanup removes what the execution ledger lists, and nothing else

use nsx::local::{error::Error, execution::Execution};

use crate::local::{deployed::deployed, rooted::rooted};
use crate::simulation::fake::machine::Machine;

#[tokio::test]
async fn after_a_failed_bring_up_nothing_the_run_created_remains() {
	let (_project, root, layout) = rooted();
	let machine = Machine {
		failing: Some("b"),
		..Machine::default()
	};
	let deployments = [deployed("a"), deployed("b"), deployed("c")];
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();
	let space = execution.space.clone();

	machine
		.spaces
		.borrow_mut()
		.insert(String::from("another"), vec![String::from("kept")]);

	let brought = execution
		.up(&root, &layout, &machine, &deployments)
		.await
		.unwrap_err();

	execution.cleanup(&root, &layout, &machine).await.unwrap();

	let spaces = machine.spaces.borrow();
	let written = Execution::left(&root, &layout, "notes").unwrap();

	assert!(matches!(brought, Error::Program(_)), "was {brought:?}");
	assert_eq!(spaces.len(), 1);
	assert_eq!(spaces["another"], ["kept"]);
	assert_eq!(execution.deployments, ["a", "b"]);
	assert_eq!(execution.removed, ["a", "b"]);
	assert!(execution.settled);
	assert!(written.is_empty());
	assert_eq!(
		*machine.asked.borrow(),
		[
			format!("open {space}"),
			format!("apply {space} a"),
			format!("apply {space} b"),
			format!("delete {space} a"),
			format!("delete {space} b"),
			format!("close {space}"),
			format!("absent {space}"),
		]
	);
}

#[tokio::test]
async fn a_settled_ledger_asks_nothing_of_the_cluster() {
	let (_project, root, layout) = rooted();
	let machine = Machine::default();
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();

	execution.cleanup(&root, &layout, &machine).await.unwrap();
	machine.asked.borrow_mut().clear();

	execution.cleanup(&root, &layout, &machine).await.unwrap();

	assert!(machine.asked.borrow().is_empty());
}

#[tokio::test]
async fn a_cleanup_cut_short_goes_on_where_it_stopped() {
	let (_project, root, layout) = rooted();
	let machine = Machine::default();
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();
	let space = execution.space.clone();

	execution
		.up(&root, &layout, &machine, &[deployed("a"), deployed("b")])
		.await
		.unwrap();
	execution.removed.push(String::from("a"));
	machine.asked.borrow_mut().clear();

	execution.cleanup(&root, &layout, &machine).await.unwrap();

	assert_eq!(execution.removed, ["a", "b"]);
	assert_eq!(
		*machine.asked.borrow(),
		[
			format!("delete {space} b"),
			format!("close {space}"),
			format!("absent {space}"),
		]
	);
}

#[tokio::test]
async fn a_deployment_that_will_not_go_does_not_keep_the_space_from_closing() {
	let (_project, root, layout) = rooted();
	let machine = Machine {
		stuck: Some("a"),
		..Machine::default()
	};
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();

	execution
		.up(&root, &layout, &machine, &[deployed("a"), deployed("b")])
		.await
		.unwrap();

	execution.cleanup(&root, &layout, &machine).await.unwrap();

	assert!(machine.spaces.borrow().is_empty());
	assert_eq!(execution.removed, ["a", "b"]);
	assert!(execution.settled);
}

#[tokio::test]
async fn a_space_that_is_still_there_is_said_with_each_refusal_met() {
	let (_project, root, layout) = rooted();
	let machine = Machine {
		stuck: Some("a"),
		clinging: true,
		..Machine::default()
	};
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();

	execution
		.up(&root, &layout, &machine, &[deployed("a"), deployed("b")])
		.await
		.unwrap();

	let refusal = execution
		.cleanup(&root, &layout, &machine)
		.await
		.unwrap_err();
	let left = Execution::left(&root, &layout, "notes").unwrap();

	assert!(matches!(refusal, Error::Left { .. }), "was {refusal:?}");
	assert_eq!(
		refusal.to_string(),
		format!(
			"The runtime's namespace {} is still in the cluster: something of the run is left. kubectl delete ended with the exit status 1.",
			execution.space
		)
	);
	assert!(!execution.settled);
	assert_eq!(execution.removed, ["b"]);
	assert_eq!(left, [execution]);
}

#[tokio::test]
async fn a_cluster_that_cannot_be_asked_is_unconfirmed_and_never_said_left() {
	let (_project, root, layout) = rooted();
	let up = Machine::default();
	let down = Machine {
		unreachable: true,
		..Machine::default()
	};
	let mut execution =
		Execution::open(&root, &layout, "notes-0001", "notes").unwrap();

	execution
		.up(&root, &layout, &up, &[deployed("a"), deployed("b")])
		.await
		.unwrap();

	let refusal = execution.cleanup(&root, &layout, &down).await.unwrap_err();

	assert!(
		matches!(refusal, Error::Unconfirmed { .. }),
		"was {refusal:?}"
	);
	assert_eq!(
		refusal.to_string(),
		format!(
			"nsx could not confirm that the runtime's namespace {} is gone. kind get ended with the exit status 1.",
			execution.space
		)
	);
	assert!(!execution.settled);
	assert!(execution.removed.is_empty());
	assert_eq!(down.asked.borrow().len(), 4);
}
